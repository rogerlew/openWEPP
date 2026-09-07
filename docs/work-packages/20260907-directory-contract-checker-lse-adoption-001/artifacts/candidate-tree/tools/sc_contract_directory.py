"""Bounded directory-v1 Markdown loading and structural validation.

This checks declared structure, never scientific equivalence or dependency completeness.
No network access; all file reads follow confined, symlink-free resolution.
"""
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import re
import stat
import os
from datetime import date

ID = r'(?:INV-[A-Z0-9]+-\d+|OBL-[A-Z0-9]+(?:-[A-Z0-9]+)?-\d+)'
ID_RE = re.compile(ID)
ANCHOR = re.compile(r'<a id="([^"\n]+)"></a>')
LINK = re.compile(r'\[[^\]]*\]\(([^\s)]+)\)')
REQUIRED = {'contract_id', 'title', 'status', 'maturity', 'owner', 'contract_version',
            'producer_scope', 'consumer_scope', 'evidence_level', 'last_reviewed',
            'supersedes', 'superseded_by', 'contract_format', 'binding_index'}
COVERAGE = {f'{prefix}.section.{i:02}' for prefix, n in [('artifact', 18), ('kernel', 14)] for i in range(1, n+1)}
INV_COLUMNS = ['Invariant ID', 'Statement', 'Authority', 'Evidence', 'Guard', 'Failure posture']
OBL_COLUMNS = ['Obligation ID', 'Statement', 'Applicability', 'Authority', 'Enforcement/failure', 'Test bindings']
BEI_COLUMNS = ['Entry ID', 'Source', 'Status', 'Binding classification', 'Canonical binding IDs', 'Review gate', 'Notes']


class ContractError(ValueError):
    """An input cannot be admitted as a declared contract set."""


def fail(path: Path, line: int, message: str):
    raise ContractError(f'{path}:{line}: {message}')


def unfenced(text: str) -> list[str]:
    result = []
    fence = None
    for line in text.splitlines():
        if fence:
            if re.fullmatch(r' {0,3}' + re.escape(fence[0]) + '{' + str(fence[1]) + r',}[ \t]*', line):
                fence = None
            result.append('')
            continue
        match = re.match(r' {0,3}(`{3,}|~{3,})(.*)$', line)
        if match and not (match[1][0] == '`' and '`' in match[2]):
            fence = (match[1][0], len(match[1]))
            result.append('')
        else:
            result.append(line)
    return result


def value(cell: str) -> str:
    return cell.strip().strip('`').strip()


def metadata(text: str, path: Path) -> dict[str, str]:
    lines = text.splitlines()
    discovery = unfenced(text)
    # YAML is not Markdown: a fence inside front matter must not hide dispatch.
    if lines and lines[0] == '---':
        end = next((i for i in range(1, len(lines)) if lines[i] == '---'), len(lines))
        discovery[:end] = lines[:end]
    format_lines = [i for i, line in enumerate(discovery)
                    if re.match(r'''^\s*['"]?contract_format\b''', line)]
    # Legacy front matter is governed by its existing consumers, including YAML
    # continuation styles outside directory-v1's bounded grammar.
    if not format_lines:
        return {}
    if not lines or lines[0] != '---':
        if format_lines:
            fail(path, format_lines[0]+1, 'format metadata outside front matter')
        return {}
    try:
        end = lines.index('---', 1)
    except ValueError:
        fail(path, 1, 'unterminated front matter')
    fields = {}
    current = None
    for i, line in enumerate(lines[1:end], 2):
        if not line.strip():
            continue
        match = re.fullmatch(r'([a-z_]+):\s*(.*)', line)
        if match:
            current = match[1]
            if current in fields:
                fail(path, i, f'duplicate metadata {current}')
            raw = match[2].strip()
            if raw.startswith(('\'', '"')) and (len(raw) < 2 or raw[-1] != raw[0]):
                fail(path, i, 'unterminated quoted metadata')
            fields[current] = raw[1:-1] if raw.startswith(('\'', '"')) else raw
        elif re.match(r'^  - \S', line) and current:
            fields[current] += '\n' + line[4:]
        else:
            fail(path, i, 'malformed front matter')
    if any(i > end for i in format_lines):
        fail(path, end+1, 'format metadata outside front matter')
    if 'contract_format' in fields and fields['contract_format'] != 'directory-v1':
        fail(path, 1, f'unsupported contract_format {fields["contract_format"]!r}')
    return fields


