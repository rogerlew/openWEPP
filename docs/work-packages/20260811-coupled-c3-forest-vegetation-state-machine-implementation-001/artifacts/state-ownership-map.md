# State Ownership Map

Status: `implemented / focused pass`

Vegetation solely owns canopy liquid, hydraulic warm starts, phenology, geometry, and all vegetation C/N pools. The diagnostic water state owns per-layer soil water; BGC owns independent layer/species mineral N and typed litter/CWD receipts; the energy operands own canopy energy closure; the orchestrator owns transaction identity and the one atomic commit. All cross-owner exchange follows immutable request, maximum authorization, finalized use, owner-candidate construction, independent validation, then commit. Thirteen injected failures prove no candidate mutates beginning state.
