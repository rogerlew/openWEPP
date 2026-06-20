# R4D Producer Selection

Status: complete.
Evidence mode: Static.

R4C left four explicit R4B storage operands:

| Candidate | Selected? | Rationale |
|---|---|---|
| `deep_seepage_m` / `D` | yes | Strong `SC-PERC-001` authority, natural `PercolationDeepSeepage -> StorageReconciliation` span, lower branch surface than ET/snow/WB19. |
| `evapotranspiration_m` | no | Larger WB17/ET branch surface and crop/PMET coupling; defer until D handoff proves another producer cleanly. |
| `subsurface_loss_m` / `Qd` | no | Strong `SC-SUBHYD-001` authority, but WB19 lateral/drainage and routing handoff are broader than a WB18 D handoff. |
| `snow_coupling_m` / `S` | no | Coupled to snow/frost storage and publication lifecycle; higher branch and fixture risk. |

Selection: R4D migrates the deep-seepage handoff producer only. It does not
migrate full WB18 percolation physics or public `Dp` publication.
