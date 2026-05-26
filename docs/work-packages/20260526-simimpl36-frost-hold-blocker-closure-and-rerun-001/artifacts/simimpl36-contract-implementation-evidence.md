# SIMIMPL36 Contract Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Canonical contract amendments implemented:
  - `SC-INFILE-SOIL-001` `0.1.7` (quoted legacy compatibility authority
    extended to `7778/9002/9003/9005`, optional `avke := 0.0` normalization).
  - `SC-RUNOFFPART-001` `20` (added `TOL-RUNOFFPART-006` near-zero reconciled
    runoff canonicalization authority for WB12/WB14 writeback/publication).
  - `SC-WATBAL-001` `38` (added `TOL-WATBAL-006` matching WB12/WB14
    canonicalization authority).

## Ran
- Evidence inspection commands against updated contracts and revisions:
  - `git diff -- docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - `git diff -- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `git diff -- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
