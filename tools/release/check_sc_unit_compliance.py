#!/usr/bin/env python3
"""Lint SC-* science contracts for unit-governance compliance."""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_CONTRACT_DIR = Path("docs/specifications/science-contracts/contracts")
REGISTRY_SOURCE = Path("crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs")

PLACEHOLDER_RE = re.compile(
    r"^\s*(?:|[-—]+|tbd|todo|unknown|unspecified|not\s+specified|\?|n/?a)\s*$",
    re.IGNORECASE,
)
HEADING_RE = re.compile(r"^(?P<marks>#{1,6})\s+(?P<title>.+?)\s*$")
TABLE_SEPARATOR_RE = re.compile(r"^\s*\|?\s*:?-{3,}:?\s*(?:\|\s*:?-{3,}:?\s*)+\|?\s*$")
CODE_SPAN_RE = re.compile(r"`([^`]+)`")
STRING_RE = re.compile(r'"((?:\\.|[^"\\])*)"')


@dataclass(frozen=True)
class Finding:
    code: str
    path: str
    line: int
    message: str

    def render(self) -> str:
        return f"{self.path}:{self.line}: {self.code}: {self.message}"


@dataclass(frozen=True)
class RegistryEntry:
    canonical_symbol: str
    aliases: tuple[str, ...]
    publication_aliases: tuple[str, ...]
    unit_label: str
    contract_id: str

    def required_aliases(self) -> tuple[str, ...]:
        aliases = []
        for alias in (*self.aliases, *self.publication_aliases):
            if alias and alias != self.canonical_symbol and alias not in aliases:
                aliases.append(alias)
        return tuple(aliases)


class RegistryLoadError(Exception):
    def __init__(self, code: str, path: Path, message: str) -> None:
        super().__init__(message)
        self.code = code
        self.path = path
        self.message = message


@dataclass(frozen=True)
class Table:
    header_line: int
    headers: list[str]
    rows: list[tuple[int, list[str]]]

    def column_index(self, name: str) -> int | None:
        normalized_name = normalize_header(name)
        for index, header in enumerate(self.headers):
            if normalize_header(header) == normalized_name:
                return index
        return None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--path",
        action="append",
        default=[],
        help="SC contract file or directory to lint. Defaults to all canonical SC contracts.",
    )
    parser.add_argument(
        "--registry-source",
        default=str(REPO_ROOT / REGISTRY_SOURCE),
        help="Rust unit registry source to parse for cross-checks.",
    )
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format for findings.",
    )
    return parser.parse_args()


