# Real Consumer And Operand Lineage

Evidence class: `Ran + Static source inspection`

| Producer | State/frame | Consumer | Retained result | Negative proof |
| --- | --- | --- | --- | --- |
| Native GSI/forest canopy advance | post-phenology growth state | snow canopy attenuation | WAT SWE/depth/density response | Static initial canopy does not carry native trace identity. |
| Same post-phenology state | day input and direct frame | WB15 interception | trace and WAT interception identity | No repeated scalar canopy sidecar. |
| Same post-phenology state | ET compute inputs | WB17 ET | WAT `Ep+Es+Er` | No canopy parameter was selected from ET residuals. |
| Leaf-off transfer | decomposition/residue frame | surface residue and frost thermal input | trace residue and WAT frost response | Needle/fine-wood nulls were not converted to zero. |
| Direct water frame | WAT publication | runoff | WAT `Q` | Runoff was not used to refit canopy. |
| Post-phenology canopy/ground cover | erosion daily consumer inputs | erosion producer | input lineage only | No erosion output exists in this run surface; consequence remains `NOT_ADVANCED`. |

All forest runs used the direct-production executor. The research trace
validated exact post-phenology producer/consumer identities before the WAT
result was summarized.
