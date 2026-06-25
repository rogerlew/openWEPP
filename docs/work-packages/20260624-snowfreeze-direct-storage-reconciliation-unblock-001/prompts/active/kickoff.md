# Kickoff: SNOWFREEZE Direct Storage Reconciliation Unblock

Autonomy: execute this package end-to-end without asking for next steps unless a
declared HOLD boundary is reached.

Close defect `SNOWFREEZE-DRSTOR-001` end-to-end: `site3_scan_mandan_nd` direct
observed comparison fails before exit-0 metric-bearing report emission at
`storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`.

Close defect `SNOWFREEZE-DRSTOR-002` end-to-end: `site4_ggd498_morris_mn` direct
observed comparison fails before exit-0 metric-bearing report emission at the
same typed guard.

Correction Authority Envelope: this package may edit only the direct R4B
explicit frost storage projection bookkeeping and focused tests/artifacts named
in `package.md`. It may distribute a valid aggregate negative frost-storage
projection delta across available nonnegative layer liquid storage. It may not
loosen typed guards, silently clamp material negative storage, change frost
physics, change observation verdict thresholds, delete compatibility runtime,
or default-activate direct runtime.

Conversion rule: if reproduction, mechanism, ownership, authority, safety,
testability, and validation are all met for an in-envelope defect, land the
contract-safe correction in this package and do not relay another diagnostic
handoff.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to read-only runtime-review and verification subagents for
final diff review, guard-integrity review, site3/site4 evidence review, and
line-count/gate legitimacy review. Expected outputs are compact findings
summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260624-snowfreeze-direct-storage-reconciliation-unblock-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

On-demand:

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`
- `/home/workdir/openWEPP/tools/snowfreeze_observed/observed_harness.py`

Required-reading budget: see `artifacts/required-reading-map.md`.

Execution sequence:

1. Reproduce the two observed compare failures and record exact evidence.
2. Add focused regression tests for valid multi-layer debit and
   insufficient-storage fail-closed behavior.
3. Implement the minimal R4B projection correction.
4. Run focused tests and site3/site4 compare commands.
5. Complete artifacts, dual review disposition, verification, line-count
   governance, and final package disposition.