def repository_root(entry: Path) -> Path:
    for parent in entry.parents:
        if (parent / '.git').exists():
            return parent
    # Isolated fixture trees have no Git metadata; no parent traversal is admitted.
    return entry.parent


def safe_path(root: Path, base: Path, raw: str, member_dir: str | None = None) -> Path:
    if not raw or '\\' in raw or raw.startswith('/') or ':' in raw or any(c in raw for c in ('\x00', '?', '%')):
        fail(base, 1, f'unsafe path {raw!r}')
    parts = raw.split('/')
    if any(p in ('', '.') for p in parts) or (member_dir and ('..' in parts or parts[0] != member_dir)):
        fail(base, 1, f'unsafe path components {raw!r}')
    cursor = base.parent
    for part in parts:
        if part == '..':
            cursor = cursor.parent
        else:
            cursor = cursor / part
        if not cursor.is_relative_to(root):
            fail(base, 1, f'path escapes repository: {raw}')
        if cursor.is_symlink():
            fail(base, 1, f'symlink prohibited: {raw}')
        if cursor.exists() and cursor.name not in {p.name for p in cursor.parent.iterdir()}:
            fail(base, 1, f'case mismatch: {raw}')
    return cursor


def read_file(path: Path) -> str:
    try:
        mode = path.stat().st_mode
        if not stat.S_ISREG(mode) or not mode & 0o444:
            fail(path, 1, 'not a readable regular file')
        return path.read_text(encoding='utf-8', errors='strict')
    except (OSError, UnicodeError) as error:
        fail(path, 1, f'unreadable UTF-8 document: {error}')


@dataclass
class Table:
    heading: str
    headers: list[str]
    rows: list[tuple[int, list[str]]]
    line: int


@dataclass
class Document:
    path: Path
    text: str
    kind: str
    parse_tables: bool = True

    def __post_init__(self):
        self.lines = unfenced(self.text)
        if self.parse_tables and '<!--' in '\n'.join(self.lines):
            fail(self.path, 1, 'HTML comments unsupported in directory authority')
        self.anchors = {}
        for n, line in enumerate(self.lines, 1):
            for anchor in ANCHOR.findall(line):
                if anchor in self.anchors:
                    fail(self.path, n, f'ambiguous duplicate anchor #{anchor}')
                self.anchors[anchor] = n
        self.explicit_anchors = set(self.anchors)
        # Existing repository dependencies use Markdown section fragments.
        for n, line in enumerate(self.lines, 1):
            if re.match(r'^#{1,6} ', line):
                slug = re.sub(r'[^\w\- ]', '', re.sub(r'^#+ ', '', line).lower()).replace(' ', '-')
                if slug in self.explicit_anchors:
                    if self.anchors[slug] != n-1:
                        fail(self.path, n, f'heading collides with explicit anchor #{slug}')
                else:
                    if slug in self.anchors:
                        self.anchors[slug] = -1  # duplicate headings are ambiguous
                    else:
                        self.anchors[slug] = n
        self.tables = []
        if not self.parse_tables:
            return
        heading = ''
        i = 0
        while i < len(self.lines):
            line = self.lines[i]
            if line.startswith('#'):
                heading = re.sub(r'^#+\s*', '', line).strip()
            if line.strip().startswith('|') and i+1 < len(self.lines) and re.fullmatch(r'\s*\|(?:\s*:?-{3,}:?\s*\|)+\s*', self.lines[i+1]):
                headers = self.cells(line)
                if len(headers) != len(set(headers)):
                    fail(self.path, i+1, 'duplicate table columns')
                if len(self.cells(self.lines[i+1])) != len(headers):
                    fail(self.path, i+2, 'table separator width mismatch')
                table = Table(heading, headers, [], i+1)
                i += 2
                while i < len(self.lines) and self.lines[i].strip().startswith('|'):
                    cells = self.cells(self.lines[i])
                    if len(cells) != len(headers):
                        fail(self.path, i+1, f'malformed {heading} table row: expected {len(headers)} cells')
                    table.rows.append((i+1, cells))
                    i += 1
                self.tables.append(table)
                continue
            if line.strip().startswith("|"):
                fail(self.path, i+1, "table row outside a well-formed table")
            i += 1

    @staticmethod
    def cells(line: str) -> list[str]:
        return [c.strip() for c in line.strip().removeprefix('|').removesuffix('|').split('|')]

    def table(self, heading: str, headers: list[str]) -> Table:
        matches = [t for t in self.tables if t.heading == heading]
        if len(matches) != 1 or matches[0].headers != headers or not matches[0].rows:
            fail(self.path, 1, f'{heading}: requires one nonempty table with columns {headers}')
        return matches[0]


