# Reason Code Map

Status: `focused runtime PASS / independent hydrology and Rust reviews GO`

Evidence class: `Static + Ran`

| Owner fact | Vegetation-facing reason | Real-owner fact retained |
|---|---|---|
| positive demand, adequate liquid | `FullySupplied` | full beginning supply |
| exact zero physiological demand | `ZeroDemand` | frozen/rooting/source facts remain separate |
| one positive eligible demand exceeds source liquid | `LiquidStorageLimit` | exact beginning `theta_m`, including nonzero scarcity |
| frozen source excluded | `FrozenExclusion` | frozen depth/water and layer identity |
| requester cannot access layer | `RootingExclusion` | configured accessibility fact |
| multiple positive equal-status demands oversubscribe one source | `CompetingDemand` | total demand and proportional shares |

The legacy vegetation reason is a projection, not the complete owner evidence.
In particular, a zero request does not erase a simultaneous frozen or rooting
fact in the real-owner receipt.
