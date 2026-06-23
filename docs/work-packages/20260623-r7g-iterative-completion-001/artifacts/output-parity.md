# Output Parity

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

Evidence class: Ran.

Captured files:

- Compatibility capture: `/tmp/r7g-cont-h2637/capture/compat/`
- Direct capture: `/tmp/r7g-cont-h2637/capture/direct-frost11/`
- Latest measured direct output path before the final no-material consumer
  safeguard: `/tmp/perfmig01-final/current/anchor/h2637_same/` from
  `direct-default-frost30`.

Protected file comparison:

| Output | Compatibility vs rollback | Direct default vs explicit direct | Compatibility vs direct | Disposition |
| --- | --- | --- | --- | --- |
| HBP | checksum maps identical | checksum maps identical | `cmp=1` | held |
| WAT Arrow | checksum maps identical | checksum maps identical | `cmp=1`; schema/row count match | held |
| PASS Arrow | checksum maps identical | checksum maps identical | `cmp=1`; schema/row count match | held |
| loss JSON | checksum maps identical | checksum maps identical | `cmp=0` | pass |
| plot | checksum maps identical | checksum maps identical | `cmp=0` | pass |
| manifest checksum maps | default vs rollback `cmp=0` | direct default vs explicit direct `cmp=0` | default vs direct `cmp=1` | held |

Reduced WAT deltas:

- `frozwt`: `34363` rows differ; max absolute delta
  `11.12017732034371 mm`.
- `frdp`: `34363` rows differ; max absolute delta
  `264.39519767438975 mm`.
- `Snow-Water`: `21305` rows differ; max absolute delta
  `183.04425009202413 mm`.
- `RM`: `14234` rows differ; max absolute delta
  `39.94882220799281 mm`.
- Downstream runoff/transfer/storage/ET columns also differ after snow/frost
  state divergence.

Reduced row evidence:

- First frost residual: year `1`, simulation day `5`, Julian day `5`, OFE `1`.
  Compatibility has `frozwt=0.005660437443662737` and
  `frdp=0.11700610732602637`; direct has both zero.
- Largest frost-depth residual: year `13`, simulation day `4388`, Julian day
  `5`, OFE `6`. Compatibility has `frdp=264.39519767438975`; direct has
  `frdp=0.0`.
- Largest snow residual: year `26`, simulation day `9220`, Julian day `89`,
  OFE `19`. Compatibility `Snow-Water=8.860545809172601`; direct
  `Snow-Water=191.90479590119674`.

Reduced PASS deltas:

- `runvol`: `12355` rows differ; max absolute delta `9239.224123229264 m^3`.
- `sbrunv`: `12407` rows differ; max absolute delta `233.70597343263216 m^3`.
- `peakro`: `7017` rows differ; max absolute delta
  `0.02257142260446447 m^3/s`.

Disposition:

- Direct-vs-direct publication is stable.
- Compatibility-vs-direct protected parity is not closed.
- The first material remaining producer family is active frost projection, with
  snowpack and downstream water-balance deltas also present.
- During the final reduction loop, preserving fine-layer carry improved the
  mechanism understanding but exposed a coarse-projection authority bug:
  `direct-default-frost30` first mismatched compatibility on day `1`, OFE `4`,
  with direct `Total-Soil=484.948003721413` versus compatibility
  `527.8768435525944`. This happened because no-material frost layer
  projection carried active-water-only `theta_after_m` into coarse publication.
- The no-material R4A consumer branch was patched after `direct-default-frost30`
  to ignore coarse layer projection and use `frwatc_net_liquid_delta_m`; H2637
  parity has not been rerun after that final safeguard.
- HOLD closure directs the next package to replace this patch chain with a
  coupled stateful frost sub-solver and then rerun protected HBP/WAT/PASS/loss
  parity from scratch.
