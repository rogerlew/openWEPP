# V42 cold-content-export coordinate implementation and validation

Status: `IMPLEMENTED_FOCUSED_AND_RESTART_GREEN_CANONICAL_PENDING`

Implemented:

- Every authentic covered support image carries the exact finite nonnegative
  Stage 3 `complete_arm_cold_content_export_j_m2` as the distinct private
  support operand `cold_content_export_j_m2`.
- Canonical support coordinates reconstruct
  `H=-C_0+L_f(L_0+L_in)+Q+X_c`; water, complete energy, latent energy, and the
  physical Stage 3 ledger are unchanged.
- Synthetic midpoint, vapor-interface, and branch-entry images contract
  `X_c` with the same single endpoint order and scalar weight as external
  liquid and ordered nonlatent energy. Zero export retains prior coordinate
  bits.
- The synthetic state's existing cold-energy-change ledger is reconstructed
  as retained cold-content change minus refreeze energy and exported cold
  content, matching the already-governed Stage 3 export accounting.
- Authentic endpoint W/H is independently compared before any private image
  can be used. Missing/nonfinite/negative, omission, reordered identity,
  independent-weight, substitution, and closure poisons refuse; all private
  images remain non-publishable.

Evidence:

- Contract-first run `faedaec3-d052-493c-beff-ac3957386404`: the contract
  assertion passed and production/source behavior was expected red only for
  absent V42 seams and five required behavior names.
- Focused V42 captured/zero-export/contraction/poison run
  `779041ca-8c9d-4576-83db-ea654ef816a5`: 5 passed.
- Source-bound V42 contract/production run
  `8260589f-36a1-4aa5-a37d-65e0a4ce564f`: 2 passed.
- The two formerly failing before/after snow-reappearance persisted-restart
  cases run `a18e7492-a6ba-43cc-bf1f-01038d00220a`: 2 passed.
- Full persisted-restart fixture run
  `a11b4e2f-249d-4b62-b17d-98a5b39336b5`: 71 passed, 0 failed, 0 skipped in
  444.519 s.
- Retained V31 reconstruction module plus V33--V42 solver regression run
  `111b7e93-1946-4b94-8eaf-c0be39eebc42`: 61 passed.
- `cargo check -p openwepp-hillslope-orchestrator --all-targets`: passed.
- `cargo fmt --all -- --check`, scoped `git diff --check`, and bounded
  diagnostic scan: passed; no `DFF_R107`, `DFF_R108`, `DFF_R109`, `DFF_V*`,
  `eprintln!`, or `dbg!` remains in the solver write set.

Canonical r110 proved V41 now dispatches early but the pre-V42 coordinate map
still returned a generic safeguarded-solve refusal. Retained log SHA-256
`9d049fc583f9d50bd055e7758b6d3c35f10f7e9c1ce09289bf312c4fd4c09c53`,
wall 5:34.24, maximum RSS 1,320,276 KiB. V42 materially changes the governed
`H` coordinate/residual on exported-cold-content supports, so canonical r111
must run before any further solver diagnosis or contract amendment.
