# PERFDEEP08 R2 Blocker Disposition

Status: HOLD.
Evidence mode: Static/Ran.

## Verdict

`HOLD`.

PERFDEEP08 does not lift the R2 blocker. The only scoped candidate was slower
than PERFDEEP07 and was reverted. The default-disabled gate remains open:

- required: three-run median `<= 676.67 s`;
- PERFDEEP07 retained: `685.85 s`;
- PERFDEEP08 candidate: `691.93 s`.

## Consequence

R2+ direct-frame runtime implementation remains blocked. The next package
should first profile or micro-benchmark the retained default path to identify a
real remaining cost center rather than editing diagnostic hooks or removing
production indexed scheduler authority.
