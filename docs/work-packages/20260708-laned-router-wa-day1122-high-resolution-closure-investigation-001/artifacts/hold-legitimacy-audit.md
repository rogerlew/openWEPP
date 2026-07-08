# Hold Legitimacy Audit

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Ran.

## Hold Condition

The active router exhibits mesh-sensitive positivity-clamp amplification on the
WA selected-cohort member. The fine diagnostic rungs fail the active day
cascade guard at day 1122, and completed coarser rungs show large
router-internal clamp/storage/outlet magnitudes on day 1418.

## Evidence

| Evidence | Result |
|---|---|
| Current WA ladder rerun | `baseline_fixed10`, `dx20`, `dx10`, `dx5` PASS; `dx2p5`, `dx1p25` FAIL. |
| `dx2p5` day 1122 | `857952342.2988955 m3` clamp, `857412328.3966482 m3` storage, residual `-0.0001100301742553711 m3`. |
| `dx1p25` day 1122 | `190055300.17018336 m3` clamp, `45708292.55406594 m3` storage, residual `0.000011086463928222656 m3`. |
| Fixed10 day 1418 lane 5 | `145554.778351 m3` clamp on `2914.7262742 m3` local source. |
| `dx10` day 1418 lane 5 | `457540698.111 m3` clamp on the same local source. |
| `dx5` day 1418 lane 5 | `27708994361.1 m3` clamp on the same local source. |
| Hydrology-source check | Maximum `H1.wat.parquet` source-row delta from `baseline_fixed10` is `0` on inspected days 1122, 1167, and 1418. |

## Why This Is a Legitimate Hold

The failing fine-rung residuals are not standalone proof of a bad source
producer or a broad output-publication failure. They are proof that the active
router can enter a large-clamp, large-storage regime where the existing
day-cascade relative guard fails by cancellation against huge operands.

The completed-rung trace shows the same family of behavior before the fine
rungs fail, including at the retained fixed `10 cells/OFE` active default. That
means the package cannot safely close the blocker as a harmless diagnostic
reference-ladder artifact.

## Why It Is Outside This Package

Closing this hold requires changing solver numerics, adding a new
contract-backed clamp-magnitude fidelity guard, or both. Those are
kernel/contract semantics, not a narrow investigation artifact edit. A safe fix
must preserve D10B conservation/oracle behavior and active routing closure, and
must be contract-first if it changes accepted residual classes.

## First Actionable Follow-On

Create
`20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001`.

Initial work:
- build a minimal WA day-1418/day-1122 reproducer,
- instrument per-step clamp and state extrema,
- decide the contract treatment of large positivity-clamp ratios,
- then implement and gate a numerics fix or a hard fidelity guard.
