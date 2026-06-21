# R6E Manifest Cutover Evidence

Evidence mode: Static + Ran.

Status: blocked by HBP parity before output writes.

Required before closure:

- direct manifest publication producer map;
- input checksum parity;
- output checksum parity;
- runtime selection provenance;
- direct runtime counter provenance;
- output policy provenance;
- warning ID parity;
- independent checksum recomputation.

R6E does not write a direct manifest in cutover mode. The CLI reproduction exits
before public output writes with
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Future closure must prove:

- direct manifest reads only typed direct projection and file checksums from the
  direct cutover output set;
- manifest runtime-selection/output-policy fields identify the successful
  direct cutover path;
- warning IDs and checksums match protected compatibility expectations where
  parity requires them;
- independent checksum recomputation passes after direct outputs are written.
