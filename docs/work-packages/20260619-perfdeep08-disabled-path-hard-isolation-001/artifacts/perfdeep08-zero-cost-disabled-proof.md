# PERFDEEP08 Zero-Cost Disabled Proof

Status: FAIL.
Evidence mode: Static/Ran.

## Proof Result

Zero-cost-disabled is not proven.

The only PERFDEEP08 candidate removed disabled diagnostic-hook overhead, but it
measured `691.93 s`, above both the P0 `<= 676.67 s` gate and the PERFDEEP07
retained `685.85 s` point. Because no viable single run reached the threshold,
the package did not run the three-run median gate.

## Static State

- Direct-frame hydrology was not implemented.
- No R2+ direct runtime code was added.
- No production Rust edit was retained.
- Production indexed scheduler runtime remains part of the current default
  path; prior PERFDEEP07 evidence showed removing it was slower.

## Remaining Blocker

The remaining default-path cost is not explained by the disabled diagnostic
hooks tested here. A future package needs a fresh profile or a narrower
attribution experiment before editing more compatibility surfaces.
