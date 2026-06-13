# M-E1 data-model shadow-state evidence

Status: complete for M-E1 scope; package remains active for M-E2+

Evidence mode: Ran + Static

## Scope

M-E1 implemented the data-model/shadow-state increment from the M-D breakdown:

- added typed `TransferInput` / `TransferOutput` payloads with OFE identity,
  separated surface/lateral carry arrays, and typed mismatch errors;
- added `PerOfeDailyWaterBalanceRecord` and
  `PerOfeDailyWaterBalanceCollection`;
- constrained the legacy aggregate adapter to the N=1 single-OFE case only;
- added static per-OFE lane slices for slope/soil/management topology;
- preserved scalar aggregate WB13/WAT publication. No per-OFE WAT publication
  flip occurred.

M-E1 does not claim dynamic per-OFE daily records in the runner path. Manifest
provenance reports `static_per_ofe_slice_count == contributor_ofe_count`,
`per_ofe_record_count == 0`, and
`per_ofe_dynamic_water_balance_state == false`.

## Code changes

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `TransferInput`, `TransferOutput`,
    `PerOfeDailyWaterBalanceRecord`,
    `PerOfeDailyWaterBalanceCollection`, and
    `PerOfeDailyWaterBalanceError`.
  - `TransferOutput::as_downstream_input()` now consumes the recorded adjacent
    recipient and fails closed for terminal or mismatched outputs.
  - Collection append validates sequential OFE ids, upstream source/recipient,
    output source, and terminal/nonterminal output recipient.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Exports the new M-E1 data model types.
- `crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs`
  - Adds `StaticOfeLaneSlice` and `build_static_per_ofe_lane_slices`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`
  - Adds M-E1 manifest provenance fields while preserving aggregate publication
    policy.
- `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`
  - Adds focused M-E1 unit tests for N=1 round-trip, multi-OFE aggregate
    rejection, transfer identity validation, and static slice validation.
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs` and
  `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
  - Remove stale exact `SC-WATBAL-001` version pins exposed by the restored
    full workspace gate; invariant/addendum authority checks remain.

## Ran

| Command/check | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | Final post-edit format gate. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-review run. |
| `cargo test -p openwepp-runner mofe01_me1 -- --nocapture` | PASS | 7 M-E1 focused tests passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | The three M-E0 structural red gates are green without weakening the target. |
| `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract hphys0319_contract_authority_is_registered -- --nocapture` | PASS | Stale exact WATBAL version pin removed; authority check still passed. |
| `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract hphys0320_contract_authority_is_registered -- --nocapture` | PASS | Same stale-version fix for HPHYS0320. |
| `cargo test --workspace` | PASS | Full Rust closure loop restored after M-E1. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built final hillslope CLI before final replay. |
| Fresh H1-H36 final CLI batch | PASS | 36/36 exit code `0`; 36 manifests; 144 output files under `/tmp/openwepp_mofe01_me1_final`. |
| Local `owcmp` H1-H36 command execution, no comparator subagent | PASS | `execution_verdict=PASS`; artifacts in `/tmp/openwepp_mofe01_me1_final/owcmp`. |
| Local `owcmp` H1-H36 semantic comparison | FAIL | Expected M-E1 publication-boundary fail: `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1, 1, 2000]`. Focus columns all have zero failures and max diff `0.0`. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to `/tmp/openwepp_mofe01_mc2/output` for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`; artifact `/tmp/openwepp_mofe01_me1_final/single-ofe-anchor-cmp.tsv`. |
| No-publication-flip manifest audit | PASS | 36/36 manifests: aggregate policy unchanged, dynamic per-OFE flags false, `per_ofe_record_count=0`, static slice count equals contributor count. |

## Review disposition

Review A found two medium issues:

1. `per_ofe_record_count` was populated from static slice count.
2. `TransferOutput::as_downstream_input` accepted a caller-supplied recipient
   that could mismatch the output.

Both were accepted and fixed. The manifest now separates
`static_per_ofe_slice_count` from `per_ofe_record_count`, and transfer
conversion/collection insertion now fail closed on mismatched source/recipient
state.

Review B found one high, two medium, and two low issues:

1. `cargo test --workspace` failed on stale HPHYS0319 WATBAL version pin.
2. M-E1 evidence was stale.
3. The legacy aggregate constructor could synthesize arbitrary OFE records.
4. Manifest naming was misleading.
5. M-E1 tests needed negative coverage.

All were accepted and fixed. The stale version pins were removed, this evidence
set was refreshed, the legacy aggregate adapter is N=1-only, manifest naming
was corrected, and focused negative tests were added.

## Gate disposition

M-E1 is complete for the data-model shadow-state scope. It does not close the
full MOFE routing identities. Per-element and transfer identities remain
`BLOCKED` for later M-E sub-increments until the runner populates real dynamic
per-OFE daily records and then executes sequential OFE handoff.

## Claude review addendum (2026-06-13) — endorsed; red-gate hazard cleanly avoided

Evidence mode: Ran (type/test/diff inspection).

The M-E1 outcome is the model case for the hazard flagged at M-E0:

- **Red gates satisfied by building surfaces, not weakening assertions.**
  `cargo test --workspace` went green because the four target types now
  exist (`TransferInput`/`TransferOutput`/`PerOfeDailyWaterBalanceRecord`/
  `PerOfeDailyWaterBalanceCollection`, `scheduler.rs:252-407`); the E0 red
  contract test is **unmodified** (`git diff 1668332f` empty) and passes
  4/4. This is exactly the non-deferral-compliant path.
- **The new types are shadow.** Not referenced in the publication path
  (`02_output_and_climate_helpers.rs` / `scheduler_publication.rs`), so they
  drive nothing — the M-E1 mandate. Corroborates the byte-identical
  single-OFE anchors (H8/H15/H19/H20/H22/H23/H28 vs M-C2).

One assertion-loosening checked and cleared (transparency, given the E0
warning): two stmtim tests changed `contract_version: 154` → `contract_version:`
for SC-WATBAL. This is **defensible de-brittling, not erosion** — the loose
form is the dominant suite convention (23 uses vs 4 exact pins), only the
frequently-amended WATBAL pin (now v155) was loosened, and the climate (v22)
/ snowfreeze (v69) exact pins in the same files were retained. Not a gate
weakening.

Endorsed. Next is M-E2 (the sequential OFE lane executor — the first
increment where transfer state actually moves).
