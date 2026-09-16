"""Compare capped Clippy streams and reconcile selected all-target coverage.

This is an observational aid.  It never treats a capped zero exit as a strict
Clippy pass and leaves acceptance/new-diagnostic attribution to review.
"""
import collections
import difflib
import gzip
import hashlib
import json
from pathlib import Path
import sys


HERE = Path(__file__).resolve().parent
PACKAGE_ROOT = HERE.parent.parent
PRIOR = PACKAGE_ROOT / "artifacts" / "execution-discretion-20260915"
STRICT = PACKAGE_ROOT / "artifacts" / "snowfree-recorder-20260915"
DEFAULT_REFERENCE = Path(
    "/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913"
)
DEFAULT_CANDIDATE = Path(
    "/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915"
)


def load_json(path):
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise SystemExit(f"cannot read JSON {path}: {error}") from error


def text_bytes(path):
    data = path.read_bytes()
    return gzip.decompress(data) if path.suffix == ".gz" else data


def stream(path):
    messages, artifacts, finished, banners = [], [], [], []
    for line in text_bytes(path).decode(errors="replace").splitlines():
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            banners.append(line)
            continue
        reason = value.get("reason")
        if reason == "compiler-message":
            messages.append(value)
        elif reason == "compiler-artifact":
            artifacts.append(value)
        elif reason == "build-finished":
            finished.append(value)
    return {"messages": messages, "artifacts": artifacts, "finished": finished, "banners": banners}


def receipt_for(label):
    path = HERE / f"{label}.json"
    return load_json(path) if path.exists() else {"missing_receipt": str(path)}


def receipt_value(receipt, key):
    if key in receipt:
        return receipt[key]
    return receipt.get("result", {}).get(key)


def command_argv(receipt):
    return receipt_value(receipt, "executed_argv") or receipt_value(receipt, "argv") or []


def source_root(receipt, fallback):
    source = receipt_value(receipt, "source") or receipt_value(receipt, "cwd")
    return Path(source) if isinstance(source, str) else fallback


def find_log(label):
    path = HERE / f"{label}.stdout"
    if not path.exists():
        path = path.with_suffix(".stdout.gz")
    if not path.exists():
        raise SystemExit(f"missing raw structured log for {label}: {path}")
    return path


def source_file(name, root, crate):
    path = Path(name)
    choices = [path] if path.is_absolute() else [root / path, root / "crates" / crate / path]
    for choice in choices:
        if choice.is_file():
            try:
                return choice.relative_to(root).as_posix()
            except ValueError:
                return choice.as_posix()
    return name


def relocation(root, candidate, name, cache):
    if name in cache:
        return cache[name]
    old, new = root / name, candidate / name
    if not old.is_file() or not new.is_file():
        cache[name] = ({}, [], [])
        return cache[name]
    old_lines, new_lines = old.read_bytes().splitlines(keepends=True), new.read_bytes().splitlines(keepends=True)
    mapping = {}
    for block in difflib.SequenceMatcher(a=old_lines, b=new_lines, autojunk=False).get_matching_blocks():
        for offset in range(block.size):
            mapping[block.b + offset + 1] = block.a + offset + 1
    starts_old, starts_new = [0], [0]
    for line in old_lines:
        starts_old.append(starts_old[-1] + len(line))
    for line in new_lines:
        starts_new.append(starts_new[-1] + len(line))
    cache[name] = (mapping, starts_old, starts_new)
    return cache[name]


def normalize(value, root, crate, relocation_root=None, candidate=None, cache=None, unmapped=None):
    if isinstance(value, str):
        return value.replace(str(root), "<SOURCE>")
    if isinstance(value, list):
        return [normalize(item, root, crate, relocation_root, candidate, cache, unmapped) for item in value]
    if not isinstance(value, dict):
        return value
    # Raw rendered text is ANSI/presentation data. The complete structured
    # diagnostic, including children/spans/suggestions, is retained verbatim.
    out = {key: normalize(item, root, crate, relocation_root, candidate, cache, unmapped)
           for key, item in value.items() if key != "rendered"}
    if "file_name" in value and "line_start" in value:
        name = source_file(value["file_name"], root, crate)
        out["file_name"] = name
        if candidate and not Path(name).is_absolute():
            lookup, old_starts, new_starts = relocation(relocation_root, candidate, name, cache)
            first, last = value["line_start"], value["line_end"]
            mapped = [lookup.get(line) for line in range(first, last + 1)]
            contiguous = mapped and all(line is not None for line in mapped) and mapped == list(range(mapped[0], mapped[0] + len(mapped)))
            if contiguous:
                out["line_start"], out["line_end"] = mapped[0], mapped[-1]
                out["byte_start"] = value["byte_start"] + old_starts[mapped[0] - 1] - new_starts[first - 1]
                out["byte_end"] = value["byte_end"] + old_starts[mapped[-1] - 1] - new_starts[last - 1]
            else:
                out["unmapped_candidate_span"] = True
                unmapped.append({"path": name, "line_start": first, "line_end": last})
    return out


