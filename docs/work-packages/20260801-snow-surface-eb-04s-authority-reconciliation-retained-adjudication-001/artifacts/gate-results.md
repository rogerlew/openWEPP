# Gate Results

Evidence mode: `Static + Ran`.

| Gate | Result | Evidence |
|---|---|---|
| Result-blind authority whitelist | PASS | `authority-freeze.json`; exactly four hashed pre-result inputs. |
| Dimensional derivation | PASS | `1e-9 m * 1000 kg m^-3 = 1e-6 kg m^-2`. |
| Dual authority review | PASS | Both reviewers returned `GO_WITH_AMENDMENTS`; all findings accepted. |
| Contract amendment verification | PASS | Agent A `PASS`; agent B `PASS_WITH_NOTES`; exact version-6 contract hash matched. |
| No model rerun | PASS | Adjudicator subprocess guard; report records zero simulation subprocesses. |
| Immutable input trees | PASS | EB-04R package and retained-output tree hashes identical before/after. |
| Frozen input bindings | PASS | Current tool/protocol, crates/tests trees, 12 fixture trees, 12 observation files with roles/filters, selectors, and eight decision dependencies match the executed attempt before scoring. |
| Retained provenance | PASS | 48/48 records and 288/288 retained file identities reconcile. |
| Physical population | PASS | 48/48 cells; maximum vapor-to-sublimation residual `8.109983287707401e-8 <= 1e-6 kg m^-2`. |
| EB-04R vapor aggregation | PASS | Maximum `7.993605777301127e-15 <= 1e-12 kg m^-2`, preserving the stricter frozen protocol. |
| Observation sequencing | PASS | Observations loaded only after complete population physical/provenance PASS. |
| Independent decision reconstruction | PASS | All aggregates, protected groups, failures, compensation, and criteria reconstruct. |
| Empirical promotion rule | FAIL (scientific nonpromotion, not package HOLD) | Score improves `177 -> 180`; robust failures remain `16 -> 16`; frozen rule requires both. |
| Contract admission baseline registry | PASS | `check_science_contract_admission.sh --base-ref HEAD --head-ref HEAD`: 40 admitted contracts; no committed science-surface diff. |
| Scoped SC unit compliance | PASS | `check_sc_unit_compliance.py --path SC-SNOWENERGY-001.md`: zero findings. |
| Markdown | PASS | 25 package files and four shared docs scanned; zero errors/warnings. |
| Python syntax and self-checks | PASS | Both package tools compile; Phase-A prospective self-check passed before freeze; terminal authority-seal verification and retained-only self-check pass after amendment. |
| Diff/security/line count | PASS | Diff check clean; no Rust changes; security and write set reconcile. |

Package outcome: `CLOSE_NONPROMOTION_EMPIRICAL_RULE`.