def normalize_header(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", value.strip().lower()).strip()


def clean_cell(value: str) -> str:
    return value.strip().replace("<br>", " ").replace("<br/>", " ")


def is_placeholder(value: str) -> bool:
    stripped = re.sub(r"[`*_]", "", value).strip()
    return bool(PLACEHOLDER_RE.match(stripped))


def split_markdown_row(line: str) -> list[str]:
    stripped = line.strip()
    if stripped.startswith("|"):
        stripped = stripped[1:]
    if stripped.endswith("|"):
        stripped = stripped[:-1]
    return [clean_cell(cell) for cell in stripped.split("|")]


def section_for_heading(lines: list[str], title_fragment: str) -> tuple[int, list[tuple[int, str]]] | None:
    start_index = None
    start_level = None
    title_fragment_lower = title_fragment.lower()
    for index, line in enumerate(lines):
        match = HEADING_RE.match(line)
        if not match:
            continue
        if title_fragment_lower in match.group("title").lower():
            start_index = index
            start_level = len(match.group("marks"))
            break
    if start_index is None or start_level is None:
        return None

    section: list[tuple[int, str]] = []
    for index in range(start_index + 1, len(lines)):
        match = HEADING_RE.match(lines[index])
        if match and len(match.group("marks")) <= start_level:
            break
        section.append((index + 1, lines[index]))
    return start_index + 1, section


def find_table(section: list[tuple[int, str]], required_headers: list[str]) -> Table | None:
    required = {normalize_header(header) for header in required_headers}
    for index in range(len(section) - 1):
        line_number, line = section[index]
        _, next_line = section[index + 1]
        if not line.strip().startswith("|") or not TABLE_SEPARATOR_RE.match(next_line):
            continue
        headers = split_markdown_row(line)
        normalized = {normalize_header(header) for header in headers}
        if not required.issubset(normalized):
            continue
        rows: list[tuple[int, list[str]]] = []
        for row_line_number, row_line in section[index + 2 :]:
            if not row_line.strip().startswith("|"):
                break
            if TABLE_SEPARATOR_RE.match(row_line):
                continue
            rows.append((row_line_number, split_markdown_row(row_line)))
        return Table(line_number, headers, rows)
    return None


def cell(row: list[str], index: int | None) -> str:
    if index is None or index >= len(row):
        return ""
    return row[index]


def symbol_tokens(value: str) -> list[str]:
    code_spans = CODE_SPAN_RE.findall(value)
    raw_tokens = code_spans if code_spans else re.split(r"[,;/]", value)
    tokens: list[str] = []
    for raw in raw_tokens:
        stripped = raw.strip()
        if not stripped:
            continue
        for part in re.split(r"\s+or\s+|\s+and\s+|,", stripped):
            token = part.strip(" `.;:")
            if token:
                tokens.append(token)
    return tokens


def normalize_unit(value: str) -> str:
    value = re.sub(r"[`*_]", "", value)
    value = value.replace("·", " ")
    value = re.sub(r"\s+", " ", value.strip())
    return value


def unit_text_contains(unit_text: str, expected_unit: str) -> bool:
    normalized_text = normalize_unit(unit_text)
    normalized_expected = normalize_unit(expected_unit)
    if not normalized_expected:
        return True
    code_spans = [normalize_unit(span) for span in CODE_SPAN_RE.findall(unit_text)]
    if any(span == normalized_expected for span in code_spans):
        return True
    if code_spans:
        return False
    unit_boundary = r"(?<![A-Za-z0-9^./:_-])"
    unit_end = r"(?![A-Za-z0-9^./:_-])"
    return bool(
        re.search(
            f"{unit_boundary}{re.escape(normalized_expected)}{unit_end}",
            normalized_text,
        )
    )


def registry_entry_for_contract_symbol(
    registry_by_symbol: dict[str, RegistryEntry], symbol: str, contract_id: str
) -> RegistryEntry | None:
    entry = registry_by_symbol.get(symbol)
    if entry and entry.contract_id == contract_id:
        return entry
    return None


def parse_rust_string(value: str) -> str:
    match = STRING_RE.search(value)
    if not match:
        return ""
    return bytes(match.group(1), "utf-8").decode("unicode_escape")


def split_top_level_commas(value: str) -> list[str]:
    parts: list[str] = []
    start = 0
    paren_depth = 0
    bracket_depth = 0
    brace_depth = 0
    in_string = False
    escaped = False
    for index, char in enumerate(value):
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
            continue
        if char == "(":
            paren_depth += 1
        elif char == ")":
            paren_depth -= 1
        elif char == "[":
            bracket_depth += 1
        elif char == "]":
            bracket_depth -= 1
        elif char == "{":
            brace_depth += 1
        elif char == "}":
            brace_depth -= 1
        elif (
            char == ","
            and paren_depth == 0
            and bracket_depth == 0
            and brace_depth == 0
        ):
            parts.append(value[start:index].strip())
            start = index + 1
    parts.append(value[start:].strip())
    return parts


def boundary_entry_calls(source: str) -> list[str]:
    calls: list[str] = []
    marker = "BoundaryUnitEntry::new("
    search_start = 0
    while True:
        start = source.find(marker, search_start)
        if start == -1:
            return calls
        body_start = start + len(marker)
        depth = 1
        in_string = False
        escaped = False
        index = body_start
        while index < len(source):
            char = source[index]
            if in_string:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    in_string = False
            else:
                if char == '"':
                    in_string = True
                elif char == "(":
                    depth += 1
                elif char == ")":
                    depth -= 1
                    if depth == 0:
                        calls.append(source[body_start:index])
                        search_start = index + 1
                        break
            index += 1
        else:
            return calls


def parse_registry(registry_source: Path) -> tuple[dict[str, RegistryEntry], dict[str, list[RegistryEntry]]]:
    if not registry_source.exists():
        raise RegistryLoadError(
            "SCUNIT-E-010",
            registry_source,
            "required boundary-symbol unit registry source does not exist",
        )
    source = registry_source.read_text(encoding="utf-8")
    by_symbol: dict[str, RegistryEntry] = {}
    by_contract: dict[str, list[RegistryEntry]] = {}
    for call in boundary_entry_calls(source):
        args = split_top_level_commas(call)
        if len(args) < 12:
            continue
        canonical = parse_rust_string(args[0])
        aliases = tuple(STRING_RE.findall(args[1]))
        unit = parse_rust_string(args[2])
        contract_id = parse_rust_string(args[7])
        publication_aliases = tuple(STRING_RE.findall(args[11]))
        if not canonical or not unit:
            continue
        entry = RegistryEntry(canonical, aliases, publication_aliases, unit, contract_id)
        by_contract.setdefault(contract_id, []).append(entry)
        by_symbol[canonical] = entry
        for alias in aliases:
            by_symbol[alias] = entry
        for publication_alias in publication_aliases:
            by_symbol[publication_alias] = entry
    if not by_contract:
        raise RegistryLoadError(
            "SCUNIT-E-010",
            registry_source,
            "required boundary-symbol unit registry source yielded no parseable entries",
        )
    return by_symbol, by_contract


def collect_paths(paths: list[str]) -> list[Path]:
    if not paths:
        return sorted((REPO_ROOT / DEFAULT_CONTRACT_DIR).glob("SC-*.md"))
    collected: list[Path] = []
    for raw in paths:
        path = Path(raw)
        if not path.is_absolute():
            path = REPO_ROOT / path
        if path.is_dir():
            collected.extend(sorted(path.glob("SC-*.md")))
        else:
            collected.append(path)
    return sorted(dict.fromkeys(collected))


def contract_id_for(path: Path) -> str:
    return path.stem


def lint_contract(
    path: Path,
    registry_by_symbol: dict[str, RegistryEntry],
    registry_by_contract: dict[str, list[RegistryEntry]],
) -> list[Finding]:
    relative_path = path.relative_to(REPO_ROOT) if path.is_relative_to(REPO_ROOT) else path
    path_text = str(relative_path)
    lines = path.read_text(encoding="utf-8").splitlines()
    findings: list[Finding] = []
    contract_id = contract_id_for(path)
    variables_declared: set[str] = set()
    alias_map_tokens: set[str] = set()

    variables_section = section_for_heading(lines, "Variables and Units")
    if variables_section is None:
        findings.append(
            Finding(
                "SCUNIT-E-001",
                path_text,
                1,
                "missing Variables and Units section",
            )
        )
    else:
        section_line, section_lines = variables_section
        table = find_table(section_lines, ["Symbol", "Units"])
        if table is None:
            findings.append(
                Finding(
                    "SCUNIT-E-002",
                    path_text,
                    section_line,
                    "Variables and Units section lacks a table with Symbol and Units columns",
                )
            )
        else:
            symbol_index = table.column_index("Symbol")
            unit_index = table.column_index("Units")
            for line_number, row in table.rows:
                symbol_value = cell(row, symbol_index)
                unit_value = cell(row, unit_index)
                row_symbols = symbol_tokens(symbol_value)
                variables_declared.update(row_symbols)
                if is_placeholder(unit_value):
                    findings.append(
                        Finding(
                            "SCUNIT-E-003",
                            path_text,
                            line_number,
                            f"symbol row {symbol_value!r} has missing or placeholder units",
                        )
                    )
                    continue
                for symbol in row_symbols:
                    entry = registry_entry_for_contract_symbol(
                        registry_by_symbol, symbol, contract_id
                    )
                    if entry and not unit_text_contains(unit_value, entry.unit_label):
                        findings.append(
                            Finding(
                                "SCUNIT-E-004",
                                path_text,
                                line_number,
                                (
                                    f"symbol {symbol!r} declares units {unit_value!r}, "
                                    f"but registry requires {entry.unit_label!r}"
                                ),
                            )
                        )

    alias_section = section_for_heading(lines, "Symbol Alias Map")
    if alias_section is None:
        findings.append(
            Finding("SCUNIT-E-005", path_text, 1, "missing Symbol Alias Map section")
        )
    else:
        section_line, section_lines = alias_section
        table = find_table(
            section_lines,
            ["Canonical symbol", "Boundary/API name", "Scope", "Units check"],
        )
        if table is None:
            findings.append(
                Finding(
                    "SCUNIT-E-006",
                    path_text,
                    section_line,
                    (
                        "Symbol Alias Map lacks required columns: Canonical symbol, "
                        "Boundary/API name, Scope, Units check"
                    ),
                )
            )
        else:
            canonical_index = table.column_index("Canonical symbol")
            boundary_index = table.column_index("Boundary/API name")
            units_check_index = table.column_index("Units check")
            for line_number, row in table.rows:
                canonical_value = cell(row, canonical_index)
                boundary_value = cell(row, boundary_index)
                units_check = cell(row, units_check_index)
                alias_map_tokens.update(symbol_tokens(canonical_value))
                alias_map_tokens.update(symbol_tokens(boundary_value))
                if is_placeholder(units_check):
                    findings.append(
                        Finding(
                            "SCUNIT-E-007",
                            path_text,
                            line_number,
                            f"alias row {canonical_value!r} has missing or placeholder Units check",
                        )
                    )
                    continue
                symbols = symbol_tokens(canonical_value) + symbol_tokens(boundary_value)
                checked_registry_units: set[str] = set()
                for symbol in symbols:
                    entry = registry_entry_for_contract_symbol(
                        registry_by_symbol, symbol, contract_id
                    )
                    if entry and entry.unit_label not in checked_registry_units:
                        checked_registry_units.add(entry.unit_label)
                        if not unit_text_contains(units_check, entry.unit_label):
                            findings.append(
                                Finding(
                                    "SCUNIT-E-008",
                                    path_text,
                                    line_number,
                                    (
                                        f"alias row for {symbol!r} does not mention "
                                        f"registry unit {entry.unit_label!r} in Units check"
                                    ),
                                )
                            )

    registered_entries = registry_by_contract.get(contract_id, [])
    if registered_entries:
        for entry in registered_entries:
            if entry.canonical_symbol in variables_declared:
                continue
            if any(alias in variables_declared for alias in entry.aliases):
                findings.append(
                    Finding(
                        "SCUNIT-E-012",
                        path_text,
                        1,
                        (
                            f"registry symbol {entry.canonical_symbol!r} for {contract_id} "
                            "is covered only by an alias in Variables and Units; canonical symbol is required"
                        ),
                    )
                )
                continue
            findings.append(
                Finding(
                    "SCUNIT-E-009",
                    path_text,
                    1,
                    (
                        f"registry symbol {entry.canonical_symbol!r} for {contract_id} "
                        "is not declared in Variables and Units"
                    ),
                )
            )

        for entry in registered_entries:
            for alias in entry.required_aliases():
                if alias not in alias_map_tokens:
                    findings.append(
                        Finding(
                            "SCUNIT-E-011",
                            path_text,
                            1,
                            (
                                f"registered alias {alias!r} for canonical symbol "
                                f"{entry.canonical_symbol!r} is missing from Symbol Alias Map"
                            ),
                        )
                    )

    return findings


def main() -> int:
    args = parse_args()
    findings: list[Finding] = []
    try:
        registry_by_symbol, registry_by_contract = parse_registry(Path(args.registry_source))
    except RegistryLoadError as error:
        findings.append(
            Finding(
                error.code,
                str(error.path),
                1,
                error.message,
            )
        )
        registry_by_symbol, registry_by_contract = {}, {}
    for path in collect_paths(args.path):
        if not path.exists():
            findings.append(
                Finding("SCUNIT-E-000", str(path), 1, "contract path does not exist")
            )
            continue
        findings.extend(lint_contract(path, registry_by_symbol, registry_by_contract))

    findings.sort(key=lambda finding: (finding.path, finding.line, finding.code))
    if args.format == "json":
        print(json.dumps([asdict(finding) for finding in findings], indent=2))
    else:
        for finding in findings:
            print(finding.render(), file=sys.stderr)
        if findings:
            print(f"FAIL: {len(findings)} SC unit compliance finding(s)", file=sys.stderr)
        else:
            print("PASS: SC unit compliance lint found no findings")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
