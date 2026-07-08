# Gate Results

Status: complete.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran `git diff --check`; command produced no output. |
| Markdown/doc lint | PASS | Ran scoped `markdown-doc lint`; output: `20 files validated, 0 errors, 0 warnings`. |
| SC unit compliance | PASS | Ran `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output: `PASS: SC unit compliance lint found no findings`. |
| BEI non-strict | PASS-DEFERRED | Ran `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output: `PASS-DEFERRED ... 10 binding exposure row(s), 9 science-review-follow-on row(s) not yet consolidated`. |
| BEI strict | DEFERRED-NONZERO | Ran `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; output repeats `PASS-DEFERRED` and exits nonzero because existing SC-OFEROUTE deferred rows are not consolidated. This package does not claim strict consolidation. |
| Rust gates | NOT APPLICABLE | No Rust code or tests changed. |
| Comparator/timing gates | NOT APPLICABLE | No runtime behavior changed. |
| Anti-evasion guards | NOT APPLICABLE | No cohort fixture, required-case binding, external-authority suite posture, or test obligation posture was touched. |
