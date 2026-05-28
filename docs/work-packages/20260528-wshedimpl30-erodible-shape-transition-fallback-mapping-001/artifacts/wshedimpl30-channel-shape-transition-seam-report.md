# WSHEDIMPL30 Channel Shape-Transition Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Baseline authority (legacy):
  - `chnrt.for`: uses `flagc = ishape(ichan)` with erodible fallback:
    - upper boundary: `if (flagc.eq.3.and.depb(ichan,i-1).le.1e-4) flagc = 2`
    - lower boundary: `if (flagc.eq.3.and.depa(ichan,i).le.1e-4) flagc = 2`
  - `detach.for`: uses `flagct` + `depa(ichan,i)` rectangular fallback before
    `dcap` in detach lanes.
- WS30 runtime mapping implemented:
  - `ishape` supports explicit `1..=3`.
  - Segment-local upper/lower fallback flags applied from `depb`/`depa`.
  - Flag propagation wired into `hydchn`, `dcap`, WS23/WS24 closure calls, and
    terminal `tc` hydraulic computation.

## Ran
- WS30 vectors pass:
  - `wshedimpl30_contract_ws20_ishape3_erodible_lane_vector_executes`
  - `wshedimpl30_contract_ws20_ishape3_depa_depb_fallback_mapping_affects_outputs`
