# Annual Litter-Transfer Provenance

Evidence class: `Ran` plus pinned-source reconstruction

The Linux 260725/source-native 9002 lane provides the operator-accepted,
output-precision-bounded gross annual transfer estimate.

The producer provenance chain is:

- release executable:
  `/workdir/wepp-forest_260430_baseline/release/wepp_260725`;
- executable SHA-256:
  `7e0ccad2a79cebf63ad821b140ef3007ca5846ca9b646e87559448c38e4d0d91`;
- release commit: `4b71c6f5b557533a8b02c14cf2961916d50c9080`;
- producer source:
  `/workdir/wepp-forest_260430_baseline/src/grow.for`; and
- `grow.for` SHA-256 at that release commit:
  `049e7f53185935250c819fcf1a9edc192c7d0cfab4fc340162df94f14fcf0df3`.

The retained working-tree source has the same hash as the file at the release
commit.

For cropland perennials, pinned `grow.for` computes:

`delvd = vdmx * (1 - dropfc) / spriod`

on each senescence day, subtracts `delvd` from `vdmt`, then adds the identical
daily loss `vdmy - vdmt` to both rill and interrill current residue before
forming `rmogt`. Active growth and perennial senescence are mutually exclusive
branches. Internally, annual gross transfer is
`vdmx * (1 - dropfc)` for these managements. The published crop output rounds
daily `vdmt` to `0.001 kg/m2`, so the harness estimates `vdmx` with
`max_daily_published_vdmt`. This is a bounded reconstruction, not direct
observation of the unrounded internal operand and not the earlier ambiguous
first-day/year-end stock proxy.

`wepp_observe.on` was enabled and its fixed-callsite logs were retained. The
stock 260725 binary contains no litter-specific Observe callsite, so no
nonexistent tag is claimed. As an independent check, summing published daily
positive live-biomass declines agrees with the formula estimate to at most
`0.00088 kg/m2/year`, consistent with the daily output precision.

The original intent requested a direct value or exact operands. The operator's
subsequent instruction that this precision is “close enough for what we are
doing” explicitly accepts this bounded reconstruction for campaign use and
lifts the package hold. It does not make the value exact or foliage-only.
The quantity is gross aboveground live-to-current-residue transfer; material
composition remains unresolved.

| Arm | 100-year mean | Years 91--100 mean | Unit |
| --- | ---: | ---: | --- |
| Hubbard constant | 0 | 0 | kg/m2/year |
| Hubbard hardwood 0.95 | 0.82563 | 0.99403 | kg/m2/year |
| Hubbard hardwood 0.92 | 0.89957 | 1.00124 | kg/m2/year |
| Santee constant | 0 | 0 | kg/m2/year |
| Santee mixed 0.93 | 1.42949 | 1.63683 | kg/m2/year |

These values are for the **Linux 260725/source-native 9002 lane**. Machine rows
are retained in
`linux-9002-260725/annual-results.csv` and
`linux-9002-260725/litter-transfer-summary.csv`.
