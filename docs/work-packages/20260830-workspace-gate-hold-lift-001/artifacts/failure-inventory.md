# Failure inventory

Status: `INTAKE`

Evidence mode: `Static + retained Ran`

Retained current full-profile evidence: 3,628 attempted, 3,503 pass, 96 fail,
29 timeout; log SHA-256
`dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`.

Retained current Clippy evidence: exit 101 after two root diagnostics; log
SHA-256
`aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`.

| ID | Owner/path | Classification | Prospective correction | Focused evidence |
|---|---|---|---|---|
| WGHL-CLIPPY-001A | `crates/openwepp-coupled-time/src/event.rs` | behavior-preserving iterator spelling | replace boolean `filter_map` with equivalent `filter` + `map` | affected crate Clippy/tests |
| WGHL-CLIPPY-001B | `crates/openwepp-biogeochemistry/src/lib.rs` | behavior-preserving local name | rename `used` to a semantically distinct local | affected crate Clippy/tests |
| WGHL-CLIPPY-001C | `crates/openwepp-coupled-time/tests/authority.rs` | test-only structure; newly exposed `too_many_lines` | extract shared accepted-noop setup and split rejection cases while preserving their post-acceptance clock state and every assertion | coupled-time authority test plus affected crate Clippy |
| WGHL-FULL-001A | typed SnowEnergy assurance source adoption; five exact generated lock/report paths plus one transaction receipt | 82-failure generated-identity cascade; current `SC-SNOWENERGY-001` source hash differs from retained assurance generation | apply the admitted `adopt-report-source` transaction after a successful read-only `--check`; do not hand-edit generated identity | assurance validation and transaction-declared scientific-full gates |
| WGHL-CLIPPY-001D | `crates/openwepp-vegetation/src/v11.rs` mutation-set iterators | two behavior-preserving iterator spelling diagnostics | replace each boolean `filter_map` with ordered equivalent `filter` + `map` | vegetation Clippy/tests |
| WGHL-CLIPPY-001E | `crates/openwepp-vegetation/src/v11.rs::restore_with_bgc_scope` | production function 147/100 lines | mechanically extract only accepted-segment replay into a private state carrier/helper; preserve public signature, operation/error order, exact accumulation, and final moves | V11 restore/custody focus, vegetation suite, A0 |
| WGHL-CLIPPY-001F | `crates/openwepp-vegetation/src/v11.rs` test module | 110/100-line authority test and assign-op spelling | extract existing setup fragments without moving assertions; use equivalent `+= 1` in test-only poison | named V11 authority test + vegetation suite |
| WGHL-CLIPPY-001G | `crates/openwepp-vegetation/src/v11/tests/v11_bgc_tests.rs` | test-fixture `usize` precision-loss lint | checked `u32::try_from(rank)` then lossless `f64::from`; fixture ranks remain exact 0/1/2 | named BGC test + vegetation suite |
| WGHL-FULL-001B | `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | one test-helper path-resolution defect | resolve fixture-authoring `run_dir` against the supplied child `current_dir` while preserving the relative CLI argument | exact relative-run-dir watershed CLI test |
| WGHL-FULL-001A-CHAIN | `tests/integration/vegetation_boundary_authority_contract.rs` | source-coupled assurance generation-chain binding exposed by the admitted new transaction | extend the asserted chain through typed receipt `fbebcf40...` and current generation `0102e72c...`; retain every historical chain assertion | exact integration test plus assurance validation |
| WGHL-A0-001 | `tools/release/authority-policy/impact-map.json` | newly touched Stage-3 seed authoring in the runner watershed behavior test lacks an exact current SC binding | add one critical exact-path `SC-SNOWENERGY-001` test-surface binding; do not alter suite/test selection | A0 admission and anti-evasion |
| WGHL-FULL-001C | generic watershed Stage-3 seed binder plus P102 committed seed/manifest/authority row | invalid test-fixture profile: generic corn/open fixtures and explicitly cropland/no-strata P102 were re-authored as forest `CompleteOwner`, causing terminal chronology, V8 bijection, and avoidable canopy cost | preserve and validate an existing explicit seed; otherwise author generic watershed seeds as `AdaptiveNoStrataOwner`; regenerate P102 through the authority API for its exact H1 identity, bind its checksum, and record empty vegetation roots | fixture-authority suite, wshedw2, wshedw7r, manifest checksum, anti-evasion |
| WGHL-FULL-001D | `SC-SNOWENERGY-001`, `snow_stage3_v11_adaptive_execution_tests.rs`, `v11_covered/{fixed_point.rs,open_snow.rs,open_snow_convergence_tests.rs}`, `dff_ws2_ksatadj_direct_runtime.rs`, exact impact-map bindings | production exact-floor terminal-phase transition defect: the authentic map oscillates between a mixed `0 C` state and a dry frozen `196.469 K` state; componentwise contraction correctly refuses the phase/posture crossing, while the cold authentic image cannot enter the unchanged `>=200 K` LSE domain | contract-first v31 admission of terminal-one-volume phase-aware unpublished contraction in canonical `W=ice+liquid`, `H=L_f*liquid-cold_content` coordinates, reconstructed from immutable beginning plus complete support mass/energy operands and canonical phase projection; retain exact structure/density/topology/custody/receipts/closure, raw authentic history, unchanged cap/tolerances/floor/rollback, and fresh-authentic-only final replay/acceptance/publication | retained rejected v30 componentwise attempt; captured mixed/frozen `1860..1980 s` vector with independent mass/energy reconstruction; phase-kink/zero-enthalpy/fusion-capacity/vapor-sign/structure/nonfinite poisons; no intermediate accept/publication; real captured support then full `dff_ws2` |
| WGHL-FULL-001E | `direct_runtime/stage3_committed_publication{,_wat5,_tests,_tests_tail}.rs`, `direct_runtime/01_publication.rs`, `v9_real_consumer_shadow_publication_retention.rs`, `v9_real_consumer_shadow_wb14_routing_tests.rs`, runner WAT5 transaction tests, CLI derived publication test, impact map | production accepted-publication projection defect: requested WAT5 lacks an accepted source/opt-in before generation; internal same-support `UpstreamRunon` custody is sealed in accepted ingress receipts, while open-ingress/LSE forcing parcels correctly contain no such routed handoff; public `UpStrmQ`/`SubRIn` then read normalization aliases instead of the sealed downstream runon-carry transfer operands | expose crate-private sealed accepted ingress-receipt access; reconstruct destination-basis/timed runon and independent transfer closure from exact accepted receipt dispositions; publish `UpStrmQ`/`SubRIn` from `runon_carry_downstream_operands` without overwriting or double-counting normalization; assemble requested WAT5 source only from accepted receipt/replay operands and install opt-in/payload before generation; retain atomic pending-frame publication and typed WAT5/transaction failures | pre-edit red WAT5 transaction and corrected no-alias CLI expectation paired with nonzero real routed-receipt consumer; focused publication/WAT5 suites; independent operand reconstruction and real per-OFE/transfer/hillslope closure |
| WGHL-FULL-001F | `SC-LANDSURFACEENERGY-001`, `openwepp-land-surface-energy/src/solver_covered_solve.rs` and its exact inline tests, unchanged `v9_real_consumer_shadow_wb14_tests.rs` interior-terminal tests, impact map if required | production deterministic termination defect: current residuals pass; full trial fails the existing no-update witness because its hydraulic step is `1.2616934700542904e-7 > 1e-7`; first eligible halved trial is domain-valid with every governed step passing, but strict residual decrease is unobservable at binary64 roundoff and the 20-halving ceiling rejects | contract-first extension of the existing no-update witness: only after full trial is domain-invalid or any governed prospective step exceeds its unchanged threshold, inspect the first domain-valid halved candidate; require current residual vector and that candidate's exact prospective component steps all pass, then accept current iterate with no update; otherwise preserve strict-decrease actual updates and fail closed | retained real pre-red; full-domain-invalid and full-step-too-large positive vectors, residual poison, each step-coordinate poison, full-trial accepted witness, genuine strict-decrease update, backtracking failure, exact rollback, then both unchanged interior terminal-event tests |
| WGHL-FULL-001F-BIND | `tests/integration/land_surface_energy_balance_authority_contract.rs`, contract index, exact impact-map binding | source-bound authority test still asserts LSE contract v12 and cannot see the new no-update witness invariant | advance only the exact version/source binding to v13/INV-139 and add one critical exact-path impact binding; preserve every scientific assertion | exact integration test, contract admission, A0, anti-evasion |
| WGHL-FULL-001G | distinct successor `docs/work-packages/20260830-frozen-forest-litter-phase-authority-001/` | baseline-historical but unwaived production authority gap: valid snow-free forest-litter liquid can cool below the liquid-only phase domain; current LSE/surface-liquid contracts explicitly reject frozen/thawing state | execute the scaffolded contract-first successor using retained R-156 plus pinned official SURFEX v8 bounded kinetic phase authority; new immutable LSE/state identities, exact liquid/ice/fusion-energy custody, phase-specific vapor, restart/rollback and no hidden cleanup | successor dual-reviewed gates plus unchanged p61 and native-forest real consumers; then parent full profile |
| WGHL-FULL-001H | `snow_stage3_v11_adaptive_execution{,_tests}.rs`, `snow_stage3_v11_qualification_crossjoin{,_child_tests}.rs`; unchanged attachment validator and two 001F real consumers | latent producer partition defect exposed after 001F: terminal receiver `[540,600)` is followed by a successor extending past the sealed accepted child `[420,900)`, normally `[600,1800)`, so qualification correctly rejects chronology | derive the terminal accepted-child end from the last sealed accepted-microstep receipt and cap successor cadence there; after `900 s`, resume ordinary cadence to parent end; do not weaken validator or add general microstepping | producer partition/receipt/restart vectors; crossing-successor poison stays rejected; exact `[540,600),[600,900),[900,1800)` chronology; unchanged two interior-terminal real consumers and complete ledger/owner closure |
| WGHL-FULL-001I | `SC-SURFACELIQUID-001`, `SC-LANDSURFACEENERGY-001`, authority tests/index/impact bindings; LSE exact-dyadic/owner/transaction modules; orchestrator soil-thermal owner/serialization/canonical bytes and receiver/finalization modules; persisted-restart successor owner/checkpoint/projection/host/transaction modules; exact focused tests | strict representability defect: canonical accepted WAT5 infiltration carries `-8.0670339832330148e-19 J m^-2` into soil high term `-34315.42154113602 J m^-2`; the credit is nonzero and conserved but only `1.10875e-7` ULP, so scalar bits cannot change and `SURFACELIQUID-E-003` correctly refuses | contract-first v15 receiver-owned exact normalized dyadic carry with total `E=exact(H_hi)+R`; aggregate canonical accepted operands exactly, round high once nearest-even, retain exact remainder, bind V2 owner/receipt/restart/checkpoint identities, freeze V1 bytes, zero-carry migrate only, refuse downgrade/tolerance/nextafter/forced ULP/diagnostic persistence, preserve exact rollback | isolated v15 expected-red; canonical WAT5 vector plus signs/tie/crossing/cancellation/order/subnormal/overflow; V1 byte lock/migration/tag/downgrade; receipt identity/order/substitution; exact closure and rollback; restart split before/after credit; real WAT5 nonzero carry and unchanged p61/native successor consumers |
| WGHL-FULL-001D-V32 | `SC-SNOWENERGY-001`, index/impact/assurance binding; `v11_covered/{fixed_point.rs,open_snow.rs,open_snow_convergence_tests.rs}`; exact adaptive/direct support and real `dff_ws2` tests | v31 real-consumer HOLD: exact private `1860..1920 s` support images reverse from pure deposition `V=+2.12159691239571346e-4`, `Q_v=+649.057936925197964` to pure sublimation `V=-4.61661230425127085e-3`, `Q_v=-13081.6326253264015`; v31 correctly refuses the vapor active-set crossing | contract-first v32 retains v31 same-disposition W/H projection and adds an unpublished pure opposite-sign vapor-root interface plus zero-to-one-sided branch entry at any exact support `>=60 s`; set `V=D=S=Q_v=+0` at unique root, interpolate only external liquid/nonlatent ordered energy, recompute Q and canonical W/H phase projection, require later fresh authentic acceptance; refuse mixed disposition and every closure/identity/event/custody mismatch | exact captured operands/log hashes and affine-latent wrong-formula rejection; opposite-sign/root/branch-entry, same-sign v31, >60 s direct support; mixed/capacity/nonfinite/component/identity/event/cap/rollback/publication poisons; real DFF counts/widths/runtime/reasons and exact mass/energy/vapor closure |

Focused correction evidence:

- format and affected all-target/all-feature check: `PASS`;
- affected coupled-time/biogeochemistry tests: 28/28 `PASS`, run
  `185a81ce-6d40-4f16-ac2c-157ac3e7afef`;
- coupled-time authority tests after the mechanical test extraction: 18/18
  `PASS`, run `13cc83d2-7f51-4bd4-8f69-376380e59de6`;
- warnings-denied all-target/all-feature Clippy passes independently for both
  affected crates after `WGHL-CLIPPY-001C`.

Full-profile family classification is pending delegated baseline/current
inventory comparison. No additional implementation path is authorized yet.

The read-only typed amendment check passed and proposed generation
`5c275785... -> 0102e72c...`, exactly five generated lock/report mutations,
three affected reports, no invalidated authority, and one generated transaction
receipt. Its declared impact is `scientific-full`; no proportional focused-gate
claim is made.

The typed source adoption was applied as transaction
`fbebcf402b46751e84527f878fd2580e5282ffeee8d92738728847c7d29f97da`;
generation advanced exactly as checked, `validate --all` passes 3/3, the repeat
check is a no-op, and the assurance-amendment profile passes 58/58 (73 skipped),
run `49352baa-20f1-45df-bbc7-979e1e190ad3`.

The vegetation mechanical plan is recorded in
`mechanical-refactor-plan.md`; no contract amendment, lint allowance, public
surface change, formula change, or error-precedence change is authorized.

Retained comparison proves the same P102 source reaches the profile timeout
with an API-authored no-strata seed but fails immediately with the committed
two-strata seed. Production bijection/chronology guards remain unchanged; this
correction repairs fixture authorship instead of weakening those guards.

`WGHL-FULL-001D` rejected the v30 componentwise safeguard after the real fixture
proved that correct phase/posture guards prevent activation. Version 31 may
reconstruct only the unpublished terminal-one-volume numerical iterate through
canonical total-water/enthalpy phase projection; it may not interpolate phase
components or cumulative ledgers independently. Fresh authentic acceptance and
every physical owner/closure guard remain unchanged.
