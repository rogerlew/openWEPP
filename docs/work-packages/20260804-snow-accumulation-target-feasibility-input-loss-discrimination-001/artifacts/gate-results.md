# Gate Results

Status: `complete / pass`

Evidence mode: `Ran`

| Gate | Result |
|---|---|
| package-local Python compilation | PASS, four tools |
| focused operator and custody tests | PASS, 6/6 |
| independent retained-table reduction | PASS, `LOSS_PRIORITY_SIGNAL` reproduced |
| package and accepted-target JSON parsing | PASS |
| rejected-v1 versus accepted-v2 scientific CSV identity | PASS, five tables byte-identical |
| PRCPSA current versus intake Git content identity | PASS |
| tracked evidence-manifest hash reconciliation | PASS |
| four figure/source hash chains and visual inspection | PASS |
| targeted Markdown lint and link/path checks | PASS, zero warnings |
| `uk2us` spelling preview | PASS, no changes proposed |
| external-authority anti-evasion script | PASS |
| `auth11_required_suite_obligation_guards_contract` | PASS, 3/3 |
| exact write-set reconciliation | PASS, 42 paths and no production diff |
| `git diff --check` | PASS |

The accepted analysis streamed 61,364 exact retained trace rows. Maximum
storage closure is `9.9973e-13 m`, and maximum accumulation residual is
`3.4368e-13 m`, both below the frozen `1e-9 m` tolerance.

No full-workspace Rust regression was run. The exact production, kernel,
contract, fixture, and observation diff is empty; the applicable package-local
and authority-cohort gates above were selected directly.
