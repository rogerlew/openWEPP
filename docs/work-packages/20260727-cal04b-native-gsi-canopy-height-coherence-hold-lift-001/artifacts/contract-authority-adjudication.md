# Contract Authority Adjudication

Status: `A0 ADMITTED`

Evidence class: `Static + Ran`

The pre-amendment authority was incomplete. CP-GSI02 defined same-day foliar
biomass `Bf`, persistent structural biomass `Bs`, LAI, and cover, but not
native height. Pinned PL16 height used total above-ground dry biomass `vdmt`;
legacy rangeland geometry did not authorize substituting foliar mass.

Revision 24 makes the smallest explicit inference admitted by the package's
authority model:

`Bt = Bs + Bf`

`Hc = (1 - exp(-bbb * Bt)) * hmax`

`Bt` is a checked internal projection operand. It does not replace `Bf` on
foliar or interception surfaces. The expression is Chapter 8 Equation 8.2.8
and matches pinned `grow.for` lines 504-511 at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; the source's historical
Equation-8.2.6 comment is not used as the primary manual citation.

Admission requires checked finite arithmetic, positive finite `bbb/hmax`,
finite `Hc` in floating-point `[0,hmax]`, exact zero height at `Bt=0`, and
typed failure if positive `Bt` produces non-positive height. No empirical fit,
surrogate, static fallback, cover inversion, or foliar-only alias is admitted.

Independent reviewer A and reviewer B both returned final `PASS` after all
findings were accepted and corrected. The strict Binding Exposure Index,
unit-governance lint, checker regressions, Markdown lint, and diff hygiene pass.
