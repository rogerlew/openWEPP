# Work Packages

> **Canonical roadmap: [../ROADMAP.md](../ROADMAP.md)** — a **forward-only planning
> queue** (what is next and deferred). The section below is this roadmap's
> **execution log**: the home for **completed** work — package status, detail, and
> commits. When a queue item closes it is removed from `ROADMAP.md` and recorded
> here. If the two disagree on what is next, `ROADMAP.md` wins.
> Scaffolded but unexecuted packages stay discoverable from `ROADMAP.md`, their
> package-local `package.md`, and the active/held package pointer below; they
> enter the execution log after closure.

## Current Active/Held Packages

State as of `2026-07-06`:
- `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/` —
  SCAFFOLDED: the §6.1 D10 hold-lift. Reconciles the
  `SC-OFEROUTE-001#GAP-OFEROUTE-005` source-authority hold via three legs:
  (A) bind limiter/CFL/dissipation to the TVD family primaries (all four
  in hand: Mingham 2001, Garcia-Navarro 1992, and — acquired 2026-07-06
  from R-63's own citation chain — Davis 1984 and Tseng 2010); (B)
  re-anchor Case-4 acceptance
  to the Iwagaki 1955 primary (characteristics oracle, experimental
  hydrographs, `n = 0.009` via a named definitional mapping) and demote the
  digitized enhanced-WEPP trace to an ADR-0017 comparator flag; (C)
  reclassify the OFE sampled handoff under the conservation hard gate and
  adjudicate the H2637 resolution sensitivity with a seam-decomposed
  ledger. Clean-room boundary preserved; no production/default activation. A
  non-HOLD closure lifts the D15-rerun blocker; a HOLD must narrow D10's
  boundary and leaves D15 blocked.
- `20260705-mofefid-d15-opt-in-production-activation-001/` —
  EXECUTED-HOLD-SOURCE-AUTHORITY: Lane D opt-in production activation
  preflight. D15 confirmed that `SC-OFEROUTE-001` rev 23 still blocks active
  routed-water publication through `INV-OFEROUTE-011` / `GAP-OFEROUTE-005`
  (Case-4 shock-numerics/source-authority hold). No runtime, contract, schema,
  fixture, or test files changed; the next actionable item is the D10
  source-authority reconciliation, then a D15 rerun against the D14 runtime
  budget.
- `20260705-mofefid-d14-laned-runtime-profile-optimization-001/` —
  EXECUTED-COMPLETE: Lane D runtime profiling/optimization package. D14
  re-pinned the H2637 Lane D shadow cost at release grade (+65.3 s user over
  the 2.3 s default path on the 2-year fixture), attributed it with
  persistent slot diagnostics (`ofe_routing::profile`,
  `OPENWEPP_LANED_SHADOW_PROFILE=1`) plus perf evidence (~97 % solver math;
  51 % redundant alpha recomputation; pow 36 %), and landed three
  bit-identical optimizations (single alpha evaluation per cell per step,
  per-solver step scratch, hoisted skin rain term): shadow-on 67.6 s →
  29.9 s (2.26×), overhead −58 %, protected outputs and the `laned_shadow`
  manifest diagnostics bit-identical, solver trajectory counters identical
  (10,334,879 steps). D15 receives the fresh runtime budget. No production/default
  activation, D10 shock-numerics correction, D11/D12/D13 semantic changes,
  D15/D16 policy, or surrogate physics.
- `20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/` —
  EXECUTED-COMPLETE: Lane D routed-hydrograph erosion-shape package.
  `SC-OFEROUTE-001` rev 23 and `SC-SED-001` rev 53 bind the active-candidate
  erosion hourly-shape rule: when Lane D routed water owns the surface-water
  path, Wave-1 erosion consumes the routed hydrograph rather than DC01
  source-shape weights. Default/off remains on DC01; no production/default
  activation, D10 shock-numerics, D11 friction-source, D12 melt-limb, D14
  profiling, D15/D16 policy, or watershed/channel routing work.
- `20260705-mofefid-d12-melt-limb-hourly-shape-001/` —
  EXECUTED-COMPLETE: Lane D melt-limb hourly source-shape package.
  `SC-OFEROUTE-001` rev 22 ratifies the producer-owned
  `snow.hourly_routed_melt_m` limb bound to
  `SC-RUNOFFPART-001#INV-RUNOFFPART-022`; the real DC01/ADR-0036/Lane D
  consumer path reads the closed vector. H2637 records
  `days_uniform_shape_with_routed_melt=0`; the remaining `6`
  `days_uniform_shape` are no-authorized-source-shape residuals and remain
  diagnostic-only. No production/default activation, D10 shock-numerics, D11
  friction-source, D13 erosion-shape, D14 profiling, or D15/D16 policy work.
- `20260706-mofefid-d11-gap007-dynamic-friction-closure-001/` —
  EXECUTED-COMPLETE: D11 hold-lift package closed the remaining dynamic
  `GAP-OFEROUTE-007` operands for the opt-in Lane D shadow. `SC-OFEROUTE-001`
  rev 21 ratifies source/timing for skin rainfall intensity `I` from live WB14
  hourly rainfall depth (`/3600 s`), post-growth `LAI` from the executed direct
  day frame, and canopy height `h_c` from typed-management `canhgt`; tests and
  review prove the real shadow consumer reads those operands. No
  production/default activation or Case-4 shock acceptance claim.
- `20260705-mofefid-d11-friction-operand-authority-001/` —
  EXECUTED-HOLD-SOURCE-AUTHORITY then follow-on PARTIAL-CLOSED: Lane D
  friction operand authority package. Rev 19 recorded the original
  `GAP-OFEROUTE-007` boundary; follow-on rev 20 plus commit `f72e7749`
  ratified native management `routing_coefficients` for static Lane D shadow
  operands (`k_o`, form `C_d`, `D_r`, `lambda`, vegetation `C_d`) and made
  `OPENWEPP_LANED_SHADOW=1` fail closed unless every scheduled MOFE lane
  landuse has a complete, schedule-consistent native extension. Dynamic
  `I`/`LAI`/`h_c` closure completed in
  `20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- `20260705-mofefid-d10-shock-numerics-gap005-001/` —
  EXECUTED-HOLD-SOURCE-AUTHORITY: Lane D shock-numerics defect-closure
  package. Case 4 and the real-H2637 shadow reproduction were rerun, TVD
  primaries were read/acquired into `SC-OFEROUTE-001` rev 18, and Case-4-only
  D-val resolution controls were added. `GAP-OFEROUTE-005` remains held because
  the available source authority does not yet bind the reduced KWE limiter,
  lateral-source/OFE sampled-handoff treatment, and Iwagaki Manning-`n` to
  D-val friction operands. No production activation or D11-D13 work.
- `20260705-mofefid-d9-dval-disposition-001/` —
  EXECUTED-COMPLETE: Lane D D-val disposition package. Cases 1-3 were re-run
  after D8 and retain named non-numerics dispositions, the Figure 9 Zone 1/Zone
  2 taxonomy is executed, and the exact Case-4 acceptance handoff is isolated
  to `GAP-OFEROUTE-005` / D10. No production activation.
- `20260705-mofefid-laned-activation-increment-001/` —
  EXECUTED-REVIEWED: Lane D runtime SHADOW landed under
  `OPENWEPP_LANED_SHADOW=1`; live publication surfaces feed the real
  `ofe_routing` cascade diagnostics on the lane-local `runvol/area`
  source-depth basis, the real H2637 vector executes, protected outputs
  remain byte-identical, and Codex subagent review findings are
  dispositioned. Production activation remains blocked by the enumerated
  flip preconditions (`INV-OFEROUTE-011`, `GAP-OFEROUTE-005`, melt-limb
  coverage, future production-consumer proof for the rev-21 friction operand
  path, and the ADR-0036 erosion hourly-shape switch).
- `20260705-mofefid-laned-seam-implementation-001/` — COMPLETE, MERGED TO
  MAIN `0cccf263` 2026-07-05: `ofe_routing::seam` lands the GAP-006
  machinery (rate series + `/3600` helper, hourly-lane precondition, DC01
  exclusion, forcing sampler, closure identity) with BOTH
  `INV-OFEROUTE-012` gate fixtures passing at the solver/identity tier;
  SC-OFEROUTE-001 rev 14 (monotonic renumbering, reconciled seam status,
  `latqcc` mm governance). Runtime wiring + the real-H2637 executed
  vector = the activation increment; the activation BLOCK stands.
- `20260705-mofefid-laned-gap006-subsurface-seam-design-001/` — COMPLETE,
  MERGED TO MAIN `bce8da7a` 2026-07-05 (docs-only): `GAP-OFEROUTE-006`
  design-RESOLVED — the Lane D activation seam binds to existing surfaces
  (`ui_SCrunf` exfiltration with the recorded depth→rate helper,
  `ui_LfCrf` stays subsurface, outlet `latqcc` bypasses, hourly-lane
  precondition); SC-OFEROUTE-001 rev 4; both `INV-OFEROUTE-012` gate
  fixtures specified.
- `20260705-forest-lanuse-sediment-tie-in-001/` — COMPLETE, MERGED TO MAIN
  `e3015f5b` 2026-07-05: `is_cropland` resolves from the schedule-scoped
  parsed lanuse (cropland ⇒ the legacy `drinti` branch, forest ⇒
  `intdr = 1`, mixed/missing fails closed); first native-forest sediment
  proof (HJ Andrews `ow-lanuse-1`: minor-1 event + intake closure);
  evidence in band (p61 0.93×, p102 0.76× legacy). SC-SED-001 rev 52.
- `20260705-erosion-ground-cover-authority-defect-closure-001/` — COMPLETE,
  MERGED TO MAIN `f7b82dcd` 2026-07-05 after Codex 3-round review: closes
  `GAP-SED-009` (erosion ground-cover authority — pools seeded from the
  declared IC covers per `init1.for`, covers re-derived per `covcal.for`;
  p61 3.97 vs legacy 4.2 kg/m, p102 17.4 vs ~19.4 kg/m/yr).

- `20260703-dff-ws3-directional-burn-validation-001/` is held as DFF WS-3
  DIRECTIONAL BURN VALIDATION + PEAKFLOW MAGNITUDE ADJUDICATION. Result:
  `EXECUTED-HOLD-DFF-WS3-SEDIMENT-PRODUCTION`. It installed the full McKenzie
  Bridge 80-cell disturbed-burn matrix fixture, proved representative p1/p4
  runoff and peakflow direction under direct production, and confirmed openWEPP
  does not reproduce the legacy river-scale peakflow artifact. It holds before
  sediment ordering because production direct erosion still disables Wave-1 and
  publishes zero `tdet`, `tdep`, and `sedcon_*`.
- `20260703-dff-ws3a-wave1-wave2-sediment-production-001/` is queued as DFF
  WS-3A WAVE-1/WAVE-2 SEDIMENT PRODUCTION. Result: `QUEUED`. It is the
  dedicated hold-lift package for `HOLD-DFF-WS3-SEDIMENT-PRODUCTION`: implement
  real contract-backed EROD13 Wave-1 and EROD14 Wave-2 production in the direct
  runtime, prove the downstream HBP parquet consumer path, and then resume WS-3
  sediment ordering. It should coordinate with
  `20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/`.
- `20260702-mofefid-d8-routing-fidelity-defect-closure-001/` is executed and
  review-ready as MOFEFID-D8 ROUTING FIDELITY DEFECT CLOSURE. Result:
  `EXECUTED-REVIEW-READY`. It closed the four D7 D-val discrepancies under
  `SC-OFEROUTE-001` rev 9: skin SI `I` convention corrected/pinned, sampled
  hydrograph timing corrected, Case 2 classified `Ks` operand-limited, Case 3
  classified comparator-surface/operand boundary, Case 1 classified Green-Ampt
  operand-limited, and Case 4 carried as `GAP-OFEROUTE-005` shock-capture
  resolution sensitivity. Routing remains shadow-first; no production wiring.
- `20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/` is
  queued as WSHED-W7DC01 HILLSLOPE SEDIMENT PRODUCTION HOLD LIFT. Result:
  `QUEUED`. It is the next concrete watershed hold-lift action from
  `docs/ROADMAP.md`, scoped to closing `WSHED-W7-HOLD-001`: production
  hillslope HBP sediment remains zero for inspected real multi-OFE source
  substrates even when EROD14 is enabled.
- `20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/`
  is held as WSHED-W7 SEDIMENT-ACTIVE WATERSHED FIXTURE AND PUBLICATION
  CLOSURE. Result: `EXECUTED-HOLD-HILLSLOPE-SEDIMENT-PRODUCTION-MISSING`. It
  fixed relative `--run-dir` generated child input canonicalization and proved
  the public watershed path still runs, but no inspected committed or local
  candidate produced production-generated nonzero openWEPP sediment. The
  package holds before fixture adoption, serial/parallel identity, and
  conservation reconstruction.
- `20260630-typed-direct-setup-symbol-map-elimination-001/` is held after Stage
  0. Result: `EXECUTED-HOLD-STAGE0-PREMISE-CORRECTED`.
- `20260630-typed-direct-setup-symbol-map-carrier-deletion-001/` is held as
  TYPED DIRECT SETUP + SYMBOL-MAP CARRIER DELETION. Result:
  `EXECUTED-HOLD-STAGE1-TYPED-SEED-AUTHORITY-MISSING`.
- `20260630-stage1-seed-authority-migration-001/` is held as STAGE 1
  SEED-AUTHORITY MIGRATION. Result:
  `EXECUTED-HOLD-STAGE1C-TYPED-LANE-SEED-AUTHORITY-MISSING`. Stage 1B moved
  direct runoff publication `efflen_m` to typed topology geometry and verified
  H2637 plus multi-OFE/Wave-2 identity, reducing the seed-read inventory
  `208 -> 207`. Stage 1C is blocked until a typed per-lane seed-authority
  carrier exists for day-zero constructor and day-input authority state.
- `20260630-typed-seed-authority-carrier-rearchitecture-001/` is held as TYPED
  SEED-AUTHORITY CARRIER RE-ARCHITECTURE. Result:
  `EXECUTED-HOLD-PHASE1-TYPED-PROJECTION-APIS-MISSING`. Static execution
  confirmed the remaining seed authority is an ordered computed symbol-map
  pipeline, not independent reads. The package cannot build a legitimate typed
  carrier until the static per-lane seed projection and WB11 day-zero projection
  are factored into typed projection APIs with surface-writer adapters kept only
  for compatibility replay and shadow comparison.
- `20260630-typed-day-zero-seed-computation-001/` is held as TYPED DAY-ZERO
  SEED COMPUTATION. Result:
  `EXECUTED-HOLD-PHASE3-SEAM-BOUNDARY`. It completed the typed carrier,
  cut production direct consumers over to typed seed authority, removed direct
  setup's static `HillslopeWritebackSurface` seed construction, removed the
  obsolete day-zero seed-surface bridge and dead lane-authority surface reader,
  and moved snowbench/PySnobal diagnostics to typed seed bridge values. H2637
  protected outputs are byte-identical, `compatibility_edge_invocations=0`, RSS
  is `84776 KiB`, and full gates passed (`1880` nextest passed). It holds before
  deleting `scheduler.rs`, `day_frame.rs`, and carrier types because ADR-0030
  still retains the explicit deprecated `--compatibility-runtime`
  replay/comparator seam.

- `20260630-compatibility-runtime-full-deletion-001/`,
  `20260630-kernel-boundary-typed-diagnostic-events-001/`, and
  `20260630-kernel-boundary-typed-phase-runoff-family-001/` are superseded by
  `20260630-kernel-boundary-terminal-typing-001/`, which completed the
  coordinated terminal deletion instead of continuing the held slices.

## Execution Log

- `20260705-local-ci-timing-profile-optimization-001/` is complete as LOCAL CI
  TIMING AND PROFILE OPTIMIZATION. Result: `EXECUTED-COMPLETE`. It added
  persistent local nextest timing diagnostics under `tools/local_ci/`, installed
  the local-CI gate-selection standard, added an erosion domain nextest profile,
  and empirically tuned `forest` fixture concurrency. Measured cap decisions:
  `cli-fixture 2 -> 4` (`245.884 s -> 120.590 s`), `runner-fixture 2 -> 4`
  (`71.948 s -> 31.159 s`), `frost-fixture 2 -> 4`
  (`1.878 s -> 0.929 s` on the non-snowbench subset), and `snowbench` remains
  serial because cap 2 did not improve the representative subset
  (`262.323 s` vs `263.345 s`). Bernoulli's review findings were accepted and
  fixed; Bernoulli re-check plus Locke second review found no remaining
  blockers. Full-suite closure remains required for branch-head implementation
  gates; the package changes local iteration and review-response gate
  selection.
- `20260702-wshedw6-publication-large-watershed-scaling-001/` is complete as
  WSHED-W6 PUBLICATION AND LARGE-WATERSHED SCALING. Result:
  `EXECUTED-COMPLETE`. It moved public watershed publication directly onto
  `WatershedPublicationFrame` via `write_typed_publication_parquet_outputs`,
  kept unavailable process operands null rather than fabricated, carried
  source-runfile or manifest area operands into typed publication, adopted the
  full `1305`-hillslope `onshore-xenophobia` committed fixture, and retained
  `carnivorous-adobo` as the full `32`-hillslope development gate. Full
  scaling passed for `onshore-xenophobia` `--jobs 1/48`
  (`1:31:51` -> `3:27.55`) and `carnivorous-adobo` `--jobs 1/32`
  (`0:19.11` -> `0:01.07`), with all `14` required watershed parquet outputs
  schema/row-identical across job counts. Pinned legacy same-fixture full runs
  also completed for both fixtures. Final gates passed:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full` (`1205` passed, `1` skipped),
  `cargo deny check`, fixture manifests, authority anti-evasion guards, docs
  lint, and `git diff --check`.
- `20260701-hillperf-frost-single-solve-001/` is complete as the sub-5×
  program's WP-2 (finding F1). Result: `EXECUTED-COMPLETE`, rubric bar
  passed. The winter frost partition is solved exactly once per (lane, day)
  from start-of-day lane state and applied at the `r4w` frost ingress
  (between R4C and R4I); the R4A re-solve, layer overwrite, and rebalance
  bridges are deleted. Acceptance = frost observation rubric no-worse in
  every verdict-bearing cell (frost-tube sites improved, Morris −4.9 cm max
  residual; snow columns bit-identical; one non-verdict Mandan isotherm cell
  +57 recorded as bounded). H2637 46.69 → 32.77 s (3.52× legacy, quiet
  3-rep); first-divergence and paired-solve diagnostics archived;
  Codex-reviewed, findings dispositioned.
- `20260701-hillperf-mechanical-winter-overhead-001/` is complete as the
  sub-5× program's WP-1 (identity-preserving lane: F2/F3/F5/F6/F7/F8).
  Result: `EXECUTED-COMPLETE`, byte-identical on all five H2637 protected
  outputs at every commit. Guard-symbol construction deferred to failure
  branches (~85 sites), seasonal frost curve fit hoisted to once-per-lane,
  construct-behind-the-gate fixes; F3 verified non-viable (like-for-like
  rule), F8 skipped (manifest-entangled). H2637 71.4 → 46.69 s (4.80×,
  quiet 3-rep); Codex-reviewed, findings dispositioned.
- `20260701-wshedw5-old-watershed-runtime-deletion-001/` is complete as
  WSHED-W5 OLD WATERSHED RUNTIME DELETION. Result: `EXECUTED-COMPLETE`. It
  deleted the watershed-specific old request/writeback runtime, removed
  obsolete old-surface WS10/WS11/WS12 tests, trimmed stale runtime-input
  taxonomy, and kept public routing on `WatershedNetworkFrame` plus
  `execute_watershed_dispatch_with_frame`. Replacement typed coverage now
  proves direct dispatch/publication, WS11 branch closure, WS18/WS20
  transport-capacity sensitivity, WS12 inactive and active min-controller
  behavior, and WS12 non-finite/domain guard taxonomy. Source guards forbid the
  deleted old runtime in production watershed routing. Final gates passed:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full` (`1196` passed, `1` skipped),
  and `cargo deny check`.
- `20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/` is
  complete as WSHED-W4DC01 TYPED ROUTING KERNEL WRITEBACK CLOSURE. Result:
  `EXECUTED-COMPLETE`. It closed `WSHED-W4-HOLD-001` for the production public
  watershed CLI path by replacing compatibility writeback routing with
  `execute_watershed_dispatch_with_frame` over `WatershedNetworkFrame` and
  typed routed-state publication. It added direct typed channel/impoundment
  execution over frame records, reused actual WS11, WS12, WS18, and WS20 helper
  physics, preserved W2/W3 public behavior, and recorded that
  carnivorous-adobo remains an input/parser fixture rather than a current CLI
  output-identity fixture.
- `20260701-wshedw4-typed-watershed-network-frame-001/` is held as WSHED-W4
  TYPED WATERSHED NETWORK FRAME. Result:
  `EXECUTED-HOLD-TYPED-ROUTING-KERNEL-WRITEBACK-REMAINS-COMPATIBILITY-EDGE`.
  It landed typed `WatershedNetworkFrame` and `WatershedPublicationFrame`
  handoff in the public watershed CLI path, including typed hillslope
  contribution collection and typed publication consumption, but could not
  close complete until W4DC01 replaced routing through
  `compatibility_writeback_surface` and `execute_watershed_dispatch_with_kernel`.
- `20260701-wshedw3-bounded-worker-pool-001/` is complete as WSHED-W3 BOUNDED
  WORKER POOL. Result: `EXECUTED-COMPLETE-WSHED-W3`. It added
  `HillslopeWorkerPool`, removed the temporary public `--jobs > 1` rejection,
  preserved invalid/zero/negative `--jobs` rejection, records per-job
  concurrency/failure policy timing, clears stale generated pass/log/timing
  artifacts before launch, and proves focused `--jobs 1`/`--jobs 3`
  row-equivalence plus child-failure and missing-pass fail-closed behavior.
  User-authorized fixture-only `radly` normalization resolved the committed
  carnivorous-adobo direct-runtime guard without changing production clipping
  posture. The canonical release scaling matrix passed `1/2/4/8/16/32`, three
  repeats each, all row-equivalent to `jobs1-rep1`; average wall time improved
  from `36.96 s` at `--jobs 1` to `2.04 s` at `--jobs 32`. Final gates passed:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full` (`1283` passed, `1` skipped),
  and `cargo deny check`.
- `20260701-wshedw2-serial-watershed-supervisor-skeleton-001/` is complete as
  WSHED-W2 SERIAL WATERSHED SUPERVISOR SKELETON. Result:
  `EXECUTED-COMPLETE-WSHED-W2`. It implemented the public
  `openwepp-cli-watershed --jobs 1` serial supervisor skeleton with
  `WatershedRunPlan`, `HillslopeJob`, and `PassInventory`; generated
  hillslope jobs now use isolated per-job output/log/timing paths, stale
  generated artifacts fail closed before routing, missing latest-event payloads
  fail closed, and routed-stage reuse remains explicit through
  `use_existing_pass_file = true`. Focused watershed CLI coverage passed
  (`20` tests), and final gates passed: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full` (`1280` passed, `1` skipped),
  and `cargo deny check`.
- `20260701-wshedfixture01-committed-watershed-fixture-adoption-001/` is
  complete as WSHED-FIXTURE01 COMMITTED WATERSHED FIXTURE ADOPTION. Result:
  `EXECUTED-COMPLETE-WSHED-FIXTURE01`. It adopted the 32-hillslope
  carnivorous-adobo development fixture under
  `tests/fixtures/watershed/carnivorous-adobo/`, recorded source provenance,
  topology summary, required input/runfile inventory, and checksum manifest, and
  added a focused integration gate proving the persistent fixture path is the
  committed openWEPP path rather than `/wc1` or wepppy.
- `20260701-wshedperf01-watershed-baseline-performance-characterization-001/`
  is complete as WSHEDPERF01 WATERSHED BASELINE PERFORMANCE CHARACTERIZATION.
  Result: `EXECUTED-COMPLETE-WSHEDPERF01`. It measured pinned legacy
  `full-legacy-watershed` at `0:07.86`, openWEPP routed-stage from existing HBP
  at `0:00.07-0:00.08`, and practical full openWEPP command-chain repeats at
  `1:02.38`, `1:01.41`, and `1:01.06` (`avg 1:01.62`) plus a full-chain profile
  run at `1:02.07`. The package closes with no active blockers; legacy and
  openWEPP timing scopes remain non-equivalent unless a legacy-equivalent
  openWEPP surface is introduced. The initial comparator subagent dispatch
  errored due model capacity, so benchmark runs completed locally with
  command-level evidence.
- `20260701-wshedarch01-watershed-runtime-architecture-specification-001/` is
  complete as WSHEDARCH01 WATERSHED RUNTIME ARCHITECTURE SPECIFICATION. Result:
  `EXECUTED-COMPLETE-DRAFT-SPEC-REV4-CLAUDE-STATIC-VERIFICATION-DISPOSITIONED`.
  It reviewed WSHEDPERF01 and current
  watershed CLI/orchestrator seams, then authored the draft architecture spec at
  `docs/architecture/watershed-runtime-architecture-specification.md`. The spec
  specifies a ground-up watershed runtime rewrite with full deletion of the
  existing runtime after replacement. The staged direction is bounded
  subprocess fanout, typed run planning, deterministic pass inventory, typed
  watershed network-frame replacement, and old-runtime/test retirement.
  Dual-review findings were accepted and dispositioned in Revision 2, adding
  fail-closed latest-event semantics, consumer-path proof, cross-scope legacy
  wording, ADR-owned `--jobs` defaults, deletion coverage restoration, and Rust
  closure gates. Revision 3 adds the fixture ladder: arboreal-dendrite remains
  smoke/baseline only, carnivorous-adobo is the preferred 32-hillslope
  near-term development fixture, larger 1,000+ hillslope fixtures are required
  after runtime progress, and adopted gate fixtures must be committed to the
  repository for auditability. Revision 4 dispositions Claude static
  verification by adding sidecar-discovery benchmark taxonomy, a ROADMAP
  WSHED-ADR/W2-W6 queue, `chan_out` naming, and contract-first `NoEvent`
  follow-up. It remains draft authority pending W2/W3/W5 implementation
  evidence.
- `20260701-wshedadr01-watershed-runtime-ratification-001/` is complete as
  WSHEDADR01 WATERSHED RUNTIME RATIFICATION. Result:
  `EXECUTED-COMPLETE-ADR0032-WATERSHED-RUNTIME-RATIFIED`. It ratified ADR-0032:
  the public watershed runtime remains `openwepp-cli-watershed`, the `--jobs`
  default is deterministic serial `1`, CPU scaling is explicit through
  positive `--jobs N`, and canonical benchmark/ratification evidence uses
  `strict-committed-fixture` mode with legacy sidecar discovery disabled.
  WSHED-ADR was removed from `docs/ROADMAP.md`; WSHED-FIXTURE01 remains the
  next watershed runtime queue item.
- `20260701-kernel-boundary-cqr-row2-climate-parser-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 2 CLIMATE PARSER. Result:
  `EXECUTED-COMPLETE-ROW2-CQR`. It executed row #2 of
  `kernel-boundary-cqr-burndown-execplan.md`; row #2 was already CRAP-clean and
  remained `0 -> 0` owned production functions above CRAP 30. It restored typed
  assertions for non-breakpoint direct climate forcing, breakpoint direct
  climate forcing, datver-0 override behavior, itemp runtime rejection, and
  direct-day out-of-range errors. Full gates passed (`1272` nextest passed);
  H2637 measured `1:07.75` / `77720 KiB`, protected H2637 outputs were
  byte-identical, and `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row1-soil-parser-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 1 SOIL PARSER. Result:
  `EXECUTED-COMPLETE-ROW1-CQR`. It executed row #1 of
  `kernel-boundary-cqr-burndown-execplan.md`; row #1 was already CRAP-clean and
  remained `0 -> 0` owned production functions above CRAP 30. It restored typed
  assertions for 9002 disturbed policy/measured FC-WP values, corrected typed
  theta stores, restrictive conductivity projection, and harmonic vertical
  `ssc` behavior. Full gates passed (`1267` nextest passed); H2637 measured
  `1:06.89` / `77756 KiB`, protected H2637 outputs were byte-identical, and
  `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row3-management-parser-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 3 MANAGEMENT PARSER. Result:
  `EXECUTED-COMPLETE-ROW3-CQR`. It executed row #3 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #3 owned production
  offenders from 1 unique entry (`2` duplicated report rows) to `0` entries
  above CRAP 30. It added typed assertions for disabled primary drain
  projection, enabled drain geometry projection, dangling drain references, and
  zero enabled-drain geometry fail-closed behavior. Full gates passed (`1264`
  nextest passed); H2637 measured `1:06.75` / `80052 KiB`, protected H2637
  outputs were byte-identical, and `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row8-per-ofe-mofe-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 8 PER-OFE MOFE. Result:
  `EXECUTED-COMPLETE-ROW8-CQR`. It executed row #8 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #8 owned production
  offenders from 2 unique entries (`4` duplicated report rows) to `0` entries
  above CRAP 30. It added typed assertions for R7H trace day/lane filtering,
  percolation JSON-line serialization, and subsurface saturation JSON-line
  serialization while preserving trace schema and best-effort append behavior.
  Full gates passed (`1260` nextest passed); H2637 measured `1:07.23` /
  `79420 KiB`, protected H2637 outputs were byte-identical, and
  `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row6-growth-decomposition-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 6 GROWTH DECOMPOSITION. Result:
  `EXECUTED-COMPLETE-ROW6-CQR`. It executed row #6 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #6 owned production
  offenders from 2 unique entries (`4` duplicated report rows) to `0` entries
  above CRAP 30. It added typed assertions for annual/perennial growth schedule
  branches and growth equation guard families, and split the direct growth
  schedule/equation validators into behavior-preserving helper groups. Full
  gates passed (`1257` nextest passed); H2637 measured `1:06.19` / `79828
  KiB`, protected H2637 outputs were byte-identical, and
  `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row5-hydrology-wb-kernel-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 5 HYDROLOGY WB KERNEL. Result:
  `EXECUTED-COMPLETE-ROW5-CQR`. It executed row #5 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #5 owned production
  offenders from 11 unique entries (`22` duplicated report rows) to `0`
  entries above CRAP 30. It added typed assertions for hydrology guard
  code/display coverage, snow albedo display variants, snow-density
  boundary-mass updates, R7G frost trace string escaping, frozen-soil k-factor
  resolution, snow-density guard mapping, SIMIMPL29 melt branches, and
  active-snow coupling edge paths. Full gates passed (`1254` nextest passed);
  H2637 measured `1:08.04` / `79916 KiB`, protected H2637 outputs were
  byte-identical, and `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row9-direct-runtime-physics-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 9 DIRECT RUNTIME PHYSICS. Result:
  `EXECUTED-COMPLETE-ROW9-CQR`. It executed row #9 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #9 owned production
  offenders from 14 unique entries (`28` duplicated report rows) to `0`
  entries above CRAP 30. It added typed assertions for PMET compute/storage
  terms, staged soil evaporation, R4N surface ET PMET/manual demand paths,
  day/lane constructor validators, snow/frost carry guards, day commit
  layer-source priority, and R4A frost rebalance. Full gates passed (`1246`
  nextest passed); H2637 measured `1:07.27` / `79736 KiB`, protected H2637
  outputs were byte-identical, and `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row7-wb-publication-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 7 WB PUBLICATION. Result:
  `EXECUTED-COMPLETE-ROW7-CQR`. It executed row #7 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #7 owned production
  offenders from 17 unique entries (`34` duplicated report rows) to `0`
  entries above CRAP 30. It restored typed assertions for retained publication
  frame guards, snow/frost insulation, snow selector parsing, Sturm climate
  normals, growth/residue projection, Priestley-Taylor demand, no-final-frost
  rebalance, frost carry projection, WB11 frozen-depth refresh, and WB16
  equivalent-plane alpha. Full gates passed (`1239` nextest passed); H2637
  measured `1:07.39` / `79588 KiB`, protected H2637 outputs were
  byte-identical, and `compatibility_edge_invocations=0`.
- `20260701-kernel-boundary-cqr-row4-runtime-inputs-001/` is complete as
  KERNEL-BOUNDARY CQR ROW 4 RUNTIME INPUTS. Result:
  `EXECUTED-COMPLETE-ROW4-CQR`. It executed row #4 of
  `kernel-boundary-cqr-burndown-execplan.md`, reducing row #4 CRAP offenders
  from 24 unique production entries (`48` duplicated report rows) to `0`
  entries above CRAP 30. It added typed assertions for runtime input error
  code/display coverage, annual-extension and perennial grazing projection,
  and SIMIMPL28 hourly winter forcing branches, plus behavior-preserving helper
  extraction in SIMIMPL28 sunmap and winter window normalization. Full gates
  passed (`1229` nextest passed); H2637 measured `1:06.99` / `79684 KiB`,
  protected H2637 outputs were byte-identical, and
  `compatibility_edge_invocations=0`.
- `20260630-kernel-boundary-terminal-typing-001/` is complete as
  KERNEL-BOUNDARY TERMINAL TYPING. Result:
  `EXECUTED-COMPLETE-TERMINAL-SINGLE-AUTHORITY`. It deleted the compiled
  symbol-map scheduler/day-frame runtime, carrier exports, scheduler
  trace/publication/seed support, and scheduler-only tests. Production hillslope
  execution is direct-only; forbidden carrier/runtime names remain only in
  source-guard literals. Full gates passed (`1221` nextest passed), H2637
  measured `1:10.69` / `79284 KiB`, protected H2637 outputs were
  byte-identical, and `compatibility_edge_invocations=0`.
- `20260630-kernel-boundary-survivor-inventory-001/` is complete as
  KERNEL-BOUNDARY SURVIVOR INVENTORY. Result:
  `EXECUTED-COMPLETE-SURVIVOR-CLASSIFICATION`. It executed step 1 of the
  kernel-boundary typing program from the array-native runtime specification:
  static scans classified the remaining symbol-map survivor surface into
  executable scheduler/day-frame runtime, kernel request/writeback boundary,
  diagnostic/trace support, WB13/publication/audit support, tests, and genuine
  intake/output adapters. The core survivor scan found `1,284` matches across
  `74` Rust files, while `BoundarySymbol`/`BoundaryValue` accounted for another
  `4,137` lower-level serialization/guard references. No code behavior changed.
- `20260630-direct-publication-streaming-sink-001/` is complete as DIRECT
  PUBLICATION STREAMING SINK. Result:
  `EXECUTED-COMPLETE-STREAMING-RSS-REDUCTION`. It made the production direct
  publication endpoint stream `DirectPublicationDayRow` values into compact
  summary state and requested WAT/PASS parquet row-group writers, then drop each
  row instead of retaining `DirectRunPublicationFrame.rows` whole-run. It also
  added incremental WAT/PASS parquet writers while preserving the existing
  slice-writing helpers. H2637 full-output RSS dropped from the prior held
  package's `316212 KiB` to `112652 KiB`; H2637 required-output RSS dropped
  from `184644 KiB` to `52228 KiB`. The required-output endpoint is now close to
  the `16437`-day W9 observed fixture (`47856 KiB`) despite H2637 emitting
  `235961` rows. H2637 HBP/loss/plot/WAT/PASS and cli01 HBP/loss/plot/WAT data
  outputs are byte-identical to the retained-row baseline. Full gates passed,
  including `cargo nextest run --workspace --profile full` after restoring the
  expected untracked `.venv` Python dependencies (`pyarrow`, `pandas`).
- `20260630-direct-publication-rss-reduction-001/` is held as DIRECT
  PUBLICATION RSS REDUCTION. Result:
  `EXECUTED-HOLD-PARTIAL-RSS-REDUCTION`. Stage A corrected the Stage 0 RSS
  attribution one level further: the dominant direct endpoint allocation was a
  typed direct setup vector,
  `Vec<DirectDayConstructorInputs>`, preallocated for every H2637 day/OFE
  (`235961` rows x `4040 B` = about `909 MiB`) even though production direct
  execution constructs day inputs dynamically. The package removed that
  preallocation, moved retained direct publication execution instead of cloning
  it, and skipped WAT/PASS projection row construction when those outputs are
  not requested. H2637 full-output RSS dropped from `1159672 KiB` to
  `316212 KiB`, and H2637 HBP/loss-only dropped from `1159296 KiB` to
  `184644 KiB`, with HBP/WAT/PASS/loss/plot bytes unchanged for full output and
  HBP/loss bytes unchanged for minimized output. The package remains held
  because RSS is materially lower but not yet run-length-flat: the direct
  publication frame still retains all `DirectPublicationDayRow` values, and
  full-output WAT/PASS projection plus parquet/Arrow buffers still scale with
  row count. Full-profile `nextest` also failed in this worktree because
  Python-backed harness tests could not launch `.venv/bin/python`.
- `20260630-typed-direct-setup-symbol-map-elimination-001/` is held as TYPED
  DIRECT SETUP PATH + SYMBOL-MAP CARRIER ELIMINATION + RSS REDUCTION. Result:
  `EXECUTED-HOLD-STAGE0-PREMISE-CORRECTED`. Stage 0 built the release CLI and
  profiled H2637 direct production (`1:09.18`, `1159672 KiB`), an H2637
  minimized-output variant (`1:13.77`, `1159296 KiB`), and the small `cli01`
  fixture (`0:00.09`, `19584 KiB`). Both H2637 manifests selected
  `direct-production-executor` with `compatibility_edge_invocations=0`, but
  removing optional WAT/PASS/plot outputs did not move RSS. Static audit found
  the setup-time symbol-map carrier is still present, but the dominant RSS
  suspect is whole-run retained direct publication/ledger state and
  unconditional output projection row materialization. The package stops before
  Stage 1 under blocker `BLOCKED-BY-RETAINED-DIRECT-PUBLICATION-RSS`; next work
  should stream/drop retained direct publication artifacts before resuming typed
  setup and symbol-map carrier deletion.
- `20260630-compatibility-runtime-deletion-001/` is complete as COMPATIBILITY
  RUNTIME DELETION. Result: `EXECUTED-COMPLETE-PARTIAL-DELETION`. It added
  ADR-0030, removed obsolete skeleton/shadow/cutover runtime selections and CLI
  flags, deleted the compatibility-shaped direct publication day-input builder
  and cutover adapter family, removed stale transition-mode tests, and added a
  source guard blocking their reintroduction. The no-env default and legacy
  sidecar-discovery paths remain direct production, with explicit
  `--compatibility-runtime` retained only as a deprecated replay/comparator seam.
  Full Rust gates passed (`cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`,
  `cargo deny check`, authority anti-evasion, and required-suite obligation
  guards). Full deletion of setup-time symbol-map carriers and the replay seam is
  deferred to a separate typed-setup/full-deletion package; RSS reduction was
  out of scope.
- `20260629-frost-direct-cutover-correction-001/` is complete as FROST DIRECT
  CUTOVER CORRECTION. Result:
  `EXECUTED-COMPLETE-DIRECT-CUTOVER-CORRECTION`. It amended
  `SC-SNOWFREEZE-001` v115 to supersede the v114 compatibility-fallback carve-out:
  current hillslope no-env `DefaultCandidate` runs, including multi-OFE/Wave-2
  and legacy sidecar-discovery surfaces, must select direct production. The
  package removed the default-candidate fallback gate, relabeled remaining
  explicit compatibility as a deprecated deletion seam, and closed the legacy
  sidecar-discovery direct replay ledger defect by carrying the PMET soil
  evaporation storage-return operand into direct storage reconciliation. Focused
  gates passed for PL14S legacy-discovery replay, R7E default-candidate selection,
  and R7H frost storage source isolation; full gates are recorded in the package.
- `20260629-frost-ratification-default-activation-001/` is complete as FROST
  RATIFICATION AND DEFAULT ACTIVATION. Result:
  `EXECUTED-COMPLETE-PRODUCTION-DEFAULT-ACTIVATION`. It ratified
  `INV-SNOWFREEZE-047/048/050`, adjudicated the Step 1 `>0.25`
  systematic-timing-fraction cutoff as diagnostic-local rather than invariant
  authority, recorded the H1b `frdp` bottom-extent / `thdp` top-thaw
  correspondence, re-dispositioned `GAP-SNOWFREEZE-002` as
  open-but-attributed/bounded, and flipped the no-env hillslope default to direct
  production for supported modern single-OFE runs. Its multi-OFE/Wave-2 and
  legacy sidecar-discovery compatibility-fallback carve-out is superseded by
  `20260629-frost-direct-cutover-correction-001/`. Full gates passed:
  `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`,
  authority-suite anti-evasion, required-suite obligation guards, and scoped
  Markdown lint.
- `20260629-frost-h1b-state-machine-thaw-asymmetry-check-001/` is complete as
  FROST H1b STATE-MACHINE THAW-ASYMMETRY CHECK. Result:
  `EXECUTED-COMPLETE-DIAGNOSTIC-NARROW-EDGE`. It consumed the post-residue
  Sleepers R7G frost traces, the Step 3 seasonal WAT outputs, and the prior
  thaw-residual H1b cells (`site2_sleepers_w9_hardwood_vt:1995:thaw` and
  `site2_sleepers_w9_hardwood_vt:2010:thaw`). Static code reading classifies the
  top-down thaw path as present: positive surface thaw selects branch 3, which
  calls `thaw_fine_top_with_resistance_feedback` and reduces surface fine-layer
  frozen depth/ice. The two H1b cells are not structural top-thaw failures:
  every no-`frdp`-retreat warm/material day in the cell windows shows `thdp`
  growth, so branch 3 is creating a surface-thawed cap while `frdp` remains the
  bottom extent of the frozen domain. Full Sleepers prevalence scan found `570`
  branch-3 warm/material days, `497` with next-day `frdp` retreat, `58` with no
  `frdp` retreat but `thdp` advance, and only `15` (`0.026`) with neither
  retreat. `GAP-SNOWFREEZE-002` remains open for snow-persistence uncertainty
  and the snow-free wet-heat/Qwet subset, but H1b is not a ratification blocker.
  No solver, state-machine, contract, detector, fixture, default, or schema
  change was made.
- `20260629-frost-snow-persistence-decomposition-001/` is complete as FROST
  SNOW-PERSISTENCE DECOMPOSITION. Result:
  `EXECUTED-COMPLETE-DIAGNOSTIC-SPARSE-OBS-NO-UNDER-MELT`. It consumed the
  snow-buried thaw-late cells from
  `20260629-frost-thaw-residual-diagnostic-001/`, paired Sleepers observed
  snow-depth rows, and the post-residue Step 3 seasonal WAT outputs. The `9`
  scoped cells (`7` snow-buried plus the buried portions of `2` mixed cells)
  route to `8` `INCONCLUSIVE-SPARSE-OBS`, `1`
  `OVER-ACCUMULATION-FORCING-LIMITED`, and `0`
  `SPRING-UNDER-MELT-FIXABLE`. The diagnostic therefore does **not** establish
  that the Sleepers frost thaw-late residual unifies with the
  SNOWDENSITY-10.3.8/10.3.10 spring-melt residual. `GAP-SNOWFREEZE-002`
  remains open; do not promote a snow melt-rate fix from these cells alone, and
  keep `Qwet` limited to the two snow-free persistent cells. No melt-model,
  snow-model, frost-model, contract, fixture, default, or schema change was made.
- `20260629-frost-thaw-residual-diagnostic-001/` is complete as FROST
  THAW-RESIDUAL DIAGNOSTIC. Result:
  `EXECUTED-COMPLETE-DIAGNOSTIC-SNOW-BURIED-DOMINANT`. It consumed the post-residue
  Step 3 seasonal Sleepers runs plus R7G frost traces and bucketed the remaining
  `13` candidate-defect timing cells. The `11` thaw-late cells split to `9`
  `H1a` missing wet/advective thaw energy, `2` `H1b` state-machine thaw
  asymmetry, and `0` `H2` tiny-tail cells through material thresholds up to
  `0.05 m`; only an unadopted `0.10 m` material threshold would classify four
  cells as H2. The Claude review finding was accepted: H1a over-routed to
  `Qwet` because snow depth was not controlled. Post-review snow-depth
  re-bucketing at diagnostic `0.10 m` routes the thaw-late cells to `7`
  snow-buried cells (`5` under-melt/linger, `2` accumulation/near-balance), `2`
  snow-free persistent `Qwet` candidates, and `2` mixed cells; the snow-buried
  count is stable across `0.05`, `0.10`, and `0.20 m`. The two early-onset cells
  are material early-freeze cells and remain a separate onset diagnostic.
  `GAP-SNOWFREEZE-002` remains open but is narrowed: next primary route is
  snow-persistence decomposition before any broad `Qwet` build. No solver,
  detector, fixture, contract, default, or output-schema change was made.
- `20260629-frost-residue-cover-implementation-001/` is complete as FROST
  RESIDUE-COVER IMPLEMENTATION, the contract-first dynamic seasonal forest
  litter/residue-depth coupling for the frost surface heat path. Result:
  `EXECUTED-COMPLETE-IMPLEMENTATION-BRANCH-A`. It amended `SC-RESIDUE-001` to
  revision 11 with `INV-RESIDUE-019` and `SC-SNOWFREEZE-001` to revision 113
  with `INV-SNOWFREEZE-083`, then wired direct-production surface-residue mass
  through a dynamic mass-to-depth publisher consumed by frost thermal inputs.
  Phase 0 showed the existing `Dec_*` mass path was flat under zero-rate/no-input
  management, so the implementation also added the missing recurring forest
  litter input limb: pending non-fall senescence mass is conserved until the
  45-day fall litter-drop window ending on the management fall date, with a
  forest-litter turnover fallback when `oratea=0`. Review disposition aligned
  that fallback to the cited authority (`k=0.5 yr^-1`) and recorded the
  management fall-date (`jdharv`) litter-drop anchor as a known limitation until
  the physical frost/daylength phenology backlog lands. The real Step 3 entry
  gate now passes (`autumn=0.165028 m`, `spring=0.159910 m`, max month October),
  and the Sleepers A-vs-B rerun routes to branch A as a partial contributor:
  candidate-defect timing cells reduced from `18` to `13`, with `13` cells
  remaining for follow-up frost attribution. No canopy leaf-on/off, Qwet,
  frozen-K, SFCC, impedance, frost-default activation, public schema,
  snow-model, fixture repoint, or legacy-comparator targeting change was made.
- `20260629-frost-step3-residue-parameterization-001/` is complete as FROST
  STEP 3, the diagnostic residue-parameterization test for the two Step
  1-unblocked Sleepers sites. Result:
  `EXECUTED-COMPLETE-DIAGNOSTIC-BRANCH-C`. The entry gate ran the existing
  `hubbardbrook_deciduous_nh` `Dec_4899` seasonal deciduous fixture with
  `OPENWEPP_R7G_FROST_TRACE_PATH` enabled and confirmed the solver-side
  `residue_depth_m` path is flat: `32874` trace rows, min/max/mean all
  `0.02302585092994045 m`, and equal autumn/spring means. Because `Dec_*` did
  not produce a seasonal residue trajectory at the frost solver, the package
  stopped before the Sleepers A-versus-B re-score. `GAP-SNOWFREEZE-002`
  remains open; the follow-on is to promote the surface residue / forest litter
  cover dimension of
  `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  before rerunning Sleepers timing attribution. That follow-on was executed by
  `20260629-frost-residue-cover-implementation-001/`. No frost-model, snow-model,
  production fixture, contract-physics, default, output-schema, selector, or
  harness-default change was made.
- `20260629-frost-step2-sleepers-attribution-001/` is complete as FROST STEP 2,
  the diagnostic attribution package for the two Step 1-unblocked Sleepers
  sites. Result: `EXECUTED-COMPLETE-DIAGNOSTIC-ATTRIBUTION`. It consumed the
  Step 1 current-snow reports without rerunning the harness, scored
  onset/thaw/frozen-duration timing signatures against the `+/-14 day`
  forcing-robust tolerance, reconstructed full frost-depth residual
  distributions from the recorded WAT outputs, and applied sign coherence
  against the modeled-over-observed snow residual. Both sites expose
  candidate frost-model timing defects not explained by deeper modeled snow:
  South Field has `4` thaw-late candidate cells, while W9 Hardwood has `14`
  early-onset/thaw-late candidate cells. Magnitude remains forcing-limited and
  non-verdict-bearing: South Field is mixed-sign, W9 is sign-incoherent. The
  Step 3 pointer is residue-lifecycle handoff (`static` vs dynamic `resdep`),
  with the legacy-envelope outlier set as ADR-0017 comparator context; absent
  `Qwet` is not the primary pointer from this sign-coherence pass. No frost,
  snow, Qwet, frozen-K, SFCC, impedance, contract-physics, default, fixture,
  schema, selector, or ratification change was made.
- `20260629-frost-step1-current-snow-control-rerun-001/` is complete as FROST
  STEP 1, the current-default snow-control rerun for observed frost sites.
  Result: `EXECUTED-COMPLETE-DIAGNOSTIC-ROUTING`. It reran the existing
  five-site frost observation harness through `openwepp-cli-hill` on the
  current no-env snow default (`coe_liquid_holding_capacity_v1 +
  physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`) and preserved
  diagnostic-only scope. The legacy scalar `INV-SNOWFREEZE-048` audit still
  reports three paired snow-depth failures plus two no-paired-snow sites, but
  applying `INV-SNOWFREEZE-050` forcing-robust tiering narrows the route:
  Sleepers South and Sleepers W9 are `FORCING-LIMITED` (frost timing
  attributable; magnitude carries snow-depth forcing uncertainty), Morris
  remains `BLOCKED` by systematic snow-cover timing/regime mismatch, and
  Mandan/Reynolds Creek remain `INCONCLUSIVE-NO-PAIRED-SNOW`. No frost-model,
  snow-model, contract-physics, default, fixture, schema, selector, or
  ratification change was made.
- `20260629-paradigm-2-multilayer-promotion-001/` is complete as PARADIGM-2
  MULTILAYER PROMOTION, ratifying the Stage 3-Decouple snow-neutral
  water-temperature arm as a production-supported internal opt-in capability.
  Result: `EXECUTED-COMPLETE-PRODUCTION-OPT-IN`. It amended
  `SC-SNOWFREEZE-001` to v112 with `INV-SNOWFREEZE-082` and
  `OBL-SNOWFREEZE-P-057`, kept the no-env bulk snow default and rollback
  unchanged, and publishes nullable hillslope WAT parquet
  `MeltwaterTemperature` in `degC` from the Stage 3 meltwater-flux temperature
  when `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` is
  selected. Real cross-SNOTEL/cancov guardrails reconfirmed exact current-
  default equivalence (`15` robust fails / `179`, `0` worse robust cells) and
  runoff/timing `0` worse cells. Real WAT output evidence found `27965`
  non-null opt-in meltwater-temperature rows across the observed corpus and
  `35730` non-null rows on H2637, all `0.0 degC`; default/rollback runs publish
  null values. H2637 measured `70.65 s` / `1153680 KiB`, within the ADR-0025
  `<=10x` budget. No default activation, density/frost change, Stage 1
  densification dependency, CoE melt-mass replacement, HBP/watershed
  serialization, full in-stream routing, fixture, parser/runfile/user CLI,
  WEPPpy, `.run`, or compatibility-runtime change was made.
- `20260629-paradigm-2-stage-3-decouple-water-temperature-001/` is complete as
  PARADIGM-2 STAGE 3-DECOUPLE, the snow-neutral water-temperature opt-in arm.
  Result: `EXECUTED-COMPLETE-OPT-IN-CAPABILITY`. It amended
  `SC-SNOWFREEZE-001` to v111 with `INV-SNOWFREEZE-081` and
  `OBL-SNOWFREEZE-P-056`, removed the Stage 3 runtime requirement for
  `physics_bulk_multilayer_density_v1`, and now runs
  `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` over a
  bulk-equivalent private layer stack using the current bulk density model. The
  real cross-SNOTEL/cancov guardrail exactly matched the no-env default (`15`
  robust fails / `179`; `0` better, `90` equal, `0` worse robust cells) and
  runoff/timing was no-worse (`0` better, `40` equal, `0` worse cells).
  Focused conservation/temperature tests, full workspace tests, clippy, deny,
  authority anti-evasion guards, scoped Markdown lint/validate, and H2637
  performance passed; H2637 measured `70.68 s` / `1150612 KiB`, within the
  ADR-0025 `<=10x` budget. No default activation, Stage 1 per-layer
  densification requirement, CoE melt-mass replacement, public schema change
  beyond the diagnostic meltwater-temperature flux, full in-stream temperature
  routing, fixture, frost, density-cap, parser/runfile/user CLI, `.run`,
  Qwet/frzftp, or compatibility-runtime change was made.
- `20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/` is
  complete as PARADIGM-2 STAGE 3, the opt-in per-layer liquid-routing and
  meltwater-temperature diagnostic candidate. Result:
  `HOLD-FULL-ARM-SNOW-GUARDRAIL-NON-PROMOTION`. It amended
  `SC-SNOWFREEZE-001` to v110 with `INV-SNOWFREEZE-080`, added per-layer
  thermal/liquid/refreeze state, consumed Stage 0 surface-energy and conduction
  primitives, routed the existing CoE melt/rain liquid through Stage 1 layers
  diagnostically, and produced typed meltwater-flux temperature for the future
  stream-temperature program. Focused conservation/temperature tests, the
  deferred cross-SNOTEL/cancov observed guardrail wrapper, H2637 endpoint
  timing/RSS, adjacent snow regressions, workspace clippy, full workspace tests,
  and `cargo deny check` passed; ADR-0025 hot-frame size bounds pass with
  optional boxed diagnostic carry, and H2637 measured `72.59 s` / `1150608 KiB`
  within the `<=10x` budget. No activation, CoE melt-mass replacement, fixture,
  public schema, density-cap, frost behavior, parser/runfile/user CLI, `.run`,
  or full stream-temperature routing change was made. Stage 3's incremental
  liquid/temperature path was neutral versus Stage 1 rollback (`0` worse robust
  cells; `0` worse runoff/timing cells), but promotion remains blocked because
  the full opt-in arm inherits Stage 1's non-promoted snow profile (`16`/`177`
  versus the current default `15`/`179`).
