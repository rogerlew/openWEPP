# R6E Parity Evidence

Evidence mode: Static + Ran.

Status: blocked by `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Compatibility fixture context:

- `H5.hbp`: `1654` bytes;
- `H5.loss.json`: `342` bytes;
- `H5.plot.parquet`: `202` bytes;
- `H5.wat.parquet`: `14503` bytes;
- `openwepp_hillslope_run_manifest.json`: `6864` bytes.

Direct cutover result:

- exit status `1`;
- no HBP/WAT/PASS/loss/manifest public output files written;
- held marker:
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`;
- HBP byte comparison reached with direct and compatibility byte lengths both
  `1654`, but bytes differ.

Therefore these closure gates remain blocked:

- HBP byte identity;
- WAT Arrow row/schema/metadata parity;
- PASS Arrow row/schema/metadata parity;
- loss JSON identity;
- run manifest provenance/checksum parity;
- compatibility/default-disabled protected output identity under successful
  direct cutover.
