# Gate Results

Evidence mode: Static + Ran.

Status: executed-held.

| Gate | Status | Evidence |
|---|---:|---|
| Current cutover failure reproduced | PASS | Direct CLI cutover exits `1`, emits `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`, reaches HBP byte comparison, and writes no outputs. |
| Blocker ledger complete for executed scope | PASS | `r6e-blocker-ledger.md` records R6E-B001 through R6E-B005 with dispositions. |
| Production direct-runtime input binding | PASS | Focused unit test proves direct run-frame/executor/capture counters execute, direct compute/state/downstream/shadow counters are nonzero, and the old B003 marker is absent. |
| Direct producers parity-grade | BLOCKED | Held at HBP direct process parity; current direct process operands are not accepted as public parity-grade output authority. |
| Direct consumer path cutover | BLOCKED | Cutover stays fail-closed before public writes while HBP byte identity fails. |
| HBP byte identity | BLOCKED | Direct and compatibility HBP byte lengths are both `1654`, but bytes differ. |
| WAT Arrow/schema/metadata parity | BLOCKED | WAT writes remain blocked behind HBP parity. |
| PASS Arrow/schema/metadata parity | BLOCKED | Current CLI fixture lacks a PASS Parquet target and writes remain blocked behind HBP parity. |
| loss JSON identity | BLOCKED | Loss writes remain blocked behind HBP parity. |
| manifest provenance/checksum parity | BLOCKED | Manifest writes remain blocked behind HBP parity. |
| Anti-alias fixtures | BLOCKED | No public output family accepted; anti-alias fixtures remain required before cutover closure. |
| Independent reconstruction | BLOCKED | No public output family accepted; reconstruction remains required before cutover closure. |
| No-compatibility proof | PASS | Static cutover scan has no forbidden compatibility sources; focused tests prove zero skeleton-run and compatibility-edge counters. Complete no-compatibility proof still waits for successful cutover. |
| Default-disabled isolation | PASS | Cutover-only helper returns `Ok(None)` outside `DirectPublicationFrameCutover`; default compatibility path is not activated by R6E changes. |
| Line-count governance | PASS | `00_runner_intake_and_lane_setup.rs` is `2787` lines; touched WARN-band files remain below hard threshold. |
| `cargo fmt --check` | PASS | Ran with clippy command; passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Passed. |
| `cargo test --workspace` | PASS | Full workspace test suite passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| docs lint | PASS | `wctl doc-lint --path docs/work-packages` validated `960` files with `0` errors and `0` warnings. |
| `git diff --check` | PASS | Passed. |

## Focused Commands Run

```bash
cargo test -p openwepp-runner \
  r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity \
  -- --nocapture
cargo test -p openwepp-runner \
  r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity \
  --test r6_direct_publication_cutover_cli_contract \
  -- --nocapture
```

Both focused tests passed.

## Final Commands Run

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
wctl doc-lint --path docs/work-packages
git diff --check
```

All final commands passed.
