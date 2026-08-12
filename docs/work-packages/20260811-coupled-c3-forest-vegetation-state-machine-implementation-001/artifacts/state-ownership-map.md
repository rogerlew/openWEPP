# State Ownership Map

Status: `FROZEN`

Vegetation solely owns canopy liquid, hydraulic warm starts, phenology, geometry, and all vegetation C/N pools. Hydrology owns soil water; BGC owns mineral N and litter/CWD receivers; energy owns its closure surface; the orchestrator owns transaction identity and the one atomic commit. All cross-owner exchange follows immutable request, maximum authorization, finalized use, validation, then commit. No candidate mutates beginning state.
