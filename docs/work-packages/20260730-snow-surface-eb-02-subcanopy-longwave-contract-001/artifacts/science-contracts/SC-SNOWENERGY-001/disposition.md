# Contract Review Disposition

Status: `all findings accepted, corrected, and verified / PASS`.

Evidence class: Static + Ran.

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| `EB02-RA-01` | Review A | High | accepted | Restricted unity emissivity to canopy/snow; retained variable atmospheric effective emissivity throughout. | `SC-SNOWENERGY-001` Scientific Scope, Algorithm Specification, Constants | Removes a real equation contradiction. |
| `EB02-RA-02` | Review A | High | accepted | Bound hourly `T_a` evaluation with daily `e_a` and clearness-derived cloud state held across the day; added immutable hourly/cadence vectors. | contract Variables/Algorithm/INV-013; `analytical-test-vectors.csv` | Prevents an unauthorized nonlinear daily-mean substitution. |
| `EB02-RA-03` | Review A | Medium | accepted | Added finite `[0,1]` derived-emissivity authority guard, typed no-clamp failure, executed out-of-authority vector, and retained transfer gap. | contract Algorithm/Branch/INV-014/GAP-006 | Separates physical output admissibility from empirical transfer claims. |
| `EB02-RA-04` | Review A | Medium | accepted | Bound equivalent homogeneous/random-orientation/isotropic-diffuse regime and structural-floor optical-depth interpretation. | contract Scientific Scope/Algorithm/Branch; `canopy-sky-view-derivation.md` | Makes the inferential bridge and excluded geometry explicit. |
| `EB02-RA-05` | Review A | Medium | accepted | Executed invalid/non-finite cover, temperature, vapor, radiation, cloud, and flux guards; replaced unreachable `k_t=-0.1` with `k_t=0`; executed polar-night unavailable branch. | `tools/execute.py`; `analytical-test-vectors.csv` | Evidence now tests the guards it claims. |
| `EB02-B-001` | Review B | High | accepted | Expanded all 14 invariants to the required authority/evidence/guard/failure schema; added full Guard Map; mapped INV-013/014 in BEI. | contract Invariants and Guard Map; Binding Exposure Index | Restores profile completeness and traceability. |
| `EB02-B-002` | Review B | High | accepted | Added owner contract to aliases and per-symbol registry/helper/scalar/publication dispositions to Unit Governance Map. | contract Symbol Alias Map; Unit Governance Map | Makes future dimensional boundaries and ownership explicit. |
| `EB02-B-003` | Review B | High | accepted | Replaced self-derived expectations with immutable expected values, calculated numeric status from contract tolerances, required exact case/HOLD vocabulary, and asserted monotonic response. | `tools/execute.py`; `contract-test-evidence.md` | A formula regression can no longer regenerate itself as a pass. |
| `EB02-B-004` | Review B | Medium | accepted | Added the ADR-0042 status triple and all ten readiness obligations with evidence/rationale. | contract Calibration and Identifiability; `calibration-readiness-matrix.md` | Distinguishes canonical authority from absent runtime and inapplicable calibration. |
| `EB02-B-005` | Review B | Medium | accepted | Assigned EB-03 coherent thermal provider, canopy-temperature interface, polar-night policy, and `R_a,min`; rewrote EB-01/01A rows as dated outcomes. | snow-surface energy-balance roadmap | Makes the runtime-hold lift owner discoverable. |

No finding was rejected or deferred. Corrected-tree Review A and B both pass;
their exact-tree hashes and rerun evidence are recorded in the adjacent review
artifacts.
