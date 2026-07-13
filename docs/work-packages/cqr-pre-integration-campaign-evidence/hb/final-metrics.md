# High-B Final Metrics

Evidence class: **Ran + Static**

## Measurement

- Source commit: `a7737e3d4e3b27c11e60a545a4b65741860f5da5`.
- Workspace was clean before measurement.
- Binding slug/phase: `hb` / `final`.
- LCOV: exit 0, `35:23.47`, max RSS `891,376 KB`.
- JSON: exit 0, `35:43.26`, max RSS `829,820 KB`.
- CRAP: exit 0, `0:01.17`, max RSS `209,572 KB`.

| Artifact | SHA-256 |
| --- | --- |
| `final.lcov` | `89e82f8c17eadc54acda2206b721e05dac73907a0a0a25e5cf0c8b53fb7684a1` |
| `final.json` | `e7088349ed830f636f0ddffb45ac535de91968a76078b47b286c647bb90cec02` |
| `final-crap.json` | `34b49799188e7e897ebb6c3b18fe4617471f5df5ea1fc70adc07915fd02698ca` |
| `production-over30.json` | `292c40b7d9eb3dd757c6d4d8cf3e4656bb9bfea00f845e5d065ba37e3fa37118` |
| `hb-target-rows.json` | `37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570` |

## Rerank

The production-over-30 census falls from 54 rows/35 modules at High-B start
to 32 rows/25 modules. Stable `(file,function)` comparison finds 22 removals,
zero new identities and zero common-row CRAP regressions. All 21 fixed High-B
rows are removed; the additional removal is `GwcoeffParseError::fmt`.
`hb-target-rows.json` is empty (`[]`). No touched High-B module contains a new
or regressed over-30 identity.

The 32 retained rows predate High-B and remain the actionable input for the
next tranche rerank; this measurement introduces no new identity requiring a
High-B defect or module reopening.

## Ignored-Run Attribution

Both formats reproduce only known shared-environment failures: the H2637
active/conflict selector family and the R3C lane-transfer audit-counter test.
LCOV observes three H2637 failures; JSON observes four. Both observe the same
single R3C failure. Fixture-runner `FAILED` text for expected negative fixtures
is not a Rust test failure. No High-B target-related or unattributed failure is
present, and no retry was performed.
