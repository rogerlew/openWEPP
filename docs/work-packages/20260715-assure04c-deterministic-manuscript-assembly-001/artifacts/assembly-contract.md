# ASSURE-04C Assembly Contract

Status: frozen before production edits

Evidence class: Static

## Boundary

The assembler substitutes only explicit typed directives. Every byte outside a
directive is copied from the canonical UTF-8 manuscript or supplement without
rewriting, reflowing, or interpretation. Assembly has no conditional, loop,
include, expression, environment, plugin, shell, network, agent, or clock
facility.

ASSURE-04C implements source contract version 2. Catalog and report schemas are
version 2; retained result objects remain schema version 1. The version change
adds assembly records without changing the fixture's scientific claim envelope
or lifecycle.

## Directive Grammar

A directive is ASCII and begins with `{{` and ends with `}}`. Nested braces,
newlines, tabs, control characters, empty fields, and an unrecognized directive
kind fail. All IDs use the existing v2 logical-ID grammar.

Inline directives:

```text
{{quantity:<value-binding-id>}}
{{reference:<reference-id>}}
{{link:report|<label>}}
{{link:supplement|<label>}}
{{link:research-object:<research-object-id>|<label>}}
{{link:usersum:<usersum-root-relative-path>|<label>}}
```

Block directives occupy an otherwise empty line and include their terminating
newline when one is present:

```text
{{table:<table-id>}}
{{figure:<figure-id>}}
```

`quantity`, `reference`, and `link` directives may appear in prose or table-cell
source. `table` and `figure` directives fail when embedded in prose. Every
declared value binding, table, figure, reference, and public-safe research
object must be used by the manuscript or supplement; a directive may not refer
to an identity outside the content source's declared ID lists.

Authored literals may not contain a numeric token followed by any declared unit
symbol. They also may not contain Markdown links, bare/autolink URL forms, or
email-autolink forms. Numeric quantities and links must therefore enter only
through typed directives; refreshing a source digest cannot admit a bypass.

Labels are authored prose. They must be nonempty single-line text without raw
braces, NUL, or Markdown link delimiters. A `usersum` path is confined,
root-relative within the future `usersum/` tree, names an existing tracked
regular non-symlink file, and may not enter `assurance/reports/`; the assembler
computes the output-relative route. It never emits a path into `docs/`,
`crates/`, work packages, sources, exports, snapshots, releases, or vendors.

## Typed Assembly Records

### Value binding

One value binding contains:

- stable `id`, `title`, and accountable `owner`;
- one `result_id` and one `value_id` from that retained result object;
- the expected `unit_id`;
- `transform`: `identity` or `absolute`; and
- `display`: `integer`, `fixed:<0..15>`, or `scientific:<0..15>`.

The assembler rejects an absent/duplicate result value, a unit mismatch,
nonfinite transformed value, negative value under `absolute` only after taking
its finite magnitude, unsupported display syntax, or precision above 15.
`integer` requires an exactly integral binary64 value. Fixed/scientific output
uses locale-independent ASCII, no thousands separators, and normalizes
scientific exponents to `e±NN` without platform-dependent formatting.

A value may have multiple bindings when the manuscript scientifically requires
different declared displays (for example, a rounded finding and a retained
binary64 operand). Each rendering remains tied to the same result value. The
free-text result precision policy remains visible provenance; the binding is
the executable display contract. Changing a binding changes the source root
and makes an existing staged check fail until intentionally rebuilt.

An inline `quantity` directive always emits the selected display value and the
unit symbol from the same typed binding. Authored unit suffixes are not an
assembly mechanism and cannot substitute for this coupled value-unit rendering.

### Table

One table contains stable identity/ownership, caption, alternative text,
ordered columns, and ordered rows. A column has a nonempty label and an explicit
nullable unit ID. A row has a nonempty authored label plus exactly one value-
binding ID per column. All referenced bindings must be declared and all units
must match their columns. The renderer emits one CommonMark table, with the row
label in the first column and units in column headings. Cells are rendered only
through value bindings; numeric literals are not accepted in table records.
Authored titles, captions, alternatives, labels, and citations are single-line
metadata. The renderer escapes Markdown and raw-HTML metacharacters before
placing them in any Markdown context; metadata cannot introduce links or HTML.

### Figure

ASSURE-04C admits exactly one visualization: `linear_magnitude_bars`. A figure
contains stable identity/ownership, its existing result IDs, ordered value-
binding IDs, title, caption, alternative text, and visualization kind. Figure
bindings must use `absolute`, share one unit, be positive finite values, and
collectively resolve exactly the declared result-ID set.

The renderer emits deterministic UTF-8 SVG with a white background, black
outlines, distinct monochrome fill patterns, persistent text labels, embedded
`<title>` and `<desc>`, declared `role="img"`, and a fixed view box. Bar width is
linear relative to the largest displayed magnitude; exact rendered labels are
printed beside bars. Color is not a carrier of meaning. The Markdown figure
block contains the image, caption, and a visible two-column value/unit table as
the alternative. A zero/nonfinite value, missing alternative/caption, unsafe
SVG text, output collision, or inaccessible/missing SVG fails.

## References And Research Objects

