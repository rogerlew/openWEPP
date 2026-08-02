# Invalidated Pre-Snowbench-Phase-Fix Exact Cohort

The release cohort using binary SHA-256
`b28e241bed0fa3d21eaf94cab4ab7bbb4e642734027eed90b269901e66fd3ded`
completed 16/16 cells, and its stable analysis and retained-output comparison
passed. It was superseded when Review A found that the retained snowbench CoE
adapter derived phase diagnostics from source snow-water mass while the runtime
accumulation branch uses physical snowfall depth times the canonical `0.1` SWE
ratio. The existing density-100 fixture hid the mismatch.

The adapter now derives its diagnostic active total and fractions from the exact
runtime operand, and a non-100-density regression proves closure and executable
behavior. Although this adapter does not alter the four exact mountain cohort
inputs, it changes a receipt-bound production source and release binary. The
old receipt/cohort/analysis/neutrality are therefore nonterminal evidence; raw
outputs were moved intact to
`target/snow_surface_eb04w_accumulation_diagnostics_pre_snowbench_phase_fix`
before the final rebuild and rerun.
