# PERFDEEP08 H2637 Identity and Timing Evidence

Status: FAIL.
Evidence mode: Static/Ran.

## Timing

Ran one H2637 default-disabled candidate timing:

```text
perfdeep08_hook_cache_rep1  691.93  229444
```

The run was above the required median threshold `<= 676.67 s` and above the
PERFDEEP07 retained point `685.85 s`. The package did not run the three-run
median gate because the screening run failed.

## Identity

The candidate completed successfully and the manifest reported protected output
checksums matching the existing anchor paths:

- HBP:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
- WAT:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
- PASS:
  `d99e9f269fceb61e87e6b394461dfb59a328e0eb1a26c580c580eb8a2d0d4de8`
- plot:
  `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`
- loss:
  `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`

Because timing failed, no additional identity comparison was run for a retained
candidate.
