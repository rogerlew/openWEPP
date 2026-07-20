# Kernel Profile Compliance

Evidence mode: `Static + Ran`

Status: `PASS — CP-GSI02 profile obligations verified and contracts active`

| Profile obligation | CP-GSI02 evidence | Result |
|---|---|---|
| Purpose and authority anchors | SC-PLANT purpose, Jolly et al., pinned baseline canopy relation | pass |
| Variables/units and state surfaces | typed native operand table and daily algorithm | pass |
| Reproducible algorithm and branches | numbered seven-step algorithm; first-day, daily, cap, and failure branches | pass |
| Guard/invariant map | INV-PLANT-033 through 037 plus schema/runtime typed failures | pass |
| Alias map | canonical native symbols mapped to YAML, projection, typed API, growth, and residue names | pass |
| Constants/parameters | existing constants table and CP-GSI02 operand table | pass |
| Unit governance | SC-PLANT map records typed paths, scalar registry gap, conversion helper, and no-output disposition | pass |
| Tolerances/numerics | exact domains, roundoff-only mass closure, one-day transformed phase gate | pass |
| Test vectors | endpoint, cold-start, closure, strict domain, full phase, replay, and real-consumer tests | pass |
| Gap/promotability | no provisional physics; dual verification passed and all three contracts are active | pass |

Ran focused plant, schema, runner, integration, formatting, and selected Clippy
commands recorded in `implementation-and-test-evidence.md`. External Level-4
authority suites do not register any of the three touched contracts; A3 remains
the required repository-wide authority node selected by TESTGATE. Exact C13
passed A0 admission, all three canopy A1 gates, A3, full workspace, and global
CRAP; receipt `ed12971d...` passed dual independent verification.