- `20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001/` is complete
  as PARADIGM-2 STAGE 2, the opt-in snow-to-frost insulation-profile coupling
  package. Result: `HOLD-GATE-FAILURE-NON-PROMOTION`. It amended
  `SC-SNOWFREEZE-001` to v109 with `INV-SNOWFREEZE-079`, instrumented the Stage
  1 density-gradient entry gate, and implemented internal
  `OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL=layered_resistance_v1` by deriving
  an insulation-equivalent bulk density from the prior-day Stage 1 layer-stack
  thermal resistance. The gradient entry gate passed (`56831` multi-layer rows;
  `49548` positive basal-minus-surface gradient rows), but the real
  frost-observation primary gate did not improve: bulk handoff and layered
  resistance both scored `3` robust fails / `49` with `0` improved robust cells.
  No activation, default, rollback, fixture, public schema, frost output,
  density-cap, melt, phase, canopy, radiation, parser, runfile, user CLI, `.run`,
  Qwet/frzftp, compatibility-runtime, or site-calibration change was made.
- `20260628-adr0029-paradigm-2-ratification-001/` is complete as the ADR-0029
  Paradigm 2 ratification package. Result: `RATIFIED-COMPLETE`. It ratified
  ADR-0028 first as the observed-data admission basis, then ratified ADR-0029 as
  the staged Paradigm 2 multilayer snow program commitment. The package verified
  `INV-SNOWFREEZE-050` operationalization, the `15` / `179` current-default snow
  floor, SNOWDENSITY-10.3.22's `HOLD-GATE-FAILURE-NON-PROMOTION`, the ADR-0026
  variable-layer frost precedent, and PARADIGM-2 Stage 0 completion. The
  paradigm-assessment WP-local Paradigm-1-first ADR candidate is superseded by
  ADR-0029. No code, physics, science-contract, fixture, schema, default, or
  runtime behavior changed.
