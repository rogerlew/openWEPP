# Rust Code Review

Status: `PASS — no open Rust correctness blockers`

Evidence mode: `Static + Ran`

Reviewer independence: the verdict was derived from the package, canonical
science contracts, actual working-tree diff, production sources, and focused
tests. No other review artifact was used to determine the findings.

## Findings

No open findings.

The following findings were accepted, corrected in the reviewed tree, and
reverified:

| Finding | Severity | Closed disposition |
|---|---|---|
| `WAT5-RCR-001` | HIGH | `crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs` now refuses an existing target with a typed error, publishes the completed temporary inode with no-replace hard-link semantics, and removes temporary files on failure. Existing-target preservation and failed-validation cleanup are tested. |
| `WAT5-RCR-002` | HIGH | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` validates WAT5 hyetograph support inside `[0, 86_400] s` before splitting. The bounded day clock removes the former unbounded-allocation/non-progress path; negative and after-day vectors fail as `WAT5-E-003`. |
| `WAT5-RCR-003` | HIGH | The raw profile now carries depression-storage change and `subhourly_generation.rs` validates `rainfall = infiltration + post-depression generation + depression-storage change` before hourly normalization. A positive-depression-storage vector covers the identity. |
| `WAT5-RCR-004` | MEDIUM | Hourly and five-minute overlap allocation now share `add_depth_to_fixed_bins`; hourly and five-minute earliest-depth removal share `remove_depth_from_bins_earliest`. The duplicated conservation-sensitive arithmetic was removed. |
| `WAT5-RCR-005` | MEDIUM | `DirectDayFrame` stores the opt-in event as `Option<Box<DirectFiveMinuteGenerationEvent>>`. The diagnostics-off resident layout remains within the enforced 15,600-byte ceiling rather than weakening the guard. |
| `WAT5-RCR-006` | MEDIUM | `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` uses named typed `m -> mm` and `m s^-1 -> mm h^-1` boundary conversions, including a finite signed-residual wrapper. Known-vector and invalid-domain tests pass. |
| `WAT5-RCR-007` | MEDIUM | `crates/openwepp-hillslope-output/src/contracts.rs` rejects pairwise duplicate optional output targets before any writer opens, after lexical normalization of `.` and `..`. Direct WAT5 aliases with WAT/PASS and a parent-component alias are tested. |
| `WAT5-RCR-008` | MEDIUM | `HillslopeWatSubhourlyError` now exposes stable `OHOUT-WAT5-E-001..003` codes, and every publication variant binds its display to contract guard `WAT5-E-005`. |
| `WAT5-RCR-009` | MEDIUM | Before publication, the WAT5 writer reopens the completed temporary Parquet file and verifies physical row count plus the complete Arrow schema/metadata against the expected values. A forced count mismatch proves that neither target nor temporary file remains. |

Static review also found no new production `unwrap`, `expect`, or panic path;
no WAT5 value enters HBP, watershed routing, public peak, or erosion; and the
power-equivalent columns remain null under `water_only_no_erosion_adoption`.
The optional replay runs only after the authoritative WB14/WB19 hourly surface
and returns immediately when WAT5 is not requested.

### Terminal A0 correction re-review

The terminal A0 correction does not change production runtime Rust. Its Rust
delta adds fail-closed contract assertions in
`tests/integration/advisory_linter_authority_contract.rs` and
`tests/integration/subhourly_generation_contract.rs`. Static inspection found
the final authority bindings internally consistent:

- explicit `--worktree` admission compares the base to the complete tracked
  worktree, adds untracked paths, and fingerprints every science surface plus
  the contract and policy inputs used to admit it;
- all 17 changed WAT5 science-crate paths have one atomic
  `SC-OUTPUT-WAT5-001` mapping, while the four shared Plant/WAT5 paths retain
  exactly two separate atomic mappings rather than merging authorities;
- the blocking exact-inventory A1 definition covers all five affected Rust
  packages; and
- approved, active `SC-OUTPUT-WAT5-001` owns `TOL-WAT5-001`. The draft
  `SC-WATBAL-001` amendment is absent, and its existing peak invariants
  `INV-WATBAL-102..104` remain unchanged.

No arithmetic, clamp/guard precedence, unit conversion, domain handling,
serialization, or runtime error-taxonomy behavior changed in this correction.
No new finding was opened.

## Residual Risk and Missing Tests

Ran from `/home/workdir/openWEPP` against the final reviewed working tree:

- Orchestrator WAT5 behavior plus the day-frame layout guard: PASS, 18/18,
  nextest run `c3cc26c2-a8df-431f-97e4-f09577bedf7c`.
- WAT5 writer, completed-file validation, error taxonomy, and output-contract
  path guards: PASS, 11/11, nextest run
  `61c232b9-ef36-43da-a22e-d74d7a0e4e48`.
- Peak/WAT5 contract, property, round-trip, and HBP-exclusion targets: PASS,
  13/13, nextest run `d7fa0d54-ee3e-4f7a-a6dc-d03bbf3b959c`.
- Runner named unit-boundary conversions: PASS, 1/1, nextest run
  `d40f5549-38c1-423c-8668-0df9c970ec90`.
- Recorded final authority repair: PASS, 11/11 twice, nextest runs
  `b9c026ee-5708-4fca-ad63-0a0040d5ff9e` and
  `fb68f3d0-4f4a-4a9d-aa39-46c3a3dd7f72`.
- Exact current three-target authority set, including the added worktree
  fingerprint assertion and required-suite obligations: PASS, 12/12, nextest
  run `509cefc4-f07d-4ac3-9194-52d3923fc200`.
- Exact worktree science-contract admission: PASS, 43 contracts and 17 science
  surfaces, authority SHA-256
  `6f95845b5065e9134cded858e69ed359b2e42bd32318f800f87801d4088d1298`.
- Exact post-A0 full workspace: PASS, 2,380/2,380 with 33 skipped,
  nextest run `b920db77-070f-4686-a7bf-2e2727094374`; post-A0 workspace
  doctests: PASS, zero failures.
- Authority-suite anti-evasion shell guard, changed-test Clippy with
  `-D warnings`, `cargo fmt --all -- --check`, and `git diff --check`: PASS.

The package now records refreshed exact-source real-consumer evidence:
independent Parquet reconstruction closes the emitted hourly water depths,
diagnostics-on/off protected outputs are byte-identical, and the incomplete
positive-supply case fails through typed `WAT5-E-001` without publishing a
target or temporary file. These real workflow receipts complement rather than
derive from the source-level integration assertions.

The exact post-A0 campaign supersedes the earlier 2,379-test receipt and closes
the prior pending-full residual. It ran after the final authority repair while
the reviewed production runtime Rust remained unchanged. No Rust correctness
or validation residual remains from the terminal A0 correction.

Output-path equality is lexical. A symlink can still alias two differently
spelled paths because the repository has no output-path symlink/canonicalization
policy; this is a filesystem-level residual rather than a WAT5 arithmetic or
no-replace defect.

The refreshed line-count artifact records named split intents for the three
WARN files: `crates/openwepp-runner/src/hillslope/03_tests.rs` (2,905 lines),
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` (2,869),
and `direct_runtime/00_core_frames.rs` (2,713). No reviewed changed Rust file
reaches the 3,000-line closure threshold.

## Gate-Legitimacy Check

`PASS` for the Rust code-review gate. All current-scope Rust findings are
closed on the reviewed diff; none was deferred. Exact post-A0 full-workspace
and doctest requirements are satisfied; any remaining package lifecycle
reconciliation is parent-owned administration.

## Verdict

Approved for Rust correctness review. No implementation blocker remains in
the reviewed WAT5 opt-in water-output path.
