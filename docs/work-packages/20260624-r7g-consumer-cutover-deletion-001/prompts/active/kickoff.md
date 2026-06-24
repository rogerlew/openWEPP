# R7G Consumer Cutover And Deletion Kickoff

Execute `docs/work-packages/20260624-r7g-consumer-cutover-deletion-001/package.md`.

Cut R4A and direct-publication consumers from the temporary
`DirectFrostRunoffSurface` / `DirectFrostLiquidPartition` bridge to typed
winter-column state and outcomes. Delete the bridge fields/API from production
direct runtime after consumers move. Do not close complete while a current-scope
production consumer still reads the old bridge.