- `20260628-paradigm-2-stage-0-surface-energy-balance-001/` is complete as
  PARADIGM-2 STAGE 0, the pure `openwepp-meteorology` surface energy-balance
  foundation. Result: `EXECUTED-COMPLETE`. The crate now exposes
  surface-agnostic net all-wave radiation, Monin-Obukhov sensible/latent/vapor
  mass flux, conductive heat exchange, precipitation advected heat, and
  latent<->mass/balance-sum helpers. Clean-room provenance records libsnobal
  CC0 commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6` and the
  `setup.py` `license="CC0 1.0"` declaration. No production runtime wiring,
  selector, default, fixture, output schema, density cap, frost behavior, or SC
  amendment changed.
- `20260628-snowdensity-10-3-22-climate-class-density-specialization-001/` is
  held as SNOWDENSITY-10.3.22, the comprehensive climate-class snow-density
  specialization candidate. It amended `SC-SNOWFREEZE-001` with
  `INV-SNOWFREEZE-077`, `OBL-SNOWFREEZE-P-052`, Sturm 2010/1995 and
  NSIDC-0768 authority anchors, reserved opt-in
  `physics_bulk_climate_class_density_v1`, and implemented explicit-unit Sturm
  2010 density trajectory support for the five locally parameterized classes.
  The rerun closed the original authority gap by source-verifying Sturm 1995
  thresholds, then ran the real cross-SNOTEL+cancov WAT/trace gate. Result:
  `HOLD-GATE-FAILURE-NON-PROMOTION`. Source authority and conservation passed,
  but the candidate scored `16` robust fails / `168` robust score versus the
  current default at `15` / `179`, failed the bidirectional densification flip,
  and worsened `13` robust cells. No production default, fixture, schema,
  density-cap, frost, parser/runfile/user selector, or `.run` control changed.
- `20260628-snow-density-paradigm-assessment-001/` is complete as the
  post-10.3.21 snow-density paradigm assessment. Result: `PARADIGM-ASSESSED`.
  The assessment compares climate-class parameter specialization, multilayer
  snowpack physics, and accepting the current floor. Its WP-local
  Paradigm-1-first ADR candidate is superseded by ADR-0029 after
  SNOWDENSITY-10.3.22 failed the source-verified climate-class gate. The current
  `15` / `179` snow floor remains valid input for frost-attribution threshold
  work with uncertainty carried forward, while ADR-0029 now owns the staged
  Paradigm 2 program decision. No production density code, contract, fixture,
  schema, default, density-cap, or frost change was made.
- `20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/` is
  complete as SNOWDENSITY-10.3.21, the post-partition residual decomposition and
  frost-attribution-threshold input diagnostic. It consumed the 10.3.20 real
  current-default WAT/trace artifact and the 10.3.18 pre-partition rubric
  artifact under `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050` and ADR-0028. Result:
  `DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION`. The current no-env
  default remains `15` robust fails / `179` score, above the legacy flag profile
  (`16` / `176`). The residual is signature-concentrated but site-diffuse:
  densification trajectory accounts for `9/15` robust fails, humid-New-England
  depth-SWE geometry for `2/15`, and mountain timing under-persistence for
  `4/15`. The frost-threshold input read is
  `MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER`; no frost-attribution threshold decision,
  production/default/cap/schema/fixture/frost change, selector, or site
  calibration was made.
- `20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/` is complete as
  SNOWDENSITY-10.3.20, the sublimation implementation diagnosis,
  partition+sublimation composition test, and Stage B surface-layer unlock. It
  amended `SC-SNOWFREEZE-001` to v105 with `INV-SNOWFREEZE-076`,
  `OBL-SNOWFREEZE-P-051`, `REF-SNOWFREEZE-SNOWDENSITY1020`, and
  `REF-SNOWFREEZE-LIBSNOBAL-CC0`; recorded the PySnobal/libsnobal clone commit
  `bf8b41c71e3e54ae654ae04005ddf72566c47ee6` plus the `setup.py`
  `license="CC0 1.0"` declaration; added opt-in
  `coe_open_sublimation_stage_b_v1`; and ran the real cross-SNOTEL WAT/trace
  gate. Result: `NON-PROMOTION-GATE-NOT-MET`. Current default remains `15`
  robust fails / `179` score; partition + Stage A sublimation scored `19` /
  `168`; Stage B conserved vapor and phase mass but scored `15` / `178` and
  worsened three robust cells. No activation, fixture, public schema,
  density-cap, frost, parser/runfile/user CLI, `.run` disable, Qwet/frzftp,
  compatibility-runtime, or site-calibration change was made.
- `20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/` is
  complete as SNOWDENSITY-10.3.19, the direct-production default activation for
  `harder_pomeroy_hourly` phase partitioning composed with the activated
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` bundle.
  It amended `SC-SNOWFREEZE-001` to v104 with `INV-SNOWFREEZE-075` and
  `OBL-SNOWFREEZE-P-050`, then ran the real cross-SNOTEL direct-production
  activation gate. Result: `ACTIVATED`. The new no-env default scores `15`
  robust fails / `179` on the forcing-robust rubric versus the prior activated
  bundle with explicit `legacy_rst` phase at `17` / `172`; selector trace proof
  and partition conservation close (`5.55e-17 m` max residual), workspace
  validation passes, explicit `legacy_rst` rollback/test selection remains, and
  no fixture, public schema, density-cap, frost, parser/runfile/user selector,
  or `.run` disable change was made. Humid-New-England depth regression remains
  a non-representative roadmap item, and the `+23.6 kg m^-3` density-bias rise
  is tracked separately.
- `20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/` is
  complete as SNOWDENSITY-10.3.18, the diagnostic-only cross-SNOTEL mechanism x
  legacy rubric run. It consumed `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050` and
  the SNOWFROST-FIDELITY-H lineage to score SNOTEL and bound `cancov_forest`
  SWE/depth/density profiles across supported current direct-runtime
  mechanisms, rejected/archival opt-in candidates, legacy, and PySnobal flag
  profiles. Result: `DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`. The ranked
  supported next-lever read is `harder_pomeroy_partition` (`+7` robust score
  delta, `+2` robust fail delta vs activated), but this is investigation-only
  because 10.3.5c non-SNOTEL evidence still blocks promotion. The 10.3.17
  shallow-pack guard remains non-promoted, 10.3.16 sublimation is worse in this
  profile, and humid-New-England cancov residuals are not representative of the
  mountain SNOTEL activated-bundle fail signature set. No
  production/default/cap/schema/fixture/frost change and no promotion or
  activation decision was made.
- `20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/` is
  complete as SNOWDENSITY-10.3.14, the Policy-B no-regression and cap-authority
  diagnostic after the 10.3.13 residual-tail HOLD. It amended
  `SC-SNOWFREEZE-001` to v100 with `INV-SNOWFREEZE-071`,
  `OBL-SNOWFREEZE-P-046`, and a diagnostic addendum, then ran the full
  workspace gate under the current best bundle selectors:
  `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1` and
  `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1`.
  Result: `READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`. The active-cap
  bundle remains strictly better than current default (`1147 -> 498` paired
  failures), has no paired surface worse versus holding-only, and passed the
  selector-scoped workspace gate. Composite trace closure was clean
  (`1.11e-16 m` max SWE-depth-density residual; zero cap exceedances). The
  `550 kg m^-3` cap projection is mixed follow-up evidence only: among `248`
  cap-pinned paired rows, projected failures change `105 -> 102`, but `3`
  passing rows become projected under-persistence. No default activation,
  density cap, production physics, fixture/schema, parser/runfile/user selector,
  Qwet/frzftp, frost attribution, or compatibility-runtime change was made.
- `20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001/` is complete
  as SNOWDENSITY-10.3.13, the residual-tail and Policy-B diagnostic after the
  bundle activation adjudication. It amended `SC-SNOWFREEZE-001` to v99 with
  `INV-SNOWFREEZE-070`, `OBL-SNOWFREEZE-P-045`, and a diagnostic addendum, then
  classified date-level transitions across current default, holding-capacity-
  only, combined bundle, and rejected spring densification. Result:
  `HOLD-ACTIVATION-EVIDENCE-MISSING`. The bundle remains better than default
  (`1147 -> 498`) but lacks Policy-B full-surface no-regression evidence. The
  under-persistence tail is now attributed as mechanism-cost evidence:
  `177/234` bundle under-persistence rows were induced by the density arm from
  holding-only pass/over rows, while `57` persisted from holding-only under-
  persistence. The active cap remains `522 kg m^-3`; `550 kg m^-3` SNOBAL cap
  re-anchoring is follow-up only.
- `20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/` is
  complete as SNOWDENSITY-10.3.12, the activation adjudication for the current
  best combined opt-in bundle,
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`. It
  amended `SC-SNOWFREEZE-001` to v98 with `INV-SNOWFREEZE-069`,
  `OBL-SNOWFREEZE-P-044`, Activation Policy B, and the Combined Bundle
  Activation Adjudication Addendum, then reran the real direct-production WAT
  path across all seven snow-depth fidelity surfaces. Result:
  `HOLD-OPT-IN-BUNDLE`. The bundle is the
  best current path: default `1147` paired failures, holding-capacity-only
  `761`, bundle `498`, and spring densification `502`; no paired surface
  worsens versus holding-capacity-only, and direct trace rows prove both bundle
  members were selected `112502` times. The bundle is not activation-ready
  because full-model-surface no-regression evidence was not produced; frost
  attribution remains separately blocked because `498/1415` paired rows still
  fail snow control. Remaining failures split into modeled-over-observed `264`
  and modeled-under-observed `234`, with March/April now dominated by
  under-persistence (`128`) rather than compaction-only headroom (`20`).
- `20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/`
  is complete as SNOWDENSITY-10.3.11, the opt-in spring compaction/
  densification candidate. It amended `SC-SNOWFREEZE-001` to v96 with
  `physics_bulk_spring_densification_v1`, `INV-SNOWFREEZE-068`, and
  `OBL-SNOWFREEZE-P-043`, then ran the real direct-production WAT path with
  fixed `coe_liquid_holding_capacity_v1` melt/liquid boundary and compared the
  existing density-compaction bundle against the spring densification candidate.
  Result: `SPRING-DENSIFICATION-NON-PROMOTION`. The existing
  `physics_bulk_density_compaction_v1` arm under holding capacity improves
  `761 -> 498`, but the spring densification candidate worsens that baseline
  `498 -> 502` with three paired surfaces worse and clear over-densification
  into under-persistence. No default activation, fixture/schema/parser/user
  surface, density-cap, melt/radiation/canopy/phase/frost, Qwet/frzftp, or
  compatibility-runtime change was made.
- `20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/`
  is complete as SNOWDENSITY-10.3.10, the diagnostic spring pack-depletion and
  compaction adjudication gate after SNOWDENSITY-10.3.9. It consumed the
  SNOWDENSITY-10.3.8 opt-in coupled WAT candidate and used the existing
  `SC-SNOWFREEZE-001` `522 kg m^-3` upper snow-density cap to test whether
  modeled SWE can fit observed March/April snow-depth tolerance by compaction
  alone. Result: `SPRING-COMPACTION-FIRST`. Of `282` failed March/April paired
  rows, `190` are compaction-only feasible within the existing cap, `33` are
  cap-limited depletion required, `16` are patchy meltout or depletion required,
  and `43` are under-persistence. Depletion-required rows are real but secondary:
  `49/282` failed rows, with row-summed diagnostic SWE depletion at cap
  `1.230 m`, concentrated in Harvard open (`23`) and Sleepers open field (`21`).
  No production physics, default, selector, fixture, public schema, coefficient,
  radiation, canopy, phase, density, melt, rain heat, longwave, frost, or density
  cap changed. The next one-lever route is an opt-in spring
  compaction/densification candidate that preserves the cap and mass
  conservation before any separate spring depletion / patchy snow-cover process.
- `20260627-snowdensity-10-3-9-march-april-residual-attribution-001/` is
  complete as SNOWDENSITY-10.3.9, the diagnostic March/April residual
  attribution gate after the SNOWDENSITY-10.3.8 opt-in liquid holding-capacity
  correction. It consumed the 10.3.8 coupled direct-production WAT artifact
  without changing production physics, defaults, selectors, fixtures, public
  schemas, coefficients, radiation, canopy, phase partition, density, melt, rain
  heat, longwave, or frost code. Paired Sleepers/Harvard evidence shows
  March/April accounts for `282/761` remaining failures (`37.1%`) and fails
  `282/463` paired March/April rows (`60.9%`). Failures are shared across open
  field, hardwood, and open covers (`112`, `109`, and `61` rows). Dominant
  attribution is not a defended SWE-excess mass signal: depth-only
  over-persistence (`127`), density/compaction (`86`), and patchy meltout or
  snow-cover depletion (`26`) dominate, with `43` under-persistence rows and
  Harvard SWE/mass attribution correspondence-caveated. The next one-lever route
  is spring pack-depletion and compaction adjudication. HJ Andrews and Hubbard
  Brook mixed/deciduous remain observation-blocked for residual verdicts until
  paired snow-depth observations are installed.
- `20260627-snowdensity-10-3-8-liquid-holding-capacity-001/` is complete as
  SNOWDENSITY-10.3.8, the opt-in liquid holding-capacity drainage correction.
  It amended `SC-SNOWFREEZE-001` to v95 with in-repo retained-liquid authority,
  `coe_liquid_holding_capacity_v1`, `INV-SNOWFREEZE-067`,
  `OBL-SNOWFREEZE-P-042`, and the Opt-In Liquid Holding-Capacity Addendum. The
  candidate preserves `legacy_coe` default/rollback behavior, CoE melt terms,
  radiation, canopy, phase partition, density constants, rain heat, sub-canopy
  longwave, frost, fixtures, public schemas, parser/runfile/user surfaces, and
  compatibility runtime. Its only algorithmic delta is an explicit opt-in
  persistent retained-liquid snow-lane store: liquid is retained up to the
  non-fitted in-repo holding capacity (`max_liquid_water_volume_fraction = 0.01`)
  and excess drains to downstream liquid forcing. Event-window evidence improves
  paired Sleepers/Harvard under-ablation `132 -> 94` and aggregate depth-loss
  deficit `24.105 m -> 15.506 m`, with produced-artifact SWE/routed-liquid
  conservation closed. Real direct-production WAT evidence improves paired
  snow-control failures `1147 -> 761` with no paired surface worse, but the
  snow-control gate is still not cleared (`761/1415` rows fail; three surfaces
  remain observation-blocked). The result is `WINTER-THAW-MELT-RESPONSE-
  CANDIDATE-IMPROVES` / `WINTER-THAW-COUPLED-WAT-IMPROVES`, not default
  activation.
- `20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/` is
  complete as SNOWDENSITY-10.3.7, the opt-in winter-thaw melt-response
  correction. It amended `SC-SNOWFREEZE-001` to v94 with
  `coe_winter_thaw_state_loss_v1`, `INV-SNOWFREEZE-066`,
  `OBL-SNOWFREEZE-P-041`, and the Opt-In Winter-Thaw State-Loss Addendum. The
  correction preserves `legacy_coe` as default/rollback and keeps CoE melt
  terms, radiation, canopy, phase partition, density constants, rain heat,
  sub-canopy longwave, frost, public schemas, and fixtures unchanged. Its only
  algorithmic delta is the explicit opt-in positive-thaw state-loss branch when
  legacy would retain positive `wmelt` as density-only compaction below
  `350 kg m^-3`. Paired Sleepers/Harvard thaw-ablation evidence improves:
  under-ablation windows `132 -> 108`, aggregate depth-loss deficit
  `24.105 m -> 17.629 m`, modeled depth loss `15.868 m -> 26.400 m`, routed
  melt `5.895 m -> 11.235 m`, and snowpack SWE loss `4.628 m -> 10.615 m`.
  The operator-review conservation gap was resolved with zero active-ledger SWE
  balance and routed-state-loss residuals. The operator-review coupled gate was
  resolved with real direct-production WAT evidence: snow-control failures
  improve `1147 -> 978` with no paired surface worse, but the package remains
  blocked from activation because `978/1415` paired rows still fail snow control.
  The result is `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`, not default
  activation and not full snow-control closure; remaining snow-depth residuals
  route to the next one-lever adjudication package.
- `20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/` is complete as
  SNOWDENSITY-10.3.6, the rank-2 winter-thaw melt-response diagnosis. It added a
  diagnostic-only `legacy_coe` snowbench event-window tool and compared observed
  snow-depth ablation intervals against modeled snow-depth loss, CoE melt/SWE
  loss, positive-temperature snowpack hours, and warm-rain heat context across
  the four paired Sleepers/Harvard maritime surfaces. Result:
  `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`. Across `219` observed thaw-
  ablation windows, `132` under-ablated by the package threshold (`0.603`
  fraction), with `24.105 m` aggregate depth-loss deficit, `19,166` positive-
  temperature snowpack hours, `8.685 m` raw melt, `4.628 m` modeled SWE loss, and
  only `0.190 m` warm-rain heat equivalent. HJ Andrews and Hubbard Brook remain
  observation-blocked diagnostic-only surfaces. No production physics, default,
  fixture input, public schema, parser/runfile/user selector, coefficient,
  radiation, canopy, phase, density, frost, longwave, or rain-heat change was
  made. The next route is a contract-first opt-in winter-thaw melt-response
  correction package; rain heat and sub-canopy longwave stay separate later
  levers.
- `20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/` is
  complete as SNOWDENSITY-10.3.5c, the coupled WAT snow-depth impact
  adjudication for the opt-in `harder_pomeroy_hourly` phase partition validated
  in SNOWDENSITY-10.3.5b. It used the real direct-production WAT path and
  compared absent/default `legacy_rst` against
  `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly` across seven
  maritime diagnostic surfaces. The execution exposed and fixed a valid-input
  Harder-Pomeroy hydrometeor solver non-convergence by adding a bracketing
  fallback that preserves the same equation and keeps saturated identity. Final
  result: `PHASE-PARTITION-NEUTRAL-OR-WORSE`. Opt-in WAT changed all seven
  surfaces but worsened all four paired Sleepers/Harvard snow-depth surfaces:
  snow-control failures increased `1147 -> 1273`. No default activation,
  parser/runfile/user CLI selector, fixture edit, public output schema change,
  density/melt/canopy/radiation/frost change, or site calibration was made. The
  next route is 10.3.4 rank-2 winter-thaw melt response before sub-canopy
  longwave or rain heat.
- `20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/` is
  complete as SNOWDENSITY-10.3.5b, the opt-in hourly rain/snow partition and
  Jennings observed-phase validation package. It amended `SC-SNOWFREEZE-001` to
  v92, wired `openwepp-meteorology` into the hourly winter partition seam behind
  an explicit `legacy_rst` / `harder_pomeroy_hourly` selector, proved the real
  direct snow consumer reads the opt-in path, and ran the Jennings file2/file3
  validation with no site calibration. The full local validation scored
  `11,711,058` rows across `6,883` stations: Harder-Pomeroy hourly accuracy
  `0.903141` versus legacy `RST` 0 C accuracy `0.858331`. Default `RST`
  behavior and rollback isolation remain intact; no default activation,
  parser/runfile selector, fixture input edit, public output schema change,
  density/melt/canopy/radiation/frost change, or compatibility-runtime deletion
  was made.
- `20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/` is complete as
  SNOWDENSITY-10.3.5a, the production-free `openwepp-meteorology` foundation for
  robust rain/snow partition work. It amended `SC-SNOWFREEZE-001` to v91,
  added checked psychrometric primitives plus a candidate-only Harder-Pomeroy
  hydrometeor-temperature phase core, and proved no production
  `RST`/runtime/default/schema wiring changed. The follow-on 10.3.5b package
  owns production wiring and Jennings observed-phase validation.
- `20260627-snowdensity-10-3-4-maritime-overaccumulation-diagnosis-001/` is
  complete as SNOWDENSITY-10.3.4, the maritime over-accumulation diagnosis
  gate. It added a diagnostic-only legacy-CoE snowbench replay tool across HJ
  Andrews, Sleepers, Harvard, and Hubbard Brook maritime surfaces, plus focused
  contract guard tests and JSON/Markdown evidence artifacts. Result:
  `PARTITION-THAW-FIRST`. All four paired Sleepers/Harvard surfaces
  over-accumulate modeled snow depth; HJ Andrews and Hubbard Brook are
  observation-blocked because paired snow-depth tables are not installed. The
  ranked defect-eligible mechanisms are near-zero snow/rain partition,
  winter-thaw melt response, sub-canopy longwave or forest energy, and then
  rain-on-snow heat. Precipitation bias and representativeness remain
  forcing-limited, and wind undercatch is not supported as a correction lever
  for modeled-over-observed snow depth. No production physics, defaults,
  publication schema, fixture input, coefficient, radiation, canopy, albedo,
  density, partition, precipitation, frost, or promotion decision changed. The
  next route is §10.3.5 partition/thaw-window decomposition before rain-heat or
  longwave production changes.
- `20260626-snowdensity-10-3-3-gradient-melt-adjudication-001/` is complete as
  SNOWDENSITY-10.3.3, the canopy-gradient melt adjudication gate. It added a
  diagnostic-only Harvard/Marcell stratified CoE melt replay tool and ran
  `legacy_coe` versus `coe_shortwave_albedo_v1` across five exact
  verdict-bearing strata plus two diagnostic mixed aggregates. Result:
  `LOW-CANOPY-NON-PROMOTION`. Low-canopy exact-bound robust failures worsen
  `6 -> 7` with no score gain (`70 -> 70`), and whole verdict-bearing evidence
  worsens `7 -> 8` with no score gain (`92 -> 92`). Conifer and open/pasture are
  neutral; deciduous worsens by one robust failure, driven by Harvard hardwood;
  mixed aggregates worsen but remain diagnostic-only. Harvard hemlock remains
  observation-installed but unbound to a pure hemlock/conifer model hillslope.
  No production activation, default, selector, output schema, coefficient,
  radiation, canopy, albedo, density, partition, frost, or fixture-input change
  was made. The next route is §10.3.4 maritime over-accumulation diagnosis.
- `20260626-snowdensity-10-3-1a-per-day-cancov-direct-runtime-001/` is complete
  as SNOWDENSITY-10.3.1a, the per-day canopy-cover direct-runtime bridge. It
  amended `SC-SNOWFREEZE-001` to v90 with `cancov_daily_series`,
  `INV-SNOWFREEZE-063`, `OBL-SNOWFREEZE-P-038`, and the 10.3.1a addendum;
  routed direct-production day input canopy from
  `growth_state_for_publication.canopy_cover_fraction`; exported
  `canopy_series.csv` from snowbench; and changed CoE melt replay to consume the
  daily sidecar by date instead of a scalar initial canopy value. The old scalar
  field remains summary-only/backward-compatible, and the CoE boundary CSV
  schema remains unchanged. No production snow physics default, fixture input,
  parser/runfile selector, melt coefficient, albedo constant, density
  coefficient, or public WAT schema changed. Full workspace gates passed.
- `20260626-snowdensity-10-3-2-canopy-stratum-correspondence-001/` is complete
  as SNOWDENSITY-10.3.2, the canopy-stratum correspondence gate. It mapped the
  Harvard and Marcell canopy-stratified observation metadata to the current
  modeled surfaces and later revised the binding after paired stratum fixtures
  were added. Marcell conifer/deciduous/open and Harvard hardwood/open now have
  model counterparts, and follow-on observation ingest installed Harvard HF237
  and Marcell RDS-2021-0016 normalized tables under
  `tests/fixtures/cancov_forest/observations/`. Harvard hemlock remains
  observation-installed but unbound to a pure model hillslope; 10.3.3 must
  exclude it, report it unbound, or explicitly proxy-scope it before verdict use.
  No production code, science contract, output schema, default, selector,
  coefficient, radiation, albedo, density, melt, partition, or frost behavior
  changed.
- `20260626-snowdensity-10-3-1-canopy-projection-provenance-001/` is complete
  as SNOWDENSITY-10.3.1, the canopy projection provenance package. It archived
  raw `.man` canopy inputs, upstream wepppy seasonal winter `Cancov`
  projections, and current openWEPP snowbench runtime-surface `cancov` for all
  eight `tests/fixtures/cancov_forest/` sites. Result: current snowbench CoE
  melt diagnostics consume static initial-condition canopy, not the upstream
  per-day seasonal projection. HJ Andrews and Tenderfoot close as high evergreen
  controls; Berthoud and Mores Creek are RAP_TS-adjusted conifer exceptions;
  Harvard/Marcell/Hubbard Brook are static mixed/deciduous diagnostics until
  per-day canopy routing exists; Sleepers is not proven as the lowest-cancov
  endpoint (`runtime cancov = 0.50`). No fixture inputs, production physics,
  defaults, output schema, contracts, or selectors changed. Downstream gradient
  melt adjudication must either route per-day canopy or explicitly scope itself
  to static-initial-canopy evidence.
- `20260626-snowdensity-09-diagnostic-coupled-wat-rerun-001/` is complete as
  SNOWDENSITY-09, the diagnostic coupled WAT rerun. It amended
  `SC-SNOWFREEZE-001` to v89 with `INV-SNOWFREEZE-062`,
  `OBL-SNOWFREEZE-P-037`, the 09 addendum, and the paired-snow gate correction;
  added a package-bound
  diagnostic environment selector for direct-production snow density;
  extended direct-production snow trace rows with `snow_density_model`; and
  reran the non-SNOTEL frost-site rubric for default `legacy_wepp` WAT and
  diagnostic `physics_bulk_density_compaction_v1` WAT. Result: the coupled
  opt-in path is proven (`75,610` opt-in trace rows selected the model), and it
  reduces snow-depth residuals at all three paired-snow sites, but snow control
  still fails at the same three gate-eligible paired-snow sites. SCAN Mandan ND
  and Reynolds Creek ID lack observed snow-depth rows and are reported as
  diagnostic-only out-of-gate evidence, not gate pass/fail/blocker inputs.
  Frost attribution remains blocked as
  `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`. No default activation,
  parser/runfile/user CLI selector, output schema, WAT rewriting, site
  constants, tuning, or production physics change was made.
- `20260626-snowdensity-08-snow-frost-gate-rerun-001/` is complete as
  SNOWDENSITY-08, the snow/frost gate rerun. It amended
  `SC-SNOWFREEZE-001` to v87 with `INV-SNOWFREEZE-061`,
  `OBL-SNOWFREEZE-P-036`, and the 08 addendum; added a compact diagnostic
  aggregator; reran the accepted CoE-bound density lineage against the SNOTEL
  rubric; and reran the current direct-production non-SNOTEL frost-site WAT
  rubric. Result: SNOTEL density evidence still clears
  (`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`, robust failures
  `9 -> 5`, robust score `84 -> 110`, density failures `9 -> 5`, density score
  `16 -> 41`, CoE SWE identity residual about `4.44e-16 m`), but frost
  attribution stays blocked. The current non-SNOTEL path is still
  `legacy_wepp` density, there is no authorized coupled opt-in WAT/publication
  path, three non-SNOTEL sites fail snow control, and two lack paired observed
  snow rows. No default activation, parser/runfile/CLI selector, output schema,
  tuning, or production physics changed. Its next-route note is superseded by
  the completed SNOWDENSITY-09 coupled WAT rerun.
- `20260626-snowdensity-07-runtime-opt-in-001/` is complete as
  SNOWDENSITY-07, the runtime opt-in gate. It amended `SC-SNOWFREEZE-001`
  v86 with `INV-SNOWFREEZE-060`, `OBL-SNOWFREEZE-P-035`, typed
  `snow_density_model`, and CoE boundary carry surfaces; added
  `physics_bulk_density_compaction_v1` behind an explicit typed selector;
  preserved `legacy_wepp` as surface-driven default/rollback; and projected
  opt-in runtime snow depth/density plus separate CoE boundary carry through
  direct R4G state, downstream operands, shadow projection, runtime carry, and
  publication-facing winter-column state. Full workspace gates pass. Its
  next-route note is superseded by the completed SNOWDENSITY-08 gate rerun.
- `20260626-snowdensity-06b-coe-bound-density-replay-001/` is complete as
  SNOWDENSITY-06B, the CoE-bound density replay gate. It amended
  `SC-SNOWFREEZE-001` to v85 with `INV-SNOWFREEZE-059`,
  `OBL-SNOWFREEZE-P-034`, and the 06B addendum; added diagnostic-only
  `openwepp-snowbench coe-bound-density`; replayed `density_compaction_v1`
  against fixed `legacy_coe` and `coe_shortwave_albedo_v1` CoE
  melt/liquid/SWE-loss boundaries; and ran five-site SNOTEL adjudication.
  Both CoE-bound candidates preserve daily CoE SWE identity
  (`max_abs_coe_swe_identity_residual_m ~= 4.44e-16`) and beat openWEPP/legacy
  as-built on whole-rubric and density-cell gates. Best result:
  `coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`, robust failures
  `9 -> 5`, robust score `84 -> 110`, density failures `9 -> 5`, density
  score `16 -> 41`. This is an offline promotion-candidate result only; no
  default activation, production runtime selector, parser/runfile/CLI selector,
  output schema, mixed/deciduous canopy adjudication, or frost attribution
  changed. The next route is SNOWDENSITY-07 runtime opt-in.
- `20260626-snowdensity-06-density-compaction-001/` is complete as
  SNOWDENSITY-06, the density-compaction gate. It amended
  `SC-SNOWFREEZE-001` to v84 with `INV-SNOWFREEZE-058`,
  `OBL-SNOWFREEZE-P-033`, and the 06 addendum; added offline
  `density_compaction_v1`; exposed named SNOBAL-lineage PTM/POC/liquid-water
  compaction constants; and preserved baseline candidate melt constants,
  albedo, canopy, radiation, production defaults, and rollback. Five-site
  SNOTEL adjudication shows density/densification robust-cell improvement
  against legacy/as-built (`fail 9 -> 7`, score `16 -> 22`) without site tuning
  or melt changes, but whole-rubric promotion remains blocked (`robust fail
  9 -> 18`, score `84 -> 46`). No runtime/default activation, parser/runfile/CLI
  activation selector, output schema, or frost attribution changed. The
  completed SNOWDENSITY-06B package supersedes this next-route note.
- `20260626-snowdensity-05g-harness-fidelity-rerun-001/` is complete as
  SNOWDENSITY-05G, the harness-fidelity rerun gate. It amended
  `SC-SNOWFREEZE-001` to v83 with `INV-SNOWFREEZE-057`,
  `OBL-SNOWFREEZE-P-032`, and the 05G addendum; corrected diagnostic
  `openwepp-snowbench coe-melt` replay to consume configured runtime-surface
  coniferous canopy (`cancov = 0.9` for all five SNOTEL fixtures) instead of
  the prior `0.0` harness constant; recorded the PySnobal bridge shortwave
  inversion identity as like-for-like; and reran five-site SNOTEL adjudication.
  Result: representative-regime `NON-PROMOTION` for default activation because
  robust failures did not improve (`9 -> 9`), despite ordinal score increasing
  slightly (`84 -> 86`). The 05E promotion-candidate result is superseded as
  regime-limited. External review endorsed the deflating result and clarified
  that conifer neutrality should not retire `coe_shortwave_albedo_v1`; a
  low-canopy/mixed-forest package with real per-day seasonal `cancov` is the
  decisive melt-value fork. The completed SNOWDENSITY-06 package supersedes this
  density-route note.
- `20260626-snowdensity-05f-melt-closure-density-handoff-001/` is complete as
  SNOWDENSITY-05F, the melt-closure and density-handoff gate. It amended
  `SC-SNOWFREEZE-001` to v82 with `INV-SNOWFREEZE-056`,
  `OBL-SNOWFREEZE-P-031`, boundary disposition, and the 05F addendum; froze
  `coe_shortwave_albedo_v1` as an opt-in-only density-facing melt interface;
  preserved `legacy_coe` as default/rollback; ratified the same-day future
  snowfall cold-start albedo continuity rule; dispositioned the independent
  Claude review by labeling 05E replay evidence regime-limited (`cancov = 0.0`,
  PySnobal-bridge radiation); recorded the operator clarification that the
  validation management should be coniferous forest with winter `cancov` about
  `0.9`; and bound activation evidence to both 05E
  diagnostic replay and H as-built context. No default activation,
  parser/runfile/CLI selector, output schema, coefficient, radiation-source, or
  density-physics change was made. The next route note is superseded by
  `SNOWDENSITY-05G Harness Fidelity Rerun`.
- `20260626-snowdensity-05e-melt-rubric-adjudication-001/` is complete as
  SNOWDENSITY-05E, the melt-rubric adjudication gate. It added diagnostic-only
  `openwepp-snowbench coe-melt` replay for `legacy_coe` and
  `coe_shortwave_albedo_v1`, generated five-site SNOTEL rubric profiles, and
  reran the non-SNOTEL baseline. The opt-in path is a bounded
  `PROMOTION-CANDIDATE` relative to diagnostic legacy
  (`robust_fail_count 13 -> 10`, `robust_ordinal_score 61 -> 84`), but not a
  default-activation result: H as-built context remains
  `robust_fail_count=9`, `robust_ordinal_score=84`, and non-SNOTEL frost
  attribution remains blocked by snow-control failures. The next route is
  `SNOWDENSITY-05F Melt Closure / Density Handoff`.
- `20260626-snowdensity-05d-opt-in-coe-melt-implementation-001/` is complete
  as SNOWDENSITY-05D, the opt-in CoE melt implementation gate. It amended
  `SC-SNOWFREEZE-001` to v79 with `INV-SNOWFREEZE-055`,
  `OBL-SNOWFREEZE-P-030`, and
  `snow_melt_shortwave_absorbed_fraction`; wired
  `coe_shortwave_albedo_v1` into the typed production CoE melt path behind an
  explicit selector; carried optional albedo state through the direct snow
  runtime; and exposed raw melt, redistributed melt, routed `wmelt`, and
  SWE-loss lineage totals. `legacy_coe` remains default/rollback; no parser
  surface, output schema, radiation source, coefficient, or default activation
  changed. Superseded by `SNOWDENSITY-05E Melt Rubric Adjudication`.
- `20260626-snowdensity-05c-albedo-state-core-001/` is complete as
  SNOWDENSITY-05C, the albedo-state core gate. It amended
  `SC-SNOWFREEZE-001` to v78 with `INV-SNOWFREEZE-054`,
  `OBL-SNOWFREEZE-P-029`, `brock2000_temperature_age_v1`, accumulated
  positive-temperature age `Ta`, a fresh-snow reset threshold, `[0, 0.85]`
  albedo bounds, and missing-state fail-closed behavior for active future
  `coe_shortwave_albedo_v1` snowpack. It added a standalone typed Rust
  albedo-state core and changed no routed melt, production wiring, parser
  surface, output schema, radiation source, or default. Superseded by
  SNOWDENSITY-05D.
- `20260626-snowdensity-05b-shortwave-source-binding-001/` is complete as
  SNOWDENSITY-05B, the shortwave source-binding gate. It amended
  `SC-SNOWFREEZE-001` to v77 with `INV-SNOWFREEZE-053`,
  `OBL-SNOWFREEZE-P-028`, `winter_shortwave_daily_radly`, and the Shortwave
  Source Binding Addendum. The canonical openWEPP source is the existing daily
  climate `rad`/`radly` field in `Ly d^-1`; upstream gridded-provider
  selection/spatialization remains outside engine ownership; transformation is
  the existing `SC-CLIMATE-001#INV-CLIMATE-013` path into
  `winter.hourly.rad_mj_m2_####`; and ET/snowmelt share daily radiation
  authority. It changed no production runtime code, constants, parser surfaces,
  output schemas, source selectors, albedo constants, or defaults. The next
  route note is superseded by `SNOWDENSITY-05C Albedo State Core`.
