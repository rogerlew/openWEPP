# R6F Manifest Cutover Evidence

Status: blocked-behind-WAT.

The manifest is still part of R6 cutover, but R6F did not reach manifest
validation because WAT producer authority blocks public output writes first.

## Manifest Source Map

| Manifest key | Direct source | Previous compatibility source | Checksum/provenance basis | Status |
|---|---|---|---|---|
| output checksums | direct publication output files | compatibility writer output files | checksum after public file write | Blocked: public direct writes are correctly suppressed. |
| runtime selection | direct publication metadata / run manifest publication | existing manifest provenance | manifest JSON | Pending R6 continuation. |
| direct runtime counters | direct runtime audit snapshot | none | manifest provenance | Pending R6 continuation. |

## Checksum Parity

Not run. CLI cutover test confirms no output files are written while
`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP` is active.
