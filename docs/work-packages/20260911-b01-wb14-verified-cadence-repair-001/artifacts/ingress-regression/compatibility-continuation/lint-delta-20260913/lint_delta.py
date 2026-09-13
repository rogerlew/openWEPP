#!/usr/bin/env python3
"""Conservative Cargo JSON lint-delta attribution for B01-WB14.

This is an evidence aid.  It intentionally reports INCOMPLETE rather than
inferring inheritance when Cargo's per-target diagnostic coverage is absent.
"""
from __future__ import annotations

import argparse
import difflib
import hashlib
import json
import os
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_path(raw: str, source: Path) -> Path | None:
    """Resolve rustc paths emitted from either workspace or crate cwd."""
    path = Path(raw)
    candidates = [path] if path.is_absolute() else [source / path,
                                                       source / "crates/openwepp-hillslope-orchestrator" / path]
    for candidate in candidates:
        if candidate.is_file():
            return candidate.resolve()
    return None


def canonical_path(raw: str, source: Path) -> str:
    path = source_path(raw, source)
    if path is None:
        return raw
    try:
        return str(path.relative_to(source.resolve()))
    except ValueError:
        return str(path)


def parse_jsonl(path: Path) -> tuple[list[dict], list[dict], list[str]]:
    """Keep non-JSON banner lines; only line-delimited Cargo JSON is accepted."""
    records, malformed, banners = [], [], []
    for number, line in enumerate(path.read_text(errors="replace").splitlines(), 1):
        stripped = line.strip()
        if not stripped:
            continue
        try:
            value = json.loads(stripped)
        except json.JSONDecodeError:
            banners.append(line)
            continue
        if not isinstance(value, dict) or "reason" not in value:
            malformed.append({"line": number, "value": value})
        else:
            records.append(value)
    return records, malformed, banners


def target_descriptor(target: dict, source: Path) -> tuple:
    """Cargo does not attach profile.test to failed compiler-message records."""
    return (target.get("name"), tuple(target.get("kind", [])), tuple(target.get("crate_types", [])),
            canonical_path(str(target.get("src_path", "")), source), target.get("edition"),
            target.get("doc"), target.get("doctest"), target.get("test"))


def flatten(message: dict) -> list[dict]:
    out = [message]
    for child in message.get("children", []) or []:
        if isinstance(child, dict):
            out.extend(flatten(child))
    return out


def span_text(source: Path, span: dict) -> str | None:
    name = span.get("file_name")
    if not isinstance(name, str):
        return None
    path = source_path(name, source)
    if path is None:
        # Macro-expansion children can point into the pinned Rust sysroot. Their
        # JSON carries the rendered source text; preserve it without pretending
        # it is candidate-owned source that must map through the workspace diff.
        rendered = span.get("text")
        if isinstance(rendered, list):
            return "".join(item.get("text", "") for item in rendered if isinstance(item, dict))
        return None
    try:
        lines = path.read_text(errors="replace").splitlines(keepends=True)
        start, end = int(span["line_start"]), int(span["line_end"])
        if start < 1 or end < start or end > len(lines):
            return None
        return "".join(lines[start - 1:end])
    except (OSError, KeyError, TypeError, ValueError):
        return None


def diagnostics(records: list[dict], source: Path, label: str) -> tuple[list[dict], list[str]]:
    errors: list[str] = []
    rows: list[dict] = []
    for item in records:
        if item.get("reason") != "compiler-message" or not isinstance(item.get("message"), dict):
            continue
        message, target = item["message"], item.get("target")
        if not isinstance(target, dict):
            errors.append(f"{label}: compiler-message lacks target")
            continue
        # Only warnings/errors matter; a child is retained with its parent below.
        code = message.get("code")
        code_value = code.get("code") if isinstance(code, dict) else None
        level = message.get("level")
        if level not in {"warning", "error"}:
            continue
        # rustc lints such as `dead_code` are bare names, while real compiler
        # errors carry E-numbers.  Both Clippy and rustc lints are valid here.
        if not (isinstance(code_value, str) and not re.fullmatch(r"E\d{4}", code_value)):
            errors.append(f"{label}: non-lint {level} diagnostic {code_value!r}: {message.get('message')!r}")
            continue
        all_nodes = flatten(message)
        spans = []
        for node in all_nodes:
            for span in node.get("spans", []) or []:
                if isinstance(span, dict) and isinstance(span.get("file_name"), str):
                    text = span_text(source, span)
                    if text is None and not Path(canonical_path(span["file_name"], source)).is_absolute():
                        errors.append(f"{label}: unreadable span {span.get('file_name')}:{span.get('line_start')}-{span.get('line_end')}")
                    spans.append({
                        "path": canonical_path(span["file_name"], source),
                        "line_start": span.get("line_start"), "line_end": span.get("line_end"),
                        "is_primary": bool(span.get("is_primary")), "text_sha256": hashlib.sha256((text or "").encode()).hexdigest(),
                        "text_present": text is not None,
                    })
        if not spans:
            errors.append(f"{label}: lint diagnostic has no source span: {code_value}")
        rows.append({"target": {"descriptor": target_descriptor(target, source), "profile_test": "UNAVAILABLE"},
                     "code": code_value, "message": message.get("message"), "level": level, "spans": spans})
    return rows, errors


