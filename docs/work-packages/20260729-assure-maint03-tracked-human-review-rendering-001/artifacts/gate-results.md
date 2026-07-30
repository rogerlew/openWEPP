# Gate Results

Evidence class: Ran

## Assurance And Rendering

- `validate --all`: PASS; three production-domain `DRAFT` reports and zero
  public reports.
- `verify-generation --base-ref 15763d...`: PASS; current generation
  `b85b2ea962e7a9cf1461d187b8751b70043d1e6aa4f937c648ffb2b8ff49fffc`,
  27 anchored transitions.
- review renderer `--apply`: PASS; 92 files installed.
- independent review renderer `--check`: PASS; 92 files current.
- `openwepp-assurance check --all` without a staging root: PASS; zero public
  reports while the separate tracked review lane exists.

## Focused Implementation

- Python renderer unit tests: 4/4 PASS.
- Python compilation: PASS.
- `cargo fmt --all -- --check`: PASS.
- assurance crate strict Clippy with all targets/features: PASS.
- assurance crate Nextest: 32/32 PASS.
- complete amendment integration target: 20/20 PASS, 2 skipped.
- complete assembly integration target: 9/9 PASS after the zero-public consumer
  was corrected.
- independent Review A reran the combined amendment and assembly targets:
  29/29 PASS, 2 skipped.

## Reader Consumers

- all 25 tracked Markdown files parsed with `cmark-gfm`.
- assurance builder local-link checks: PASS; independent Review B separately
  found zero missing local links.
- all 21 SVG files parsed and contain one title, one description, and
  `role="img"`.
- unresolved-directive and known duplicate/invalid count-noun scan: zero.
- `markdown-doc`: 42 affected files, zero errors, zero warnings.
- scoped cached diff check excluding the byte-preserved generated review lane:
  PASS. The generated lane retains identified CSV line endings and sanitized
  Matplotlib whitespace; its governing check is the 92-file exact-byte
  renderer comparison rather than whitespace normalization.

## Protected Boundaries

The three protected SHA-256 values match intake. `usersum/assurance/reports/`
is absent. All three rendered status blocks say `DRAFT`; no approval,
publication, release, export, or vendoring authority was introduced.

The earlier unstaged `git diff --check` reports were accurate for tracked
implementation and documentation but did not include then-untracked review
files. The precommit cached check exposed retained whitespace in generated
CSV/SVG/Markdown bytes. Normalizing those copies would violate deterministic
equivalence with the real assurance builder, so the terminal check was scoped
to non-generated paths and the complete generated lane remained governed by
exact inventory and content hashes.

## Full Workspace

The first local all-feature workspace run was deliberately interrupted after a
post-review source correction made it non-governing; 189 tests had passed at
that point and its two reported failures were SIGINT cancellations, not test
assertion failures. The fresh exact-head
`cargo nextest run --workspace --all-features` completed under the authorized
read-only gate runner: PASS, 2,163 passed, zero failed, 5 skipped, in
2,645.038 seconds (2,647.50 seconds wall time).