def one_target(cell: str, path: Path) -> str:
    refs = targets(cell)
    if len(refs) != 1:
        fail(path, 1, 'requires exactly one target')
    return refs[0]


def targets(cell: str) -> list[str]:
    links = LINK.findall(cell)
    return links if links else [value(cell)]


class ContractSet:
    def __init__(self, entry: Path):
        self.entry = entry.absolute()
        self.root = repository_root(self.entry)
        # Check entry and every ancestor before any read.
        for p in [self.entry, *self.entry.parents]:
            if p.is_symlink():
                fail(p, 1, 'symlink prohibited')
        text = read_file(self.entry)
        self.meta = metadata(text, self.entry)
        self.directory = self.meta.get('contract_format') == 'directory-v1'
        self.documents = {self.entry: Document(self.entry, text, 'normative', self.directory)}
        if not self.directory:
            return
        missing = REQUIRED - self.meta.keys()
        if missing or any(not self.meta.get(k, '').strip() for k in REQUIRED):
            fail(self.entry, 1, f'missing/empty required metadata: {sorted(missing)}')
        if self.meta['contract_id'] != self.entry.stem or not re.fullmatch(r'[1-9]\d*', self.meta['contract_version']):
            fail(self.entry, 1, 'invalid contract identity/version')
        if self.meta['status'] not in {'open','in_review','approved','retired'} or self.meta['maturity'] not in {'proposed','draft','active','deprecated'}:
            fail(self.entry, 1, 'unsupported lifecycle status/maturity')
        if any(self.meta[k] == '[]' for k in ['producer_scope','consumer_scope']):
            fail(self.entry, 1, 'empty producer/consumer scope')
        inventory = self.documents[self.entry].table('Document inventory', ['Path', 'Kind', 'Purpose', 'Applicability'])
        for n, row in inventory.rows:
            if any(not c.strip() for c in row) or value(row[1]) not in {'normative', 'historical'}:
                fail(self.entry, n, 'invalid membership kind or empty inventory cell')
            path = safe_path(self.root, self.entry, value(row[0]), self.entry.stem)
            if path.suffix != '.md' or path in self.documents:
                fail(self.entry, n, 'duplicate or non-Markdown member')
            doc = Document(path, read_file(path), value(row[1]))
            if any(re.match(r'\s*contract_version\s*:', l) for l in doc.lines):
                fail(path, 1, 'chapter cannot own a contract version')
            self.documents[path] = doc
        directory = self.entry.with_suffix('')
        if directory.is_symlink():
            fail(directory, 1, 'symlink chapter directory')
        if not directory.is_dir():
            fail(directory, 1, 'missing chapter directory')
        found = set()
        def walk(parent):
            for child in parent.iterdir():
                if child.is_symlink():
                    fail(child, 1, 'symlink in chapter directory')
                if child.is_dir():
                    walk(child)
                elif child.is_file():
                    found.add(child)
                else:
                    fail(child, 1, 'unsupported directory entry')
        walk(directory)
        if found != set(self.documents) - {self.entry}:
            fail(directory, 1, f'inventory membership differs: {sorted(str(p) for p in found.symmetric_difference(set(self.documents)-{self.entry}))}')
        self.dependencies = {}
        self.definitions = {}
        self.aliases = {}
        self.validate()

    def resolve(self, source: Document, ref: str, normative=True) -> tuple[Document, str]:
        if ref.count('#') != 1:
            fail(source.path, 1, f'target requires path and explicit anchor: {ref}')
        raw, anchor = ref.split('#')
        path = safe_path(self.root, source.path, raw) if raw else source.path
        if path not in self.documents:
            if path.is_relative_to(self.entry.with_suffix('')):
                fail(source.path, 1, f'target is undeclared: {ref}')
            kind = 'external'
            for folder in path.parents:
                if not folder.is_relative_to(self.root) or folder == self.root:
                    break
                owner_entry = folder.with_suffix('.md')
                if owner_entry.name.startswith('SC-') and owner_entry.exists():
                    owner_entry = safe_path(self.root, source.path, os.path.relpath(owner_entry, source.path.parent))
                    owner_text = read_file(owner_entry)
                    if metadata(owner_text, owner_entry).get('contract_format') == 'directory-v1':
                        owner = Document(owner_entry, owner_text, 'external')
                        inventory = owner.table('Document inventory', ['Path','Kind','Purpose','Applicability'])
                        declared = {safe_path(self.root, owner_entry, value(r[0]), owner_entry.stem): value(r[1]) for _,r in inventory.rows}
                        if path not in declared or (normative and declared[path] != 'normative'):
                            fail(source.path, 1, f'external target is not declared normative authority: {ref}')
                        kind = declared[path]
                        break
            target = Document(path, read_file(path), kind, False)
            # External historical sidecars cannot act as normative authority.
            if normative and 'provenance' in path.parts:
                fail(source.path, 1, f'historical evidence is not normative authority: {ref}')
        else:
            target = self.documents[path]
        if normative and target.kind == 'historical':
            fail(source.path, 1, f'history-only authority target: {ref}')
        if not anchor or anchor not in target.anchors or target.anchors[anchor] < 0:
            fail(source.path, 1, f'missing explicit anchor: {ref}')
        return target, anchor

    def validate(self):
        entry = self.documents[self.entry]
        aliases = [t for t in entry.tables if t.heading == 'Compatibility anchors']
        if aliases:
            for n, row in entry.table('Compatibility anchors', ['Alias', 'Target']).rows:
                alias = value(row[0])
                if alias in self.aliases or alias not in entry.anchors:
                    fail(self.entry, n, f'duplicate or missing alias #{alias}')
                self.aliases[alias] = one_target(row[1], self.entry)
            for alias, ref in self.aliases.items():
                target, anchor = self.resolve(entry, ref)
                if target.path == self.entry and anchor in self.aliases:
                    fail(self.entry, 1, f'alias chain/cycle: #{alias}')
                line = target.anchors[anchor]
                if not ID_RE.fullmatch(anchor) and not any(re.match(r'^#{1,6} ', l) for l in target.lines[line-1:line+2]):
                    fail(target.path, line, 'alias target is neither definition nor section')
        routes = entry.table('Reading routes', ['Task', 'Role/check', 'Initial material', 'Expansion trigger'])
        for n, row in routes.rows:
            if any(not c.strip() for c in row):
                fail(self.entry, n, 'empty required reading route cell')
        bei_doc, bei_anchor = self.resolve(entry, self.meta['binding_index'])
        if bei_doc.path == self.entry or bei_doc.kind != 'normative':
            fail(self.entry, 1, 'BEI must be in normative binding-index chapter')
        all_bei = [t for d in self.documents.values() for t in d.tables if t.heading == 'Binding Exposure Index']
        if len(all_bei) != 1:
            fail(self.entry, 1, 'requires exactly one Binding Exposure Index')
        bei = bei_doc.table('Binding Exposure Index', BEI_COLUMNS)
        if not any(bei_doc.lines[i].startswith('## Binding Exposure Index') for i in range(bei_doc.anchors[bei_anchor], bei.line)):
            fail(bei_doc.path, bei.line, 'binding_index anchor does not locate BEI section')
        for doc in self.documents.values():
            if doc.path != self.entry and doc.kind == 'normative':
                first = next((l for l in doc.lines if l.strip()), '')
                if f'../{self.entry.name}' not in LINK.findall(first):
                    fail(doc.path, 1, 'normative chapter must start with parent-entry link')
                if '## Dependencies' not in doc.lines:
                    fail(doc.path, 1, 'missing Dependencies section')
                deps = [t for t in doc.tables if t.heading == 'Dependencies']
                if not deps:
                    dep_start = doc.lines.index('## Dependencies')+1
                    dep_end = next((i for i in range(dep_start, len(doc.lines)) if re.match(r'^#{1,2} ', doc.lines[i])), len(doc.lines))
                    if '\n'.join(l for l in doc.lines[dep_start:dep_end] if not ANCHOR.fullmatch(l.strip())).strip().lower() != 'none beyond entry':
                        fail(doc.path, 1, 'Dependencies requires table or none beyond entry')
                else:
                    table = doc.table('Dependencies', ['Target', 'Required when', 'Boundary/obligation', 'Reading extent'])
                    for n, row in table.rows:
                        if any(not c.strip() for c in row) or value(row[1]).lower() in {'optional', 'as needed', 'if relevant', 'n/a'}:
                            fail(doc.path, n, 'dependency needs concrete applicability and nonempty cells')
                        for ref in targets(row[0]):
                            self.resolve(doc, ref)
            if doc.kind != 'normative':
                continue
            definition_lines = set()
            for table in doc.tables:
                if table.headers[0] not in {'Invariant ID', 'Obligation ID'}:
                    continue
                columns = INV_COLUMNS if table.headers[0] == 'Invariant ID' else OBL_COLUMNS
                # Guard-map tables refer to definitions and are not definition tables.
                if 'Statement' not in table.headers:
                    continue
                if not set(columns).issubset(table.headers):
                    fail(doc.path, table.line, 'definition table missing required columns')
                for n, row in table.rows:
                    m = re.fullmatch(r'<a id="(' + ID + r')"></a> `\1`', row[0])
                    if not m or any(not row[table.headers.index(c)].strip() for c in columns):
                        fail(doc.path, n, 'unsupported or empty canonical definition row')
                    ident = m[1]
                    if ident in self.definitions:
                        fail(doc.path, n, f'duplicate normative definition {ident}')
                    if not ident.startswith('INV-' if columns == INV_COLUMNS else 'OBL-'):
                        fail(doc.path, n, 'definition ID/table kind mismatch')
                    self.definitions[ident] = (doc.path, ident)
                    definition_lines.add(n)
            for n, line in enumerate(doc.lines, 1):
                if re.search(r'<a\b[^>]*(?:INV-|OBL-)', line) and n not in definition_lines:
                    ids = ANCHOR.findall(line)
                    if doc.path != self.entry or not ids or any(i not in self.aliases for i in ids):
                        fail(doc.path, n, 'definition-shaped anchor outside marked table grammar')
        registry = bei_doc.table('Binding definitions', ['ID', 'Definition'])
        registered = {}
        for n, row in registry.rows:
            ident = value(row[0])
            if not ID_RE.fullmatch(ident) or ident in registered:
                fail(bei_doc.path, n, f'invalid/duplicate binding registry ID {ident}')
            target, anchor = self.resolve(bei_doc, one_target(row[1], self.entry))
            if self.definitions.get(ident) != (target.path, anchor):
                fail(bei_doc.path, n, f'{ident}: target is not its actual definition')
            registered[ident] = (target.path, anchor)
        if registered != self.definitions:
            fail(bei_doc.path, 1, 'binding definitions registry and actual definitions differ')
        coverage = bei_doc.table('Schema coverage', ['Requirement', 'Canonical target(s)', 'Applicability'])
        keys = set()
        for n, row in coverage.rows:
            key = value(row[0])
            if key not in COVERAGE or key in keys or not row[2]:
                fail(bei_doc.path, n, f'invalid/duplicate coverage key or empty applicability: {key}')
            keys.add(key)
            if value(row[1]) == 'N/A':
                if not key.startswith('kernel.') or not row[2].startswith('Non-kernel contract: '):
                    fail(bei_doc.path, n, 'unsupported N/A rationale')
            else:
                for ref in targets(row[1]):
                    target, anchor = self.resolve(bei_doc, ref)
                    line = target.anchors[anchor]
                    if not any(re.match(r'^#{1,6} ', l) for l in target.lines[line-1:line+2]):
                        fail(target.path, line, 'coverage must target a real section')
        if keys != COVERAGE:
            fail(bei_doc.path, 1, f'missing coverage keys: {sorted(COVERAGE-keys)}')
        self.deferred = 0
        entries = set()
        for n, row in bei.rows:
            ident, source, status, cls, mapped, gate, notes = [value(c) for c in row]
            if not all([ident, source, status, cls, mapped, gate]) or ident in entries:
                fail(bei_doc.path, n, 'empty or duplicate BEI entry')
            entries.add(ident)
            if status not in {'active', 'historical', 'superseded'} or cls not in {'maps-to-existing-INV', 'unpromoted-binding', 'historical-or-superseded', 'undecidable'} or gate not in {'none', 'flagged-binding-addition', 'science-review-follow-on'}:
                fail(bei_doc.path, n, 'invalid BEI status/classification/review gate')
            ids = [] if mapped == 'none' else [i.strip().strip('`') for i in mapped.split(',')]
            if mapped != 'none' and (not ids or any(not ID_RE.fullmatch(i) for i in ids)):
                fail(bei_doc.path, n, 'invalid binding IDs')
            if any(i not in registered for i in ids):
                fail(bei_doc.path, n, 'BEI refers to missing actual definition')
            if (status == 'active' or cls == 'unpromoted-binding') and not ids and gate != 'science-review-follow-on':
                fail(bei_doc.path, n, 'active binding lacks mapping')
            if cls == 'undecidable' and gate != 'science-review-follow-on':
                fail(bei_doc.path, n, 'undecidable binding lacks science-review follow-on')
            if gate == 'science-review-follow-on':
                if not all(re.search(re.escape(x)+r'\s*[^;\s][^;]*', notes) for x in ['owner:', 'next evidence gate:', 'retained:']):
                    fail(bei_doc.path, n, 'deferred residue requires owner, next evidence gate and retained binding target')
                retained = re.search(r'retained:\s*([^\s;]+)', notes)
                if not retained:
                    fail(bei_doc.path, n, 'missing retained target')
                self.resolve(bei_doc, retained[1])
                self.deferred += 1
        for alias, ref in self.aliases.items():
            target, anchor = self.resolve(entry, ref)
            if ID_RE.fullmatch(anchor) and self.definitions.get(anchor) != (target.path, anchor):
                fail(self.entry, 1, f'alias targets non-definition #{anchor}')
        self.validate_provenance(bei_doc, bei)
        self.row_count = len(bei.rows)

    def validate_provenance(self, bei_doc, bei):
        cited = {}
        for n, row in bei.rows:
            refs = targets(row[1])
            for ref in refs:
                if re.match(r'https?://', ref):
                    continue  # external citations are never fetched
                target, anchor = self.resolve(bei_doc, ref, normative=False)
                if target.kind == 'historical' or 'provenance' in target.path.parts:
                    cited[(target.path, anchor)] = (target, n, row)
        historical = {d.path: d for d in self.documents.values() if d.kind == 'historical'}
        historical.update({d.path: d for d, _, _ in cited.values()})
        for doc in historical.values():
            sections = [(i, line[3:].strip()) for i, line in enumerate(doc.lines) if line.startswith('## ')]
            if not sections:
                fail(doc.path, 1, 'historical document lacks provenance entries')
            for ix, (start, title) in enumerate(sections):
                end = sections[ix+1][0] if ix+1<len(sections) else len(doc.lines)
                body = '\n'.join(doc.lines[start+1:end])
                fields = {}
                for key, val in re.findall(r'^- ([a-z_]+):\s*(.*?)\s*$', body, re.M):
                    if key in fields:
                        fail(doc.path, start+1, f'duplicate provenance field {key}')
                    fields[key] = val
                required = ['status','source_package','effective_date','verdict','canonical_binding_ids','provenance_anchors']
                if any(not fields.get(k) for k in required):
                    fail(doc.path, start+1, 'missing sidecar provenance fields')
                # Template heading owns entry ID and title; prose owns summary.
                heading = title.split(maxsplit=1)
                summary = [l for l in doc.lines[start+1:end] if l.strip() and not l.startswith('- ') and not ANCHOR.fullmatch(l.strip())]
                if len(heading)!=2 or not summary:
                    fail(doc.path, start+1, 'sidecar entry requires ID, title and summary')
                status = value(fields['status'])
                if status not in {'active','superseded','historical'}:
                    fail(doc.path, start+1, 'invalid sidecar status')
                try:
                    date.fromisoformat(fields['effective_date'])
                except ValueError:
                    fail(doc.path, start+1, 'invalid sidecar effective date')
                ids = [] if value(fields['canonical_binding_ids']) == 'none' else [value(i) for i in fields['canonical_binding_ids'].split(',')]
                if any(i not in self.definitions for i in ids):
                    fail(doc.path, start+1, 'sidecar references missing binding definition')
                matching = [(anchor, row) for (path, anchor), (_, _, row) in cited.items() if path==doc.path and start <= doc.anchors[anchor] <= end and abs(doc.anchors[anchor]-(start+1)) <= 1]
                if len(matching)!=1:
                    fail(doc.path, start+1, 'sidecar entry must have one BEI source mapping')
                _, row = matching[0]
                mapped = [] if value(row[4])=='none' else [value(i) for i in value(row[4]).split(',')]
                if status != value(row[2]) or set(ids)!=set(mapped):
                    fail(doc.path, start+1, 'sidecar status/bindings differ from BEI')
                if status == 'superseded':
                    ref = fields.get('superseded_by', '')
                    if not ref or ref == 'none':
                        fail(doc.path, start+1, 'missing supersession target')
                    self.resolve(doc, one_target(ref, doc.path))
                if fields['verdict']=='hold' and not all(fields.get(k) for k in ['owner','next_evidence_gate']):
                    fail(doc.path, start+1, 'held provenance requires owner and next evidence gate')

    def normative_text(self) -> str:
        """Deterministic compatibility view for whole-set validator consumers only."""
        return '\n\n'.join(doc.text for doc in self.documents.values() if doc.kind == 'normative')


def main(argv=None):
    import argparse
    parser = argparse.ArgumentParser(description='Validated normative text for legacy whole-set test consumers')
    parser.add_argument('entry', type=Path)
    args = parser.parse_args(argv)
    try:
        contract = ContractSet(args.entry)
    except ContractError as error:
        print(f'FAIL {error}', file=__import__('sys').stderr)
        return 1
    print(contract.normative_text(), end='')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
