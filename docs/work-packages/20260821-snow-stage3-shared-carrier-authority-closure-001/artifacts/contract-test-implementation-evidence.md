# Contract-test implementation evidence

Status: complete / authority vectors

Evidence mode: Ran

Ran: `python3 docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/reference_model.py` emitted schema `OPENWEPP_SNOW_STAGE3_SHARED_CARRIER_REFERENCE_RESULTS_V2` with 17 cases: 9 accepted and 8 rejected, plus 3 computed restart/rollback results.

Ran: `cargo nextest run --test snow_stage3_shared_carrier_authority_contract`
returned `5 tests run: 5 passed, 0 skipped`; Draft 2020-12 schema and relational
receipt-poison mutations also pass within that focused gate.

The vectors cover shared-node ownership/equations, raw-10 m wind, independent
canopy-air, wrong-regime and canopy-intercepted-snow poisons; unequal pre/post
supports, exact maximum aggregation, both neighbor predicates, deterministic
tie-breaking, proposal/accepted tick divergence, no-candidate retry with exact
owner no-op, a one-nanosecond structural event, and an independent snow,
liquid, signed-vapor, energy, reciprocal longwave, restart, rollback, and
event-time reconstructions that poison a diagnostic melt alias.
