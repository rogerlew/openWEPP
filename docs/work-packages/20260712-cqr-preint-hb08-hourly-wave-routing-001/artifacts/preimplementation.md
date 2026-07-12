# HB-08 Preimplementation Record

Evidence class: **Static**

## Classification

The fixed `ws11_route_baseline_wave_series` row is `E-SCIENCE`, CC 44,
85.106% covered and CRAP 50.396. Coverage alone cannot reduce CRAP below its
CC floor, so a mechanical split is required after cover-first closure. Eleven
same-source helpers are also below the 75% line floor in the available capture;
fresh LLVM region evidence is binding.

The target file is 2,064 lines, triggering WARN but not the 3,000-line blocker.
HB-08 may split coherent private stages inside the same module; moving unrelated
hourly sediment/geometry code is outside scope.

## Call And Consumer Path

The production interval-water builder validates current/prior grids and calls
the target. The target selects only KW/static-MC/variable-MC, constructs pinned
time-zero and spatial state, advances every `it=1..ntchr` and segment, records
the terminal outlet/coefficient representative, reconstructs interval storage
changes and computes branch-specific terminal hydraulic storage. Returned state
feeds WS10 channel publication and the W11C runner integration.

## Refactor Hazards

- time-zero state must remain separate from public slot zero;
- prior-day `qin/q1/qlat` and storage validation/order must not change;
- lateral adjacent-state averaging and reach-length normalization are exact;
- KW/static/dynamic MC dispatch and dynamic qref grouping are fixed;
- dry MC skip, segment cap/floor, outlet epsilon and representative selection
  must retain comparisons and ordering;
- KW terminal spatial mean must not alias MC boundary mean or flux residual;
- interval and daily water/sediment closure must remain independently
  reconstructable;
- no coefficient clamp, damping, peak clip, fallback or synthesized state.

## Planned Evidence

Cover-first tests will target uncovered invalid/boundary branches and all
same-source floors, then the fixed function will be remeasured before stage
extraction. Final proof requires same-source metrics, exact operation/error
review, full orchestrator tests, the seven-test W11C real consumer, line-count
governance, two reviews and two verifications.

No production or test edit has been made by this kickoff.
