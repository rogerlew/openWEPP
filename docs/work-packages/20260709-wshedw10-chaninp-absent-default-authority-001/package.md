# WSHED-W10 chan.inp Absent Default Authority

Status: `EXECUTED-COMPLETE`
Package ID: `20260709-wshedw10-chaninp-absent-default-authority-001`
Queue row: `WSHED-W10`
Evidence mode: `Static + ran`

## Objective

Ratify and implement openWEPP's `chan.inp`-absent watershed behavior to match
legacy WEPP compatibility defaults. Missing `chan.inp` in compatibility mode
must be an explicit typed defaulted state, not a hidden `None` branch or an
unrecorded modeling default.

## User Direction

User preference: match legacy WEPP's `chan.inp`-absent defaults behavior.

## Rationale

Pinned legacy `wshinp.for` only attempts `chan.inp` when `ipeak > 2`. The open
error branch sets output controls to the no-output path (`ichout=0`,
`nchnum=0`) with `cbase=0` initialized before the open attempt, then applies the
same channel timestep normalization block. openWEPP currently has two different
representations for absent `chan.inp`: the input-contract parser returns an
explicit `DefaultedCompat` object, while the watershed CLI bypasses it with
`None` and hardcoded routing globals. WSHED-W10 closes that split.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/wepp-input-files/specs/chaninp.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to explorer and Rust review subagents for WSHED-W10 source
lineage review, contract/readiness review, and post-implementation QA; expected
outputs are package artifact notes or summarized findings in `artifacts/review.md`;
write access is read-only unless the main agent assigns a bounded disjoint
write-set.

## Included Scope

- Amend `SC-INFILE-CHANINP-001` and `SC-SYSTEM-001` to ratify the compatibility
  default branch for missing/unreadable/malformed `chan.inp`.
- Update `chaninp.spec.md` from draft/HOLD to the WSHED-W10 ratified default
  posture, while keeping strict-vs-compat distinctions.
- Make watershed runtime consume typed `ChaninpParseOutcome::DefaultedCompat` /
  `OpenErrorCollapsedCompat` options as runtime-ready compatibility defaults.
- Route missing unconfigured `chan.inp` through the parser defaulted state,
  rather than a hidden `None` branch.
- Add parser/runtime/CLI tests proving explicit default values, warning
  provenance, and no hidden modeling defaults.
- Update roadmap/work-package catalog and package artifacts.

## Excluded Scope

- Changing parsed valid `chan.inp` behavior outside the absent/default branch.
- New channel-routing physics or hourly HBP consumption.
- Changing strict mode to accept missing `chan.inp` when `ipeak > 2`.
- Changing `gwcoeff` baseflow authority or Lane D generated-baseflow behavior.

## Intended Write Set

- `docs/work-packages/20260709-wshedw10-chaninp-absent-default-authority-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/specifications/wepp-input-files/specs/chaninp.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-input-contract/src/parsers/chaninp.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/infile_chaninp_parser_contract.rs`
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`

## Phase Plan

1. Scaffold package and lineage artifacts.
2. Confirm legacy/default lineage from pinned baseline and existing parser spec.
3. Amend contracts/specification before production code.
4. Move watershed CLI absent sidecar behavior onto typed parser default output.
5. Make `WatershedNetworkFrame` accept explicit defaulted compatibility
   `chan.inp` outcomes as runtime-ready.
6. Add focused parser/frame/CLI tests for absent and malformed default behavior.
7. Run package gates and closure gates.
8. Dispatch/read subagent review and disposition findings.
9. Update artifacts, roadmap, and package catalog.

## Acceptance Criteria

- Missing/unreadable/malformed `chan.inp` in compatibility mode has one
  contract-authorized default branch with explicit warnings.
- Strict mode still rejects missing `chan.inp` when `ipeak > 2`.
- Watershed CLI no longer relies on a hidden `None` default branch for
  configured-absent `chan.inp`; runtime globals come from typed defaulted
  `ChaninpFile` options.
- Default branch values are tested: `ichout=0`, `nchnum=0`, `cbase=0`, no
  selected channel IDs, no channel output enabled, and deterministic
  timestep normalization.
- Public watershed outputs still publish successfully without `chan.inp`, and
  stderr/runfile warnings identify the compatibility default.
- No `gwcoeff`/generated-baseflow behavior changes.

## Required Gates

- Focused parser/frame/CLI tests named in `artifacts/gate-results.md`.
- `cargo check --workspace`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `markdown-doc lint --path docs/work-packages/20260709-wshedw10-chaninp-absent-default-authority-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/wepp-input-files/specs/chaninp.spec.md --path docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `git diff --check`

## Closure Statuses

`EXECUTED-COMPLETE`:

- All acceptance criteria pass with current evidence, and WSHED-W10 is no
  longer a `chan.inp`-absent default blocker.

`EXECUTED-HOLD-*`:

- Pinned legacy/default evidence contradicts the deterministic compatibility
  values, or the typed parser default cannot be consumed by watershed runtime
  without broad routing redesign.

## Final Disposition

Status: `EXECUTED-COMPLETE` (2026-07-09 UTC)

WSHED-W10 is closed. The absent/open-error `chan.inp` compatibility branch is
contract-ratified, typed at the parser boundary, consumed directly by watershed
runtime, and covered by parser/frame/CLI tests. No hidden `None` fallback remains
for unconfigured missing `chan.inp`.
