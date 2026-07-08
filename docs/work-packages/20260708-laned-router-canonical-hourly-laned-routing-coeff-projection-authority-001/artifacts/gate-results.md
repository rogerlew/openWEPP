# Gate Results

Status: complete.

| Gate | Status | Evidence |
|---|---|---|
| Ordering prerequisite | PASS | Static: M-T2A final disposition is `EXECUTED-COMPLETE-AUTHORITY`; handoff is M-T2B-ready. |
| Source-line evidence for projection claims | PASS | Static: baseline `frcfac.for`, `param.for`, `bigout.for`, and `watbal_hourly.for` line evidence recorded in `legacy-cropland-source-audit.md`. |
| BEI non-strict | PASS-DEFERRED | Ran `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output: `PASS-DEFERRED ... 9 binding exposure row(s), 8 science-review-follow-on row(s) not yet consolidated`. |
| BEI strict | DEFERRED-NONZERO | Ran `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output repeats `PASS-DEFERRED` and exits nonzero because existing deferred rows are not consolidated. This is recorded truthfully; rev 48 adds one mapped BEI row and does not claim strict consolidation. |
| SC unit compliance | PASS | Ran `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output: `PASS: SC unit compliance lint found no findings`. |
| Markdown/doc lint | PASS | Ran `markdown-doc lint --path docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001 --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/wepp-input-files/specs/plant-file.spec.md --path docs/ROADMAP.md --path docs/work-packages/README.md`; output: `24 files validated, 0 errors, 0 warnings`. |
| `git diff --check` | PASS | Ran `git diff --check`; command produced no output. |
| Rust gates | NOT APPLICABLE | No Rust production or test code changed in this authority package. |
| Runtime/comparator gates | NOT APPLICABLE | Projection authority held; no implementation or runtime eligibility broadening. |
