# Shadow Replay Scorecard

Evidence class: `Ran`

Command:

    cargo nextest run -p openwepp-gate-planner

The initial retained replay suite passed 14/14. Later directly affected focused
checks also passed, including the final deletion/reconciliation regression.

The retained replay fixture
`gate-policy/v1/fixtures/replay/retained-selection-cases.json` produced:

| Case | Expected result | Observed result | Disposition |
| --- | --- | --- | --- |
| TESTGATE-ALIGN gate-policy change | critical plus schema consistency | critical; registered gate selected; no unmapped input | PASS |
| Ordinary Rust package change | owning package plus Cargo reverse dependents | `openwepp-comparator-metadata` and root `openwepp` selected | PASS |
| Cargo lock/build-graph change | critical fallback | critical with `CARGO_GRAPH_OR_BUILD_INPUT_CHANGED` | PASS |
| Unknown authority-like path | never narrow or return zero work | one unmapped record and critical escalation | PASS |

Additional negative coverage rejects duplicate JSON, floats, invalid Git raw
shapes, legacy-to-pass ledger promotion, retroactive deferral, empty required
inventory, receipt inventory drift, unsafe reuse, revoked issuers, and
assurance currency on an unresolved impact.

Scorecard blockers:

- selection misses: 0;
- unexplained empty inventories: 0;
- nondeterministic canonicalization: 0;
- accepted identity drift: 0;
- accepted unsafe reuse: 0.

This scorecard is shadow evidence only. It does not satisfy TESTGATE-CI-01's
observation/cutover scorecard, authorize reduced testing, or override the
terminal CRAP failure.