def diagnostic_code(message):
    diagnostic = message.get("message", {})
    return (diagnostic.get("code") or {}).get("code") or "<no-lint-code>"


def diagnostic_levels(messages):
    result = collections.Counter()
    for message in messages:
        diagnostic = message.get("message", {})
        result[(diagnostic_code(message), diagnostic.get("level", "<missing-level>"))] += 1
    return [{"lint": lint, "level": level, "multiplicity": count}
            for (lint, level), count in sorted(result.items())]


def compare(package, baseline, candidate, baseline_root, candidate_root):
    crate = "openwepp-hillslope-orchestrator" if package == "orchestrator" else "openwepp-runner"
    unmapped, cache = [], {}
    old = collections.Counter(json.dumps(normalize(item, baseline_root, crate), sort_keys=True)
                              for item in baseline["messages"])
    new = collections.Counter(json.dumps(normalize(item, candidate_root, crate, baseline_root, candidate_root, cache, unmapped), sort_keys=True)
                              for item in candidate["messages"])
    added, removed = new - old, old - new
    return {
        "package": package,
        "baseline_messages": len(baseline["messages"]),
        "candidate_messages": len(candidate["messages"]),
        "matched_messages": sum((old & new).values()),
        "candidate_unmatched": [{"multiplicity": count, "message": json.loads(value)} for value, count in added.items()],
        "baseline_unmatched": [{"multiplicity": count, "message": json.loads(value)} for value, count in removed.items()],
        "candidate_unmapped_spans": unmapped,
        "baseline_levels": diagnostic_levels(baseline["messages"]),
        "candidate_levels": diagnostic_levels(candidate["messages"]),
    }


def artifact_key(artifact):
    target = artifact.get("target", {})
    profile = artifact.get("profile", {})
    return (target.get("name"), tuple(sorted(target.get("kind", []))), profile.get("test"))


def artifacts_summary(items):
    seen = collections.Counter(artifact_key(item) for item in items)
    return [{"target": name, "kind": list(kind), "profile_test": profile_test, "multiplicity": count}
            for (name, kind, profile_test), count in sorted(seen.items(), key=lambda item: str(item[0]))]


def target_coverage(package, inventory, data):
    expected = inventory["inventory"]["openwepp-hillslope-orchestrator" if package == "orchestrator" else "openwepp-runner"]
    rows = []
    for item in expected:
        key = (item["name"], tuple(sorted(item["kind"])))
        observations = [entry for entry in artifacts_summary(data["artifacts"])
                        if entry["target"] == key[0] and tuple(entry["kind"]) == key[1]]
        kind = item["kind"][0]
        required_profiles = {"lib": [False, True], "bin": [False, True], "test": [True], "example": [False]}[kind]
        observed_profiles = {entry["profile_test"] for entry in observations}
        rows.append({"target": item["name"], "kind": item["kind"], "metadata_test": item["test"],
                     "required_profile_test": required_profiles, "artifact_observed": bool(observations),
                     "artifact_profiles": observations,
                     "missing_profile_test": [profile for profile in required_profiles if profile not in observed_profiles],
                     "complete": all(profile in observed_profiles for profile in required_profiles)})
    return rows


def command_custody(label, receipt, package):
    argv = command_argv(receipt)
    joined = "\u0000".join(argv)
    required = ["--all-targets", "--locked", "--no-deps", "-D", "warnings", "--cap-lints", "warn"]
    expected_package = "openwepp-hillslope-orchestrator" if package == "orchestrator" else "openwepp-runner"
    issues = [item for item in required if item not in argv]
    if expected_package not in argv:
        issues.append(f"package:{expected_package}")
    if "--cap-lints\u0000allow" in joined or "-A\u0000warnings" in joined:
        issues.append("prohibited lint suppression observed")
    source = receipt_value(receipt, "source") or receipt_value(receipt, "cwd")
    before = receipt_value(receipt, "source_before") or {}
    return {"label": label, "source": source, "source_tree_sha256": receipt_value(receipt, "source_tree_sha256") or before.get("tree_sha256"),
            "exit": receipt_value(receipt, "exit"), "argv": argv, "issues": issues,
            "receipt_present": "missing_receipt" not in receipt}