def normalized_compiler_messages(records: list[dict], source: Path) -> Counter:
    """Preserve a full-message equality check while normalizing only source root."""
    root = str(source.resolve())
    def visit(value):
        if isinstance(value, dict):
            return {key: visit(child) for key, child in value.items()}
        if isinstance(value, list):
            return [visit(child) for child in value]
        if isinstance(value, str):
            return value.replace(root, "<SOURCE>")
        return value
    return Counter(json.dumps(visit(item), sort_keys=True, separators=(",", ":"))
                   for item in records if item.get("reason") == "compiler-message")


def line_map(reference: Path, target: Path, names: set[str]) -> tuple[dict[tuple[str, int], int], dict[str, set[int]]]:
    """Map only exact equal line blocks; unmatched target lines are changed."""
    mapping, changed = {}, defaultdict(set)
    for name in names:
        old, new = reference / name, target / name
        old_lines = old.read_text(errors="replace").splitlines() if old.is_file() else []
        new_lines = new.read_text(errors="replace").splitlines() if new.is_file() else []
        for block in difflib.SequenceMatcher(a=old_lines, b=new_lines, autojunk=False).get_opcodes():
            tag, a0, a1, b0, b1 = block
            if tag == "equal":
                for offset in range(b1 - b0):
                    mapping[(name, b0 + offset + 1)] = a0 + offset + 1
            else:
                changed[name].update(range(b0 + 1, b1 + 1))
    return mapping, changed


def mapped_spans(row: dict, mapping: dict[tuple[str, int], int]) -> tuple[tuple, ...] | None:
    out = []
    for span in row["spans"]:
        if Path(span["path"]).is_absolute():
            out.append((span["path"], span["line_start"], span["line_end"], span["is_primary"], span["text_sha256"]))
            continue
        try:
            start, end = int(span["line_start"]), int(span["line_end"])
        except (TypeError, ValueError):
            return None
        mapped = [mapping.get((span["path"], n)) for n in range(start, end + 1)]
        if not mapped or any(n is None for n in mapped) or mapped != list(range(mapped[0], mapped[0] + len(mapped))):
            return None
        out.append((span["path"], mapped[0], mapped[-1], span["is_primary"], span["text_sha256"]))
    return tuple(sorted(out))


def key(row: dict, spans: tuple) -> tuple:
    t = row["target"]
    return (tuple(t["descriptor"]), row["code"], spans)