- `20260625-snowdensity-05-melt-modernization-contract-first-001/` is complete
  as SNOWDENSITY-05A, the melt contract/sign-reconciliation gate. It amended
  `SC-SNOWFREEZE-001` to v76 with `INV-SNOWFREEZE-052`,
  `OBL-SNOWFREEZE-P-027`, opt-in
  `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`, shortwave/albedo
  operand placeholders, no-radiation-tuning guard, explicit signed
  `melt_bmelt_in` convention, and negative-benchmark disposition for
  `dense_slow_melt_v1`. It changed no production runtime code, constants,
  parser surfaces, output schemas, or defaults. Its next-route note is now
  superseded by `SNOWDENSITY-05B Shortwave Source Binding`.
- `20260625-snowdensity-04-offline-adjudication-loop-001/` is complete as the
  offline `physics_bulk` adjudication package. It added global named
  candidate variants, `tools/snowfreeze_observed/physics_bulk_adjudication.py`,
  and a four-variant/five-site comparison against the H openWEPP/legacy/PySnobal
  profile. The package closed `COMPLETE-PROMOTION-CANDIDATE` under its original
  rule (`dense_slow_melt_v1` robust fail count `9 -> 6`, robust ordinal score
  `84 -> 102` against both openWEPP and legacy as-built profiles), but the
  2026-06-25 melt-model decision supersedes that route: the variant is not
  production-promotable as-is and must not be runtime-activated.
- `20260625-snowdensity-03-offline-physics-core-001/` is complete as the
  offline Rust `physics_bulk` candidate implementation. It added the
  `openwepp-snowbench physics-bulk` command, a bounded bulk snowpack state
  model, conservation/unit tests, a production-confinement integration guard,
  and a five-site SNOTEL rubric profile. The candidate remains outside runtime
  activation and publication paths. First-profile evidence is finite and useful
  but not production-promotable: forcing-robust counts are `fail=24`,
  `marginal=13`, `pass=3`, `strong=5`, `unavailable=15`, with
  `openwepp_defective_cells=0`. The next route is `SNOWDENSITY-04 Offline
  Adjudication Loop`.
- `20260625-snowdensity-02-contract-adr-001/` is complete as the contract/ADR
  governance package for snow-density remediation. It amended
  `SC-SNOWFREEZE-001` to v75 with `INV-SNOWFREEZE-051`,
  `OBL-SNOWFREEZE-P-026`, candidate `physics_bulk` state/process authority,
  no-site-tuning language, and opt-in activation constraints; added
  ADR-0027; and added `snowdensity02_contract_adr_guard`. It changed no
  production runtime physics, constants, output schemas, parser surfaces, or
  defaults. It authorized the completed SNOWDENSITY-03 offline implementation
  route.
- `20260625-snowdensity-01-evidence-reconciliation-001/` is complete as the
  evidence-only reconciliation package that starts the snow-density strategy
  sequence. It consolidated SNOWFROST-FIDELITY-E/F/H/I0 evidence, pinned H's
  openWEPP-vs-pinned-legacy as-built density deltas (`max abs delta =
  4.351046738461008 kg m^-3`), classified SNOTEL rubric cells into actionable
  snow-physics, mixed, forcing-limited, and unavailable groups, and reviewed
  Shen 2011/2012 plus pinned `snowd.for` Eq. 3.7.5 archaeology. It changed no
  production code or science contract authority. The next route is
  `SNOWDENSITY-02 Contract + ADR` for an opt-in `physics_bulk` envelope before
  any runtime physics work.
- `20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/` is complete
  as the non-SNOTEL v74 rubric baseline before snow-depth structural
  remediation. It added
  `tools/snowfreeze_observed/non_snotel_rubric_baseline.py`, reran all five
  `tests/fixtures/snowfreeze_observed/` sites through the observed harness, and
  emitted `snowfreeze-non-snotel-rubric-baseline-v1`. Three paired-snow sites
  still fail snow control and two isotherm sites lack paired observed snow-depth
  rows; rubric counts are `fail=19`, `marginal=8`, `pass=5`, `strong=20`,
  `unavailable=63`, with `openwepp_defective_cells=0`. The next route remains
  snow-depth structural remediation before frost physics attribution.
- `20260625-snowfrost-fidelity-h-snotel-density-three-way-001/` is
  complete-with-disposition. It acquired
  and normalized the five-site SNOTEL SWE/depth/STO corpus, derived
  observed-density SSD arms from peak-SWE-period density before residual
  comparison, added the v74 `INV-SNOWFREEZE-050` rubric profile authority, and
  emitted `snotel-density-three-way-comparison-v2` profiles for openWEPP,
  pinned legacy WEPP, and PySnobal. All five sites route `STRUCTURAL` in the
  auxiliary density fork; the rubric remains profile-not-scalar. PySnobal
  water-year segmented runs pass four sites; CSS Lab WY2017 fails inside
  PySnobal's C core despite finite exported forcing and is dispositioned as a
  known upstream PySnobal/SNOBAL thin-snow numerical instability. Affected
  PySnobal profile cells are unavailable, not openWEPP failures. The next route
  is snow-depth structural remediation, not heat-flow, frozen-K, or
  migration/fringe production physics work.
- `20260625-snowfrost-fidelity-g1-pysnobal-sanity-closure-001/` is complete as
  the PySnobal sanity-closure follow-up. It kept the strict all-lane G0 hold
  visible for Morris `Tg=-0.5 degC`, added site/lane/window controls and a
  `site-sane` route, and made `openwepp-snowbench export-pysnobal` publish
  current openWEPP WAT-backed `openwepp_snow.csv` rows with climate-date
  mapping. Fresh five-site G1 evidence routes
  `PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES` for the selected
  `Tg=0.0 degC` lane: every pilot site has sane PySnobal SWE and physical
  snow depth, and PySnobal-vs-openWEPP depth metrics are now available.
  PySnobal remains diagnostic hypothesis evidence only.
- `20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/` is
  executed-held at `HOLD-PYSNOBAL-SANITY-FAILURE`, superseded for comparator
  routing by G1's explicit site-sane policy. It added the diagnostic
  Rust `openwepp-snowbench export-pysnobal` exporter, reusing openWEPP WEPP
  parsing and SIMIMPL28 daily-to-hourly forcing surfaces, plus the thin
  `tools/snowfreeze_observed/pysnobal_compare.py` runner. Exporter schema,
  anti-alias, build, focused test, clippy, workspace test, and deny gates are
  green, and 14 of 15 all-site PySnobal lanes pass sanity. The Morris
  `Tg=-0.5 degC` lane aborts inside PySnobal C code despite finite exported
  forcing, so PySnobal comparator use is blocked until that lane has a
  minimal reproducer or explicit lane-policy disposition. PySnobal output
  remains diagnostic evidence only, not correctness authority or production
  physics.
- `20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/` is
  complete as the legacy snow-depth comparator/output-capture assessment. It
  added `tools/snowfreeze_observed/legacy_snow_compare.py`, proved pinned
  legacy WAT `Snow-Water` is SWE only, captured dated legacy physical snow
  depth from daily-winter hour-24 rows, and retained large graphics
  `treal(73)=snodpy*1000` / `treal(75)=densg` only as sparse operand
  provenance. Fresh all-site evidence found legacy closer by mean absolute
  observed-depth residual on Sleepers South and Morris, current openWEPP closer
  on Sleepers W9, and both models failing snow-depth control on all three
  paired-snow sites. Current openWEPP SWE is close to legacy SWE on common
  dates, so the next route remains snow-depth producer/carry/input/settlement
  adjudication, not legacy bit-parity or frost heat-flow/frozen-K tuning.
- `20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/` is
  complete as the SNOWFROST-FIDELITY-E correspondence and direction audit. It
  added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048`, extended the observed harness
  with signed snow-depth residuals, depth-vs-SWE anti-alias evidence, and
  adjacent-day timing/stage checks, and added
  `tools/snowfreeze_observed/snow_depth_audit.py`. Fresh all-site evidence
  routes Sites 1, 2, and 4 to `SNOW-DEPTH-FIDELITY-ISSUE` with dominant
  modeled-over-observed snow depth; Sites 3 and 5 remain insufficient for
  snow-control because they lack paired observed snow-depth rows. No frost
  heat-flow, frozen-K/SFCC, impedance, or `Qwet` work is authorized from these
  residuals; the next route is snow-depth producer/carry/input/settlement
  adjudication.
- `20260625-snowfrost-fidelity-c-sfcc-frozen-k-diagnostics-001/` is
  complete as diagnostic-only SFCC/frozen-conductivity comparison tooling. It
  added `tools/snowfreeze_observed/frozen_k_diagnostics.py`, a deterministic
  JSON/Markdown diagnostic surface for Clapeyron/SFCC liquid water,
  SFCC-Mualem frozen conductivity, Watanabe/Flury-style capillary-bundle
  screening, Cheng-style impedance scaling, and Amankwah-style salinity
  sensitivity. The focused contract proves bounded/monotonic curves,
  non-production labels, salinity sensitivity, and no production `crates/`
  coupling. It does not select texture defaults, authorize `Qwet`, tune field
  residuals, change production runtime physics, or promote direct activation.
- `20260625-snowfrost-fidelity-b-no-qwet-heatflow-benchmarks-001/` is
  complete as benchmark-only no-migration heat-flow closure. It added CLIM06
  `snowfrost_b_*` gates for a Kurylyk/Stefan-style one-dimensional freezing
  bound, independent surface resistance reconstruction, snow/residue
  insulation, lower-front dry heat, and latent-energy-bounded fine-layer
  mutation. Production `crates/` still contain no `qwet`, `Qwet`, or `frzftp`
  implementation. The package does not validate field frost-depth fidelity or
  authorize physics tuning; modeled snow depth exposure and diagnostic
  SFCC/frozen-K candidate work remain follow-on scope.
- `20260625-snowfrost-fidelity-a-observation-residual-classification-001/` is
  complete as the first Snow and Frost Fidelity Adjudication package. It ran
  all five observed frost-depth pilot sites through the direct observed
  harness, added `tools/snowfreeze_observed/classify_residuals.py`, and
  classified zero sites as eligible for frost-model defect attribution. Sites
  1/2/4 are `SNOW-CONTROL-BLOCKED`; sites 3/5 are `INCONCLUSIVE`. The next
  field-validation step must expose modeled snow depth and rerun classification
  before tuning heat flow, frozen conductivity, SFCC/impedance, or migration
  heat.
- `20260625-snowfreeze-frost-depth-literature-annotation-001/` is complete.
  It annotated the newly available frost-depth physics literature, classified
  the two CC-BY vendorable PDFs (`Amico2011.pdf`, `Devoie2022.pdf`), updated
  the reference bibliography, and recorded the physics ladder for
  `GAP-SNOWFREEZE-002`: observation harness first, snow-insulation attribution,
  no-`Qwet` heat-flow baseline, then bounded SFCC/frozen-K/impedance candidate
  evaluation before any migration-heat promotion.
- `20260624-snowfreeze-direct-storage-reconciliation-unblock-001/` is complete
  as the DC successor to the observed frost-depth harness. It owns
  `SNOWFREEZE-DRSTOR-001` and `SNOWFREEZE-DRSTOR-002`: site3/site4 direct
  observed comparisons failed before exit-0 metric-bearing comparison at the
  R4B explicit frost storage projection nonnegative guard. The package closed
  the in-envelope projection defect without frost physics tuning,
  observation-threshold changes, default activation, or compatibility-runtime
  deletion; site3 and site4 now emit metric-bearing `UNRESOLVED` observation
  reports.
- `20260624-snowfreeze-observed-frost-depth-harness-001/` is complete. It
  acquired/normalized the pilot historic frost-depth observation corpus, added
  the local comparison harness under `tools/snowfreeze_observed/`, and installed
  local contract coverage for the corpus and harness. Direct remains opt-in;
  compatibility frost bit-parity is not the target. The site3/site4
  metric-bearing comparison blocker was lifted by the direct storage
  reconciliation package above.
- `20260624-r7h-iterative-completion-001/` is closed `OPT-IN` by operator
  decision on 2026-06-24. It retained the R7H direct performance fix and
  zero-compatibility evidence, left direct mode opt-in, and reclassified the
  remaining typed-frost vs compatibility divergence under reopened
  `GAP-SNOWFREEZE-002`. Do not resume R7H frost-vs-compatibility bit-parity
  from this package; the successor work is a frost-depth fidelity DC validated
  against historic observations.
- `20260624-r7h-closure-activation-gates-001/` is executed-held at
  `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`. It executed the
  ADR-0026 "Closure and activation gates" step after consumer cutover/deletion,
  fixed two in-envelope no-material frost closure failures exposed by full
  H2637 direct default-candidate runs, and advanced current direct H2637 to
  endpoint with `compatibility_edge_invocations = 0`. Activation remains
  blocked: the current direct endpoint is `113.53 s / 1083636 KiB` against the
  `91.2 s` `<=10x` budget, and protected-output parity is still red/not
  current-matrix green. Default activation remains disabled.
- `20260624-r7g-consumer-cutover-deletion-001/` is complete. It executed the
  ADR-0026 "Consumer cutover and deletion" step after typed frost solver
  extraction by moving R4A/direct-publication consumers from the temporary
  `DirectFrostRunoffSurface` / `DirectFrostLiquidPartition` bridge to typed
  winter-column state and outcomes, deleting the production bridge fields/API,
  and preserving the R7B day-frame size bound by passing typed frost compute
  inputs as execution context instead of resident day-frame state. Required
  closure gates passed. It does not claim R7G terminal output parity, default
  activation, or performance closure.
- `20260624-r7g-typed-frost-solver-extraction-001/` is complete. It executed
  the ADR-0026 "Typed frost solver extraction" step by moving production direct
  active-frost partition computation from the `DirectFrostRunoffSurface` /
  `HillslopeKernelRequest` bridge to typed winter-column inputs and typed
  `Wb11HydrologyKernel` partition compute. Production direct day input no
  longer assigns a frost runoff surface, focused source scans prove the hot
  path avoids compatibility request/surface calls, and typed-vs-adapter parity
  fixtures pass for active and inactive/no-material cases. The residual
  compatibility surface remains allowed only as a named comparator seam until
  the later consumer cutover/deletion package.
- `20260624-r7g-frost-state-skeleton-comparator-seam-001/` is complete.
  It targets the ADR-0026 "Frost state skeleton and comparator seam" step by
  making `DirectWinterColumnState.frost` the canonical direct lane/day
  skeleton, leaving `DirectFrostRuntimeCarry` as a temporary mirror, and
  isolating the remaining `DirectFrostRunoffSurface` bridge behind named
  comparator-seam helpers. Focused tests, source scans, workspace Rust gates,
  dependency policy, and scoped Markdown lint passed. It does not claim typed
  frost solver extraction, bridge deletion, output parity, default activation,
  or R7G closure.
- `20260623-r7g-snow-lane-migration-001/` is complete. It executed the
  ADR-0026 "Snow lane migration" step by making
  `DirectWinterColumnState.snow` the production direct lane/day authority for
  seeded snow state, R4G same-day snow mutation, and direct publication
  snow/frost prior-snow reads. `DirectSnowRuntimeCarry` remains only as a
  temporary direct-runtime compatibility mirror for residual frame surfaces.
  The package added focused lifecycle/source-scan tests and passed the required
  Rust closure gates. It does not claim frost subsolver migration, output
  parity closure, performance closure, default activation, or deletion of
  residual carry surfaces.
- `20260623-r7g-winter-column-mechanical-containment-001/` is complete. It
  installed the ADR-0026 winter-column containment boundary outside
  `direct_runtime`, added inert boxed `DirectWinterColumnState` ownership hooks
  to direct lane/day frames, reused existing runtime-input hourly winter forcing
  authority, proved the new module has no compatibility request/symbol
  authority, split the oversized direct-publication day-input helper into
  sub-3000-line chunks, and fixed the active-frost no-freeze hourly diagnostic
  gate blocker exposed by workspace validation. It does not claim solver
  migration, publication parity, performance closure, default activation, or
  R7G closure.
- `20260623-r7g-iterative-completion-001/` is executing against
  `R7G-006-FROST-SNOW-PROJECTION-PARITY-RESIDUALS`. It lifted the inherited
  surface-free active-snow hold by making sidecar-only snow inactive, adding
  typed active-snow partition authority, persistent snow carry, snow liquid
  event routing, and the same-day EROD14 qout handoff fix. It also installed
  production active-frost execution enough for full H2637 direct default to
  reach endpoints with `compatibility_edge_invocations = 0`; active-frost
  performance was reduced from `163.88 s` to retained timing `89.88 s`, under
  the `91.2 s` `<=10x` legacy budget. R7G remains incomplete because protected
  HBP/WAT/PASS parity is red: WAT `frozwt`/`frdp` still differ on `34363` rows,
  and `Snow-Water`/`RM` residuals are material. The current continuation must
  reduce frost/snow projection parity while preserving zero compatibility
  counters and the green performance gate, or prove a narrower legitimate
  out-of-envelope boundary after attempted in-envelope correction.
- `20260623-r7g-performance-closure-fixture-hardening-001/` is executed-held
  at
  `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`. It ran the
  same-binary H2637 matrix after R7F: default-disabled compatibility passed at
  `645.51 s / 229560 KiB`, rollback compatibility passed at
  `637.10 s / 229016 KiB`, and the two compatibility modes had identical
  protected output checksum maps. Direct default candidate and explicit direct
  both failed closed before endpoint timing because production direct lacks
  typed, surface-free active snow partition authority for lane 1. The follow-up
  must add typed active snow state/partition compute, downstream operands,
  shadow projection, publication operands, fixtures, no-compatibility scans,
  and helper line-count remediation before R7G can rerun.
- `20260623-r7f-direct-day-input-hot-loop-isolation-001/` is complete. It
  closed
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE` by
  replacing the production direct interleaved day-input builder hot-loop
  dependency with typed direct day-input/state projection. Production direct
  manifests now report `compatibility_edge_invocations = 0` because the
  hot-loop edge is removed, not because accounting missed it. Focused R7/R6
  suites, workspace clippy/tests, `cargo deny check`, `cargo fmt --check`,
  `git diff --check`, and scoped Markdown lint passed.
