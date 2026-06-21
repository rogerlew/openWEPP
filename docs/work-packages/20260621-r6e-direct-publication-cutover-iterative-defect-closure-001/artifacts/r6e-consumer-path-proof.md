# R6E Consumer-Path Proof

Evidence mode: Static + Ran.

Status: blocked at HBP comparison.

Current cutover path:

1. production climate setup builds a cutover-only retained
   `DirectPublicationExecution`;
2. the retained execution is created by `DirectFrameExecutor` with typed
   `DirectPublicationDayInput` values from parsed climate;
3. `DirectPublicationFrameCutover` consumes that retained direct execution;
4. direct projection consumers build HBP/WAT/PASS/loss/manifest candidate
   artifacts from the typed direct publication frame;
5. `require_direct_publication_cutover_gates` compares HBP bytes against the
   protected compatibility publication and fails closed before public writes.

Accepted current direct producer families:

- run/lane/day identity;
- parsed calendar;
- parsed precipitation and effective daily temperature as direct inputs;
- direct R4/R5 span-produced hydrology operands for diagnostic parity
  comparison only.

Current missing consumer-path closure:

- HBP byte identity;
- WAT/PASS Arrow parity;
- loss JSON identity;
- manifest checksum/provenance parity;
- output writes.

Negative evidence:

- cutover arm scan has no forbidden compatibility source matches;
- focused tests prove zero skeleton-run and compatibility-edge counters;
- CLI cutover writes no public outputs on parity failure.