A reference directive emits the manifest citation followed by its immutable
identity. `doi:<value>` becomes a durable `https://doi.org/<value>` link after
strict DOI-character validation. Other identities render as escaped code text;
the builder does not invent an external URL.

A public-safe research-object link resolves to
`research-objects/<source-basename>` and copies the exact identified source
bytes to the selected staging subtree. Basenames must be unique. Restricted
objects are never read, copied, or linkable. Every staged research-object hash
must equal its declared SHA-256.

## Output Contract

For report `<id>` version `<version>`, the selected subtree is:

```text
usersum/assurance/reports/<id>/
  <version>/
    index.md
    supplement.md
    build-manifest.json
    figures/<figure-id>.svg
    research-objects/<source-basename>
```

The build manifest is deterministic pretty JSON plus one terminal newline. It
contains schema version, report ID/version, 04B source-root identity, assembly-
tool identity, and path/SHA-256 rows for every sibling output except itself.
It contains no timestamp, hostname, absolute path, staging root, environment,
process ID, or file modification time.

Named and all builds invoke the same pure per-report function. The named output
map equals the corresponding all-report subset byte-for-byte. Expected bytes
are complete in memory before any staging write. A selected report is rejected
unless every node in its 04B plan is `current`.

## Staging And Check Contract

V2 assembly is selected only by `--staging-root <path>`:

```text
openwepp-assurance build (--all | --report <id>) --staging-root <path>
openwepp-assurance check (--all | --report <id>) --staging-root <path>
```

Without `--staging-root`, the inherited `build --all` and `check --all` retain
the exact zero-public ASSURE-03 behavior. `--report` without staging fails.
Staging cannot be combined with `--output-root`, snapshot, or snapshot-root.

The staging root may be outside the repository, below repository `target/`, or
below one work-package `artifacts/` subtree. Repository root and every tracked
source/public/export/snapshot/release/vendor path are rejected. Existing path
components and output targets must be real directories/files, never symlinks or
special files.

After lexical location authorization and before any creation, the assembler
opens the staging root through descriptor-relative, no-follow traversal. All
directory creation, reads, writes, enumeration, removal, and renames remain
relative to that held descriptor. A component replacement cannot redirect an
operation through a symlink between validation and use.

A build renders and validates all selected reports first, then replaces each
selected report-ID subtree through a same-filesystem temporary sibling. It does
not traverse or mutate unrelated report-ID subtrees. Any error before the
replacement leaves prior staging bytes unchanged; replacement/restore failures
are surfaced as typed I/O errors rather than hidden. The builder snapshots an
existing selected subtree before installation and reconstructs it from that
snapshot if any post-install validation or cleanup fails. Replacement,
restoration, and cleanup failures are all surfaced rather than discarded.
Identified inputs are checked after installation and again immediately before
successful completion; either failure reconstructs the prior selected subtree
from its in-memory snapshot. A failed preparation removes the current report's
temporary/restore directories as well as those of earlier prepared reports.

Generated-link resolution stays descriptor-relative, including links that walk
up within the staged `usersum` tree. Immediately before success, the assembler
reopens the requested staging pathname without following symlinks and compares
its device/inode identity with the held root capability. Path replacement or
redirection fails even when the originally opened directory remains readable.

`check` is read-only. It recomputes expected bytes, requires every expected
file to match, requires the selected report-ID subtree to contain exactly the
expected file set, validates every generated local link inside a complete
`usersum`-shaped consumer root, and reports drift for an extra, missing, or
changed file.

## Semantic-Preservation Proof

For each source document, the directive parser retains an ordered sequence of
literal and directive segments. Rendering copies every literal segment exactly
once in the same order. Tests compare the ordered literal segment stream before
and after rendering boundaries and assert no unresolved `{{...}}` remains.
The accepted fixture diff is separately reviewed to confirm that changes to
authored prose are limited to replacing previously duplicated claim-bearing
values/tables/references and adding explicit portable cross-links.

## Failure Matrix

| Condition | Required result |
| --- | --- |
| Noncurrent 04B node | typed invalid/drift failure before output mutation |
| Unknown, duplicate, malformed, or disallowed directive | typed invalid failure |
| Missing result/value or unit mismatch | typed invalid failure |
| Unsupported/changed display precision | validation or deterministic check drift |
| Unused binding/table/figure/reference/result | typed invalid failure |
| Missing caption/alternative, invalid figure value, or missing SVG | typed invalid/drift failure |
| Unsafe or unresolved local link | typed invalid/drift failure |
| Raw authored quantity, URL/autolink, or numeric table cell | typed invalid failure |
| Restricted research-object access | typed invalid failure without reading content |
| Extra, missing, or changed staged file | typed drift failure |
| Staging target is protected, escaping, symlinked, or special | typed invalid/I/O failure |
| Render/source drift during operation | typed drift failure before accepted completion |
| Requested staging-root pathname changes identity | typed invalid/drift failure with rollback |

## Non-Authority Statement

A successful build proves deterministic assembly and traceability only. It does
not approve the groundwater manuscript, establish scientific correctness,
authorize publication, create a review lock, transfer evidence to a release, or
decide application fitness.
