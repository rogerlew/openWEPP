# Evaluation Operand Lineage

Evidence class: `Static`.

| Operand family | Units/basis | Source | Status | Rejected aliases |
| --- | --- | --- | --- | --- |
| shortwave, longwave, sensible, latent, precipitation advection, active/lower conduction | `W m^-2` hourly-weighted and `J m^-2` interval/daily | existing typed Stage 3 carrier primitives and conduction exchange | diagnostic evaluation | production aggregate substituted for shadow; conduction mislabeled ground flux |
| vapor exchange | `kg m^-2` | typed turbulent mass flux integrated over evaluated seconds | diagnostic evaluation | sublimation depth or production vapor substituted |
| cold-content export | `J m^-2` | exact cold content returned by the sequential vapor/melt solid-debit primitive | sequential only | cold-content change or latent energy substituted |
| available ice, sublimation, melt | `kg m^-2` | pre-debit clone ice and exact sequential debits | sequential only | authoritative SWE loss or CoE melt substituted |
| carrier total and residual | `J m^-2` | independent sum and cold/latent/terminal reconstruction | diagnostic evaluation | producer residual accepted without operands |
| requested/evaluated support | `s`, fraction | typed tag interval and exact integrated substeps | diagnostic evaluation | row count, nonzero-hour count, or 24-hour assumption |
| fingerprints | stable 64-bit FNV-1a over exact bits/IDs | request snapshot, forcing, geometry, cadence, non-formulation inputs | diagnostic identity | platform hash or site name |

Paired fixtures must numerically distinguish every rejected energy alias.
Sequential fixtures must reconstruct `complete = cold change + fusion*melt +
terminal unallocated + residual`. Coverage must reconstruct as evaluated divided
by requested seconds.

Implementation status: PASS before review. Schema-v5 daily and hourly fields
carry these exact sources. The real-file consumer fixture deliberately sets
production and evaluation shortwave/total values unequal and reconstructs only
the shadow-specific operands.
