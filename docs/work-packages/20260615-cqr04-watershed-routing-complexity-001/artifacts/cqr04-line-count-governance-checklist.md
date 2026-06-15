# CQR04 Line Count Governance Checklist

Static:

- Touched Rust file:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
- Before line count: 1934.
- After line count: 2807.
- Governance threshold: 2000 lines.
- Package allowed write set: intra-file refactor only.
- Package excluded scope: no module/file split beyond this file.

Disposition: WARN hold.

The file now exceeds the line-count threshold because helper extraction was
constrained to a single module. This is accepted for CQR04 because the package
objective was CRAP closure without module split. Follow-on owner is the next
watershed routing refactor package or maintainer-authorized module split.