- `20260623-r7e-r7h-direct-runtime-completion-001/` is executed-held at
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`. It
  closed R7E default-candidate/rollback selection mechanics and made runtime
  selection manifest-visible, then corrected the direct runtime audit so the
  production direct interleaved day-input builder is counted as a compatibility
  edge. R7F-R7H remain blocked until that builder is replaced by typed direct
  day-input/state projection and no-compatibility counters return to zero for
  real.
- `20260623-r7d8-direct-hbp-erod15-export-alias-parity-001/` is complete. It
  lifted `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`
  by carrying producer-authoritative direct EROD15 sediment export aliases
  into HBP publication without compatibility runtime wrapping. Fresh H2637
  5-day evidence under `/tmp/r7d8ad-h2637-5day` has default/direct exits `0`,
  HBP/loss/PASS/PLOT/WAT byte identity, parsed HBP latest-event parity, and
  direct `compatibility_edge_invocations = 0`.
- `20260623-r7d7-direct-wb16-peak-publication-parity-001/` is executed-held
  with final disposition
  `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`. It
  closed the R7D6 PASS `peakro` residual by making compatibility PASS consume
  runtime `peakro` and direct PASS consume direct runoff peak authority before
  the erosion copy. Fresh H2637 5-day WAT and PASS outputs are byte-identical
  with direct `compatibility_edge_invocations = 0`; HBP now differs only on
  EROD15 sediment export aliases (`total_detachment_kg = 0.6` and
  `sediment_concentration_kg_m3 = 6.816136920064195` in compatibility versus
  direct zeros). R7D8 lifted this hold for the current H2637 5-day gate.
- `20260623-r7d6-direct-erod13-erod14-typed-producer-001/` is executed-held
  with final disposition
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`. It lifted
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT` by adding
  typed direct EROD13/EROD14/EROD15 producer authority and the direct WB16
  peak-duration producer required by active erosion publication. H2637 direct
  production exits `0` with `compatibility_edge_invocations = 0`; WAT is
  byte-identical and PASS sediment fields are parity-clean. Its PASS/HBP
  `peakro` hold was narrowed by R7D7, and the remaining HBP sediment export
  alias hold was lifted by R7D8.
- `20260623-r7d5-direct-erod14-sediment-publication-001/` is executed-held
  with final disposition
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`. It proved
  direct production had no direct sediment producer and replaced the silent
  zero active-sediment publication path with a fail-closed guard when
  `erod14_wave2_enabled` is true. Focused H2637 direct production now exits
  `1` at `R7D5 direct EROD14/EROD15 sediment producer must execute before this
  span`; R7D HBP/PASS sediment parity remains blocked on the queued R7D6 typed
  producer implementation.
- `20260622-r7d4-direct-mofe-dynamic-carry-transfer-001/` is executed-held
  with final disposition
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`. It lifted
  `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT` by copying current-lane
  R4O/R4L `ui_LfCrf`/`ui_SCrunf` arrays forward into downstream typed transfer
  buffers and making R3A/R4J consume them with area-ratio provenance. Focused
  H2637 evidence now has byte-identical WAT and PASS, and loss/plot differ
  only by `run_name`. HBP remains held because default contains nonzero
  sediment concentration/detachment/deposition payload bytes where direct still
  publishes zero erosion authority.
- `20260622-r7d3-direct-wb14-r4k-infiltration-producer-001/` is executed-held
  with final disposition
  `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT`. It lifted
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT` by
  implementing typed direct WB14/R4K infiltration/depression producer
  authority, wiring same-pass infiltration into R4A/WB18/ET/publication, and
  adding R4L direct hourly saturation addback from R4O carry arrays. H2637
  direct production exits 0 with `compatibility_edge_invocations=0`; the
  remaining dynamic transfer blocker was subsequently lifted by R7D4.
- `20260622-r7d2-multiofe-lane-seed-authority-001/` is executed-held with final
  disposition
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`. It
  lifted `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT` by replacing
  production direct topology/area-only lane seeds and single aggregate
  runtime-surface day-input profile authority with lane-indexed constructor and
  day-input seed/profile authority. Focused one-OFE HBP/loss/PASS/WAT parity
  remains green, and H2637 direct production improved to
  `182.83 s / 627436 KiB`, but HBP/PASS/WAT parity remains blocked because
  direct R4K has no baseline-authoritative WB14 infiltration/depression
  producer, so R4A still computes runoff as liquid input.
- `20260622-r7d-direct-publication-producer-authority-001/` is executed-held
  with final disposition
  `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`. It proved the
  production direct consumer path writes from `DirectRunPublicationFrame` and
  not `execution.wb13_rows`, and the focused one-OFE fixture is parity-clean.
  H2637 remains non-parity for HBP, WAT, and PASS because production direct
  still constructs topology/area-only lane frames and seeds day inputs from a
  single aggregate runtime surface instead of lane-indexed typed direct
  constructor authority.
- `20260622-r7c-production-direct-executor-path-001/` is complete with final
  disposition `COMPLETE-R7C-PRODUCTION-DIRECT-EXECUTOR-PATH`. It added the
  opt-in production direct executor route from parsed typed constructor state
  into `DirectFrameExecutor`, with manifest-visible direct counters,
  no-compatibility-edge proof, default-mode preservation, and R6J cutover
  preservation. Same-binary H2637 evidence recorded default compatibility
  `642.77 s / 228804 KB` and direct production `753.76 s / 625132 KB`;
  direct production is not performance-ready, and HBP/PASS/WAT output parity
  remains R7D scope.
- `20260622-r7b-parsed-input-typed-frame-constructors-001/` is complete with
  final disposition `COMPLETE-R7B-PARSED-INPUT-TYPED-FRAME-CONSTRUCTORS`. It
  added the R7B parsed-input typed constructor boundary for `DirectRunFrame`,
  `DirectLaneFrame`, and `DirectDayFrame`, with constructor fixtures, static
  no-compatibility scans, type-size/layout evidence, package-local
  review/verification artifacts, and full Rust closure gates. It did not
  activate production direct mode or change output schemas.
- `20260622-r7a-architecture-state-reconciliation-001/` is complete with final
  disposition `COMPLETE-R7A-ARCHITECTURE-STATE-RECONCILIATION`. It reconciled
  the array-native runtime architecture authority with the post-R6J state:
  PERFDEEP09 is recorded as the PERFDEEP07 hold-lift authority, R2-R5 are
  direct-runtime scaffold and phase-coverage evidence, R6J is explicitly
  opt-in direct publication cutover rather than default activation or full
  runtime completion, and the spec now includes a mode-state matrix separating
  compatibility, shadow, direct publication cutover, and future production
  direct mode. ADR-0025 now points to Revision 3 and the R7A-R7H burndown
  sequence. Scoped Markdown lint and `git diff --check` passed.
- `20260622-direct-runtime-section-split-001/` is complete with final
  disposition `COMPLETE-DIRECT-RUNTIME-SECTION-SPLIT`. It mechanically split
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` into ordered
  included section files under `src/direct_runtime/`, reducing the root module
  from `2922` lines to `210` lines. The new sections are `1001`, `454`, `433`,
  `391`, and `434` lines; all production direct-runtime `.rs` files are below
  `2000` lines. The crate-level direct-runtime export remains unchanged, the
  direct-runtime source-scan test now reads the included files, and Rust
  closure gates, `cargo deny check`, `git diff --check`, and scoped Markdown
  lint passed.
- `20260622-runner-intake-lane-setup-mechanical-split-001/` is complete with
  final disposition
  `COMPLETE-RUNNER-INTAKE-LANE-SETUP-MECHANICAL-SPLIT`. It mechanically split
  the execution/output/manifest and public runner-entrypoint tail out of
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  into `05_runner_execution_and_outputs.rs`, leaving the original setup file at
  `1741` lines and the new execution/output section at `1255` lines. Public
  runner exports remain unchanged; static MOFE source-scan tests now inspect
  both included runner files; Rust closure gates, `cargo deny check`,
  `git diff --check`, and scoped Markdown lint passed.
- `20260621-r6j-direct-publication-cutover-blocker-closure-001/` is complete
  with final disposition `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`. It closed
  the inherited manifest writer blocker, run-local direct counter provenance,
  direct/shadow manifest selection, current-fixture HBP/WAT/PASS/loss parity,
  direct-only production cutover writing, checksum/readback evidence, H2637
  public-output parity, and default-disabled isolation. Fresh same-binary H2637
  evidence: default `640.41 s / 227396 KiB`; direct cutover
  `637.53 s / 349400 KiB`; HBP/WAT/PASS/loss/plot byte identity; WAT
  `235961` rows and PASS `12419` rows with zero bidirectional DuckDB
  differences; manifest source `direct-publication-frame`; all direct runtime
  counters, including `compatibility_edge_invocations`, are `0`.
- `20260621-r6h-direct-pmet-day-state-carry-builder-001/` is executed-held at
  `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. It cleared
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` by replacing the
  precomputed multi-day PMET publication input vector with an interleaved
  direct day/lane builder. Current-fixture HBP byte identity remains green,
  WAT storage totals are bit-identical, and the remaining current-fixture WAT
  residual is exactly day-2 `Es` at ulp-scale PMET layer carry.
- `20260621-r6g-direct-wat-producer-authority-001/` is executed-held at
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. It reduced the R6F WAT
  producer-authority gap for the inherited current fixture: `wepp_id`,
  simulation `year`, profile fields, first-day `Es`, and first-day storage no
  longer reduce after parsed typed direct producers and residual-inclusive
  direct storage projection were bound. Current-fixture HBP identity remains
  green and the first WAT row is bit-identical. Canonical multi-OFE WAT id
  authority and lane-dimensional day inputs remain follow-up. The remaining
  WAT mismatch is exactly day-2 `Es`, `Total-Soil`, and
  `SoilWaterTotal`, blocked on an interleaved PMET day-input builder that uses
  direct-carried layer state after the previous direct day commit.
- `20260621-r6f-direct-publication-cutover-blocker-closure-001/` is
  executed-held at
  `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. It closed the inherited
  HBP byte mismatch on the current near-zero runoff fixture by publishing
  direct `peakro`/`watdur` operands, fixed the direct climate precipitation
  unit projection, proved current-fixture HBP byte identity, reduced WAT to
  exact field deltas, added typed direct process input slots, profile
  projection fields, and lane-carried layer state, and scaffolded R6G for the
  remaining production parsed-input producer authority gap. Full R6 HBP closure
  still requires nonzero peak-runoff/event-duration fixture coverage.
- `20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/` is
  executed-held at
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`. It reproduced the R6D
  fail-closed cutover state, split the direct-publication helper block out of
  the over-3000-line runner file, resolved production direct-runtime
  input-binding for parsed climate by building retained cutover publication
  through direct capture, and moved the first blocker to HBP direct process
  parity. Complete R6 closure still requires real
  `DirectPublicationFrameCutover`: HBP byte identity, WAT/PASS Arrow
  row/schema/metadata parity, loss JSON identity, manifest provenance/checksum
  parity, anti-alias fixtures, independent reconstruction, no compatibility
  authority, successful direct output writes, and default-disabled isolation.
- `20260621-r6d-production-direct-publication-producer-retention-001/` is
  executed-held at
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. It lifted the R6C
  retained-producer absence for cutover by adding a cutover-only retained
  `DirectRunPublicationFrame` producer surface to the production climate
  lifecycle. The retained frame uses parsed climate/calendar and slope geometry
  and is consumed by `DirectPublicationFrameCutover` without skeleton direct
  frame construction or post-hoc publication capture. Public direct-output
  writes remain fail-closed until parity-grade hydrology/storage/subsurface/
  evaporation/PASS/loss/manifest/erosion producers, anti-alias fixtures,
  independent reconstruction, manifest provenance, and line-count governance
  are closed.
- `20260621-r6c-direct-publication-typed-operand-bridge-001/` is
  executed-held at
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`. It corrected the R6B
  candidate so `DirectPublicationFrameCutover` fails before skeleton
  direct-frame construction/publication capture, proving the remaining blocker
  is not at the output helper but at the production climate lifecycle: retained
  direct run/lane/day publication producers do not yet exist.
- `20260621-r6b-direct-publication-parity-manifest-cutover-001/` is
  executed-held at
  `HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`. It confirmed the
  R6 cutover candidate still builds from a skeleton direct frame with
  zero/absent publication operands, retained a fail-closed diagnostic marker,
  and left anti-alias, reconstruction, manifest, output-family, default-disabled,
  and endpoint/RSS gates blocked behind the missing typed operand bridge.
- `20260621-r6-direct-publication-cutover-001/` resumed after R5E completed at
  pushed commit `d8f6bbea`. It promoted the PERFDEEP06 publication operand
  ledger into canonical architecture authority, held until R6A supplied the
  run-bound direct publication frame and direct projection consumers, then
  resumed with a guarded `DirectPublicationFrameCutover` candidate. The
  candidate fails closed at HBP byte identity and still needs manifest
  provenance cutover; wrapping compatibility WB13 rows or runtime surfaces in
  direct-named structures is not a valid cutover.

## Current roadmap execution log

State as of `2026-06-21`:

- R6G (`20260621-r6g-direct-wat-producer-authority-001/`) is executed-held
  with final disposition
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. The package bound direct
  WAT identity/year/profile producers, added parsed static/climate direct
  day-input translation for WB11/WB17/WB18/WB19 operands, preserved lane-carried
  direct layer state for later days, corrected direct projection storage to
  include residual liquid water, and proved current-fixture HBP byte identity
  plus first WAT row parity. The identity/year/profile work is current-fixture
  parity evidence; canonical multi-OFE WAT id authority, lane-dimensional day
  inputs, and allowlisted symbol lineage remain follow-up. It stopped because
  day-2 PMET `Es` and dependent storage still use precomputed component inputs
  built before the prior direct day commits carried layer state. The next R6
  package must implement dynamic interleaved PMET day-input construction rather
  than reading compatibility WB13 rows or runtime surfaces.
- R6H (`20260621-r6h-direct-pmet-day-state-carry-builder-001/`) is
  executed-held with final disposition
  `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. The package replaced the
  precomputed PMET publication input vector with an interleaved direct day/lane
  builder, added fail-closed later-day layer requirements, preserved
  current-fixture HBP byte identity, and reduced WAT from day-2 `Es`,
  `Total-Soil`, and `SoilWaterTotal` to `Es` only. R6I subsequently closed
  this `Es` parity blocker; broader multi-OFE output authority remains later
  R6 scope.
- R6I (`20260621-r6i-direct-pmet-layer-ulp-parity-001/`) is complete with
  verdict `COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`. The package localized
  the day-2 PMET `Es` residual to direct lane commit carrying the WB17
  post-root-uptake layer vector instead of the post-runoff-reconciliation
  active-frost fine-layer topology projection. It added typed
  `DirectFrostLayerCarryProjection`, builds it from direct seed-surface frost
  options and layer geometry, applies it during lane commit, and proved
  current-fixture HBP byte identity plus WAT identity with no R6G/R6H marker.
  `DirectPublicationFrameCutover` still fails closed before public writes, now
  at manifest writer wiring: `manifest direct projection is not wired to the
  production manifest writer`.
- R6F (`20260621-r6f-direct-publication-cutover-blocker-closure-001/`) is
  executed-held with final disposition
  `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. The package closed the
  R6E HBP byte mismatch on the current near-zero runoff fixture, proved
  current-fixture HBP identity on the CLI fixture, corrected the direct climate
  precipitation unit projection, and reduced the next blocker to WAT `wepp_id`,
  simulation `year`, `Es`, storage, and profile fields. It added typed direct
  process input slots, lane-carried subsurface layer state, and profile
  depth/porosity projection fields, then stopped at the remaining production
  parsed-input producer authority boundary rather than wrapping compatibility
  WB13 rows or runtime surfaces as direct authority. R6G is scaffolded to close
  that boundary; nonzero HBP fixture coverage remains a later R6 gate.
- R6E (`20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/`)
  is executed-held with final disposition
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`. The package reproduced the
  cutover failure, preserved fail-closed no-output behavior, moved
  direct-publication helpers into `04_direct_publication.rs`, reduced
  `00_runner_intake_and_lane_setup.rs` to `2787` lines, added typed
  `DirectPublicationDayInput` binding for parsed climate, and changed retained
  cutover publication to a full direct executor capture. It did not complete R6
  direct publication cutover; the next package must close contract-backed HBP
  direct process parity before WAT/PASS/loss/manifest parity work can honestly
  resume.
- R6D (`20260621-r6d-production-direct-publication-producer-retention-001/`) is
  executed-held with final disposition
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. The package added
  cutover-only retained direct publication rows to production climate execution,
  sourced from parsed climate/calendar, parsed slope geometry, and run/lane/day
  identity. The cutover branch now consumes that retained frame and no longer
  runs skeleton direct-frame publication capture. Focused runner and CLI tests
  pass and prove fail-closed no-output behavior. The next package must add
  parity-grade retained publication producers for hydrology/storage/subsurface/
  evaporation/PASS/loss/manifest/erosion families and split the monolithic
  runner helper surface before full R6 closure.
- R6C (`20260621-r6c-direct-publication-typed-operand-bridge-001/`) is
  executed-held with final disposition
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`. The package did not
  close R6 parity/manifest cutover. It changed the opt-in
  `DirectPublicationFrameCutover` path so it refuses to construct a skeleton
  direct publication frame when production direct publication producers are not
  retained. Focused runner and CLI tests pass and confirm no direct frame,
  executor, skeleton, publication capture, compatibility-edge invocation, or
  public output write occurs on the failure path. The next package must add a
  retained production direct publication producer surface to the climate
  lifecycle before parity, anti-alias, reconstruction, manifest, and benchmark
  gates can close.
- R6A (`20260621-r6a-run-bound-direct-publication-frame-001/`) is complete with
  verdict `COMPLETE-R6A-RUN-BOUND-DIRECT-PUBLICATION-FRAME`. It added
  `DirectRunPublicationFrame`, direct run/lane/day capture, the
  `DirectPublicationFrameShadow` opt-in, and direct HBP/WAT/PASS/loss/manifest
  projection consumers. Full production writer cutover remains R6 scope.
- R5E is complete with verdict
  `COMPLETE-R5E-FULL-OFE-DAY-ENDPOINT-READINESS`. The package closed R5 by
  proving the direct executor records exactly one canonical 14-phase entry per
  OFE-day in `DirectPhaseKind::ORDERED`, with R4/R5 direct spans folded under
  canonical phase entries as sub-operation counters rather than duplicate phase
  executions. R5E added explicit canonical phase-entry reporting, a focused
  endpoint-readiness test, and expanded no-compatibility source-scan coverage
  to include direct growth runtime code. Public outputs remain
  compatibility-authoritative: no HBP/WAT/PASS/loss/manifest cutover, schema
  change, default activation, or direct-only public-output endpoint occurred.
  Full Rust gates passed. Final default-disabled H2637 reps were `641.37 s`,
  `642.02 s`, and `635.47 s` (median `641.37 s`, threshold `<= 676.67 s`);
  the opt-in direct-skeleton endpoint ran at `638.33 s`. Protected output
  comparison passed through HBP/WAT/loss/plot byte identity and PASS DuckDB row
  equivalence (`12419` rows, zero bidirectional differences). Package:
  `20260621-r5e-full-ofe-day-endpoint-readiness-001/`. Pushed commit:
  `d8f6bbea`.
- R5D is complete with verdict
  `COMPLETE-R5D-GROWTH-TRANSITION-DIRECT-PHASES`. The package promoted direct
  `AnnualGrowthTransition` and `PerennialGrowthTransition` executor calls,
  adding typed inputs, direct compute, direct state mutation, downstream
  operands, and shadow projection for both phases. The direct growth state
  covers cumulative GDD, biomass, canopy cover, LAI, root mass, root depth,
  harvest index, climate/stress inputs, and legacy `gddmax` sentinel
  resolution. R5D also added an R4N direct-runtime guard so ET/root uptake can
  fail closed when required growth context is absent. Public outputs remain
  compatibility-authoritative: no WB13 ET, WAT plant metadata, PASS, loss,
  manifest, scheduler, runner API, default activation, or endpoint cutover
  occurred. Full Rust gates passed. Final default-disabled H2637 reps were
  `647.54 s`, `647.93 s`, and `644.88 s` (median `647.54 s`, threshold
  `<= 676.67 s`) with protected output comparison passing through HBP/WAT byte
  identity, PASS DuckDB row equivalence, and run-name-only normalized loss/plot
  differences. Package:
  `20260620-r5d-growth-transition-direct-phases-001/`. Pushed commit:
  `2fbd3802`.
- R5C is complete with verdict
  `COMPLETE-R5C-DECOMPOSITION-RESIDUE-DIRECT-TRANSITIONS`. The package
  promoted direct `DecompositionTransition` and `ResiduePartitionTransition`
  executor calls, adding typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection for both phases. R5C implements
  the `SC-RESIDUE-001` PL17 tracked seed-pool update over typed active
  decomposition context, validates missing/ambiguous context and invalid
  residue/fraction/rate domains as typed hard failures, and projects typed
  residue partition operands without public-output cutover. Full Rust gates
  passed. Final default-disabled H2637 reps were `639.05 s`, `646.33 s`, and
  `643.96 s` (median `643.96 s`, threshold `<= 676.67 s`) with protected
  output comparison passing through HBP/WAT byte identity, PASS DuckDB row
  equivalence, and run-name-only normalized loss/plot differences. Package:
  `20260620-r5c-decomposition-residue-direct-transitions-001/`. Pushed commit:
  `efdf6710`.
- R5B is complete with verdict
  `COMPLETE-R5B-NORMALIZATION-STORAGE-BOUNDS-DIRECT-PHASES`. The package
  promoted direct `Normalization` and `StorageBounds` executor calls, adding
  typed inputs, direct compute, direct state mutation, downstream operands, and
  shadow projection for both phases. `StorageBounds` now reports as `Executed`
  in lifecycle status counts; decomposition, residue, annual growth, and
  perennial growth remain explicit `Hold` phases for R5C-D. R5B validates the
  scalar storage/domain state available in the direct frame and does not claim
  layer-capacity physics, public output cutover, scheduler changes, or default
  activation. Full Rust gates passed. Final default-disabled H2637 reps were
  `643.38 s`, `640.54 s`, and `644.59 s` (median `643.38 s`, threshold
  `<= 676.67 s`) with protected output comparison passing through HBP/WAT byte
  identity, PASS DuckDB row equivalence, and run-name-only normalized
  loss/plot differences. Package:
  `20260620-r5b-normalization-storage-bounds-direct-phases-001/`. Pushed
  commit: `27de814c`.
- R5A is complete with verdict
  `COMPLETE-R5A-FULL-DAY-DIRECT-EXECUTOR-LIFECYCLE`. The package widened the
  direct executor from day-0 lane skeleton execution to all
  `day_count * lane_count` direct day frames, added persistent lane-state
  handoff into each day frame, added end-of-day lane commits, recorded
  day-frame commit counters, and exposed canonical phase status counts. R5A
  keeps the five non-hydrology phases reserved for R5B-D as explicit `Hold`
  statuses and leaves public output authority compatibility-owned. Full Rust
  gates passed. Final default-disabled H2637 reps were `643.98 s`,
  `647.95 s`, and `643.45 s` (median `643.98 s`, threshold `<= 676.67 s`) with
  protected output comparison passing through HBP/WAT byte identity, PASS
  DuckDB row equivalence, and run-name-only normalized loss/plot differences.
  Package: `20260620-r5a-full-day-direct-executor-lifecycle-001/`. Pushed
  commit: `3edfca66`.
- R4P/Q/Z is complete with verdict
  `COMPLETE-R4PQZ-HYDROLOGY-PROJECTION-R4-CLOSURE`. The package closed R4 by
  adding a shadow-only direct hydrology projection span after the direct
  hydrology compute chain. R4P/Q/Z requires direct upstream shadows from R4A,
  R4B, R4G, R4J, R4M, R4O, and R4N; recomputes aggregate storage from the
  final R4N layer vector; separates frozen-layer and explicit frozen storage;
  and assembles typed direct projection operands for runoff, ET, percolation,
  lateral/drainage, snow/frost, carry, profile-capacity placeholders, and
  publication comparison fields. Public output authority remains
  compatibility-owned: no WB13/WAT/PASS/loss/schema cutover, default
  activation, or scheduler change occurred. Full Rust gates passed. Final
  default-disabled H2637 reps were `645.54 s`, `644.74 s`, and `640.28 s`
  (median `644.74 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4pqz-hydrology-projection-r4-closure-001/`.
- R4N is complete with verdict
  `COMPLETE-R4N-DIRECT-WB17-ET-ROOT-UPTAKE-COMPUTE-PROMOTION`. The package
  promoted the R4E-H aggregate evapotranspiration handoff into request-free
  direct WB17 evapotranspiration and post-WB19 root-uptake compute. R4N now
  computes and shadow-projects surface/residue ET, soil-evaporation layer
  mutation, SWU/root-uptake vectors, water stress, and final aggregate ET.
  R4O consumes the R4N ET-mutated layer vector when present, and R4B requires
  final R4N ET before storage reconciliation. R4N remains no-publication,
  no-default-activation, and no-scheduler: public output paths and
  compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `643.84 s`, `650.42 s`, and `649.22 s`
  (median `649.22 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4n-direct-wb17-et-root-uptake-compute-001/`.
- R4M/O is complete with verdict
  `COMPLETE-R4MO-DIRECT-SUBSURFACE-COMPUTE-PROMOTION`. The package promoted
  the R4D/R4E-H subsurface handoff surface into request-free direct WB18/WB19
  compute from typed layer vectors. R4M computes and shadow-projects direct
  `D`, `Pe`, and per-layer percolation fluxes, mutating direct layer storage
  and feeding R4B `deep_seepage_m`. R4O computes and shadow-projects direct
  lateral `q`, tile drainage `Qdd`, final `Qd`, carry arrays, capacity/target
  diagnostics, and layer withdrawals, feeding R4B `subsurface_loss_m`. R4B now
  requires R4M and R4O shadows before storage reconciliation. R4M/O remains
  no-publication, no-default-activation, and no-scheduler: public output paths
  and compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `643.70 s`, `646.33 s`, and `639.62 s`
  (median `643.70 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4mo-direct-subsurface-compute-promotion-001/`.
- R4I-L is complete with verdict
  `COMPLETE-R4IL-DIRECT-RUNOFF-PATH-INPUT-COMPLETION`. The package implemented
  direct handoff producers for R4A `liquid_input_m`, `runon_input_m`,
  `cumulative_infiltration_m`, `depression_storage_delta_m`, and
  `surface_saturation_runoff_m`. Each producer has typed inputs, direct
  handoff compute, state mutation, downstream operands, and shadow projection.
  R4A now requires R4I liquid input, R4J runon/carry, R4K
  infiltration/depression, and R4L saturation-addback producers before runoff
  partition. R4I-L remains handoff-only, no-publication, no-default-activation,
  and no-scheduler: public output paths and compatibility runtime remain
  authoritative. The package split runoff-specific direct-runtime code into
  `direct_runtime/runoff.rs` and added a focused R4I-L test module. Full Rust
  gates passed. Final default-disabled H2637 reps were `646.47 s`,
  `642.52 s`, and `640.20 s` (median `642.52 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4il-direct-runoff-path-input-completion-001/`.
- R4E-H is complete with verdict
  `COMPLETE-R4EH-DIRECT-STORAGE-BUDGET-HANDOFF-COMPLETION`. The package
  implemented direct handoff producers for R4B `subsurface_loss_m` / `Qd`,
  aggregate `evapotranspiration_m`, and signed `snow_coupling_m`. Each
  producer has typed inputs, direct handoff compute, state mutation, downstream
  operands, and shadow projection. R4B now requires R4C storage input, R4D deep
  seepage, R4E-H subsurface loss, R4E-H evapotranspiration, R4E-H signed
  snow-coupling, and R4A runoff before storage reconciliation. R4E-H remains
  no-publication, no-default-activation, and no-scheduler: public output paths
  and compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `648.48 s`, `652.43 s`, and `642.26 s`
  (median `648.48 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4eh-direct-storage-budget-handoff-completion-001/`.
- R4D is complete with verdict
  `COMPLETE-R4D-DIRECT-DEEP-SEEPAGE-PRODUCER`. The package implemented a direct
  WB18/WB12 deep-seepage handoff producer feeding R4B `deep_seepage_m`. R4D
  consumes a dedicated direct deep-seepage handoff input, validates finite
  nonnegative `D`, mutates direct deep-seepage state and the R4B
  `deep_seepage_m` input, produces downstream operands, and shadow-projects the
  result. R4B now requires R4C storage input, R4D deep seepage, and R4A runoff
  before storage reconciliation. R4D remains no-publication, no-default-
  activation, and no-scheduler: public output paths and compatibility runtime
  remain authoritative. Full Rust gates passed. Final default-disabled H2637
  reps were `635.94 s`, `650.91 s`, and `645.47 s` (median `645.47 s`,
  threshold `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4d-direct-deep-seepage-producer-001/`.
- R4C is complete with verdict
  `COMPLETE-R4C-DIRECT-STORAGE-INPUT-PRODUCER`. The package implemented a
  direct WB12 storage-input producer and split storage-related direct-runtime
  code into `direct_runtime/storage.rs`, reducing `direct_runtime.rs` below the
  2000-line WARN band. R4C consumes R3A direct precipitation and current direct
  storage, mutates the R4B `storage_initial_m` and `precip_input_m` inputs,
  produces downstream storage-input operands, and shadow-projects the result.
  R4B now requires R4C storage input and R4A runoff before storage
  reconciliation. R4C remains no-publication, no-default-activation, and
  no-scheduler: public output paths and compatibility runtime remain
  authoritative. Full Rust gates passed. Final default-disabled H2637 reps were
  `637.63 s`, `640.25 s`, and `639.19 s` (median `639.19 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4c-direct-storage-input-producer-001/`.
- R4B is complete with verdict
  `COMPLETE-R4B-DIRECT-STORAGE-RECONCILIATION-CONSUMER-SPAN`. The package
  implemented the downstream direct WB12 storage-reconciliation consumer of the
  R4A runoff result. It consumes R4A direct `q_runoff_m`, reconciles storage
  from explicit direct operands, mutates only direct storage state, produces
  downstream storage operands, and shadow-projects storage plus closure
  residual. R4B remains no-publication, no-default-activation, and
  no-scheduler: public output paths and compatibility runtime remain
  authoritative. Full Rust gates passed. Final default-disabled H2637 reps were
  `637.34 s`, `641.14 s`, and `646.88 s` (median `641.14 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. `direct_runtime.rs` is now in the 2000+ line WARN band at 2101
  lines, below the 3000-line blocker. Package:
  `20260620-r4b-direct-storage-reconciliation-consumer-001/`.
