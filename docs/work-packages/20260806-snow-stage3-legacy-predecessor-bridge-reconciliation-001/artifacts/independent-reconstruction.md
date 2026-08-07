# Independent Reconstruction

Status: `PASS / forcing-stratified source invariant`.

Evidence mode: `Ran` at exact clean execution commit
`cb31e6f4d06fd66a3ef5b3a7711a095b3f3d84f4`.

The package consumer streamed all endpoint and retained-anchor JSONL bytes
without importing runner reductions. It independently reconstructed exact
dates, v4 hourly-to-daily aggregates, v6 primitive/support/mass/cold/conduction
closure, all 35 water-year windows, per-WY factorial effects, medians, retained
endpoint replay, and selector equivalence.

| Quantity | Canonical forcing | Development forcing |
| --- | ---: | ---: |
| Old-source cell median (MJ m^-2) | `170.25360893091587` | `188.87252883560674` |
| Current-source cell median (MJ m^-2) | `170.25360893091576` | `188.87252883560654` |
| Paired source-difference median (J m^-2) | `-8.940696716308594e-08` | `-1.043081283569336e-07` |
| Maximum absolute per-WY source delta (J m^-2) | `1.7881393432617188e-07` | `2.086162567138672e-07` |
| Failed water years | `0` | `0` |
| Frozen source gate | `PASS` | `PASS` |

The median per-WY forcing effect is `+11.899053450648978 MJ m^-2` at both
sources and the median interaction is exactly `0 MJ m^-2`. This value is not
the subtraction of separately reduced cell medians; the package correctly
forms each water-year contrast before taking its median. Maximum absolute
per-WY interaction is `5.960464477539063e-08 J m^-2`.

Both retained endpoint anchors replay daily, per WY, and at the paired median
with zero failures. E10 and E11 legacy/explicit selectors have zero maximum
daily difference. No forcing lane triggers the frozen checkpoint predicate, so
the required 14-checkpoint phase closes with an explicit `not_triggered`
receipt rather than a source-localization claim.

The independently reconstructed result is
`target/snow_stage3_legacy_predecessor_bridge_reconciliation/results/predecessor-bridge-results.json`
at SHA-256
`ca79b7392ee1abfea113bea2bea87d5193ba12ee8ad084b135a74c1d09e6c0e9`.
