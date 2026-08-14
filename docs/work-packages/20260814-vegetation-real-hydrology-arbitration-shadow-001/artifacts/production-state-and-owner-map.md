# Production State And Owner Map

Evidence class: `Static`

| Identity or state | Production owner | Shadow treatment |
|---|---|---|
| run, hillslope, OFE/lane and day | direct-runtime scheduler | retained exactly in snapshot identity |
| aggregate water | `DirectWaterState` | cloned and independently reconciled |
| ordered layer state | `Vec<DirectSubsurfaceLayerState>` | cloned completely; no scalar-map replacement |
| extractable layer liquid | layer `theta_m` | immutable authorization supply; candidate debit only |
| residual/frozen water and depth | layer state | retained; never borrowed as liquid supply |
| transfer/runon state used by arbitration | seeded day/lane state | projected into bounded snapshot bytes |
| frost/winter and unrelated production carry | day/lane state | retained in the full cloned frame and protected by whole-frame structural equality; not claimed in bounded fingerprint |
| native surface and root ET | R4N spans | omitted inside shadow clone only |
| accepted mutation | `DirectLaneFrame::commit_day` | production commit remains untouched |

Each direct-runtime lane is one OFE. Layer depth values are local metres per
OFE-ground; conversion to water mass basis is exactly
`depth_m * 1000 kg m^-3`. Lane area is used for routing/volume joins, not a
second factor in the local depth-to-mass conversion.