def strict_classes(package):
    result = {}
    for side in ("baseline", "candidate"):
        candidates = ([PRIOR / f"candidate-{package}-clippy-final.stdout.gz"] if side == "candidate" else []) + [
            STRICT / f"{side}-{package}-clippy-final.stdout.gz", STRICT / f"{side}-{package}-clippy.stdout.gz"
        ]
        log = next((path for path in candidates if path.exists()), None)
        if log is None:
            result[side] = {"available": False, "reason": "prior strict raw log absent"}
            continue
        data = stream(log)
        counts = collections.Counter(diagnostic_code(message) for message in data["messages"])
        result[side] = {"available": True, "log": str(log.relative_to(PACKAGE_ROOT)),
                        "log_sha256": hashlib.sha256(text_bytes(log)).hexdigest(), "classes": dict(sorted(counts.items()))}
    return result


def lint_classes(messages):
    return set(diagnostic_code(message) for message in messages)


def main():
    inventory = load_json(PRIOR / "final-lint-target-coverage.json")
    comparisons, coverage, custody = {}, {}, {}
    for package in ("orchestrator", "runner"):
        base_label, candidate_label = f"baseline-{package}", f"candidate-{package}"
        base_receipt, candidate_receipt = receipt_for(base_label), receipt_for(candidate_label)
        base_data, candidate_data = stream(find_log(base_label)), stream(find_log(candidate_label))
        base_root, candidate_root = source_root(base_receipt, DEFAULT_REFERENCE), source_root(candidate_receipt, DEFAULT_CANDIDATE)
        comparisons[package] = compare(package, base_data, candidate_data, base_root, candidate_root)
        custody[base_label] = command_custody(base_label, base_receipt, package)
        custody[candidate_label] = command_custody(candidate_label, candidate_receipt, package)
        base_rows, candidate_rows = target_coverage(package, inventory, base_data), target_coverage(package, inventory, candidate_data)
        base_finished = any(record.get("success") is True for record in base_data["finished"])
        candidate_finished = any(record.get("success") is True for record in candidate_data["finished"])
        coverage[package] = {"baseline": base_rows, "candidate": candidate_rows,
                             "baseline_build_finished": base_data["finished"], "candidate_build_finished": candidate_data["finished"],
                             "baseline_selected_artifacts_complete": base_finished and all(row["complete"] for row in base_rows),
                             "candidate_selected_artifacts_complete": candidate_finished and all(row["complete"] for row in candidate_rows),
                             "baseline_artifacts": artifacts_summary(base_data["artifacts"]),
                             "candidate_artifacts": artifacts_summary(candidate_data["artifacts"])}
        strict = strict_classes(package)
        capped = {"baseline": lint_classes(base_data["messages"]), "candidate": lint_classes(candidate_data["messages"])}
        comparisons[package]["strict_lint_class_representation"] = {
            "strict": strict,
            "capped_missing_prior_strict_classes": {
                side: sorted(set(strict[side].get("classes", {})) - capped[side]) for side in ("baseline", "candidate")
            },
            "note": "Severity/presentation may differ because --cap-lints warn changes compiler lint level. Class absence is retained for manual explanation; it is not a strict-pass claim.",
        }
    comparison_output = {
        "evidence_class": "Ran: deterministic processing of raw capped Cargo JSON streams; manual review assigns diagnostic attribution and acceptance.",
        "method_limits": "Cargo compiler messages do not identify feature/profile command identity. Receipts establish command/source custody; compiler-artifact records establish target and profile.test observations. Raw streams retain rendered presentation.",
        "logs": {label: {"path": str(find_log(label).relative_to(PACKAGE_ROOT)), "sha256": hashlib.sha256(text_bytes(find_log(label))).hexdigest()}
                 for label in ("baseline-orchestrator", "candidate-orchestrator", "baseline-runner", "candidate-runner")},
        "command_custody": custody, "comparisons": comparisons,
        "disposition": "OBSERVATIONAL: unmatched complete structured diagnostics and unmapped relocation spans require manual review. Capped collection does not replace retained strict Clippy FAIL.",
    }
    coverage_output = {
        "evidence_class": "Ran: raw compiler-artifact and build-finished reconciliation against prior final selected-target inventory.",
        "prior_inventory": str((PRIOR / "final-lint-target-coverage.json").relative_to(PACKAGE_ROOT)),
        "command_custody": custody, "coverage": coverage,
        "limits": "A compiler-artifact profile contains profile.test but not complete Cargo feature/profile identity. Missing artifact, missing/false build-finished, or invalid receipt remains incomplete coverage; target presence alone is not a quality acceptance.",
        "disposition": "OBSERVATIONAL: reviewer/orchestrator must assess receipt custody, every selected target/configuration, and any missing or failed build completion.",
    }
    (HERE / "complete-lint-diagnostic-comparison.json").write_text(json.dumps(comparison_output, indent=2) + "\n")
    (HERE / "complete-lint-target-coverage.json").write_text(json.dumps(coverage_output, indent=2) + "\n")
    print(json.dumps({package: {key: comparisons[package][key] for key in ("baseline_messages", "candidate_messages", "matched_messages")}
                      for package in comparisons}, sort_keys=True))


if __name__ == "__main__":
    main()
