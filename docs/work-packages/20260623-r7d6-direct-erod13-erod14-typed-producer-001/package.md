# R7D6 Direct EROD13/EROD14 Typed Producer

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close
`HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT` by adding direct
typed EROD13/EROD14/EROD15 producer authority to the production direct executor,
then continue R7D publication parity until HBP/PASS sediment fields are
producer-authoritative or a narrower SED-owned blocker is proven.

Rationale: R7D5 proved that direct production had no direct sediment producer
and was silently publishing fabricated zero erosion operands for erosion-active
H2637. It now fails closed when `erod14_wave2_enabled` is active and no direct
sediment producer has run. R7D6 must replace that guard with direct-owned
producer operands, not a compatibility alias copy.

Included scope:

- Add typed direct erosion inputs, state, downstream operands, and shadow
  projection for the minimum EROD13/EROD14/EROD15 producer chain required by
  HBP/PASS sediment publication.
- Bind direct production day-input construction to explicit EROD13/EROD14 seed
  authority with units and anti-alias checks.
- Carry EROD14 class state across direct days and lanes where required:
  `erod14_gend`, `erod14_frcflw`, `erod14_frac`, `erod14_fidel`,
  `erod14_tcf1`, `erod14_sedmax`, `sed_frac`, and related transport scalars.
- Populate `DirectPublicationErosionOperands` from direct-owned EROD15
  projection: `total_detachment_kg`, `total_deposition_kg`,
  `hbp_total_detachment_kg`, `hbp_total_deposition_kg`,
  `hbp_sediment_concentration_kg_m3`, and
  `sediment_concentration_kg_m3[0..5]`.
- Preserve `compatibility_edge_invocations = 0` and keep R7D4 WAT/PASS water
  parity from regressing.
- Re-run focused H2637. Iterate in package until direct production exits 0 and
  HBP/PASS sediment residuals are resolved, or a narrower out-of-envelope
  SED-owned blocker is proven.

Excluded scope:

- Default activation; compatibility remains default.
- Wrapping compatibility scheduler output, WB13 rows, HBP bytes, runtime
  aliases, or public-output builders as direct sediment authority.
- Broad watershed/channel sediment routing beyond hillslope direct publication.
- Claiming sediment-coupled MOFE `qin/qout` closure unless the package proves
  direct SED-owned prior-OFE `qout` plus particle/class-fraction handoff.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**` only for extracting
  reusable baseline-authoritative EROD helpers without changing compatibility
  behavior.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime*.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260623-r7d6-direct-erod13-erod14-typed-producer-001/**`

Dependencies:

- R7D5 executed hold:
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
- `SC-SYSTEM-001` HBP sediment payload authority and EROD14/EROD15 addenda.
- `SC-SED-*` authority when present for sediment process ownership. If no
  canonical SED contract covers a required producer, stop at a named hold
  rather than synthesizing physics.
- Pinned legacy baseline `/workdir/wepp-forest_260430_baseline` for static
  provenance mapping where canonical contract text delegates to baseline code.

Correction authority envelope:

- Defect: R7D direct production active-sediment publication has no direct
  EROD13/EROD14/EROD15 producer and R7D5 correctly fails closed.
- In-scope corrections: typed direct erosion state/input structs, direct phase
  span execution, direct producer tests, EROD publication row projection,
  manifest/source evidence, focused H2637 parity iteration.
- Protected boundaries: compatibility scheduler execution, WB13 rows, public
  HBP/PASS builders, and aggregate runtime aliases are comparison evidence only.

Phase plan:

1. Read R7D5 handoff, `SC-SYSTEM-001` sediment/HBP authority, any `SC-SED-*`
   contract, EROD13/EROD14 compatibility kernel code, and the array-native
   publication operand ledger.
2. Author an operand-lineage table before code edits: every direct erosion
   operand must name units, normalization/denominator, area/volume basis,
   source authority, carry behavior, and anti-alias candidates.
3. Add failing direct-runtime tests for erosion-active publication:
   nonzero detachment/deposition/concentration, class-count handling, zero
   outflow, and anti-alias separation from water-transfer and runtime aliases.
4. Implement typed direct EROD13/EROD14/EROD15 state/input/downstream/shadow
   structures and a direct erosion span. Reuse baseline-authoritative math only
   after separating it from compatibility request/wrapper authority.
5. Wire the span into `DirectFrameExecutor::run_day_spans` before publication
   row creation, and make `DirectPublicationDayRow` consume producer operands
   instead of the R7D5 missing-producer guard.
6. Re-run focused tests and H2637. If direct production exits 0 but HBP/PASS
   residuals remain in-envelope, keep iterating in this package; do not stop
   after the first nonzero producer.
7. Complete review, verification, line-count governance, docs, and
   complete-or-held disposition.

Anti-premature-stop rule:

- Do not close after creating structs, counters, or a shadow-only projection.
- Do not close after publishing one aggregate scalar while class concentration,
  total deposition, or PASS sediment fields still alias zeros or compatibility
  runtime values.
- Do not close while H2637 fails at the R7D5 guard, HBP/PASS sediment residuals
  are in-envelope, WAT/PASS water parity regresses, or direct compatibility
  edge counters are nonzero.
- A hold is allowed only for missing/contradictory canonical SED authority,
  invalid input that correctly fails closed, or a broader sediment-process
  migration outside this direct publication envelope. The hold must name exact
  symbols, residual fields/bytes, and the first code action.

Acceptance gates:

- Direct erosion operand lineage table is complete and reviewed before
  production edits.
- Focused anti-alias fixtures prove direct publication reads typed direct
  erosion producers, not zero defaults, water-transfer fields, WB13 rows,
  compatibility HBP/PASS builders, or runtime aliases.
- H2637 direct production exits 0 with `compatibility_edge_invocations = 0`.
- H2637 HBP/PASS sediment payloads are parity-clean, or the package closes in
  a narrower out-of-envelope hold with exact residuals and first action.
- R7D4 WAT/PASS water identity is preserved after the producer lands.
- Rust closure gates pass before `complete`: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full R7D closure.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, credentials, or machine-local absolute paths are
  committed as normative config.
- Direct production remains explicit opt-in and fail-closed.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Verification artifact labels `Static:` and `Ran:` evidence.
- Conservation/publication anti-tautology review before any HBP/PASS parity
  claim.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  production files block closure.

Final disposition: executed-held at
`HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`.

R7D6 landed typed direct EROD13/EROD14/EROD15 producer authority, added the
typed direct WB16 peak-duration producer needed by active erosion publication,
removed fabricated default EROD14 deposition from the MOFE03 Wave-2 seed, and
proved H2637 direct production exits `0` with direct compatibility-edge
counters at `0`. H2637 WAT is byte-identical and PASS sediment fields are
parity-clean after the `erod14_lddend` default correction.

The remaining consumer-path residual is non-sediment peak publication:
compatibility PASS/HBP publishes `peakro = 0.0` for the six H2637 PASS rows,
while direct publication now emits typed direct WB16 peak values
`3.591689245524811e-06`, `4.837293745180717e-07`,
`9.939800459642262e-07`, `4.726157673358129e-07`, `3.63e-08`, and
`3.63e-08`. R7D6 may not suppress direct WB16 authority to match the
compatibility zero without a WB16 publication-authority adjudication package.
Next package:
`docs/work-packages/20260623-r7d7-direct-wb16-peak-publication-parity-001/`.
