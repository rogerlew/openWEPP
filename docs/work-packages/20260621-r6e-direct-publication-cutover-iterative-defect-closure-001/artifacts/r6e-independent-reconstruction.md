# R6E Independent Reconstruction

Evidence mode: Static + Ran.

Status: blocked by HBP process parity.

Required reconstruction families:

- HBP event/runoff/erosion fields;
- WAT hydrology, storage, snow/frost, profile, and interception fields;
- PASS runoff/lateral/erosion volumes;
- loss JSON climate/run/static sidecar fields;
- manifest input/output checksums and provenance fields.

Reconstruction must use independently produced operands or parsed inputs, not
the same writer formula restated with the same row object.

R6E accepted parsed climate and lane geometry as direct input authority, and it
now reaches direct process operands for HBP comparison. Those process operands
are not accepted as public-output authority because HBP byte identity fails, so
independent reconstruction cannot be truthfully closed for any HBP, WAT, PASS,
loss, or manifest family.

Minimum next-package reconstruction requirements:

- reconstruct runoff/lateral/storage/snow/frost/loss operands from parity-grade
  direct phase input/state bindings, not from compatibility rows;
- reconstruct PASS volumes from direct area/volume operands and fixture PASS
  Parquet rows;
- recompute manifest input/output checksums from files written by the direct
  cutover path;
- include anti-alias fixture values where plausible wrong aliases differ from
  the accepted direct operand.
