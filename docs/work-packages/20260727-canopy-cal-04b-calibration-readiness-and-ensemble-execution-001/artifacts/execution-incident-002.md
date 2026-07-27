# Execution Incident 002

Status: `PRESERVED / ORCHESTRATION INTERRUPT BEFORE POPULATION`

Evidence class: `Ran`

The corrected observed attempt again passed `prepare`, `build_executor`, and
`build_production_runner`. Its delegated shell received an interrupt while the
first `native_proof` production case was running. The retained stderr ends in
`KeyboardInterrupt`; the wrapper did not issue a native-proof receipt, and no
population, freeze, or Harvard command ran.

All attempt files were moved without deletion to
`/home/workdir/cal04b-objects-interrupted-native-proof-002`. The next attempt
uses a new empty active object root and restarts the complete observed DAG from
`prepare`. No receipt or partial native-proof output is reused.