def touches_changed(row: dict, changed: dict[str, set[int]], compatibility: set[str]) -> bool:
    for span in row["spans"]:
        if span["path"] not in compatibility:
            continue
        try:
            if any(n in changed[span["path"]] for n in range(int(span["line_start"]), int(span["line_end"]) + 1)):
                return True
        except (TypeError, ValueError):
            return True
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference-log", type=Path, required=True)
    parser.add_argument("--target-log", type=Path, required=True)
    parser.add_argument("--reference-record", type=Path, required=True, help="parent command record with exit and stderr_path")
    parser.add_argument("--target-record", type=Path, required=True, help="parent command record with exit and stderr_path")
    parser.add_argument("--reference-source", type=Path, required=True)
    parser.add_argument("--target-source", type=Path, required=True)
    parser.add_argument("--compatibility-base-source", type=Path, required=True,
                        help="immutable available145 source before the shared enum alternatives")
    parser.add_argument("--compatibility-path", action="append", default=[], help="repo-relative path of each shared enum-alternative edit")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    problems: list[str] = []
    for path in (args.reference_log, args.target_log, args.reference_record, args.target_record):
        if not path.is_file(): problems.append(f"missing raw log: {path}")
    for path in (args.reference_source, args.target_source, args.compatibility_base_source):
        if not path.is_dir(): problems.append(f"missing source directory: {path}")
    if problems:
        args.output.write_text(json.dumps({"status": "INCOMPLETE", "problems": problems}, indent=2) + "\n")
        return 2
    ref_records, ref_malformed, ref_banners = parse_jsonl(args.reference_log)
    tgt_records, tgt_malformed, tgt_banners = parse_jsonl(args.target_log)
    if ref_malformed or tgt_malformed or not ref_records or not tgt_records:
        problems.append("raw logs do not provide complete line-delimited Cargo JSON")
    def complete(label: str, records: list[dict], record_path: Path) -> None:
        try:
            command = json.loads(record_path.read_text())
            stderr_path = Path(command["stderr_path"])
            exit_code = command["exit"]
            stderr = stderr_path.read_text(errors="replace")
        except (OSError, KeyError, TypeError, json.JSONDecodeError) as error:
            problems.append(f"{label}: unreadable command record/stderr: {error}")
            return
        summaries = re.findall(r"could not compile `openwepp-hillslope-orchestrator` \((lib(?: test)?)\) due to (\d+) previous errors", stderr)
        modes = {mode: int(count) for mode, count in summaries}
        error_count = sum(1 for item in records if item.get("reason") == "compiler-message" and item.get("message", {}).get("level") == "error")
        if exit_code != 101:
            problems.append(f"{label}: command exit is {exit_code!r}, expected Cargo lint failure 101")
        if modes.keys() != {"lib", "lib test"}:
            problems.append(f"{label}: incomplete lib/lib-test stderr summaries: {summaries!r}")
        elif sum(modes.values()) != error_count:
            problems.append(f"{label}: stderr error total {sum(modes.values())} does not reconcile Cargo JSON error count {error_count}")
        if not any(item.get("reason") == "build-finished" and item.get("success") is False for item in records):
            problems.append(f"{label}: missing failed build-finished Cargo record")
    complete("reference", ref_records, args.reference_record)
    complete("target", tgt_records, args.target_record)
    ref, ref_errors = diagnostics(ref_records, args.reference_source, "reference")
    tgt, tgt_errors = diagnostics(tgt_records, args.target_source, "target")
    problems.extend(ref_errors + tgt_errors)
    source_paths = {span["path"] for row in ref + tgt for span in row["spans"]
                    if not Path(span["path"]).is_absolute()}
    mapping, _ = line_map(args.reference_source, args.target_source, source_paths)
    compatibility = set(args.compatibility_path)
    for path in compatibility:
        if not (args.reference_source / path).is_file() or not (args.target_source / path).is_file() or not (args.compatibility_base_source / path).is_file():
            problems.append(f"compatibility path missing on one side: {path}")
    _, compatibility_changed_reference = line_map(args.compatibility_base_source, args.reference_source, compatibility)
    ref_exact = Counter(key(row, tuple(sorted((s["path"], s["line_start"], s["line_end"], s["is_primary"], s["text_sha256"]) for s in row["spans"]))) for row in ref)
    ref_content = Counter((tuple(row["target"]["descriptor"]), row["code"], tuple(sorted((s["path"], s["is_primary"], s["text_sha256"]) for s in row["spans"]))) for row in ref)
    rows = []
    for row in tgt:
        mapped = mapped_spans(row, mapping)
        exact = key(row, mapped) if mapped is not None else None
        content = (tuple(row["target"]["descriptor"]), row["code"], tuple(sorted((s["path"], s["is_primary"], s["text_sha256"]) for s in row["spans"])))
        compatibility_lines_in_target: dict[str, set[int]] = defaultdict(set)
        for (path, target_line), reference_line in mapping.items():
            if path in compatibility and reference_line in compatibility_changed_reference[path]:
                compatibility_lines_in_target[path].add(target_line)
        if touches_changed(row, compatibility_lines_in_target, compatibility):
            classification, reason = "current_scope", "span intersects/encompasses a shared compatibility enum alternative"
        elif exact is not None and ref_exact[exact]:
            ref_exact[exact] -= 1
            ref_content[content] -= 1
            classification, reason = "inherited_unchanged", "all primary and secondary spans map through exact equal diff blocks"
        elif ref_content[content]:
            ref_content[content] -= 1
            classification, reason = "changed_relocated", "same target, lint code and exact span content; location is not an exact diff mapping"
        else:
            classification, reason = "current_scope", "no matched reference diagnostic with exact mapped spans or content"
        rows.append({**row, "classification": classification, "reason": reason, "mapped_spans": mapped})
    status = "INCOMPLETE" if problems else "COMPLETE"
    full_reference = normalized_compiler_messages(ref_records, args.reference_source)
    full_target = normalized_compiler_messages(tgt_records, args.target_source)
    output = {"status": status,
              "inputs": {"reference_log": str(args.reference_log), "target_log": str(args.target_log),
                         "reference_log_sha256": sha256(args.reference_log), "target_log_sha256": sha256(args.target_log),
                         "reference_record": str(args.reference_record), "target_record": str(args.target_record),
                         "reference_source": str(args.reference_source), "target_source": str(args.target_source),
                         "compatibility_base_source": str(args.compatibility_base_source),
                         "compatibility_paths": sorted(compatibility)},
              "preserved_non_json_banners": {"reference": ref_banners, "target": tgt_banners},
              "mode_attribution": "UNAVAILABLE: failed compiler-message records do not carry profile.test; coverage is instead reconciled against both stderr lib/lib-test summaries.",
              "full_compiler_message_multiset": {"reference_count": sum(full_reference.values()), "target_count": sum(full_target.values()),
                                                 "equal_after_source_root_normalization": full_reference == full_target},
              "problems": problems, "diagnostics": rows,
              "counts": dict(Counter(row["classification"] for row in rows))}
    args.output.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    return 0 if status == "COMPLETE" else 2


if __name__ == "__main__":
    raise SystemExit(main())
