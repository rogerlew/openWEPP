# Heavy Attempt 13 Merged-Coverage Publication

Evidence class: Ran.

The delegated one-process transition executed from exact clean head
`32022d8cc4bd3e56c62233552b5886bf2648c1bd` with admission ID
`e6b44b77a3c5f1a3ff1890a5c1037d3f45258cfc6edaeb23815117738f44aa7d`.
It exited `0` and reported:

`quality-observation: PASS
id=f641feeda798047dac30ad7ef760bbadc31b71265e32415353be71b53e8b5544
debt=PASS actionable=0`

## Execution

| Profile | Result | Inventory | Duration |
| --- | --- | ---: | ---: |
| `full` | `PASS` | 2,279 / 2,279 | `3473.738s` |
| `science-manual` | `PASS` | 36 / 36 | `482.897s` |
| canonical workspace | exact union | 2,315 | n/a |

The profile inventories were disjoint and their union exactly matched the
independently enumerated workspace inventory. JUnit reconstruction reported
zero failures, errors, or skipped admitted identities.

## Coverage And Debt

- Coverage mode:
  `workspace-default-features-instrument-coverage-cfg-coverage`.
- Merged LCOV SHA-256:
  `bf1d37163d3c9ab92d8d9c8faa855307f58d370af4307b628c0f954156aa0677`.
- Merged files: 212.
- All 18 historical snowbench rows had a proven `science-manual`
  contribution, passed the snowbench gate, and retained no false debt.
- Production entries assessed: 11,432.
- Raw CRAP rows over 30: 2.
- Adjudicated rows: 2.
- Actionable rows: 0.
- Debt status: `PASS`.
- Execution integrity: `PASS`.
- `closure_eligible=false` is the required observational ADR-0041
  disposition; it is not a correctness or evidence failure.

## Publication

The bounded publication contains exactly 11 regular files totaling 1,421,222
bytes:

- `quality-envelope.json`
- `quality-payload.json`
- `run-status.json`
- `inventory-full.json`
- `inventory-science-manual.json`
- `inventory-workspace.json`
- `junit-full.xml`
- `junit-science-manual.xml`
- `adjudicated-crap-report.json`
- `adjudicated-crap-report.md`
- `coverage-summary.json`

The canonical payload indexes the eight subordinate artifacts totaling
1,409,797 bytes. Raw LCOV, profiles, build trees, and caches remain outside the
publication.

Durable evidence root:
`/home/workdir/openWEPP-quality-history/20260725-order3-local-attempt13-b3OXcS`.
