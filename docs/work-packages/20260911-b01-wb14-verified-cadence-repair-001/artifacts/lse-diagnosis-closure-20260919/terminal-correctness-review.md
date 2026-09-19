### Terminal independent correctness review — final LSE first-trial diagnosis

Reviewer: `/root/lse_terminal_correctness`  
Scope: final private observation source, source/build/input identity, recorded correction execution, noninterference, and inference boundaries. The reviewer did not author the source or design the next numerical method.

**Evidence class**

Static: inspected the actual three-file patch, detached final source, recorder and solver hooks, governing `SC-LANDSURFACEENERGY-001/solve-boundary.md`, retained receipts, trace, and QA review.

Ran: read-only hashing, snapshot reconstruction, JSON counts, byte comparisons, symlink checks, and external-input copy comparisons. No Rust build/test, evaluator call, model/reader run, acquisition, numerical continuation, or physical evolution was performed by this reviewer.

**Findings**

HIGH — Retained scientific finding, nonblocking for diagnostic closure: the R0/V11SnowCovered represented-snow Potential solve lawfully returns `LsebE034/NumericalBacktrackingLimit`. At iteration 6, all canonical factors `b=0..20` are domain-invalid; the first observed lawful factor, `b=21`, locally decreases the residual norm. This establishes a finite-grid/domain obstruction for this decision. It establishes neither an admissible root, eventual convergence, global Jacobian correctness, nor authority for extending the grid. Current behavior conforms to `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` and `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs`.

MEDIUM — Retained presentation defect: `covered_failure_residuals` at `solver_covered_solve.rs:372` assigns fixed flux/energy identities and units to branch-specific anchor rows. The dominant terminal value is the represented-snow ground-temperature anchor `(275.6524973010761−265.15) K / 1e-9 K`, not a ground-energy imbalance. This does not alter the refusal or invalidate the diagnosis.

MEDIUM — Retained presentation defect: failure construction at `solver_covered_solve.rs:1047` uses the previous accepted iteration-5 step norms when iteration 6 has no domain-valid evaluated trial. Those norms do not describe a rejected iteration-6 trial. The final private trace correctly labels their provenance. This does not alter the refusal.

No blocking finding was found in the final private observation source or recorded correction evidence.

**Terminal fix and identity verification**

- Read-only reconstruction verified the exact detached source at `/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918`: 748 entries, SHA-256 `8cd4866f66b06d6cf96403753cdb3d41ba75f6ca87652341ebe082a86a1b4bc5`. Its 747-entry baseline reconstructs to `85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963`. Exactly three paths differ: `lib.rs`, `solver_covered_solve.rs`, and the added `lse_first_trial_diagnosis.rs`.
- `artifacts/lse-first-trial-20260918/final-from-baseline-relative.patch`, SHA-256 `214a32362cd53708330843120a3bd59c1ee025928738ce201dae74ff82058fde`, contains all three files, including the complete 60-line new module. Its recorded applied-output hashes equal the actual final files.
- All five support links resolve to the recorded targets. All 19 external build-input originals and retained copies independently matched their recorded hashes and byte lengths. The dependency-file-derived manifest, Nix pins, and conservative source-referenced inputs support the stated complete bounded build identity.
- The frozen executable independently hashes to `551738aa5123cffa9b24f3e7e7b193cfaa95f2c16a803df5f2eb278ed9c12ba0`. The published and external `root-freeze.json`, final execution receipt, and final trace are byte-identical.
- The recorder is compiled only under `test`/`test-support`, remains crate-private, and activates only through `OPENWEPP_LSE_FIRST_TRIAL_TRACE`. The post-refusal probes run after the canonical failure object is constructed, use local trial vectors, never assign the solver iterate, and return the unchanged failure.
- The final trace explicitly records effective `R0`, `V11SnowCovered`, 60-second support, and `caps=null`. Policy code defaults an uninstalled selector to R0; the package name supplies no policy-14 inference.
- The final trace contains 95 unchanged ordinary records plus three diagnostic probes. Removing the R0 header and probes makes those 95 records byte-equal to the original trace; removing only the R0 header makes the complete final trace byte-equal to the prior directional trace. All three retained terminal diagnostic payloads are byte-equal to `reader-exact-02`, preserving transaction 41, OFE `ofe-1`, tile `forest`, iteration 6, 65 cumulative backtracks, and `LsebE034/NumericalBacktrackingLimit`.
- The directional and final runs each made three frozen and one unfrozen diagnostic evaluator calls: eight total, six frozen and two unfrozen. Final canonical iteration-6 search made zero evaluator calls because every `b=0..20` point failed domain admission. No diagnostic point was installed or counted as physical advancement.
- The recorded final build passed and its inventory selected exactly one test. The recorded correction execution exited 101 at the preserved typed refusal after 1.367 seconds, without retry, timeout, disk stop, source drift, binary drift, support-link drift, or external-input drift. These are inspected supplied execution records, not commands run by this reviewer.

**Residual risk and limitations**

The first two command receipts remain post-execution reconstructions. The first no-probe execution lacks a separately retained complete source cut; the second measurement cut is preserved. Historical `from-baseline.patch` omitted the recorder module and remains invalid recovery evidence. The contemporaneously bound final source, patch, build, binary, and execution do not retroactively repair those historical limitations.

No accepted `b=21` update or later solve was executed. Convergence, root existence, day-0 completion, reader reachability, original E008 correction, rollback/conservation beyond the reported hashes, production promotion, and cadence remain unproved. The two diagnostic presentation defects remain separate from the proximate refusal.

`/root/reader_qa`’s terminal PASS is reusable unchanged; this review found no evidence affecting its scope. Targeted strict Clippy remains an inherited 27-diagnostic FAIL as already recorded.

**Verdict**

APPROVE the final private observation patch and recorded correction execution for the bounded first-trial diagnostic closure only. The required terminal correctness review is satisfied with no blocker. This approval does not accept a numerical correction, reader integration, convergence claim, authority amendment, or production change.
