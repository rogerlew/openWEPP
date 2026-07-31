# Gate Results

Status: `complete / HOLD`

Evidence mode: `Ran`

## Required Profiles

- `cargo nextest run --workspace --profile frost`: PASS, 323 passed, 1 slow,
  1,858 skipped, 558.632 seconds.
- `cargo nextest run --workspace --profile quick`: NOT PASS. Two exact-head
  attempts reached the profile's 600-second timeout in the same assurance
  amendment/publication cases under broad parallel load. The first completed
  156 tests before cancellation; the uncontended retry did the same. No test
  assertion failed.
- The two timed-out tests then passed independently under the same `quick`
  profile: report-source adoption, 1/1 in 292.993 seconds; approval/release
  conflict handling, 1/1 in 244.550 seconds. The cancellation-survivor
  authority-byte negative matrix also passed independently, 1/1 in
  396.819 seconds.

The isolated passes diagnose suite-level wall-time contention, but do not
convert the required full `quick` profile into a pass. This unmet gate is
retained honestly and is compatible with the package's existing scientific
`HOLD`.

After the profile runs, verification added only the terminal
`GAP-SNOWENERGY-007` register row and closure artifacts; no runtime source
changed. The exact-head focused 13-test suite, strict Binding Exposure/unit
checks, assurance validation/render check, formatting, and diff hygiene all
passed after that documentation correction. The earlier frost result is
therefore retained as implementation evidence, not represented as an
exact-document-byte rerun.

## Focused Rust

- EB-03 contract/runtime plus Stage 0 contract: PASS, 13/13.
- `openwepp-meteorology`: PASS, 21/21.
- `openwepp-hillslope-orchestrator`: PASS, 412/412, including three slow
  routing-oracle tests.
- strict workspace Clippy, all targets: PASS.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.

## Science And Unit Governance

- strict Binding Exposure checks: PASS for `SC-SNOWENERGY-001` and
  `SC-SNOWFREEZE-001`.
- SC unit-compliance checks: PASS for both contracts.
- raw unit-conversion guard on the new meteorology implementation/error files:
  PASS.
- full touched-production inventory reported retained raw literals in
  pre-existing lines. A zero-context diff scan found none of those reported
  literals on EB-03-added lines; the new unit-bearing meteorology surface is
  the enforced passing scope. The broader inventory is not reported as a
  touched-file PASS.
- authority-suite anti-evasion guard: PASS.
- required authority-obligation guard target: PASS, 3/3.

## Real Consumer And Figures

- six-cell direct-production consumer replay: expected command exit 1 because
  S/LS fail closed; JSON disposition `HOLD`. B absent/empty/disabled and L
  complete; S/LS retain material SWE at the typed `0 K` provider boundary.
- deterministic figure regeneration: PASS; all three hashes repeat exactly.
- SVG XML validation: PASS for all three figures.

## Assurance And Documentation

- assurance `validate --all`, `plan --all`, and zero-public `check --all`:
  PASS.
- complete tracked human-review rendering: PASS, all 92 files current.
- scoped Markdown lint commands: PASS with zero errors/warnings. The tool
  reported zero discovered files for several single-file/tree invocations;
  package and contract structure are additionally exercised by focused
  contract tests and assurance rendering.
