#!/usr/bin/env python3
"""Render a repository's commit history into a static, replayable HTML animation."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path

SCHEMA_VERSION = 1
SUBJECT_MAX = 72
COMMIT_PREFIX = "C|"
LOG_FORMAT = COMMIT_PREFIX + "%H|%at|%s"

CONFIG_KEYS = {
    "title",
    "repo",
    "tz",
    "labelStrip",
    "excludeFromLanes",
    "groups",
    "counters",
}
GROUP_KEYS = {"key", "label", "lanes", "autoDiscover", "sharedLaneLabel"}
LANE_KEYS = {"key", "label", "paths"}
COUNTER_KEYS = {"key", "label", "kind", "path", "exclude"}

# Validated against surface #0b0c10 in dark mode; see specification.md section 8.
GROUP_COLORS = {"crates": "#d4794a", "docs": "#6a93e0"}
GROUP_COLOR_FALLBACK = "#6a93e0"

# Matches the numstat rename forms: "dir/{old => new}/f", "{a/p => b/p}",
# and the empty-side variants "dir/{ => sub}/f" and "dir/{sub => }/f".
RENAME_RE = re.compile(r"\{([^{}]*) => ([^{}]*)\}")


class ConfigError(Exception):
    """Raised when the config file violates the specification."""


# --------------------------------------------------------------------------
# config
# --------------------------------------------------------------------------


def _reject_unknown(obj, allowed, where):
    unknown = sorted(set(obj) - allowed)
    if unknown:
        raise ConfigError(f"{where}: unknown key(s) {', '.join(unknown)}")


def load_config(path: Path) -> dict:
    """Load and fail-closed validate the lane/group/counter config."""
    try:
        cfg = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ConfigError(f"{path}: invalid JSON: {exc}") from exc

    _reject_unknown(cfg, CONFIG_KEYS, str(path))
    for required in ("title", "tz", "groups"):
        if not cfg.get(required):
            raise ConfigError(f"{path}: missing required key '{required}'")
    if not isinstance(cfg["groups"], list):
        raise ConfigError(f"{path}: 'groups' must be a list")

    seen_groups: set[str] = set()
    seen_lanes: set[str] = set()
    for group in cfg["groups"]:
        _reject_unknown(group, GROUP_KEYS, f"{path}: group")
        for required in ("key", "label"):
            if not group.get(required):
                raise ConfigError(f"{path}: group missing required key '{required}'")
        if group["key"] in seen_groups:
            raise ConfigError(f"{path}: duplicate group key '{group['key']}'")
        seen_groups.add(group["key"])
        if not group.get("lanes") and not group.get("autoDiscover"):
            raise ConfigError(
                f"{path}: group '{group['key']}' declares neither 'lanes' nor 'autoDiscover'"
            )
        for lane in group.get("lanes", []):
            _reject_unknown(lane, LANE_KEYS, f"{path}: lane")
            for required in ("key", "label", "paths"):
                if not lane.get(required):
                    raise ConfigError(
                        f"{path}: lane in group '{group['key']}' missing '{required}'"
                    )
            if lane["key"] in seen_lanes:
                raise ConfigError(f"{path}: duplicate lane key '{lane['key']}'")
            seen_lanes.add(lane["key"])
        auto = group.get("autoDiscover")
        if auto is not None:
            if auto.get("kind") != "dir-children":
                raise ConfigError(
                    f"{path}: group '{group['key']}' autoDiscover.kind must be 'dir-children'"
                )
            if not auto.get("path", "").endswith("/"):
                raise ConfigError(
                    f"{path}: group '{group['key']}' autoDiscover.path must end with '/'"
                )

    seen_counters: set[str] = set()
    for counter in cfg.get("counters", []):
        _reject_unknown(counter, COUNTER_KEYS, f"{path}: counter")
        for required in ("key", "label", "kind", "path"):
            if not counter.get(required):
                raise ConfigError(f"{path}: counter missing required key '{required}'")
        if counter["kind"] != "dir-count":
            raise ConfigError(f"{path}: counter kind must be 'dir-count'")
        if not counter["path"].endswith("/"):
            raise ConfigError(f"{path}: counter path must end with '/'")
        if counter["key"] in seen_counters:
            raise ConfigError(f"{path}: duplicate counter key '{counter['key']}'")
        seen_counters.add(counter["key"])

    return cfg


# --------------------------------------------------------------------------
# git
# --------------------------------------------------------------------------


def git(repo: Path, *args: str) -> str:
    proc = subprocess.run(
        ["git", "-C", str(repo), *args],
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        sys.exit(f"git {' '.join(args)} failed: {proc.stderr.strip()}")
    return proc.stdout


def normalize_path(path: str) -> str:
    """Resolve numstat rename brace forms to the post-rename path."""
    if "{" not in path:
        return path
    resolved = RENAME_RE.sub(lambda m: m.group(2), path)
    return resolved.replace("//", "/").strip("/")


def read_history(repo: Path, rev: str):
    """Yield (sha, timestamp, subject, [(add, del, path), ...]) oldest-first."""
    raw = git(
        repo,
        "log",
        "--no-merges",
        "--numstat",
        f"--format={LOG_FORMAT}",
        rev,
    )
    commits = []
    current = None
    for line in raw.splitlines():
        if line.startswith(COMMIT_PREFIX):
            if current is not None:
                commits.append(current)
            _, sha, stamp, subject = line.split("|", 3)
            current = (sha, int(stamp), subject, [])
            continue
        if not line.strip() or current is None:
            continue
        fields = line.split("\t")
        if len(fields) != 3:
            continue
        added, deleted, path = fields
        current[3].append(
            (
                0 if added == "-" else int(added),
                0 if deleted == "-" else int(deleted),
                normalize_path(path),
            )
        )
    if current is not None:
        commits.append(current)
    commits.reverse()  # git log is newest-first; the sweep runs oldest-first
    return commits


# --------------------------------------------------------------------------
# lanes
# --------------------------------------------------------------------------


class Lane:
    __slots__ = ("key", "label", "group", "paths", "shared", "total", "index")

    def __init__(self, key, label, group, paths, shared=False):
        self.key = key
        self.label = label
        self.group = group
        self.paths = tuple(paths)
        self.shared = shared
        self.total = 0
        self.index = -1


def strip_label(name: str, prefixes) -> str:
    for prefix in sorted(prefixes or [], key=len, reverse=True):
        if name.startswith(prefix):
            return name[len(prefix) :]
    return name


def discover_children(commits, root: str, excludes=()) -> set[str]:
    """Immediate child directory names of `root` seen anywhere in history.

    Excluded prefixes are skipped here rather than left to produce an empty
    lane that the zero-value filter happens to drop later.
    """
    found = set()
    for _, _, _, files in commits:
        for _, _, path in files:
            if not path.startswith(root):
                continue
            if any(path.startswith(prefix) for prefix in excludes):
                continue
            tail = path[len(root) :]
            if "/" in tail:
                found.add(tail.split("/", 1)[0])
    return found


def build_lanes(cfg, commits):
    """Resolve config plus history into ordered Lane objects, grouped."""
    strip = cfg.get("labelStrip", [])
    excludes = tuple(cfg.get("excludeFromLanes", []))
    groups = []
    for group in cfg["groups"]:
        lanes = [
            Lane(lane["key"], lane["label"], group["key"], lane["paths"])
            for lane in group.get("lanes", [])
        ]
        auto = group.get("autoDiscover")
        if auto:
            root = auto["path"]
            for name in sorted(discover_children(commits, root, excludes)):
                lanes.append(
                    Lane(name, strip_label(name, strip), group["key"], [root + name + "/"])
                )
            if group.get("sharedLaneLabel"):
                lanes.append(
                    Lane(
                        f"{group['key']}__shared",
                        group["sharedLaneLabel"],
                        group["key"],
                        [root],
                        shared=True,
                    )
                )
        groups.append({"key": group["key"], "label": group["label"], "lanes": lanes})
    return groups


def match_lane(path: str, lanes, excludes) -> Lane | None:
    for prefix in excludes:
        if path.startswith(prefix):
            return None
    for lane in lanes:
        for prefix in lane.paths:
            if path.startswith(prefix):
                return lane
    return None


# --------------------------------------------------------------------------
# accumulation
# --------------------------------------------------------------------------


def build_dataset(cfg, commits, repo: Path, rev: str, generated_at: str):
    groups = build_lanes(cfg, commits)
    excludes = tuple(cfg.get("excludeFromLanes", []))
    counters = cfg.get("counters", [])

    # Declaration order decides matching, except that a residual lane rooted at
    # the discovery path must lose to its own child directories. Python's sort
    # is stable, so declaration order survives within each tier.
    ordered = sorted(
        (lane for group in groups for lane in group["lanes"]),
        key=lambda lane: lane.shared,
    )

    # First pass: totals, so lanes can be ordered and zero lanes dropped.
    for _, _, _, files in commits:
        for added, deleted, path in files:
            lane = match_lane(path, ordered, excludes)
            if lane is not None:
                lane.total += added - deleted

    dropped = []
    for group in groups:
        keep, drop = [], []
        for lane in group["lanes"]:
            (keep if lane.total > 0 else drop).append(lane)
        keep.sort(key=lambda lane: (-lane.total, lane.key))
        group["lanes"] = keep
        dropped.extend(drop)

    live = [lane for group in groups for lane in group["lanes"]]
    for index, lane in enumerate(live):
        lane.index = index
    live_set = set(id(lane) for lane in live)
    group_ordinal = {group["key"]: i for i, group in enumerate(groups)}
    group_of = [group_ordinal[lane.group] for lane in live]

    seen_dirs = [set() for _ in counters]
    counter_totals = [0] * len(counters)
    total_add = total_del = 0
    t0 = commits[0][1] * 1000 if commits else 0
    rows = []
    running = [0] * len(live)
    peaks = [0] * len(live)
    group_running = [0] * len(groups)
    group_peaks = [0] * len(groups)

    for _, stamp, subject, files in commits:
        lane_delta: dict[int, int] = {}
        counter_delta = [0] * len(counters)
        commit_add = commit_del = 0
        for added, deleted, path in files:
            commit_add += added
            commit_del += deleted
            lane = match_lane(path, ordered, excludes)
            if lane is not None and id(lane) in live_set:
                lane_delta[lane.index] = lane_delta.get(lane.index, 0) + added - deleted
            for i, counter in enumerate(counters):
                root = counter["path"]
                if not path.startswith(root):
                    continue
                tail = path[len(root) :]
                if "/" not in tail:
                    continue
                name = tail.split("/", 1)[0]
                if name in counter.get("exclude", []) or name in seen_dirs[i]:
                    continue
                seen_dirs[i].add(name)
                counter_delta[i] += 1
        total_add += commit_add
        total_del += commit_del
        for i, value in enumerate(counter_delta):
            counter_totals[i] += value
        touched_groups = set()
        for index, value in lane_delta.items():
            running[index] += value
            if running[index] > peaks[index]:
                peaks[index] = running[index]
            group_running[group_of[index]] += value
            touched_groups.add(group_of[index])
        for g in touched_groups:
            if group_running[g] > group_peaks[g]:
                group_peaks[g] = group_running[g]
        rows.append(
            [
                stamp * 1000 - t0,
                commit_add,
                commit_del,
                counter_delta,
                sorted([index, value] for index, value in lane_delta.items() if value),
                truncate(subject),
            ]
        )

    head = git(repo, "rev-parse", rev).strip()
    head_date = git(repo, "log", "-1", "--format=%ad", "--date=iso-strict", rev).strip()
    first_date = git(
        repo, "log", "-1", "--format=%ad", "--date=iso-strict", commits[0][0]
    ).strip()
    branch = git(repo, "rev-parse", "--abbrev-ref", "HEAD").strip()

    meta = {
        "title": cfg["title"],
        "repo": cfg.get("repo", ""),
        "branch": branch,
        "head": head,
        "headShort": head[:8],
        "headDate": head_date,
        "firstDate": first_date,
        "generatedAt": generated_at,
        "rev": rev,
        "commitCount": len(commits),
        "mergesExcluded": True,
        "t0": t0,
        "spanMs": max((row[0] for row in rows), default=0),
        "tz": cfg["tz"],
        "totals": {"add": total_add, "del": total_del},
        "groups": [],
        "lanes": [],
        "counters": [
            {"key": c["key"], "label": c["label"], "head": counter_totals[i]}
            for i, c in enumerate(counters)
        ],
    }
    cursor = 0
    for ordinal, group in enumerate(groups):
        span = len(group["lanes"])
        meta["groups"].append(
            {
                "key": group["key"],
                "label": group["label"],
                "laneRange": [cursor, cursor + span],
                "color": GROUP_COLORS.get(group["key"], GROUP_COLOR_FALLBACK),
                "head": group_running[ordinal],
                "peak": group_peaks[ordinal],
            }
        )
        cursor += span
        for lane in group["lanes"]:
            meta["lanes"].append(
                {
                    "key": lane.key,
                    "label": lane.label,
                    "group": group["key"],
                    "head": lane.total,
                    "peak": peaks[lane.index],
                }
            )

    return {"schemaVersion": SCHEMA_VERSION, "meta": meta, "commits": rows}, dropped


def truncate(subject: str) -> str:
    subject = subject.strip()
    if len(subject) <= SUBJECT_MAX:
        return subject
    return subject[: SUBJECT_MAX - 1].rstrip() + "…"


# --------------------------------------------------------------------------
# rendering
# --------------------------------------------------------------------------


def js_string(payload: str) -> str:
    """Escape a JSON payload for a single-quoted JS literal inside <script>."""
    return (
        payload.replace("\\", "\\\\")
        .replace("'", "\\'")
        .replace("<", "\\x3c")
        .replace(" ", "\\u2028")
        .replace(" ", "\\u2029")
    )


def render(template: str, dataset: dict, section_id: str) -> str:
    meta = dataset["meta"]
    aria = (
        f"Animated replay of {meta['commitCount']:,} commits in {meta['title']}, "
        f"from {meta['firstDate'][:10]} to {meta['headDate'][:10]}, showing net "
        f"lines added per crate and documentation category as of commit "
        f"{meta['headShort']}."
    )
    payload = json.dumps(dataset, separators=(",", ":"), ensure_ascii=False)
    return (
        template.replace("{{SECTION_ID}}", section_id)
        .replace("{{TITLE}}", meta["title"])
        .replace("{{ARIA_LABEL}}", aria)
        .replace("{{DATA_JSON}}", js_string(payload))
    )


def wrap_document(fragment: str, title: str) -> str:
    return (
        "<!doctype html>\n"
        '<html lang="en">\n<head>\n<meta charset="utf-8">\n'
        '<meta name="viewport" content="width=device-width, initial-scale=1">\n'
        f"<title>{title} · commit timeline</title>\n"
        "<style>\n"
        "  html { color-scheme: dark; }\n"
        "  body { margin: 0; padding: 2rem 1rem; background: #07080b;\n"
        "         font-family: ui-sans-serif, system-ui, -apple-system, sans-serif; }\n"
        "  main { max-width: 76rem; margin: 0 auto; }\n"
        "</style>\n</head>\n<body>\n<main>\n"
        f"{fragment}\n"
        "</main>\n</body>\n</html>\n"
    )


# --------------------------------------------------------------------------
# reporting
# --------------------------------------------------------------------------


def report(dataset: dict, dropped, stream=sys.stdout) -> None:
    meta = dataset["meta"]
    out = stream.write
    out(f"\n{meta['title']} commit timeline — {meta['headShort']} ({meta['rev']})\n")
    out(f"  commits      {meta['commitCount']:,} (merges excluded)\n")
    out(f"  range        {meta['firstDate'][:10]} → {meta['headDate'][:10]}\n")
    out(f"  lines        +{meta['totals']['add']:,}  -{meta['totals']['del']:,}\n")
    for counter in meta["counters"]:
        out(f"  {counter['label']:<12} {counter['head']:,}\n")
    for group in meta["groups"]:
        start, end = group["laneRange"]
        lanes = meta["lanes"][start:end]
        out(f"\n  {group['label']} — {len(lanes)} lanes\n")
        for lane in lanes:
            # A peak above the snapshot value means the lane shrank later; the
            # bar's denominator choice is visible there. See specification 4.2.
            shrank = (
                f"   peaked {lane['peak']:,} (-{lane['peak'] - lane['head']:,} since)"
                if lane["peak"] > lane["head"]
                else ""
            )
            out(f"    {lane['label']:<40} {lane['head']:>9,}{shrank}\n")
        out(f"    {'total':<40} {group['head']:>9,}\n")
    if dropped:
        out("\n  omitted (net <= 0):\n")
        for lane in dropped:
            out(f"    {lane.key:<40} {lane.total:>9,}\n")
    out("\n")


# --------------------------------------------------------------------------
# cli
# --------------------------------------------------------------------------


def strip_generated_at(dataset: dict) -> dict:
    clone = json.loads(json.dumps(dataset))
    clone["meta"]["generatedAt"] = ""
    return clone


def main(argv=None) -> int:
    here = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=here.parent.parent)
    parser.add_argument("--config", type=Path, default=here / "config" / "openwepp.json")
    parser.add_argument("--rev", default="HEAD")
    parser.add_argument("--template", type=Path, default=here / "template.html")
    parser.add_argument("--out-json", type=Path, default=here / "data" / "commit-timeline.json")
    parser.add_argument(
        "--out-html", type=Path, default=here / "dist" / "commit-timeline.html"
    )
    parser.add_argument(
        "--out-fragment",
        type=Path,
        default=here / "dist" / "commit-timeline.fragment.html",
    )
    parser.add_argument("--section-id", default="owct")
    parser.add_argument("--generated-at", default=None)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--quiet", action="store_true")
    args = parser.parse_args(argv)

    try:
        cfg = load_config(args.config)
    except ConfigError as exc:
        sys.exit(f"config error: {exc}")

    generated_at = args.generated_at or (
        "" if args.check else dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
    )
    commits = read_history(args.repo, args.rev)
    if not commits:
        sys.exit(f"no commits found for rev '{args.rev}'")
    dataset, dropped = build_dataset(cfg, commits, args.repo, args.rev, generated_at)

    template = args.template.read_text(encoding="utf-8")
    fragment = render(template, dataset, args.section_id)
    document = wrap_document(fragment, dataset["meta"]["title"])
    payload = json.dumps(dataset, indent=None, separators=(",", ":"), ensure_ascii=False) + "\n"

    if args.check:
        drift = []
        for path, expected in (
            (args.out_json, payload),
            (args.out_html, document),
            (args.out_fragment, fragment),
        ):
            if not path.exists():
                drift.append(f"{path} is missing")
                continue
            actual = path.read_text(encoding="utf-8")
            if path == args.out_json:
                try:
                    same = strip_generated_at(json.loads(actual)) == strip_generated_at(dataset)
                except json.JSONDecodeError:
                    same = False
            else:
                same = _ignore_stamp(actual) == _ignore_stamp(expected)
            if not same:
                drift.append(f"{path} is out of date")
        if drift:
            for line in drift:
                print(f"drift: {line}", file=sys.stderr)
            print("run gen_timeline.py to refresh generated outputs", file=sys.stderr)
            return 1
        if not args.quiet:
            print("commit-timeline outputs are current")
        return 0

    for path, content in (
        (args.out_json, payload),
        (args.out_html, document),
        (args.out_fragment, fragment),
    ):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    if not args.quiet:
        report(dataset, dropped)
        for path in (args.out_json, args.out_html, args.out_fragment):
            print(f"  wrote {path.relative_to(args.repo)} ({path.stat().st_size:,} bytes)")
        print()
    return 0


_STAMP_RE = re.compile(r'\\?"generatedAt\\?":\\?"[^"\\]*\\?"')


def _ignore_stamp(text: str) -> str:
    return _STAMP_RE.sub("generatedAt", text)


if __name__ == "__main__":
    raise SystemExit(main())
