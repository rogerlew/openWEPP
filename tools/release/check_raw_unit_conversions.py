#!/usr/bin/env python3
"""Guard against raw dimensional conversion literals in production Rust paths."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

DEFAULT_PATHS = [
    Path("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs"),
    Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"),
    Path("crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs"),
]

ALL_PRODUCTION_ROOTS = [
    Path("crates/openwepp-hillslope-orchestrator/src"),
    Path("crates/openwepp-runner/src"),
    Path("crates/openwepp-climate-runtime-adapter/src"),
]

ALLOW_MARKER = "UNIT-CONVERSION-ALLOW:"

NUMERIC_LITERAL_RE = re.compile(
    r"(?<![\w.])"
    r"(?P<number>(?:\d[\d_]*(?:\.[\d_]*)?|\.\d[\d_]*)(?:[eE][+-]?\d[\d_]*)?)"
    r"(?P<suffix>f(?:32|64)|[iu](?:8|16|32|64|128|size))?"
    r"(?![\w.])"
)

STRING_LITERAL_RE = re.compile(r'"(?:\\.|[^"\\])*"')

RAW_LITERAL_VALUES = {
    "langley_to_mj_m2": (0.04184,),
    "mm_m_scale": (1_000.0, 0.001),
    "hour_second_scale": (3_600.0, 3_600_000.0, 1.0 / 3_600.0, 0.000_277_78),
    "legacy_snow_melt_scale": (39.37, 0.0254, 1_609.0),
    "cm_m_scale": (100.0, 0.01),
}

NUMERIC_MATCH_ABSOLUTE_TOLERANCE = 1.0e-10
NUMERIC_MATCH_RELATIVE_TOLERANCE = 1.0e-10


@dataclass(frozen=True)
class Finding:
    path: Path
    line_number: int
    literal_class: str
    literal: str
    line: str

    def render(self) -> str:
        return (
            f"{self.path}:{self.line_number}: {self.literal_class} "
            f"literal={self.literal}: "
            f"{self.line.strip()}"
        )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Fail when guard-enforced production files contain unauthorized "
            "raw dimensional conversion literals."
        )
    )
    parser.add_argument(
        "--path",
        action="append",
        default=[],
        help="Path to scan. May be supplied more than once. Defaults to first-wave enforced files.",
    )
    parser.add_argument(
        "--inventory-all-production",
        action="store_true",
        help="Inventory all configured production roots without failing.",
    )
    return parser.parse_args()


def normalize_path(path: Path) -> Path:
    path = path if path.is_absolute() else REPO_ROOT / path
    return path.resolve()


def iter_rust_files(paths: list[Path]) -> list[Path]:
    files: list[Path] = []
    for input_path in paths:
        path = normalize_path(input_path)
        if path.is_dir():
            files.extend(sorted(path.rglob("*.rs")))
        elif path.suffix == ".rs":
            files.append(path)
    return [path for path in files if path.name != "tests.rs"]


def repo_relative(path: Path) -> Path:
    try:
        return path.resolve().relative_to(REPO_ROOT)
    except ValueError:
        return path.resolve()


def marker_allows_class(marker_line: str, literal_class: str) -> bool:
    if ALLOW_MARKER not in marker_line:
        return False
    marker_text = marker_line.split(ALLOW_MARKER, 1)[1]
    return literal_class in {
        token.strip(" ,;.")
        for token in re.split(r"[\s,;]+", marker_text)
        if token.strip(" ,;.")
    }


def line_is_allowed(lines: list[str], index: int, literal_class: str) -> bool:
    current_line = lines[index]
    if marker_allows_class(current_line, literal_class):
        return True
    if index == 0:
        return False
    previous_line = lines[index - 1].strip()
    return previous_line.startswith("//") and marker_allows_class(previous_line, literal_class)


def code_without_comments_and_strings(line: str) -> str:
    code = line.split("//", 1)[0]
    return STRING_LITERAL_RE.sub('""', code)


def parse_numeric_literal(raw_literal: str) -> float | None:
    number = raw_literal
    for suffix in ("f64", "f32"):
        if number.endswith(suffix):
            number = number[: -len(suffix)]
            break
    number = number.replace("_", "")
    try:
        return float(number)
    except ValueError:
        return None


def numeric_matches(value: float, expected: float) -> bool:
    tolerance = max(
        NUMERIC_MATCH_ABSOLUTE_TOLERANCE,
        abs(expected) * NUMERIC_MATCH_RELATIVE_TOLERANCE,
    )
    return abs(value - expected) <= tolerance


def classify_literal(raw_literal: str) -> str | None:
    if not any(marker in raw_literal for marker in (".", "e", "E", "f")):
        return None
    value = parse_numeric_literal(raw_literal)
    if value is None:
        return None
    for literal_class, expected_values in RAW_LITERAL_VALUES.items():
        if any(numeric_matches(value, expected) for expected in expected_values):
            return literal_class
    return None


def iter_production_lines(lines: list[str]):
    in_test_module = False
    pending_test_attr = False
    brace_depth = 0

    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("#[cfg(test)]"):
            pending_test_attr = True
            yield index, line, True
            continue

        if pending_test_attr and re.search(r"\bmod\s+tests\b", stripped):
            in_test_module = True
            pending_test_attr = False

        if in_test_module:
            brace_depth += line.count("{") - line.count("}")
            yield index, line, True
            if brace_depth <= 0 and "}" in line:
                in_test_module = False
                brace_depth = 0
            continue

        pending_test_attr = False
        yield index, line, False


def scan_file(path: Path) -> list[Finding]:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    findings: list[Finding] = []
    for index, line, is_test_line in iter_production_lines(lines):
        if is_test_line or line.lstrip().startswith("//"):
            continue
        code = code_without_comments_and_strings(line)
        for literal_match in NUMERIC_LITERAL_RE.finditer(code):
            raw_literal = literal_match.group(0)
            literal_class = classify_literal(raw_literal)
            if literal_class is None:
                continue
            if line_is_allowed(lines, index, literal_class):
                continue
            findings.append(
                Finding(
                    path=repo_relative(path),
                    line_number=index + 1,
                    literal_class=literal_class,
                    literal=raw_literal,
                    line=line,
                )
            )
    return findings


def main() -> int:
    args = parse_args()
    if args.inventory_all_production:
        paths = ALL_PRODUCTION_ROOTS
        fail_on_findings = False
    else:
        paths = [Path(value) for value in args.path] if args.path else DEFAULT_PATHS
        fail_on_findings = True

    findings: list[Finding] = []
    for path in iter_rust_files(paths):
        findings.extend(scan_file(path))

    if findings:
        header = "Raw dimensional conversion literal findings:"
        print(header, file=sys.stderr if fail_on_findings else sys.stdout)
        for finding in findings:
            print(f"  {finding.render()}", file=sys.stderr if fail_on_findings else sys.stdout)
        if fail_on_findings:
            print(
                f"Add a named conversion helper or an explicit {ALLOW_MARKER} rationale.",
                file=sys.stderr,
            )
            return 1

    if not findings and not args.inventory_all_production:
        print("raw unit conversion guard passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
