# PL08 Plant/Residue Parity Investigation

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL08 requires plant/residue parity direction analysis after PL05/PL06/PL07.
- Semantic parity is the target; comparator deltas are interpreted per confidence tier.

Ran:
- Assessed plant/residue-related outputs from the executed single-OFE comparator lane:
  - keyed check on `H5.wat.dat` shared daily fields (`Ep`, `Es`, `Er`)
  - strict comparator result on `H5.plot.dat`

## Investigation Outputs

1. `H5.wat.dat` keyed by `(OFE,J,Y)`
- matched rows: `1095`
- `Ep` mismatch rows: `0`
- `Es` mismatch rows: `0`
- `Er` mismatch rows: `0`
- max abs diff for `Ep`/`Es`/`Er`: `0`

2. `H5.plot.dat`
- comparator status: `identical`
- strict pass: `true`
- output hashes match exactly:
  - baseline: `6f0f32ee8e15302d6da2ab8e68bf07c3f2aa87d0375fba397451549a8db1e3ea`
  - candidate: `6f0f32ee8e15302d6da2ab8e68bf07c3f2aa87d0375fba397451549a8db1e3ea`

## Assessment

- Surrogate lane signal for plant/residue-related shared fields is positive.
- This does not clear Tier-A comparator acceptance because:
  1. `H5.wat.dat` remains `structure_diff` under strict comparator due schema divergence.
  2. Lane is legacy-vs-legacy surrogate and does not directly establish openWEPP-vs-legacy parity direction.

## Residual Risk

- OpenWEPP comparator-ready Tier-A daily water-balance output surface is still missing in this workspace, so plant/residue parity remains non-authoritative for PL promotion decisions.
