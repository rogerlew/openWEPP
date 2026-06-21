# Disposition

Status: complete.
Evidence mode: Static + Ran.

Verdict: `COMPLETE-R6A-RUN-BOUND-DIRECT-PUBLICATION-FRAME`.

R6A implemented the missing run-bound direct publication frame and downstream
direct projection consumers required to lift
`HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT`.

Completed:

- added `DirectRunPublicationFrame` and typed output-family operand groups;
- added direct publication capture during direct run/lane/day execution;
- added explicit opt-in `DirectPublicationFrameShadow` and CLI flag;
- added direct HBP/WAT/PASS/loss/manifest projection consumers;
- proved default compatibility constructs no direct publication frame;
- proved opt-in direct publication capture does not use the old skeleton counter
  as closure evidence;
- ran full Rust, docs, and whitespace gates.

Not claimed:

- production public writer cutover;
- byte/Arrow identity;
- metadata/checksum parity;
- direct erosion/event/profile/frost producer completion beyond current
  optional/absent-authority frame fields.

Next package: resume R6 direct publication writer cutover from
`docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`.