- R4A is complete with verdict
  `COMPLETE-R4A-DIRECT-RUNOFF-PARTITION-SPAN`. The package implemented the
  first direct hydrology-process span: a narrow SC-RUNOFFPART-authoritative
  runoff-partition closure slice. It consumes direct liquid, runon,
  cumulative-infiltration, depression-storage, and saturation-addback operands;
  computes direct runoff partition state; mutates only direct runtime water
  state; produces direct downstream runoff operands; and shadow-projects the
  direct runoff result. R4A remains no-publication and no-default-activation:
  it does not migrate full WB12/WB14, Green-Ampt infiltration, scheduler paths,
  compatibility APIs, output schemas, or production publication. Full Rust gates
  passed. Final default-disabled H2637 reps were `644.01 s`, `646.84 s`, and
  `643.66 s` (median `644.01 s`, threshold `<= 676.67 s`) with protected output
  identity and PASS DuckDB row equivalence. Package:
  `20260620-r4a-direct-runoff-partition-span-001/`.
- R3C is complete with verdict
  `COMPLETE-R3C-DIRECT-MULTILANE-TRANSFER-SPAN`. The package implemented a
  run-level direct-runtime span,
  `LateralTransfer -> RunoffReconciliation -> ClosureDiagnostics`, that consumes
  direct lane topology, upstream-area ratios, lane areas, and direct transfer
  buffers; computes a diagnostic per-lane transfer ledger; mutates direct
  run-level state; produces downstream operands; and shadow-projects run-level
  transfer totals. R3C added reciprocal topology validation after review and
  remains diagnostic-only: it does not migrate hydrology-process equations, cut
  over publication, activate direct mode by default, or claim endpoint
  improvement. Full Rust gates passed. Final default-disabled H2637 reps were
  `640.85 s`, `643.41 s`, and `644.07 s` (median `643.41 s`, threshold
  `<= 676.67 s`) with protected output identity. Package:
  `20260620-r3c-direct-multilane-transfer-span-001/`.
- R3B is complete with verdict `COMPLETE-R3B-DIRECT-WATER-LEDGER-SPAN`. The
  package implemented a second direct-runtime span,
  `RunoffReconciliation -> StorageReconciliation -> ClosureDiagnostics`, that
  consumes R3A input-accounting state plus direct water and publication fields,
  computes a signed diagnostic ledger residual, mutates direct ledger state,
  produces downstream ledger operands, and shadow-projects the result. The
  residual is diagnostic-only; R3B does not migrate hydrology-process equations,
  cut over publication, claim endpoint improvement, or activate direct mode by
  default. Full Rust gates passed. Final default-disabled H2637 reps were
  `640.67 s`, `643.05 s`, and `639.21 s` (median `640.67 s`, threshold
  `<= 676.67 s`) with protected output identity. Package:
  `20260620-r3b-direct-water-ledger-span-001/`.
- R3A is complete with verdict `COMPLETE-R3A-PHASE-SPAN`. The package
  implemented direct transfer-input accounting as the first complete direct
  phase span on top of the R2A skeleton:
  `DirectPhaseKind::Normalization -> DirectPhaseKind::LateralTransfer`.
  The span includes typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection. Phase-span identity passed with
  exact binary-fraction fixture evidence; no-compatibility proof passed by
  forbidden-token source scan, scheduler no-diff, and runtime counters; the
  explicit opt-in path records one production compatibility-edge handoff while
  direct span execution records zero edge invocations. Full Rust gates passed.
  Final default-disabled H2637 reps were `630.31 s`, `640.85 s`, and
  `632.08 s` (median `632.08 s`, threshold `<= 676.67 s`) with protected
  output identity.
  R3A did not cut over publication, activate direct mode by default, or claim
  R4/R6/endpoint readiness. Package:
  `20260620-r3a-first-direct-phase-span-001/`.
- R2A is complete with verdict `COMPLETE-R2A-SKELETON`. The package introduced
  a distinct direct-runtime namespace, typed direct-frame shells, a no-op/shadow
  direct executor skeleton, explicit one-time runner setup selection, default
  inactivity proof, and executable no-compatibility proof hooks. Review removed
  misleading reserved forbidden-call counters; forbidden-call absence is proven
  by direct-runtime source/call-graph evidence, while runtime counters prove
  default-disabled direct-skeleton inactivity and explicit opt-in skeleton
  execution. Final default-disabled H2637 reps were `634.06 s`, `636.01 s`,
  and `640.93 s` (median `636.01 s`, threshold `<= 676.67 s`), with protected
  output identity. No phase math, publication cutover, endpoint-improvement
  claim, or default activation occurred. Follow-on: R3A first complete direct
  phase span. Package: `20260619-r2a-direct-runtime-skeleton-001/`.
- PERFDEEP09 executed with verdict `READY-FOR-R2`. Same-machine no-edit control
  reproduced the default-disabled blocker at `682.65 s`, RSS `228924 KB`.
  The retained remediation collapses repeated per-root perennial decomposition
  indexed-overflow scans into one slot/crop pass while preserving typed guard
  behavior. Final H2637 default-disabled reps were `634.61 s`, `635.65 s`,
  and `636.58 s` (median `635.65 s`, RSS `228856/228280/228168 KB`), clearing
  the `<= 676.67 s` P0 gate. HBP, loss, WAT, and plot checksums were stable;
  PASS parquet passed the established Arrow/DuckDB row-equivalence identity
  lane. R2+ direct-frame runtime implementation is unblocked for the next
  package, but remains unimplemented in PERFDEEP09. Package:
  `20260619-perfdeep09-disabled-path-iterative-defect-closure-001/`.
- PERFDEEP08 executed with verdict `HOLD`. The package tested one scoped
  disabled-path hard-isolation candidate: caching the PERFDEEP02 roundtrip env
  lookup and short-circuiting inactive indexed-shadow hooks. The candidate
  preserved protected output checksums but measured `691.93 s`, RSS
  `229444 KB`, slower than PERFDEEP07's retained `685.85 s` and above the P0
  `<= 676.67 s` gate. The candidate was reverted; no production Rust edit was
  retained. R2+ direct-frame runtime implementation remains blocked. Package:
  `20260619-perfdeep08-disabled-path-hard-isolation-001/`.
- R0/R1 array-native schema and frame planning is complete with verdict
  `COMPLETE-PLANNING-ONLY`. The package recorded the direct runtime schema
  envelope, direct-frame type-boundary decision, R1 constructor/projection
  plan, publication-ledger promotion plan, no-compatibility proof plan, and
  PERFDEEP07 hold-lift conditions. It made no Rust, test, output schema, or
  contract edits and does not authorize R2+ runtime implementation. Package:
  `20260619-r0-r1-array-native-schema-frame-planning-001/`.
- PERFDEEP07 executed with verdict `HOLD`. The package partially reduced the
  default-disabled tax (`701.95 s` -> `685.85 s`) but did not pass the P0
  three-run median threshold `<= 676.67 s`, so direct-frame hydrology
  implementation was not started. PERFDEEP02/03/05 opt-ins remain fail-closed,
  and R2+ array-native runtime work remains blocked until the hold is closed or
  explicitly superseded. Package:
  `20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/`.
- PERFDEEP06 is executed with verdict `READY-FOR-PERFDEEP07`. The package
  produced the array-native fast-path frame inventory, publication operand
  ledger, direct-frame API plan, layout/allocation ledger, no-hot-loop-map
  proof, and follow-on package sequence. It also recorded the default-disabled
  regression as a P0 follow-on gate: PERFDEEP05 default-disabled H2637 measured
  `701.95 s` versus the `669.97 s` reference, and PERFDEEP03 default-disabled
  measured in the `697-708 s` band. PERFDEEP07 must make the opt-in plumbing
  zero-cost when disabled before adding more direct-frame machinery. No
  production activation or Rust implementation occurred in PERFDEEP06. Package:
  `20260619-perfdeep06-array-native-fast-path-inventory-001/`.
- PERFDEEP05 is complete with verdict
  `NO-GO - sync hotspot removed, endpoint still fails activation gate`. The
  package removed `sync_from_writeback_surface` from the PERFDEEP03 opt-in daily
  H2637 hot loop, applies MOFE transfer input directly to lane-owned dense state
  through cached transfer symbol ids, and added cached-slot daily refresh for
  prepared hot/static symbols. Final-code H2637 identity passed: HBP/WAT
  byte-identical, PASS Arrow-equivalent, and plot/loss differences limited to
  `run_name`. Final-code default-disabled H2637 measured `701.95 s`,
  `227712 KB`; final-code opt-in measured `911.11 s`, `229820 KB`, versus the
  PERFDEEP01 `669.97 s` activation reference. The PERFDEEP04 full-sync hotspot
  is gone from the profile, but remaining dense-edge costs dominate:
  `refresh_cached_slots_from_writeback_surface` (`16.20%` children,
  `9.07%` self), `apply_kernel_writeback_payload` (`10.47%` children),
  `SymbolRegistry::id_of` (`7.72%` children), and
  `flush_dirty_to_writeback_surface` (`6.72%` children). No default activation.
  Follow-on: PERFDEEP06 fast-path inventory/API planning, not another
  compatibility-edge optimization. Package:
  `20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/`.
- PERFDEEP04 is complete with verdict
  `PROFILED - cut PERFDEEP05 at lane-dense sync removal`. The package scaffolded
  and executed matched `perf record` profiles for the PERFDEEP03 opt-in H2637
  lane-dense no-go and the default-disabled H2637 path. Opt-in measured
  `1164.31 s`, `519160 KB`, `61248` samples under profiler; default measured
  `704.82 s`, `320640 KB`, `37051` samples. The top PERFDEEP03-specific hotspot
  is `HillslopeLaneDenseState::sync_from_writeback_surface` at `33.49%`
  inclusive / `14.19%` self, absent from default. Dense reads helped
  (`state_value_for_symbol` fell from `14.83%` inclusive default to `3.80%`
  opt-in), but daily logical/indexed-to-dense resync, hot-symbol vector rebuilds,
  symbol-id lookup, and boundary BTreeMap flush dominate. Follow-on:
  `PERFDEEP05 - Lane-Dense Transfer Authority and Sync Removal`. Package:
  `20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/`.
- PERFDEEP03 is complete with verdict
  `NO-GO - section 7 falsification / re-profile before expanding`. The package
  implemented the PERFDEEP02 ownership correction: lane-owned persistent compact
  dense state carried through `OfeLanePersistentState`, compact dense slot views
  on `HillslopeKernelRequest`, direct dense writeback application, dirty-slot
  boundary flush, and default-disabled runner activation behind
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1`. Correctness gates passed:
  HBP/WAT byte identity, PASS Arrow equivalence, 235961 diagnostic roundtrip
  rows with zero mismatches, full Rust gates, and `cargo deny`. The load-bearing
  opt-in H2637 endpoint failed: `1147.96 s`, `229580 KB` versus the PERFDEEP01
  `669.97 s` reference. Default-disabled identity passed, but default endpoint
  flatness was not proven (`697.36 s` / `707.80 s`), so there is no default
  activation. Follow-on work must re-profile the current no-go implementation
  before expanding the island or deleting more logical surfaces. Package:
  `20260619-perfdeep03-persistent-lane-owned-dense-state-001/`.
- PERFDEEP02 is complete with verdict `NO-GO - performance blocked`. The
  package implemented the Stage-1 dense-slot `HillslopeDayFrame` hydrology
  island mechanics, dense-first request reads, dirty-id frame writeback flush,
  and focused full-family frame roundtrip tests. Full Rust gates passed.
  Production opt-in H2637 endpoint attempts failed by more than 2x versus the
  PERFDEEP01 `669.97 s` reference, so the island is fail-closed behind
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`. Follow-on work must remove per-day/OFE
  frame lifecycle cost before default activation. Package:
  `20260619-perfdeep02-hydrology-island-core-001/`.
- PERFMIG02 is executed-redirect. The rung preserved identity while migrating hot
  scalar helpers to dense-first reads and retiring logical materialization for
  six internal WB11/WB12/WB14 symbols, but the final-code H2637 no-UI endpoint
  was flat/negative versus PERFMIG01 (`669.97s` -> `672.14s` / `675.00s`, RSS
  `228144 KB` -> `227636 KB` / `228152 KB`). The strict package attribution
  subgate also failed: artifact-local `apply_indexed` materialize-all measured
  `104.752336 us/payload`, while the conservative skip-six policy measured
  `105.460510 us/payload` because fail-closed stale-logical removal costs more
  than six avoided inserts. Verdict: REDIRECT; next perf work should pivot to a
  deep single-phase array-native read+compute+write migration rather than another
  writeback-only or tiny materialization-retirement rung.
  Package:
  `20260618-perfmig02-wb11-consumer-cluster-boundary-retirement-001/`.
- PERFMIG01 is complete with verdict `CONTINUE`. ADR-0023 was ratified and the
  production WB11 warm-rain runoff writeback branch now emits a dense
  `SymbolId`-backed payload: 543 state updates plus 8 flux updates, with the
  logical payload empty on the migrated success path. Focused tests proved exact
  materialized map equality and exact `f64::to_bits()` equality. The H2637
  no-UI endpoint rerun was semantically identical to PERFIDX06 outputs but
  measured `669.97s`, `228144 KB` versus PERFIDX06 `666.82s`, `228508 KB`
  (`+0.47%`). The transition apply boundary measured `107.531649 us/payload`
  (`25.373275s` projected over H2637 OFE-days), so the first-rung regression is
  a named retireable compatibility-boundary result. Next perf rung should
  migrate a contiguous WB11-consumer cluster. Package:
  `20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001/`.
- PERFARCH03 is complete with verdict `GO - branch floor clears <=5x and
  <=10x`. The artifact-local full array-native WB11 runoff branch prototype
  validated 543 state outputs plus 8 flux outputs against the current production
  kernel by exact numeric `to_bits()` equality. Median array combined hot-loop
  cost was `0.959423 us/OFE-day` (`0.024823x` legacy us/OFE-day; projected
  `0.226386 s` over H2637 OFE-days), while one-shot boundary materialization was
  measured separately at `108.068963 us/OFE-day`. Dense slot working set was
  `18,208 bytes` and release-binary RSS was `3,072 KiB`. Verdict authorizes a
  follow-on array-authoritative production migration package / ADR-0023 revival,
  starting with WB11 runoff; it does not claim full H2637 endpoint closure yet.
  Package: `20260618-perfarch03-full-array-native-floor-prototype-001/`.
- POST-BASECOND01-H2637-MAGNITUDE-DISPOSITION is complete. The package
  synthesized FARPOINT01, MAGPARITY01, STAGE2-LATQCC, REFINTENT001,
  STAGE2-BASE-CONDUCTIVITY, and BASECOND01 evidence, then resolved the H2637
  `71.0036550031206%` magnitude flag as `CORRECT-BY-CONSTRUCTION` / `NO DEFECT`
  for the internal openWEPP lateral lineage. The remaining absolute physical
  magnitude question is an external-authority `CONTRACT-GAP`, recorded as
  `docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`;
  it is not a queue blocker and does not authorize a production edit. Package:
  `20260618-post-basecond01-h2637-magnitude-disposition-001/`.
- BASECOND01 is complete-with-correction. `SC-INFILE-SOIL-001` v0.1.11 now
  explicitly separates vertical `ssc` from hourly lateral `ui_ssh`: the top
  normalized 200 mm interval uses the baseline top source-layer `ksat` rule,
  lower split-source vertical `ssc` is inverse-conductivity/harmonic, and
  `wb19_lateral_ssh` remains arithmetic from `ksat*anisotropy`. Regression tests
  prove the surfaces are non-aliased. The H2637 no-UI rerun was aggregate-inert
  (`runvol_pct_precip` remained `71.0036550031206`), so BASECOND01 closes the
  vertical `ssc` defect but did not by itself close the remaining FARPOINT01
  magnitude flag; POST-BASECOND01 closed that flag by disposition after the full
  evidence chain was synthesized. Package:
  `20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/`.
- STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE is complete with verdict
  `OPENWEPP-DEFECTIVE`. The package proved base `ksat` is byte-live on H2637
  (`ksat_x0.9` changed WAT/PASS checksums, aggregate `latqcc`, PASS `runvol`,
  and peak WAT `latqcc`). Source intent splits the surfaces: vertical
  `wb18_perc_ssc` is inverse-conductivity normalized, while modern hourly
  `wb19_lateral_ssh` is arithmetic `ssc2*ui_anisrt`. At the time of that
  package, openWEPP made vertical `ssc` arithmetic too, inflating H2637
  split-layer `ssc` from `117.955408163210` to `270.8259 mm/h`. FARPOINT01
  remained open and routed to BASECOND01 for vertical `ssc` 200 mm
  normalization while preserving hourly `ui_ssh`. Package:
  `20260618-stage2-base-conductivity-h2637-magnitude-001/`.
- REFINTENT001-KSATADJ-SATFRAC is complete. WB14 `ksatadj` now forms
  `sat_frac` from the ratified source-intent operands
  `avsat/(avpor*avcpm)` with the two `avsat` caps and top-two tillage weighting;
  the old `sum(theta)/sum(ul)` surrogate is removed. Focused WB14 tests, full
  workspace gates, H2637 both UI variants, and the OFE1-OFE5 ladder passed.
  H2637 `runvol` remained `71.003655003121%` of precipitation because
  `ksatadj = 0` on H2637, so REFINTENT001 did not close FARPOINT01. Package:
  `20260618-refintent001-ksatadj-satfrac-defect-closure-001/`.
- STAGE2-LATQCC-H2637-MAGNITUDE is complete with verdict `CONTRACT-GAP`.
  H2637 `latqcc` was traced through WB19 per-substep operands for selected
  high-magnitude days across all 19 OFEs; emitted WAT `latqcc` equals WB19 `q`,
  and recomputed Eq [6.2.4]/Dun-style potential matches to floating-point
  precision. No openWEPP equation, withdrawal, conductivity-override,
  active-depth, or `drfc` formula defect was found. The remaining FARPOINT01
  Stage-2 flag is an absolute lateral-flow magnitude authority gap, not a
  defect-closure handoff. Package:
  `20260618-stage2-latqcc-h2637-magnitude-001/`.
- REFACTOR022 is complete for behavior-preserving monolith line-count cleanup.
  The four target-tier WARN-band files closest to the 3000-line required-refactor
  threshold were split by domain responsibility:
  `routing.rs`, `scheduler_seed_and_runtime.rs`, `core_types.rs`, and
  `hydrology_phase_lateral_drainage.rs`. Every resulting parent/section file is
  below 2000 lines, the true pre-refactor HEAD anchor closed with
  `anchor_mismatches = 0`, and required Rust gates passed. The six 2000-2500
  line files remain deferred advisory WARN work. Package:
  `20260618-refactor022-monolith-line-count-split-001/`.
- PERFARRAY02 is executed-NO-GO (WB11 request/accessor authority split +
  integrated floor). The flag-gated array request/accessor seam and real WB11
  runoff pilot landed, and default-vs-pilot identity passed on OFE5 and H2637
  (HBP/loss/plot/wat checksums equal; pass parquet rows equal). The H2637
  array-native pilot measured `817.810 us/OFE-day`, above the `386 us/OFE-day`
  <=10x budget and `193 us/OFE-day` 5x stretch. Boundary seed/materialize was
  `1685.023 us/OFE-day` and reported separately. Verdict: do not ratify
  ADR-0023 from this pilot; do not proceed to broad Stage C-F migration without
  a new kernel output/writeback-shape decision. Package:
  `20260618-perfarray02-wb11-request-accessor-authority-split-001/`.
- PERFARRAY01 is executed-NO-GO as scoped (WB11 integrated
  array-authoritative pilot, Stage A + B). Stage A landed a default-unwired
  array contract shell in `openwepp-kernel-contract` and focused crate gates
  passed. Stage B did not run: static inspection showed the current
  `HillslopeKernelRequest` and scheduler still require logical `BTreeMap`
  state/flux maps for kernel reads, consumer-boundary validation, logical
  writeback apply, and indexed mirror synchronization. Any pilot from that seam
  would violate the package's no per-day export or no dual-write proofs. No
  H2637 floor measurement; ADR-0023 remains unratified. Package:
  `20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001/`.
