# V40 parity-monotone active-set implementation and validation

Status: `IMPLEMENTED_FOCUSED_GREEN_CANONICAL_R108_PENDING`

Implemented:

- exactly four private, failed-but-valid rolling reset windows are required;
- support, static joins, phase predicates, opposite pure-vapor sides,
  promoted-root coordinates, and two-map physical-evaluation cadence remain
  exact;
- root/reset drift is finite, positive, tolerance-independent, and strictly
  decreasing, while adjacent stagnation and parity `A/B/A` are typed refusals;
- the final observation ordinal must equal the current shared-budget charge,
  and the unchanged 96-evaluation budget must retain the exact minimum
  initial/Jacobian/trust/probe/replay reserve;
- eligibility dispatches only the unchanged V38/V39 finalization-equivalent
  physical solver. It cannot admit, replay, finalize, accept, or publish;
- any fourth-window poison is returned as typed adaptive refinement with no
  fallback admission or state publication.

The retained V31 exact-floor midpoint oracle was restored as unpublished
diagnostic/refusal evidence after its source obligation exposed a governed
dead-helper deletion. `open_snow.rs` contains no call to that oracle.

Ran:

- contract-first source-bound expected red: run
  `8b404b02-89b4-4fd1-ae4e-7c616dc782ad`, failed only for the absent V40
  production seams and five required behavior names;
- V40 focused runtime behaviors: run
  `8fc30110-befd-462c-ad0e-ffb60ea1367f`, 5 passed;
- V40 contract and production source-bound obligations: run
  `04623d7e-416f-4155-bc2c-ca40e11e74d0`, 2 passed;
- owned V33--V40 focused regressions excluding the then-missing retained V31
  oracle: run `52cddf6a-dfde-461e-aa8f-fb435f5fd53d`, 43 passed;
- retained V31 source oracle, exact canonical `W/H` midpoint behavior, and V40
  behaviors after restoration: run `d00b60a2-75a5-43ea-b257-5ddf67dbb8ab`,
  7 passed;
- `cargo check -p openwepp-hillslope-orchestrator --all-targets`: passed;
- `cargo fmt --all -- --check`: passed.

Canonical r108 remains root-owned and pending. No temporary r107 or V40
diagnostic seam remains.
