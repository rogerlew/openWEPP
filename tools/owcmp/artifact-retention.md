# owcmp Artifact Retention

Use this policy for routine comparator and validation-suite evidence.

## Default Commit Set

Commit these artifacts when recording a routine `owcmp` run:

- `summary.json`
- `summary.md`
- `command-log.json`
- package review, verification, and disposition evidence

These files are the parent-agent handoff contract and are small enough to review
without loading raw per-row reports into context.

## Local-Only by Default

Leave these under the declared local output root unless a package explicitly
needs them for audit:

- `logs/`
- `reports/`
- `raw/`
- per-hillslope semantic JSON
- per-row dumps
- converted/intermediate surfaces

When raw artifacts are kept local, record the output path and, when needed,
hashes or counts in the package evidence.

## When to Commit Raw Reports

Commit raw reports only when one of these is true:

- An independent reviewer must inspect the raw report content in Git.
- The package is explicitly preserving a comparator evidence bundle.
- The raw artifact is small and is itself the asserted contract surface.
- The run cannot be reproduced from retained paths and command logs.

If raw reports are committed, mention that choice in the package disposition and
record the size of the artifact directory.
