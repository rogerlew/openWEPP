# PERFDEEP09 Attribution Loop

Status: complete.
Evidence class: Static + Ran.

| Loop | Symptom | Evidence | Attributed mechanism | In envelope | Next action |
|---|---|---|---|---|---|
| 0 | Current branch missed P0 gate | No-edit control `682.65 s`, RSS `228924 KB` | default-disabled overhead persists | yes | screen ranked candidates |
| 1 | Symbol lookup appeared in prior profiles | PERFDEEP05 opt-in profile and static registry lookup | private `SymbolRegistry` reverse map shape | yes | patch and screen |
| 2 | Candidate 1 slower and identity-risky | `689.30 s`, RSS `229352 KB`; PASS raw checksum drift | `HashMap` registry lookup not retained | yes | revert and continue |
| 3 | Prior default profile named decomposition overflow guard | PERFDEEP04 default profile: `9.18%` children / `7.72%` self; static repeated scans | seven full-map indexed overflow scans per perennial control | yes | collapse to one pass |
| 4 | Candidate 2 screened below gate | `634.61 s`, RSS `228856 KB` | one-pass guard removes default-path scan tax | yes | run final three reps |
| 5 | Final median passed | `634.61/635.65/636.58 s`, median `635.65 s` | blocker cleared | yes | close `READY-FOR-R2` |
