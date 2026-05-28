# WSHEDIMPL26 Channel Branch Payload Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Seam under review:
  - WS23 iterative detach closure in `detach.for` depends on `dcap(flagm=2)`
    max-detachment limiter behavior during excess-adjustment iterations.
  - Prior runtime used a flagm1-only helper in iterative lanes.
- WS26 seam closure:
  - Added explicit `flagm` handling in runtime `dcap` helper and migrated
    `flagm=2` clipping to `maxe` behavior.
  - Preserved `flagm=1` behavior for upper-boundary detachment lanes.

## Ran
- Unit proof:
  - `wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe` passed and confirms
    `flagm=2` cap at `maxe`.
- WS11 proof:
  - `wshedimpl26_contract_ws21_case4_iterative_closure_stress_vector_remains_resolved`
    passed with no unresolved-detachment diagnostics emission.
