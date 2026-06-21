# Worker Handoff

Status: executed-hold.
Evidence mode: Static + Ran.

## Current Hold

`HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`

## Original Target

Resolve `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER` by lifting
`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

R6C did not close the original target. It proved the bridge cannot be
implemented at the publication-artifact builder boundary because the production
climate lifecycle does not retain direct publication producers.

## Do Not Skip

- Do not populate direct publication from compatibility WB13 rows, runtime
  surfaces, writeback payloads, stale logical state, or wrappers around them.
- Do not accept skeleton/zero direct rows as cutover evidence.
- Do not enable public direct writes before HBP/WAT/PASS/loss/manifest parity,
  anti-alias, independent reconstruction, manifest, no-compatibility, and
  benchmark gates pass.

## First Actionable Item

Superseded by R6D
(`20260621-r6d-production-direct-publication-producer-retention-001/`):
cutover now retains a production `DirectRunPublicationFrame` during the climate
lifecycle and consumes it without skeleton publication capture.

Remaining actionable item:

Implement parity-grade retained direct publication producers for hydrology,
storage, subsurface, evaporation, PASS, loss, manifest, and erosion families.
R6D holds at
`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`.

Original R6C handoff:

Implement retained production direct publication producers in the climate
lifecycle, then build `DirectRunPublicationFrame` from those retained direct
operands.

The follow-on must:

- extend production direct execution so direct run/lane/day publication operands
  are produced and retained during the climate day loop;
- keep WB13 rows, runtime surfaces, writeback payloads, and stale logical state
  out of direct publication authority;
- only re-enable cutover parity comparisons after direct producers exist;
- add anti-alias and independent-reconstruction fixtures before accepting each
  output family.

## Evidence To Reuse

- Code marker:
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.
- Focused test:
  `r6_cutover_candidate_fails_closed_before_skeleton_publication_capture`.
- CLI contract:
  `r6_direct_publication_cutover_cli_flag_fails_closed_before_outputs`.