- PERFARCH02 is complete (architecture scoping + floor prototype). Verdict:
  CONDITIONAL GO to an integrated WB11 array-authoritative pilot. The
  artifact-local prototype preserved exact exported-map identity for the
  prototyped writeback/guard flow, preserved fail-closed rejection/message-id
  class with lazy failure subjects, and measured the array-authoritative
  writeback/guard path at roughly 49.9x faster than the current logical
  writeback/guard path. Interpretation: <=10x remains credible only through an
  integrated WB11 pilot; 5x remains unproven. Package:
  `20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/`.
- PERFIDX06 is complete (Stage 6: high-OFE target assessment). Same-machine H2637
  measurements pinned the PERFIDX04 endpoint at `666.82s` no-UI and `667.44s`
  with UI; pinned legacy medians were `9.12s` no-UI and `11.54s` with UI. The
  resulting ratios are `73.12x` no-UI and `57.84x` with UI. Verdict: `<=10x`
  is not closed, `<=5x` is not plausible under the current read-mirror design,
  and the next perf move is redesign scoping, not more narrow write-side
  id-table work. Package:
  `20260618-perfidx06-high-ofe-target-assessment-001/`.
- PERFIDX05 is HELD (Stage 5: writeback/guards by SymbolId). Bit-identical but
  performance-NEGATIVE (H2637 −5.3–5.8%) — the write/guard-side dual-write cost
  (logical + mirror) exceeds the id saving; a structural ceiling of the read-mirror
  design, not incompleteness. Code discarded, record kept. Package:
  `20260617-perfidx05-writeback-guards-by-id-001/`.
- PERFIDX04 is complete (Stage 4: resolve-once hot-symbol-id tables + indexed
  read-mirror). H2637 −24.3%/−25.2%, bit-identical, irrigation excluded. Package:
  `20260617-perfidx04-hot-symbol-id-tables-001/`. Endpoint stands as the perf state.
- PERFIDX03B is complete as the blocker-closure follow-on to held PERFIDX03.
  Scope: indexed kernel seam/export-cache work needed before Stage 4. Package:
  `20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/`.
- CQR36 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.
  Final target `parse_impoundment` CRAP is `15.0`, with zero unique
  target-file rows above `30`. WARNs remain for `cargo crap` LCOV source-map
  warnings.
  Package:
  `20260616-cqr36-watershed-impoundment-parser-complexity-001/`.
- CQR35 is complete-with-warnings for live-metric
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  Fresh before and after metrics prove the highest target-file row is
  `Wb11HydrologyKernel::wb19_lateral_transfer_inputs` at CRAP
  `26.541362973760947`, with zero target-file rows above `30`. WARNs remain
  for `cargo crap` LCOV source-map warnings and the target file line count
  above the older caution threshold.
  Package:
  `20260616-cqr35-lateral-drainage-complexity-001/`.
- CQR34 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-summary-accumulator/src/lib.rs`. The scoped target reduced
  `SummaryAccumulatorError::fmt` CRAP from `240.0` to `1.0`; the extracted
  private helper `SummaryAccumulatorError::write_display` is CRAP `15.0`.
  WARNs remain for `cargo crap` LCOV source-map warnings and the same-file
  out-of-scope `Wb13DailyWaterBalanceRow::from_surface` row above CRAP `30`.
  Package:
  `20260615-cqr34-summary-accumulator-complexity-001/`.
- CQR33 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`. The
  scoped target reduced `WatershedStructureParseError::fmt` CRAP from `240.0`
  to `1.0`; the extracted private helper
  `WatershedStructureParseError::write_display` is CRAP `15.0`. WARNs remain
  for `cargo crap` LCOV source-map warnings and the same-file out-of-scope
  parser row above CRAP `30`. Package:
  `20260615-cqr33-watershed-structure-parser-complexity-001/`.
- CQR32 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/climate.rs`. The scoped target
  reduced `ClimateParseError::fmt` CRAP from `240.0` to `1.0`; the extracted
  private helper `ClimateParseError::write_display` is CRAP `15.0`. WARNs
  remain for `cargo crap` LCOV source-map warnings, same-file out-of-scope
  parser rows above CRAP `30`, and target-file line coverage below the ADR-0021
  glue-tier threshold. Package:
  `20260615-cqr32-climate-parser-complexity-001/`.
- CQR31 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`.
  The scoped target reduced `build_simulation_owned_wb13_row_for_ofe` CRAP from
  `251.62932776803854` to `16.0`, with every newly extracted helper CRAP
  `<= 12.584884659264825`. WARNs remain for `cargo crap` LCOV source-map
  warnings and the pre-existing same-file out-of-scope
  `derive_profile_fc_store_from_authoritative_layers` row above CRAP `30`.
  Package: `20260615-cqr31-runner-output-climate-complexity-001/`.
- CQR30 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`.
  The scoped target reduced `Wb11HydrologyKernel::run_erod13_wave1_core`
  CRAP from `265.2636791582994` to `8.0`, with every newly extracted helper
  CRAP `<= 29.0`. WARNs remain for `cargo crap` LCOV source-map warnings.
  Package: `20260615-cqr30-erod13-wave1-complexity-001/`.
- CQR29 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`.
  The scoped target reduced `Wb11HydrologyKernelGuardError::fmt` CRAP from
  `272.0` to `1.0`, with every newly extracted helper CRAP
  `<= 8.000751314800901`. WARNs remain for `cargo crap` LCOV source-map
  warnings.
  Package: `20260615-cqr29-guard-errors-complexity-001/`.
- CQR28 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`.
  The scoped target reduced `run_percolation` CRAP from
  `281.82979375564685` to `17.19373252009578`, with every newly extracted
  helper CRAP `<= 22.896222121074196`. WARNs remain for `cargo crap` LCOV
  source-map warnings and pre-existing same-file out-of-scope rows above CRAP
  `30`. Package: `20260615-cqr28-plant-percolation-complexity-001/`.
- CQR27 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-input-contract/src/parsers/management.rs`. Package:
  `20260615-cqr27-management-parser-complexity-001/`. Final target:
  `parse_yearly_annual_fallow`, CRAP `4.0`.
- CQR26 is complete-with-warnings for live-metric closure of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  Package:
  `20260615-cqr26-lateral-drainage-complexity-001/`.
- CQR25 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
  Final target CRAP: `12.4198250729`. Package:
  `20260615-cqr25-runner-intake-lane-setup-complexity-001/`.
- CQR24 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
  The scoped target reduced `produce_wb16_ealpha_from_runtime_surface` CRAP
  from `317.2103869084884` to `6.010666666666666`, with every newly extracted
  WB16 helper at CRAP `<= 15.401920438957477`, without changing public API,
  runtime symbols, publication formulas, typed guard behavior, parser
  compatibility, or science-contract behavior. WARNs remain for target-file
  coverage below the ADR-0021 line threshold and pre-existing same-file
  out-of-scope rows above CRAP `30`. Package:
  `20260615-cqr24-scheduler-seed-runtime-complexity-001/`.
- CQR23 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
  The scoped target reduced `run_erod19_route_segment_migration` CRAP from
  `351.9234211799049` to `9.00460855712335`, with every newly extracted helper
  below `15`, without changing public API, runtime symbols, writeback order,
  typed guard behavior, parser compatibility, or science-contract behavior.
  WARNs remain for target-file coverage below the ADR-0021 line threshold and
  the pre-existing out-of-scope `erod19_depend` row at CRAP
  `87.98408081839372`. Package:
  `20260615-cqr23-erod19-route-segment-complexity-001/`.
- CQR22 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of `crates/openwepp-input-contract/src/parsers/soil.rs`.
  Package: `20260615-cqr22-soil-parser-complexity-001/`. Final target CRAP:
  `5.0`.
- CQR21 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of `crates/openwepp-climate-runtime-adapter/src/lib.rs`.
  Package: `20260615-cqr21-climate-runtime-adapter-complexity-001/`. Final
  target CRAP: `2.0`.
- CQR20 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`.
  Package: `20260615-cqr20-projection-helpers-complexity-001/`. Final target
  CRAP: `9.0`.
- CQR19 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`.
  Package: `20260615-cqr19-watershed-runtime-types-complexity-001/`. Final
  target CRAP: `6.0`.
- CQR18 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`.
  Package: `20260615-cqr18-hbp-payload-validator-complexity-001/`. Final
  target CRAP: `9.0`.
- CQR17 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
  Package: `20260615-cqr17-hydrology-erod19-complexity-001/`. Final target
  CRAP: `2.0`.
- CQR16 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-sim-contract/src/units_mod/registries.rs`.
  The scoped target reduced `BoundaryUnitRegistryError::fmt` CRAP from
  `506.0` to `6.0`, with every newly extracted helper at CRAP
  `11.00102848303003` or lower, without changing public API, registry rows,
  aliases, units, publication units, scalar exceptions, parser compatibility,
  or science-contract behavior. Required Rust closure gates passed. WARNs
  remain for target-file coverage below the full ADR-0021 module threshold and
  the pre-existing out-of-scope `validate_entry` row at CRAP
  `62.4742520806637`. Package:
  `20260615-cqr16-unit-registries-complexity-001/`.
- CQR15 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
  The scoped target reduced `seed_wb11_runtime_surface_inputs` CRAP from
  `580.6018405181356` to `15.0`, with every newly extracted helper at CRAP
  `23.01930315500686` or lower, without changing public API, runtime symbols,
  lane policy, typed guard behavior, formulas, parser compatibility, or
  science-contract behavior. Required Rust closure gates passed. WARNs remain
  for target-file coverage below the full ADR-0021 module threshold, target
  file line count above `2000`, and unrelated out-of-scope target-file rows
  above CRAP `30`. Package:
  `20260615-cqr15-scheduler-seed-runtime-complexity-001/`.
- CQR14 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/release.rs`. The scoped target reduced
  `lint_release_directory` CRAP from `650.0` to `4.0`, with every newly
  extracted release-lint helper below `9`, without changing public API,
  release sidecar schema, binary role classification, stable error variants,
  candidate filtering, HBP pair parity, hash, timestamp, or JSON field
  behavior. Required Rust closure gates passed. WARN remains for the
  pre-existing out-of-scope `validate_release_sidecar_unlocked` row at CRAP
  `31.459079074798446`. Package:
  `20260615-cqr14-runner-release-complexity-001/`.
- CQR13 is complete for live-metric closure of the rank-7
  CRAP/cyclomatic-complexity row in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
  Fresh before metrics proved the snapshot row had already been closed by
  prior runtime core type decomposition: the highest current target-file CRAP
  row is `HillslopeRuntimeInputError::soil_core_code` at
  `14.0478515625`, with every row below `30` and target-file line coverage
  `497/515`. No production refactor was needed. Required Rust closure gates
  passed. Package:
  `20260615-cqr13-runtime-core-types-complexity-001/`.
- CQR12 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  The scoped quality target reduced
  `seed_hillslope_runtime_surface_from_irrigation_depletion` CRAP from
  `1122.0` to `2.0`, with every newly extracted depletion helper below
  `10`, without changing public API, typed guard classes, stable error fields
  and allowed strings, depletion irrigation symbols, units, parser
  compatibility, period iteration, sprinkler/furrow field meanings, or
  kernel-facing projection behavior. Required Rust closure gates passed. WARN
  holds remain for target-file coverage below the science-tier threshold and
  the pre-existing out-of-scope frost `too_many_lines` suppression. Package:
  `20260615-cqr12-irrigation-depletion-runtime-001/`.
- CQR11 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-input-contract/src/parsers/management.rs`. The scoped
  quality target reduced `parse_yearly_perennial` CRAP from `1406.0` to `4.0`,
  with every newly extracted perennial parser helper below `10`, without
  changing public parser API, typed error variants, stable error IDs, field
  names, count/cardinality guards, branch compatibility, parser output shape, or
  runtime/kernel-facing management semantics. Required Rust closure gates
  passed. WARN holds remain for target-file coverage below the science-tier
  threshold and pre-existing out-of-scope CRAP rows above `30`. Package:
  `20260615-cqr11-management-parser-complexity-001/`.
- CQR10 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  The scoped quality target reduced
  `seed_hillslope_runtime_surface_from_irrigation_fixeddate` CRAP from
  `1482.0` to `4.0`, with every newly extracted fixed-date helper below
  `15`, without changing public API, typed guard classes, stable error fields
  and allowed strings, fixed-date irrigation symbols, units, parser
  compatibility, event order, furrow formulas, or kernel-facing projection
  behavior. Required closure gates passed. WARN holds remain for target-file
  coverage below the science-tier threshold and the pre-existing out-of-scope
  depletion CRAP row above `30`. Package:
  `20260615-cqr10-irrigation-fixeddate-runtime-001/`.
- CQR09 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`.
  The scoped quality target reduced `build_annual_decomposition_control` CRAP
  from `1497.0871919084125` to `9.179748500041095`, with every newly extracted
  annual helper below `14`, without changing public API, typed guard classes,
  stable error reasons, decomposition symbols, units, parser compatibility,
  scheduler payload fields, or output formulas. Required closure gates passed.
  WARN holds remain for target-file coverage below the science-tier threshold
  and pre-existing out-of-scope CRAP rows above `30`. Package:
  `20260615-cqr09-decomposition-equations-complexity-001/`.
- CQR08 is complete for behavior-preserving function-length/lint-debt
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
  The scoped quality target removed the `HillslopeRuntimeInputError`
  `fmt::Display` `#[allow(clippy::too_many_lines)]` suppression and reduced the
  target error-code/display CRAP rows from `964.0467577461321` and `4290.0` to
  helper rows all below `15`, without changing stable error codes, display text,
  typed variant semantics, runtime projection guards, or public API behavior.
  Required closure gates passed. Package:
  `20260615-cqr08-runtime-core-types-display-001/`.
- CQR07 is complete-with-warnings for behavior-preserving
  function-length/lint-debt decomposition of
  `crates/openwepp-runner/src/watershed_wat.rs`. The scoped quality target
  removed the `read_batch_into` `#[allow(clippy::too_many_lines)]` suppression,
  reducing `read_batch_into` CRAP from `4830.0` to `4.0`, without changing WAT
  reader, aggregation, optional-column, fail-closed, or public publication
  behavior. Required closure gates passed. WARN holds remain for target coverage
  below the science-tier threshold and pre-existing out-of-scope CRAP rows above
  `30`. Package:
  `20260615-cqr07-watershed-wat-complexity-001/`.
- CQR06 is complete-with-warnings for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  The scoped quality target decomposed WB19 lateral-transfer, drainage, and
  top-layer conductivity adjustment helpers so every eligible target-module
  function has CRAP `<= 26.541362973760947`, without changing WB19 formulas,
  typed guard IDs, symbol names, arithmetic grouping, thresholds, unit
  conversions, writeback order, or public crate APIs. Required closure gates
  passed. WARN holds remain for target-file line count over 2000 and target
  coverage below the science-tier threshold after private helper extraction.
  Package:
  `20260615-cqr06-lateral-drainage-complexity-001/`.
- CQR05 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`.
  The scoped quality target decomposed `run_erod14_wave2` so every eligible
  target-module function has CRAP `<= 23.0`, without changing EROD14 Wave-2
  formulas, typed guard IDs, symbol names, arithmetic grouping, thresholds,
  writeback order, or public crate APIs. Required closure gates passed. WARN
  hold remains for target coverage below the science-tier threshold after
  private helper extraction. Package:
  `20260615-cqr05-erod14-wave2-complexity-001/`.
- CQR04 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`.
  The scoped quality target decomposed high-risk watershed channel routing
  helpers, especially `ws20_route_case12_segment_family`, so every eligible
  target-module function has CRAP `<= 30`, without changing WS10/WS11/WS20-WS24
  routing behavior, typed guard IDs, symbol names, arithmetic grouping,
  thresholds, or public crate APIs. Required closure gates passed. WARN holds
  remain for target-file line count over 2000 and target coverage below the
  science-tier threshold after private helper extraction. Package:
  `20260615-cqr04-watershed-routing-complexity-001/`.
- CQR03 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.
  The management runtime projection dispatcher and primary live-canopy
  assimilation helper are decomposed into private stage helpers, the obsolete
  target-file `too_many_lines` suppressions are removed, and every eligible
  target-module function has CRAP `<= 17.16724537037037` after the refactor.
  Required closure gates passed. Package:
  `20260615-cqr03-management-runtime-inputs-complexity-001/`.
- CQR02 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`.
  `parse_layout` is now a staged dispatcher over private parser helpers, public
  HBP parser APIs are unchanged, and every eligible target-module function has
  CRAP `<= 20.0` after the refactor. Required closure gates passed. Package:
  `20260615-cqr02-hbp-layout-parser-complexity-001/`.
- CQR01 is complete for behavior-preserving code-quality decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
  `compute_active_frost_coupling` no longer carries the
  `#[allow(clippy::too_many_lines)]` suppression, remains public-surface
  compatible, and its target CRAP row improved from `238.28646229402713` to
  `8.003859752282304`. Required closure gates passed. Package:
  `20260615-cqr01-frost-entry-complexity-001/`.
- REFACTOR024 is complete for a behavior-preserving line-count split of
  `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`. The root
  integration test is now an 11-line module harness with support and concern
  modules under `tests/integration/clim06_frost_frozen_soil_kernel_contract/`;
  all split files are below 1000 lines. The original 46 test functions remain
  present and the required closure gates passed. Package:
  `20260614-refactor024-clim06-frost-test-line-count-split-001/`.
- REFACTOR023 is complete for the 3000+ line
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  mechanical split. Scope is behavior-preserving module extraction only:
  `coupling.rs` remains the snow/interval wiring surface while frost helpers
  moved under `support_helpers_mod/coupling/`. Final line counts:
  `coupling.rs=230`, `coupling/frost.rs=1838`,
  `coupling/frost_entry.rs=1000`. Required closure gates passed. Package:
  `20260614-refactor023-hillslope-coupling-line-count-split-001/`.
- HPHYS0320 **closed the SIMIMPL28 storm-start timing seam** (`wnttim < 1.0 -> 1.0`,
  `INV-CLIMATE-018`). This was the first real forcing correction of the entire
  HPHYS0298->0320 snow-comparator arc — and it was a **climate-forcing timing
  defect, not snow physics**. The snow surface was only where the symptom showed.
- The HPHYS0298->0320 snow/`RM` comparator route (the combined `57` carried rows)
  remains retired per ADR-0017 (comparator is a flag, not a target). **Do not open
  HPHYS0321 to continue that route.**
- The snow science review (`docs/backlog/20260605-snow-code-deferred-science-review.md`)
  is now **promoted and split into two stages** (static analysis of the J-95
  negative-SWE site, 2026-06-06): **Stage 1 = snow mass conservation /
  single-sourcing** — an architecture/conservation hard gate that sits on rung-1's
  closure gate, so it is **active now** (see the SNOWSCI Stage-1 package below);
  **Stage 2 = snow physics-magnitude** — the `snowd.for` equation adjudication,
  which **stays deferred behind the protected boundary.** Snow *conservation* is no
  longer suspended; snow *magnitude* still is.
- **WSHED01 closed the openWEPP-native totalwatsed3 CLI + closure** (2026-06-14,
  item 9) — the WBVAL06/6a end-to-end totalwatsed3 deferral is **resolved** on
  openWEPP-native output (`openwepp-cli-totalwatsed3`, ADR-0019/0020), closing
  ex-day-1 at `−0.41 mm/2191 d` with independent operands. Channel
  water-balance routed output (`chanwb`) is a **separate** follow-on
  (`WATERSHED-CHANWB-ROUTED-OUTPUT`), decoupled from the hillslope-only
  totalwatsed3 per ADR-0020.
- **FARPOINT01 closed the MOFE >10-OFE far-point demonstration** (2026-06-16,
  item 11) — openWEPP's three identities close at 19 OFEs on H2637, past the
  legacy ≤10-OFE ceiling. F-B closed (contract-first) a frost `watbtm`
  double-count the substrate surfaced; F-C contrasted closure (legacy with_ui
  runoff = 127.7 % of precip — q-cap violation — vs openWEPP 71 %, bounded); the
  `watpdg` branch-out resolved as a validated non-defect. **MAGPARITY01 closed
  2026-06-18** with no transfer/area/export defect. **STAGE2-LATQCC closed
  2026-06-18** with no WB19 equation or operand-bound defect; the remaining
  bounded runoff delta is an absolute lateral-flow magnitude authority gap. The
  ~80–110× high-OFE
  wall-clock gap is scaffolded as `PERFHO01`.

Active work sequence (each rung adds one mechanism on an already-closed
foundation; boundaries are closure gates, not calendar phases).

[kernel refactor follow-on package-complete-with-hold] complete `lib_mod/kernel.rs` decomposition
from `kernel_core.rs` into bounded modules before any bounded surface migration.
 WBVAL02 and
WBVAL03 are Defect-Closure ExecPlan unblockers created from WBVAL01 evidence;
they are bounded defect closures, not a return to diagnostic relay packages.
WBVAL04 is the right-sized post-climate-fix redo of WBVAL01, gated first by a
publication-safe Daymet CLI audit:

1. **WBVAL01** *(executed-hold)* — validation/characterization of single-OFE
   water-balance **conservation closure** on a real CLIGEN daily (non-breakpoint)
   Rocky Mountain run (`/wc1/runs/in/indispensable-presenter`, DRIGGS ID).
   Execution discovered `22` single-OFE hillslopes plus `pw0` as a 9-OFE
   observed-only surface. `12/22` single-OFE hillslopes emitted complete WAT
   ledgers and all `12` are `conservation-break` for years `2..6`; `10/22`
   failed closed before WAT publication (`CLIM-RUNTIME-E-017` or
   `HKERNEL-WB11-PERC-E-003`). This grounds frost targets for emitted ledgers
   while preserving a required follow-on unblocker for the domain-guarded
   hillslopes and the missing year-1 initial-storage surface.
2. **WBVAL02-SIMIMPL28-RADBOUND** *(complete: validated invalid upstream input)* — closed defect
   `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND` for the six WBVAL01 radiation-bound
   fail-closed single-OFE hillslopes (`p2`, `p4`, `p6`, `p9`, `p14`, `p17`).
   The shared DRIGGS daily climate record is invalid at the active SIMIMPL28
   source seam: on `1990-02-18`, `radly=486 Ly d^-1` exceeds baseline `sunmap`
   horizontal potential `r3=453.068716 Ly d^-1`. WBVAL02 amended
   `SC-CLIMATE-001`, added contract tests, and moved the fail-closed evidence
   to typed source symbol `radly`; no radiation guard was loosened and no
   snow/percolation compensation was authorized.
3. **WBVAL03-SNOWMELT-WB-CLOSURE** *(executed-hold)* — close the four
   WBVAL01 J-95 `HKERNEL-WB11-PERC-E-003` fail-closed hillslopes (`p7`, `p11`,
   `p18`, `p20`) and attribute the emitted-ledger conservation residual using a
   complete water-balance identity. Authority/write-set is
   snowmelt/storage/percolation/WAT publication. The closure leak is
   diagnostic-first only inside the package; it is not a diagnostic-only
   package. Current execution is legitimately held behind the upstream DRIGGS
   `radly` source-bound defect (`WBVAL04`): after WBVAL02, all four J-95
   targets and all 12 prior WAT-emitting hillslopes fail earlier at
   `CLIM-RUNTIME-E-017`, `radly=486`.
4. **WBVAL04-WBVAL01-REDO** *(executed-hold)* — reran the whole WBVAL01 Rocky
   Mountain single-OFE validation population after the observed-Daymet producer
   emitted CLI-safe radiation. The climate precondition now passes with zero
   `rad > baseline sunmap.r3` rows. The release validation batch ran all `22`
   single-OFE hillslopes: `18` emitted WAT and all `18` are
   conservation-break for years `2..6`; `p7`, `p11`, `p18`, and `p20` still
   fail closed at J-95 with `HKERNEL-WB11-PERC-E-003`. WBVAL04 routes two
   defect-shaped follow-ons: `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
   `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
5. **WBVAL05-J95-PERCOLATION** *(executed, hold-boundary)* — landed a
   contract-first WB18 fix (`SC-PERC-001` v29: WB18 consumes a published
   `wb12_infiltration` instead of recomputing the WB14/WB12 snow-liquid partition
   and re-validating snow state it does not own); no guard loosening. This cleared
   `HKERNEL-WB11-PERC-E-003` but relocated the fail-closed to
   `HKERNEL-WB14-RUNOFF-E-003`, exposing the true root cause: **negative
   `snow.runtime_swe = -0.006171`**. Legitimately held at the snow boundary; its
   negative-SWE follow-on is folded into SNOWSCI Stage 1.
6. **SNOWSCI Stage 1 — snow mass conservation / single-sourcing**
   *(closed-with-follow-up-postreview)* — closed
   `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` for the observed J-95 negative-SWE
   fail-closed mechanism by single-sourcing routed snowpack melt to the
   authoritative post-hourly depth/density store. The fix removed the WBVAL05
   publication blocker for `p7`, `p11`, `p18`, and `p20` without a snow
   physics-magnitude change or silent clamp. Post-review gates ran
   `cargo test --workspace`, workspace clippy, `cargo deny check`, fresh
   H1..H39 release/semantic validation, and WBVAL06 before/after residual
   measurement. WBVAL06 annual residual attribution was closed by
   `20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/`.
   Package:
   `20260606-snowsci-stage1-snow-mass-conservation-closure-001/`.
6a. **totalwatsed3 interception-flux audit companion** *(DONE — wepppy
   `aeef2cc6c`)* — WBVAL06 published the daily interception flux as
   `H.wat.Interception`, but openWEPP closure was shown only under its own
   identity audit. The acceptance surface is the **totalwatsed3** WB audit, which
   closes `P - (Runoff + Lateral + ET + Percolation) - ΔStorage`. This WP added
   `Interception` as an optional first-class outflow in
   `wepppy/wepp/interchange/totalwatsed3.py` and
   `tools/totalwatsed3_daily_closure_audit.py` (default 0 when absent, so legacy
   runs close unchanged; **`ET` untouched**). On openWEPP post-WBVAL06 output the
   totalwatsed3 closure identity now closes to ~`2e-7 mm/yr` for years `2..6`
   (vs ~15-19 mm without interception). WP:
   `wepppy/docs/work-packages/20260607_totalwatsed3_interception_flux_closure/`.
   Note: acceptance used a WAT-aggregated totalwatsed3-like surface; a full
   end-to-end totalwatsed3 run awaits openWEPP watershed outputs (MOFE rung).

   **RUNG-1 (single-OFE water-balance closure) is COMPLETE:** SNOWSCI-S1 (snow
   conservation) + WBVAL06 (interception publication) + 6a (totalwatsed3 audit
   consumes interception) → single-OFE WB closes and is auditable on the real
   surface. Next rung: **frost** (item 7).
