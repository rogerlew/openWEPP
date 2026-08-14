# Architecture And Dependency Freeze

Status: `frozen before constitutive edits`

Evidence class: `Static`

Child 3 uses a dedicated `openwepp-land-surface-energy` crate below the
hillslope orchestrator. It depends on dependency-neutral kernel identities and
does not depend on the orchestrator or meteorology helper. The crate owns the
joint `OPENWEPP_C3_WOODY_V8` plus `OPENWEPP_SNOW_FREE_LSE_V1` potential and
fixed-cap column solve; this avoids treating the V7 post-hoc
`OccupancyEnergyProposal` as a coupled input.

The crate boundary is:

```text
openwepp-kernel-contract
        ^
        |
openwepp-land-surface-energy
        ^
        |
openwepp-hillslope-orchestrator
```

The low crate exposes strict configuration, persistent state, forcing,
source-keyed D/A/F, diagnostics and five-owner candidate DTOs plus potential
and final fixed-cap execution. It exposes no commit method. The orchestrator
adapts actual vegetation and hydrology beginnings, performs the one real-owner
authorization, validates all candidates, and owns later atomic shadow commit.

No constitutive ground physics is placed in vegetation, meteorology, runner,
diagnostic helpers or tests. No production selector or scheduler call site is
changed in this child.
