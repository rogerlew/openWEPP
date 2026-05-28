# WSHEDIMPL27 Channel Branch Payload Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Seam under review:
  - WS21 case4 enddet closure in baseline `enddet.for` uses two bracket-update
    paths: midpoint rebracketing (`xdsmal`) and upper-bracket replacement
    (`xdbig`).
  - Prior runtime loop only advanced `xdsmal` and did not preserve explicit
    `xdbig` branch behavior.
- WS27 seam closure:
  - Added `ws27_case4_enddet_bracket_closure` helper and migrated baseline-style
    branch sequencing:
    - `nt < cnpart`: `xdsmal = xdbmin`; midpoint rebracket with
      `xdbmin = (xdsmal + xdbig) / 2`.
    - `nt == cnpart`: `xdbig = xdbmin`; recompute `xdbeg` branch.
  - Kept typed-guard/no-silent-default posture unchanged in surrounding WS10
    runtime flow.

## Ran
- Unit proof:
  - `wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`
    passed and confirms both branch paths execute.
- WS11 proof:
  - `wshedimpl27_contract_ws21_case4_bracket_migration_vector_remains_resolved`
    passed with no unresolved-detachment diagnostics emission.