7. **frost** *(rung-2 — FROSTVAL01 complete after follow-ons)* —
   infiltration/percolation gate (`ksflag`/`ksatadj`) on the closed single-OFE
   vertical balance, with no routing to alias it. **FROSTVAL01** originally ran
   the standard-WEPP `ksflag = 1` frost validation on
   `/wc1/runs/al/algebraic-radium` (43 single-OFE; all lanuse=1→ksflag=1; gridmet
   daily; comparator `wepp_260606`) and held. Findings from that first run (per
   Claude review): 37/43 blocked by `HS-RUNTIME-E-062` (soil-coverage); the
   frost-closure ledger was broken (its ~10 mm inputs were a tool-aggregation bug
   — openWEPP WAT `P` was verified correct/complete at 911 mm/yr — so the
   `frost-break` verdict was withdrawn); and openWEPP's own output showed real
   zero-term anomalies on the runnable cohort (`Q`/`Ep`/`Er`/`Interception` = 0)
   plus likely frost non-activation (`frozwt`=0 at a freezing site with real
   water). The ordered follow-ons closed those blockers: **FQ-1** soil-coverage
   unblock; **FQ-2** ledger fix folded into FQ-4; **FQ-3** ET/runoff zero-term
   characterization/closures; **FQ-4** frost-activation closure. A 2026-06-11
   FROSTVAL01 rerun over all 43 single-OFE prefixes now satisfies activation and
   closure-under-frost: `43/43` `frsoil.active=true`, `43/43` nonzero `frozwt`,
   paired frost-off runs change hydrology on all prefixes, and annual closure
   over 258 rows has max abs residual `3.22e-11 mm`. This exercises the standard
   `ksflag` gate, not the forest `ksatadj` model (separate concern). Package:
   `20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/`.
7a. **FQ-1 soil corrected-layer coverage** *(executed-hold-boundary)* — closed
   the population-scale `HS-RUNTIME-E-062` soil coverage blocker from
   FROSTVAL01. `SC-SOIL-001` v23 now requires valid parser-layer corrected
   diagnostics to extend the deepest normalized corrected interval to parser
   profile bottom while preserving normalized WB11/WB18/WB19 seed-grid authority.
   Post-fix algebraic-radium validation has zero `HS-RUNTIME-E-062` failures:
   `42/43` prefixes emit `H.wat.parquet` + `H.hbp`; `p11` now fails later at the
   protected percolation boundary with `HKERNEL-WB11-PERC-E-003` on `1990-162`.
   Handoff: `FQ1-P11-HKERNEL-WB11-PERC-E-003-J162`. Package:
   `20260608-fq1-soil-corrected-layer-coverage-closure-001/`.
7b. **FQ-3 runoff `Q/QOFE` underproduction** *(complete)* — closed
   `FQ3-DC-RUNOFFPART-QQOFE-001` for the post-FQ1 algebraic-radium single-OFE
   population. `SC-RUNOFFPART-001` v39 now requires WB12/WB14 to apply the
   top-two-layer storage limit before same-pass infiltration publication and to
   consume the WB18/percolation-produced infiltration value when it already
   owns the same-pass storage update. Post-fix validation produced nonzero
   `Q/QOFE` on all `42/42` runnable prefixes while preserving annual WAT closure
   at numerical noise (`max_abs=2.81e-11 mm`). Package:
   `20260608-fq3dc-runoffpart-q-qofe-closure-001/`.
7c. **FQ-3 Corn annual ET/canopy engagement** *(complete)* — closed
   `FQ3-DC-ET-CORN-ENGAGEMENT-001` for the post-FQ1 algebraic-radium Corn
   population. The annual PL activation sentinel was being deleted on pre-plant
   days and the scheduler calendar `day` symbol was day-of-month instead of
   Julian day, so annual Corn never reached its `jdplt` activation path.
   `SC-PLANT-001` v18 and `SC-EVAP-001` v26 now require annual pre-plant skips
   to be day-local and preserve PL schedule sentinels. Validation over all
   `36/36` Corn prefixes produced nonzero `Ep` and `Interception` with annual
   closure at numerical noise (`max_abs=3.16e-11 mm`). Upstream FQ-3 evidence
   classified `Er=0` as expected-config-zero (`legacy=0`), so this package
   closes the Corn engagement defect for `Ep`/canopy interception and records
   the original `Er` wording as an overclaim, not an unresolved defect. Package:
   `20260608-fq3dc-et-corn-engagement-closure-001/`.
7d. **FQ-4 ksflag frost activation + closure** *(complete)* — closed
   `FQ4-FROST-KSFLAG-ACTIVATION-001`. The root cause was an overbroad activation
   gate: openWEPP treated `frost.options.frost_file_present=0` as disabling
   frozen-soil coupling even when parsed missing-file defaults supplied valid
   standard frost controls with `wintRed=1`. `SC-SNOWFREEZE-001` v53 now makes
   frost file presence provenance-only for activation; `wintRed=1` plus active
   thermal/runtime triggers activates `frsoil`. Post-fix validation ran all `43`
   single-OFE prefixes: all emitted WAT, all had `frsoil.active=true`, all had
   nonzero `frozwt`, and annual closure with `SoilWaterTotal` held at numerical
   noise (`max_abs=3.22e-11 mm`). The old FROSTVAL01 `frost-break` verdict is
   withdrawn as a defective ledger artifact. Package:
   `20260608-fq4-ksflag-frost-activation-closure-001/`.
7e. **FDMC01 frost-depth comparator characterization** *(complete)* — sized the
   frost depth-model gap left open by FQ-4: openWEPP's freeze-index proxy
   (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m) vs legacy heat-flow.
   Verdict **materially off** — depth capped 200 mm vs legacy 240–503 mm
   (43/43 exceed the cap), depth-series median correlation 0.13, frozen
   duration +258 days (ratchet over-persistence). This verdict + the
   settle-vertical-before-routing principle promoted frost-depth heat-flow
   parity to ROADMAP queue item 1 ahead of MOFE (2026-06-07). Package:
   `20260608-fdmc01-frost-depth-comparator-characterization-001/`.
7f. **FDHP01 frost-depth heat-flow parity** *(complete)* — replaced the
   freeze-index proxy with the single-OFE fine-sublayer heat-flow frost state
   machine (`INV-SNOWFREEZE-006`/`-012`, legacy `frostn` lineage, CRM Ch. 3.8,
   Dun et al. 2010), added WAT `frdp` publication, restored WAT
   `SoilWaterTotal` as the unfrozen `Total-Soil` alias, and bound WAT `frozwt`
   to the layered `Σ soilf(i)` store. The D3 staged arc landed daily
   `frwatc` handoffs, fine-layer freeze/thaw arms, capacity/overflow
   ownership, in-hour resistance feedback, seasonal lower-front heat,
   residue/shallow-front resistance, fixed frozen-path conductivity authority,
   and legacy `hr_tmp`/`tmpadj` surface-temperature synthesis. Dk certified the
   package at the declared ADR-0017 boundary: the Dj/Dk forced-snow cohort is
   `43/43` clean, years 2-6 independent `Total-Soil + frozwt` closure is
   `5.09e-7 mm`, profile-bound pinning is gone (`0/43`), mean/median max depth
   are `501.36/492.36 mm`, median depth correlation rose from the FDMC01
   `0.13` baseline to `0.764`, and frozen-duration residual collapsed from
   `+258` to `+61` days. Residual items are handoffs, not blockers: F4 snow
   density/depth-split magnitude, `p2` individual attribution, dynamic
   residue/decomposition `resdep` lifecycle exposure, and characterized
   upper-envelope subgroup deltas. `SC-SNOWFREEZE-001` v69 closes/re-states
   `GAP-SNOWFREEZE-002`; MOFE is now the next ROADMAP item. Package:
   `20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`.
8. **MOFE** *(rung 3 — MOFE01 hillslope water-routing closure complete)* —
   closed inter-OFE run-on/run-off routing on the frost-settled per-element
   balance using the `/wc1/runs/ar/arboreal-dendrite/wepp` graded 1–5-OFE
   ladder. M-H ran all 36 hillslopes with fresh openWEPP outputs: 36/36 exited
   zero, row cardinality matched exactly (`271808/271808` rows), transfer
   residual max was `0.0 mm`, per-element residual max was
   `5.968558980384842e-13 mm`, aggregate cancellation residual max was
   `0.0 mm`, downstream `QOFE == Q` alias rows were zero, hydrology clone
   active days were zero, and the 7 single-OFE anchors were 28/28
   byte-identical to the M-F-REDO2 anchor. M-I added the independent in-runner
   hillslope-total identity and closed it at `3.306423012547295e-13 mm`
   against `1e-9 mm`, with all multi-OFE cases nonzero-at-noise; it also
   source-guards the mutually exclusive multi-OFE persistent and single-OFE
   aggregate scheduler lifecycles. Local `owcmp` was run directly without the
   comparator subagent: row keys align for all 36 hillslopes, while semantic
   value-family comparison remains an ADR-0017 investigation signal, not an
   acceptance target. M-G deliberately left sediment-coupled erosion `qin/qout`
   plus particle-fraction handoff as a named follow-on. Package:
   `20260612-mofe01-inter-ofe-routing-closure-001/`. **Closure (2026-06-14):** MOFE01 water-routing closure is done-done on the 36-run 1–5-OFE ladder. Named follow-ons: `MOFE-FARPOINT01` (>10-OFE exceed-the-ceiling demonstration), `MOFE-MAGPARITY01` (completed 2026-06-18; no transfer/area/export defect, Stage-2 lateral/subsurface magnitude flag), `REFACTOR022` (line-count split), plus watershed/totalwatsed3 (queue item 1) and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment coupling).

   **Next rung — WSHED01 (openWEPP-native totalwatsed3 CLI + closure)** *(complete 2026-06-14)*: closed the end-to-end totalwatsed3 water-balance audit on openWEPP-native output (the WBVAL06/6a deferral). See item 9 below for the W-arc→T-arc pivot (ADR-0019/0020), the three-iteration runvol fix, and the closure evidence. Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
9. **openWEPP-native totalwatsed3 CLI + closure** *(WSHED01 complete
   2026-06-14 — the WBVAL06/6a deferral resolved)* — consume the closed MOFE
   hillslope outputs and close the end-to-end `totalwatsed3` water-balance audit
   on **openWEPP-native** output. The package began as a watershed-CLI route
   (W-A→W-D): W-A/W-B/W-C cleared the `jpond=0` no-impoundment parser defect
   (`IMP-E-004`/`CLIWAT-E-010`, the `IMP-E-007` count-mismatch split) and the
   over-strict WS10 zero-sediment/`nchnum=0` channel guards, reaching watershed
   output; W-D's audit then exposed that `totalwatsed3` is **hillslope-only**
   (no channel terms) and that the producer was filling `runvol` from WAT `Q`
   (a self-consistency check, not conservation). **Pivot (operator-directed):**
   two ADRs — [ADR-0019](../decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)
   (openWEPP owns its output surface; `wepppyo3 wepp_interchange` frozen
   legacy-only) and [ADR-0020](../decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md)
   (totalwatsed3 is a dedicated `openwepp-cli-totalwatsed3`, an
   output-aggregation tier separate from the simulation binaries) — redirected
   the close to a native T-arc (T-A scope → T-B CLI → T-B2 native PASS `runvol`
   → T-C closure). The native `runvol` is sourced from the MOFE outlet-OFE
   routed runoff (the same surface the M-I hillslope-total identity closes on),
   genuinely independent of WAT `Q`. **Runvol took three iterations** —
   `QOFE·A_hillslope` over-scaled 2.5× (runoff > precip; caught by the closure),
   `Q·A_outlet` under-scaled ~4× (a crossed pairing that passed the one-sided
   `≤precip` bound and a self-restating test; caught by reconstructing the
   export from independent operands), and finally **`QOFE_outlet·A_outlet`**
   (≡ `Q·A_hillslope`). **Closure (`openwepp-cli-totalwatsed3` on the native
   arboreal-dendrite PASS/WAT):** `Σ runvol = 27.691 Mm³` (coeff 0.554), runoff
   < precip every year, independent of the WAT-`Q` column (18.895 Mm³); the
   `P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` identity
   closes ex-day-1 at `−0.41 mm` over 2191 days with nonzero-at-noise daily
   residuals `[−0.248, +0.005] mm` (day-1 `+30.95 mm` is the storage-prepend
   init, present for any correct producer). Anchors byte-identical
   (`anchor_mismatches=0`); MOFE physics untouched (output-surface-only). The
   forensic record of the runvol arc is `artifacts/review-tb2-runvol-area-defect.md`;
   the durable geometry fact is agent memory `reference-qofe-q-area-duality`.
   Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
   **Named follow-ons:** `WATERSHED-CHANWB-ROUTED-OUTPUT` (the decoupled
   channel-routing / `chanwb`/`chnwb` watershed output, distinct from the
   hillslope-only totalwatsed3 per ADR-0020 — the W-B/W-C watershed-CLI work
   landed there but channel water-balance routed output remains its own rung)
   and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment-coupled routing).
10. **snow physics-magnitude (Stage 2, deferred)** — the `snowd.for`
   melt/settling/density/partition equation adjudication against external authority
   (CRM Ch. 3.7, WEPP User Doc), behind the protected boundary. Distinct from snow
   *conservation* (Stage 1, item 6, done now); judged last against a fully closed,
   routed balance.

11. **MOFE >10-OFE far-point demonstration** *(FARPOINT01 complete 2026-06-16)* —
   `20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/`.
   Demonstrated openWEPP's three-identity conservation closure on the **H2637
   19-OFE** substrate (in-repo wepp-forest provenance; legacy comparator
   `wepp_260606`), past the legacy ≤10-OFE ceiling.
   - **F-A** staged the fixture + clean legacy baseline (both `wepp_ui` variants);
     the openWEPP run **surfaced** a per-element WB13 fail-closed at OFE5 on a
     frost day (residual ≡ `watbtm`).
   - **F-B** *(Defect-Closure ExecPlan)* closed it **contract-first**: the frost
     bottom-overflow `watbtm` was double-counted (inflow frost adjustment **and**
     `Dp` outflow). SC-WATBAL-001 v161→v162 + `per_ofe_internal_wb13.rs:432` fix +
     regression; all four AGENTS gates green; H2637 both variants then run to
     completion (235,961 wat rows × 19 OFEs × 34 yr, exit 0). Commits
     `41469058`, `a724e2ae`.
   - **F-C** contrasted closure: legacy `wepp_ui` outlet runoff = **127.7 % of
     precip** (runoff > precip — the WB-05A q-cap, quantified) vs openWEPP **71 %**
     (bounded, `wepp_ui`-invariant, conservation-closed). Comparator a flag
     (ADR-0017); the 71 % vs 55.5 % magnitude gap → `MOFE-MAGPARITY01`.
   - **`watpdg`** branch-out **resolved**: instrumented detection found `watpdg>0`
     on 4 OFE-days with the gates still closing → it cancels on both sides →
     validated non-defect (no change). Commit `877ff25f`.
   Follow-ons: Stage-2 lateral/subsurface magnitude (`MAGPARITY01` completed the
   no-transfer-defect adjudication), `PERFHO01` (the ~80–110× high-OFE wall-clock
   gap — characterized, item 12), `WATERSHED-CHANWB-ROUTED-OUTPUT`,
   `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF`.

12. **High-OFE hillslope performance characterization** *(PERFHO01 complete
   2026-06-16, Codex-executed)* —
   `20260616-perf-high-ofe-hillslope-characterization-001/`. Attributed openWEPP's
   ~80–110× single-hillslope wall-clock gap vs legacy on H2637 (`978.55 s` vs
   ~10 s). CPU-bound (`977.99/978.55` user s) — **not** I/O/parquet; OFE-count
   scaling roughly linear-to-modestly-superlinear (`b≈1.12`), i.e. a large
   constant per-OFE-day cost. GDB-sampled dominant cost (perf blocked by
   `perf_event_paranoid`): per-OFE-day symbol-keyed `BTreeMap` runtime-surface
   churn + success-path writeback validation (`11/15` samples); the scaffold's
   WB13-string lead was tested and found **not** dominant. Verdict: not acceptable
   as-is → follow-on `PERFOPT01` (bit-identical, determinism-preserving;
   ~1.5–2.5× expected, 3.75× Amdahl cap — first step, not full closure). No
   production/contract edit. Claude review: sound and honest (15-sample limit +
   residual-gap caveats disclosed).

13. **Runtime-surface map-churn optimization** *(PERFOPT01 complete 2026-06-16,
   Codex-executed + Claude-reviewed)* —
   `20260616-perfopt01-runtime-surface-map-churn-001/`. Behavior-preserving
   optimization of the PERFHO01-named hot path: removed the per-OFE-day
   report-to-persistent-state + climate-overlay runtime-surface clones (move/extend
   not clone) and made kernel-writeback validation detail lazy. **~1.15×** on H2637
   (`978.55→849.86 s`; 10–18 % on the 1–5-OFE ladder), **bit-identical**
   (`anchor_mismatches = 0` across 7 fixtures — HBP byte + parquet table equality),
   determinism-preserving, all four gates green. Independent Claude review proved
   the lazy fast-path **exactly equivalent** to the original validation (inclusive
   bounds match `check_min`/`check_max`/`check_range`; inverted-bounds delegated,
   not suppressed) and re-confirmed bit-identity against a **separate** pre-opt
   baseline — resolving the no-independent-dual-review caveat Codex flagged.
   Residual → `PERFHO02` (now characterized). No contract/physics/output change.

14. **Post-PERFOPT high-OFE performance characterization** *(PERFHO02 complete
   2026-06-16, Codex-executed)* —
   `20260616-perfho02-post-perfopt-characterization-001/`. Characterized the
   post-PERFOPT01 H2637 residual with a 20-sample GDB window, then supplemented
   it with `perf record` after `kernel.perf_event_paranoid=0` became visible in
   the session. Dominant sampled cost:
   hydrology typed-symbol lookup, dynamic symbol formatting, frost/decomposition
   and PL guard work (`13/20`, 65 %). Secondary residual:
   `apply_kernel_writeback` sorting/allocation/insertion (`4/20`, 20 %).
   Scheduler/daily-loop insertion/allocation plus consumer-boundary validation
   accounted for the rest; `perf record` confirmed `execute_persistent_scheduler_kernel_lifecycle`
   at `96.24 %` children and `apply_kernel_writeback` at `12.46 %`; output
   writers were again absent. Follow-on:
   `PERFOPT02-symbol-access-and-writeback-application`. No production/contract
   edit.

15. **Indexed runtime-surface architecture design** *(PERFARCH01 complete
   2026-06-16, Codex-executed)* —
   `20260616-perfarch01-indexed-runtime-surface-design-001/`. Designed the
   architectural replacement for the string-keyed runtime surface: a frozen
   run-scoped `SymbolRegistry`, sorted-order `SymbolId`, and dense indexed
   state/flux storage while preserving the logical `BoundarySymbol` seam. A
   standalone prototype over 6,396 symbols measured 109.85× faster dense clone,
   219.16× faster pre-resolved lookup, and 115.77× faster update batches versus
   the modeled `BTreeMap<String, f64>` pattern; sorted id order matched string
   sort. Feasibility verdict: <=10× is plausible if implementation migrates
   roughly 89-90 % of elapsed time out of string-keyed surface mechanics; <=5×
   needs roughly 95-96 % and is not a storage-only promise. Proposed ADR:
   `docs/decisions/0022-indexed-runtime-surface-representation.md`. Follow-on:
   `PERFIDX01-run-scoped-symbol-registry-001`. No production/contract edit.

16. **Run-scoped symbol registry** *(PERFIDX01 complete 2026-06-16,
   Codex-executed)* —
   `20260616-perfidx01-run-scoped-symbol-registry-001/`. Implemented ADR-0022
   Stage 1: `SymbolId`, frozen sorted `SymbolRegistry`, BTreeMap export adapter,
   and an env-gated no-lazy-interning audit path. Completeness passed on H2637
   both UI variants plus OFE1-5 (`unknown_symbol_count = 0`); bit identity and
   determinism passed (`ANCHOR_MISMATCHES=0`, `DETERMINISM_MISMATCHES=0`).
   Runtime storage authority remains the existing BTreeMap surface. Follow-on:
   `PERFIDX02-indexed-shadow-runtime-surface-001`.

17. **Indexed shadow runtime surface** *(PERFIDX02 complete 2026-06-16,
   Codex-executed)* —
   `20260616-perfidx02-indexed-shadow-runtime-surface-001/`. Implemented
   ADR-0022 Stage 2: a sparse sorted `Vec<(SymbolId, BoundaryValue)>` shadow
   surface and an env-gated shadow report hook, while keeping BTreeMap storage
   authoritative. The tightened H2637 registry is 44,746 symbols, with 0
   unknown symbols on H2637 both UI variants plus OFE1-OFE5. H2637 sparse clone
   speedup measured 69.882x without UI and 54.096x with UI; shadow equality,
   bit identity, determinism, and full cargo gates passed. Follow-on:
   `PERFIDX03-indexed-surface-authority-001`.

Acceptance target at each rung is **closure** (does it conserve), not **magnitude**
(is the forcing physically right) and not comparator-match. See memory
`project-work-sequencing-wb-frost-mofe-snow` for the rationale and the two
ladder invariants (single-before-MOFE hard dependency; frost is per-column so
single-OFE fully settles it).

## Series index

Per-package execution logs are split by work-package series (newest first within
each). The narrative above is the live cross-cutting state; the docs below are the
archival per-package detail.

| Series | Head package | State | Log |
|---|---|---|---|
| HPHYS | `hphys0320` (2026-06-06) | snow/`RM` comparator arc **retired** per ADR-0017 — do not continue | [series/hphys.md](series/hphys.md) |
| WBVAL | `wbval06` (2026-06-06) | rung-1 single-OFE WB closure **complete** | [series/wbval.md](series/wbval.md) |
| SNOWSCI | `snowsci-stage1` (2026-06-06) | Stage 1 (conservation) **closed**; Stage 2 (magnitude) deferred | [series/snowsci.md](series/snowsci.md) |
| Governance / ADR | `adr0017` (2026-06-05) | comparator-distrust ratified | [series/governance.md](series/governance.md) |

**Frost (FROSTVAL / FQ / FDMC / FDHP):** the recent rung-2 frost packages are logged
inline in the active-work-sequence narrative above (items 7, 7a–7f), not in a
separate series doc.

**Other / historical series** (`auth`, `soilauth`, `infile`, `inspec`, `sci`,
`simimpl`, `wshedimpl`, `inimpl`, `arch`, `pl`, `clim`, `erod`, `wb`, `mofe`,
`refactor`, …): these predate this curated log or were never carried in it. Their
detail lives in each package's dated directory (`package.md` + `artifacts/`). They
are not summarized here; the canonical forward queue is
[../ROADMAP.md](../ROADMAP.md).

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

## Autonomous execution intent (required)
- A work package is an execution-ready plan, not a lightweight task note.
- Planning must be front-loaded into the package so execution can proceed
  autonomously from kickoff through disposition without user intervention.
- `package.md` and kickoff prompts must define concrete sequencing, explicit
  file targets, gate commands, and expected evidence updates.
- Kickoff prompts must include an explicit `Autonomy:` line requiring
  end-to-end execution for the declared scope without additional user
  intervention unless hard-blocked.
- Kickoff prompts default to `Execution mode: package-end-to-end` and should
  direct execution across all package phases through disposition.
- Single-phase kickoff prompts are exception-only and must declare
  `Execution mode: phase-only (exception)` plus explicit rationale and
  follow-on trigger.
- Kickoff prompts must include a `Required reading` list with explicit path
  references to orientation and authority documents so agents do not need to
  independently search onboarding context.
- Kickoff prompts must tier required-reading as `Core`, `Conditional`, and
  `On-demand` to preserve authority while minimizing unnecessary pre-read load.
- `Core` should remain small and stable (global governance + package-local
  authority). Put large mechanism-specific authorities in `On-demand` unless
  package scope requires them before edits.
- Each package should include `artifacts/required-reading-map.md` documenting:
  path, tier, rationale, applicability trigger, and when it was read.
- Kickoff prompts should record required-reading budget metrics for local-repo
  files, using canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.
- When `REQUIRES-JUSTIFICATION` is reached, author must explain why each heavy
  pre-read is mandatory and cannot be deferred to `On-demand`.
- Work-package authoring must reference and follow:
  `docs/codex_exec_plans.md`.
- Mechanical refactor packages should additionally follow:
  `docs/standards/mechanical-refactor-authoring-guide.md`.

## Dual review and disposition (required)

- Every work package must include two independent review artifacts:
  `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.
- Every review finding must be dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up` before package closure.
- Accepted findings must be fixed and verified; rejected findings must include
  rationale; deferred/follow-up findings must be linked from
  `artifacts/disposition.md` and `artifacts/worker-handoff.md`.
- Dual verification artifacts must verify both technical gates and that no
  review findings remain undispositioned.

## Phase shape (inherited from wepp-palimpsest)
- **Phase 0**: docs-only audit / inventory
- **Phase 1**: architecture decision with operator-signed acceptance
- **Phase 2**: single-mechanism implementation, replay-and-checkpoint between mechanisms
- **Phase 3**: closeout disposition

## Conventions
- Dates are UTC.
- Evidence classification per claim: `[DIRECT]` (read source / contract / output) vs `[INFERENCE]` (reasoned from evidence).
- Evidence mode per assessment: **Static** (read and reasoned) vs **Ran** (commands actually invoked).
- Single-mechanism rule: one landed change per replay checkpoint.
- Correctness over completion: unresolved contract/invariant correctness gaps keep package disposition in `HOLD` until explicitly resolved or risk-accepted.
- Kernel-affecting packages (including runtime projection controlling kernel branches) must list:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  as dependencies, and must include a kernel-profile compliance checklist artifact.
- Code-authoring work packages should use contract-first sequencing when applicable:
  1. implement/ratify canonical contract amendments,
  2. implement contract-derived tests,
  3. record a pre-implementation contract gate, then
  4. modify production code.
- `package.md` dependencies for authored packages should include:
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
- Missing kernel-profile/procedure compliance keeps disposition in `HOLD`.

## Active / Recent Snow-Density Packages

- `20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/` —
  executed non-promotion for the Stage A opt-in open-surface sublimation /
  latent mass-loss candidate; cap-limited tail improved `30 -> 27`, but
  under-persistence worsened `54 -> 57`.
- `20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/` —
  executed non-promotion for the opt-in `physics_bulk_shallow_guard_v1`
  shallow-pack density guard. The candidate reached the real direct-production
  WAT path but did not meet gates: induced under-persistence improved only
  `177 -> 176`, `harvard_hardwood` recovered `0` induced-under rows
  (`73 -> 73`), over-persistence worsened `264 -> 267`, total snow-control
  failures worsened `498 -> 500`, and downstream mass terms changed despite
  local SWE-depth-density identity closure. No default activation, density cap,
  fixture/schema, user-surface, rollback, or frost-attribution change was made.
