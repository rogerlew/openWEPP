# Pre-Implementation Contract Gate

Evidence mode: `Static:` plus existing pre-fix `Ran:` ledger.

Pre-fix evidence:

- `/tmp/frostval01/full/run_status.tsv` shows `37/43` prefixes failing with
  `CLIHILL-E-011 ... HS-RUNTIME-E-062`.
- The six controls `p8,p13,p22,p23,p26,p28` returned `0`.
- Representative legacy completion: `p1.err` and `p8.err` end with
  `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`.

Contract gate:

- Authority amended first in `SC-SOIL-001` as `INV-SOIL-017`.
- Correction was limited to parser-layer corrected diagnostic/constitutive symbol
  coverage.
- The package did not authorize hydrology seed-grid tail compensation, guard
  loosening, silent defaults, or downstream percolation/frost/runoff/snow edits.
