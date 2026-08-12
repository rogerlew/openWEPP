# Crate And Dependency Graph

Status: `FROZEN`

`openwepp-kernel-contract` owns dependency-neutral resource DTOs. `openwepp-vegetation` and `openwepp-biogeochemistry` each depend on it plus serde/typed-error utilities, but never on each other. `openwepp-hillslope-orchestrator` may depend on both only for default-off diagnostic coordination. This directed graph is acyclic; vegetation cannot mutate hydrology or BGC state.
