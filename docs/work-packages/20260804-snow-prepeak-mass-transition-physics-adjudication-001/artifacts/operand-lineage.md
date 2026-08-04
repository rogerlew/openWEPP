# Operand Lineage

Status: `frozen before result execution`

Evidence mode: `Static`

| Boundary | Operand | Units/time basis | Authority | Rejected alias |
|---|---|---|---|---|
| snow storage | runtime SWE before/after | `m` SWE, daily state | authoritative runtime store | layer liquid or depth alone |
| solid input | accumulation | `m` SWE, daily sum | hourly phase snowfall sum | physical snow depth |
| liquid input stored | retained rain | `m` water, daily | CoE rain store | released rain |
| solid-to-liquid | raw signed melt | `m` SWE, daily diagnostic | signed hourly applied sum | pack loss |
| solid-to-liquid | redistributed positive melt | `m` SWE, daily diagnostic | existing redistribution | gross-positive hourly sum |
| solid-to-liquid | snowpack SWE loss | `m` SWE, daily nonnegative | authoritative state mutation | raw or routed melt |
| shared handoff | liquid handoff | `m`, daily nonnegative | exact downstream argument | inferred adjacent scalar |
| released rain | rain released | `m`, daily nonnegative | CoE rain release | direct rain total |
| Stage 3 | incoming liquid | `m`, daily nonnegative | exact shared handoff | top-level routed melt |
| Stage 3 | routed liquid | `m`, daily nonnegative | layer routing solve | CoE routed melt |
| Stage 3 | retained delta | `m`, daily signed | layer liquid-store change | retained store level |
| Stage 3 | refrozen liquid | `m`, daily nonnegative | layer thermal/liquid solve | doubled latent/refreeze term |
| Stage 3 | residual | `m`, daily signed | diagnostic producer | accepted without reconstruction |
| thermal context | cold content before/after | `J m^-2`, daily state | Stage-3 thermal state | CoE negative melt |
| empirical driver | A/B/C/D | `m` melt-depth contribution, hourly | CoE formula operands | identifiable physical heat flux |

Independent acceptance uses primitive operands from the real JSONL file. The
analysis tool may compare producer residuals only after reconstructing each
identity independently.
