# H2637 `latqcc` Handoff

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Disposition

`CONTRACT-GAP`.

Do not author a defect-closure ExecPlan from this package. No
`OPENWEPP-DEFECTIVE` condition was found.

Do not close the FARPOINT01 71% lateral-magnitude flag as physically correct.
The structural and conservation portions are closed by MAGPARITY01, but
absolute lateral-flow magnitude remains unadjudicated by external authority.

## Follow-On Work

Recommended follow-on:

1. Author an `SC-SUBHYD-001` external-authority amendment or companion suite for
   absolute lateral-flow magnitude.
2. Include at least one site/class benchmark or bounded synthetic benchmark
   where conductivity, drainable depth, slope/length, and `drfc` are known and
   the expected lateral flow has an independent acceptance envelope.
3. Bind H2637-like forest `solwpv=9002` behavior explicitly, including the
   24-substep lane and the soil-file conductivity lineage.
4. Keep legacy as an ADR-0017 flag only. Legacy no-UI may motivate test design,
   but it must not define the expected value.

## Non-Follow-Ons

- No code fix from this package.
- No `SC-*` contract edit was made here.
- No conservation or area-scaling work remains from this package.
- No legacy parity target should be created.

## Evidence to Carry Forward

Ran:

- `latqcc` equals WB19 `q` exactly in WAT units across the traced rows.
- Recomputed Eq [6.2.4] residual is at floating-point precision.
- PASS numeric totals remain the MAGPARITY01 totals:
  `runvol = 14,085,670.078744758 m3`,
  `sbrunv = 884,949.9416133772 m3`.

Static:

- Current `cas_l4_subhyd_*` authority is insufficient for an absolute H2637
  magnitude verdict.
- ADR-0017 comparator posture remains binding.
