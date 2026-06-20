# R2A Kernel Profile Compliance Checklist

Status: complete.
Evidence mode: Static + Ran.

| Requirement | Status | Evidence |
|---|---|---|
| No process-physics formula changes | PASS | Static diff review: only direct skeleton structural types, selection plumbing, tests, and package docs changed. |
| No output schema/unit/metadata meaning changes | PASS | Static diff review plus protected H2637 output identity evidence. |
| No publication cutover | PASS | Compatibility output path remains production path; direct skeleton has no output writer. |
| Direct mode inactive by default | PASS | Default runner selection is `Compatibility`; default fixture test leaves all direct skeleton counters zero. |
| Direct-frame storage excludes compatibility types | PASS | Direct runtime source-token prohibition test and `rg` scan passed. |
| Direct skeleton avoids forbidden compatibility calls | PASS | Direct runtime source scan passed; `scheduler.rs` has no diff. |
| Default-disabled H2637 regression gate passes | PASS | Median `636.01 s <= 676.67 s`. |
| Gate Evidence Non-Deferral checked | PASS | Full Rust gates, markdown lint, benchmark, proof scans, review, verification, and disposition are recorded before closure. |

Review correction:

- Runtime counters prove default-disabled inactivity and explicit skeleton
  execution only.
- Forbidden compatibility calls are proven by static direct-runtime source scan
  and no scheduler diff, not by reserved zero-only counters.
