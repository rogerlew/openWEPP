# Gate Results

Evidence class: `Ran` only where explicitly recorded.

## Intake

- PASS: exact HEAD `af9a989063aa8751dfadb14c442e1b360653658c`.
- PASS: clean `main`, synchronized with `origin/main` at intake.
- PASS: instruction discovery completed before edits.

Further results append below; failed attempts are retained.

## Contract-first attempt 1

- FAIL: `cargo nextest run --test surface_liquid_hydrology_custody_authority_contract --profile quick` — 1/4 passed; three lexical bindings failed on Markdown wrapping or missing explicit `byte-identical rollback` wording. No production code had been edited. Canonical rollback wording and wrap-insensitive assertions were corrected without changing the admitted algorithm.
- FAIL: `cargo fmt --all -- --check` — new test required standard formatting.
- PASS: `check_sc_unit_compliance.sh` for `SC-WATBAL-001.md`.
- PASS: package Markdown lint, 8/8.

## Contract-first attempt 2

- FAIL: focused authority test, 2/4 passed; two remaining assertions depended on line-wrapped phrases.
- PASS: preserved LSE authority suite, 7/7.
- PASS: formatting and diff hygiene.
- FAIL: science-admission gate correctly rejected changing draft `SC-WATBAL-001`. The custody authority was moved into dedicated `SC-SURFACELIQUID-001` and WATBAL restored byte-for-byte; no production code had been edited.

## Contract-first attempt 3

- FAIL: focused authority test, 1/4 passed; the new dedicated contract used indexed symbols and wrapped ingress prose while three assertions still expected the discarded WATBAL wording. Assertions were rebound to the exact canonical symbols without weakening their semantic obligations.

## Contract-first attempt 4

- FAIL: focused authority test, 2/4 passed; the last two assertions expected unindexed shorthand rather than the canonical indexed state equation and prose enum declaration. They were corrected to the exact contract notation.

## Contract-first attempt 5

- FAIL: focused authority test, 3/4 passed; one digest assertion crossed a Markdown wrap. It now binds the exact excluded field name rather than layout.

## Contract-first attempt 6

- PASS: focused authority test, 4/4.
- PASS: preserved LSE authority suite, 7/7.
- FAIL: unit-governance lint found the new tables used semantically equivalent but noncanonical header names. Headers and alias-map unit columns were corrected; no authority changed.
- PASS: formatting, diff hygiene and package Markdown lint.

## Contract-first attempt 7

- FAIL: the expanded independent-vector authority target ran 1/6 before five
  failures. Four were wrap-sensitive contract-string assertions; one used a
  one-epsilon bound for a multi-operation independent attribution sum. The
  numerical expected values were unchanged, the assertion was corrected to an
  eight-epsilon representation bound, and no production code had been edited.
- PASS: `SC-SURFACELIQUID-001` unit-compliance lint.
- FAIL: formatting check identified only standard Rust layout changes in the
  expanded authority test; `cargo fmt` was applied before the retry.

## Contract-first attempt 8

- PASS: expanded authority target, 6/6, including independent D/A/F,
  condensation, attribution, retention, enthalpy, identity-digest, and actual
  WB14 source-binding checks.
- FAIL: the preserved LSE authority invocation used the wrong target name
  (`land_surface_energy_authority_contract`). Cargo reported the registered
  target as `land_surface_energy_balance_authority_contract`; this was a
  command-name error, not a test failure.
- PASS: `SC-SURFACELIQUID-001` unit-compliance lint and diff hygiene.

## Contract-first attempt 9

- FAIL: expanded v3 authority target did not compile because four numerical
  poison operands lacked explicit `f64` suffixes. No production code ran.
- PASS: the machine-readable unit-registry gate passed 21/21 tests plus its
  workspace check after adding the nine custody seams.
- FAIL: SC unit compliance correctly required every new registry canonical
  symbol in Variables and Units and every exact alias in the Symbol Alias Map
  (18 findings). The missing table rows were added without changing equations.

## Contract-first attempt 10

- FAIL: v3 authority target ran 4/8; three assertions retained v2 wording and
  the actual WB14 vector infiltrated its complete 0.010 m supply because its
  storage cap was 0.030 m. The vector now uses a 0.005 m infiltration cap so
  both actual infiltration and excess are nonzero; no production arithmetic
  was changed.
- PASS: machine-readable unit-registry gate, 21/21 plus workspace check.
- FAIL: SC unit compliance found eight alias-map unit cells used abbreviated
  descriptions rather than exact registry unit labels. The exact labels were
  inserted without changing boundaries.

## Contract-first attempt 11

- FAIL: v3 authority target ran 7/8; one assertion retained the v2 phrase
  `No per-parcel Green-Ampt` while v3 binds the stronger stateful rule as
  `never once per source parcel`. The assertion was rebound to the canonical
  v3 wording.
- PASS: SC unit-compliance lint and diff hygiene.

## Contract review v3, round 1

- PASS (hydrology reviewer): no material findings after cadence, exact-one
  depression ownership, no-duplication, mixed enthalpy, and unequal-area
  amendments.
- HOLD (contract/profile reviewer): the continuation was absent from the
  composite restart schema; OFE-wide capacity allocation introduced
  unauthorized cross-tile redistribution; routed parcel basis was not re-keyed;
  and the invariant table overstated runtime continuation/parity evidence.
- Disposition: all four findings accepted. The schema now carries an exact
  continuation map; attributed excess retains exact tile/source custody;
  routed parcels carry and re-key `basis_ofe_id`; a 48-step independent cadence
  vector is present; and actual runtime parity remains explicitly pending the
  implementation gate.

## Pre-implementation admission attempt 1

- FAIL: the focused authority suite ran 8/9 because its lifecycle assertion
  still expected the former production-edit prohibition after the gate was
  truthfully promoted. It now asserts implementation authority plus retained
  production-dispatch prohibition.
- PASS: LSE authority 7/7, AUTH11 3/3, unit registry 21/21, SC unit compliance,
  anti-evasion, formatting, and diff hygiene.
- FAIL: science admission required current atomic impact-map bindings for the
  two changed unit-registry source files. Both files are now bound to
  `SC-SURFACELIQUID-001`; this was governance metadata, not a science change.
- FAIL: the attempted Markdown command referenced a nonexistent local script;
  it is not counted as Markdown evidence and will be replaced by the repository
  canonical documentation command discovered from package guidance.

## Pre-implementation admission attempt 2

- PASS: focused authority suite, 9/9.
- FAIL: science admission found the new impact-map entries used an A1 gate
  whose executable packages omitted the declared `openwepp` and
  `openwepp-sim-contract` targets. They now use the existing WAT5/runtime A1
  definition that executes both targets and the affected hydrology packages.
- FAIL: the first canonical `markdown-doc` invocation passed paths as positional
  arguments. The documented `--path` option is used for the retry.
- PASS: formatting and diff hygiene.

## Pre-implementation admission attempt 3

- PASS: science-contract admission from
  `af9a989063aa8751dfadb14c442e1b360653658c`; 46 admitted contracts and two
  changed science surfaces were reconciled.
- PASS: surface-liquid authority 9/9, preserved LSE authority 7/7, AUTH11 3/3,
  unit registry 21/21, SC unit compliance, anti-evasion, formatting, and diff
  hygiene.
- PASS: canonical Markdown lint for the 10-file package, contract, and unit
  registry (12 files total), with zero errors or warnings.
- PASS: final contract/profile and hydrology/ownership reviews reported no
  material finding. The pre-implementation gate is closed.

## Authority amendment v4 intake

- Static: implementation-schema preparation found the contract's required
  open-versus-covered branch lacked a strict configuration discriminator.
  Inferring it from caller-provided ingress would weaken exact-one water
  ownership. Contract v4 adds `ground_ingress_mode`, binds it into configuration
  digest bytes, and requires one matching variant per tile. Runtime edits remain
  paused pending focused amendment review/admission.

## Authority amendment v4 review

- PASS: focused authority suite 9/9, SC unit compliance, formatting, and diff
  hygiene.
- PASS: bounded contract/profile and hydrology/ownership amendment reviews;
  both reported no material finding.
- PASS: final v4 science admission from the frozen campaign base and SC unit
  compliance. Runtime implementation authority is restored.

## Runtime owner attempt 1

- PASS: affected orchestrator `cargo check` after the initial strict
  configuration/state and D/A/F implementation.
- WARN: the initial public re-export was unreachable from the crate root and
  all new owner surfaces were therefore dead code. The crate root now exports
  the default-off API; no production dispatch reference was added.
- FAIL: five of six focused tests assumed input record order even though the
  strict constructor canonically sorts by topology and complete key. Tests were
  corrected to select records by typed tile identity; implementation ordering
  was retained.

## Runtime owner attempt 2

- PASS: surface-liquid owner focused suite, 6/6.
- FAIL: strict orchestrator Clippy found two oversized implementation functions
  and one strict floating comparison in a test. Validation, routing, resource
  and condensation helpers were decomposed; the test uses a bounded scalar
  assertion. No broad Clippy allowance was added.
- PASS: strict orchestrator Clippy on the focused owner bytes after correction.

## LSE condensation receipt increment

- PASS: LSE quick suite, 27/27.
- PASS: strict LSE all-target Clippy.
- Static: accepted condensation now produces a typed hydrology credit only for
  an admitted surface/litter source. Amount, transaction, owner, OFE, tile,
  surface, temperature and canonical specific-liquid enthalpy are retained.
  A soil-layer condensation target remains rejected.

## Production WB14 continuation parity attempt 1

- PASS: 48-step cumulative infiltration was bit-identical to the unchanged
  daily WB14 wrapper and legacy depression retention was exact zero.
- FAIL: total excess compared bitwise differed by one ULP because the
  continuation summed 48 segment excesses while the daily result summed 24
  hourly bins. The retry uses the existing admitted scale-aware depth closure
  rule for this differently grouped diagnostic sum. Production Green-Ampt
  arithmetic, infiltration, state acceptance and conservation were unchanged.

## Complete implementation focused gate

- FAIL: science admission initially found that the changed orchestrator
  manifest lacked an atomic current-contract impact-map binding. The manifest
  and every dual-contract surface now have separate atomic bindings; no runtime
  or scientific equation changed.
- PASS: science admission from
  `af9a989063aa8751dfadb14c442e1b360653658c`, 46 contracts and 11 changed
  science surfaces.
- PASS: affected crate checks and strict all-target Clippy for the orchestrator
  and land-surface-energy crates.
- PASS: focused surface-liquid owner and ingress selection, 14/14.
- PASS: real-hydrology/LSE shadow integration, 8/8, including the actual public
  potential and fixed-cap LSE protocol.
- PASS: land-surface-energy crate quick suite, 27/27.
- PASS: complete orchestrator quick suite, 521/521. Three pre-existing
  OFE-routing oracle tests exceeded 60 seconds; all completed successfully.
- PASS: surface-liquid authority 9/9, LSE authority 7/7, AUTH11 3/3,
  anti-evasion, SC unit compliance, formatting and diff hygiene.
- FAIL: one preserved command used the nonexistent target name
  `land_surface_energy_authority_contract`. Cargo identified the registered
  target; the corrected `land_surface_energy_balance_authority_contract`
  invocation passed 7/7.
- PASS: canonical Markdown lint, 13 files with zero errors or warnings.
- PASS: after binding the complete soil-plus-surface beginning snapshot digest,
  the focused integration target remained 8/8 and affected check, strict
  Clippy, formatting, diff hygiene, and science admission remained green.

Earlier focused raw logs were retained outside the checkout under the short
`/tmp` directories reported by their command output. The later comparator logs
under `artifacts/gate-run-20260814-*` are retained package evidence, including
failed attempts.

## Implementation review round 1

- HOLD (hydrology/ownership): five material findings covering real infiltration
  recipients, independent closure, strict restart combinations, production
  identity binding, and canonical failure payloads.
- HOLD (Rust correctness): six material findings covering the complete receiver
  set, candidate sealing, canonical persistence, canonical snapshot bytes,
  line-count governance, and duplicated WB14 transition.
- Disposition: all eleven findings accepted; none rejected, deferred, or moved
  to follow-up.

## Review remediation focused gate

- PASS: surface owner/ingress/shared-WB14 unit selection, 25/25.
- PASS: surface-liquid authority contract, 9/9.
- PASS: real-LSE/real-hydrology integration contract, 13/13.
- PASS: advisory authority linter, 7/7; the earlier full-workspace failure from
  the missing `SC-SURFACELIQUID-001` expected authority is preserved in
  `artifacts/gate-run-20260814-2/04_nextest_workspace.log`.
- PASS: affected orchestrator strict all-target Clippy.
- PASS: complete orchestrator quick suite, 532/532 in 145.986 seconds; retained
  slow OFE-routing tests completed successfully.
- PASS: LSE quick suite, 27/27, and strict all-target Clippy.
- PASS: affected kernel-contract, LSE, and orchestrator checks.
- PASS: science admission from the frozen base, 46 contracts and 14 changed
  science surfaces.
- PASS: anti-evasion, AUTH11 3/3, SC unit compliance, formatting, diff hygiene,
  JSON impact-map parse, and package Markdown lint (18 files, zero findings).
- Static: `runoff.rs` is 2,852 lines; all remaining files are below 3,000 and
  WARN files are dispositioned in `line-count-governance.md`.

## Exact-byte re-review round 1

- HOLD (hydrology/ownership): two material findings. Actual receivers are now
  mutated correctly, but their ordered soil-mass and energy ending equations
  lack independent reconstruction; the E001--E011 schema is not yet emitted
  end to end with complete contextual rollback payloads.
- PASS evidence retained by reviewer: surface selection 25/25 and real-LSE
  integration 13/13. These tests were insufficient to detect the two findings.
- Disposition: both findings accepted for immediate in-package correction.
- HOLD (Rust correctness): three high and two medium findings. Two overlap the
  hydrology receiver/error findings; the additional defects are mutable
  arbitration, unchecked canonical state serialization, and mutable duplicated
  ingress/unified candidate representations.
- Disposition: all five Rust findings accepted. No finding is rejected,
  deferred, or moved to follow-up.

## Re-review round 1 correction gate

- PASS: surface owner/ingress/WB14 selection, 30/30.
- PASS: real-LSE/real-hydrology integration, 15/15.
- PASS: complete orchestrator quick suite, 537/537.
- PASS: affected check and strict all-target Clippy.
- PASS: science admission, 46 contracts and 14 science surfaces; authority
  anti-evasion; surface authority 9/9; AUTH11 3/3; SC unit compliance;
  formatting, diff hygiene, and Markdown lint (20 files, zero findings).
- Static: all five accepted re-review finding families have implemented poison
  coverage and no file exceeds 3,000 lines. Final exact-byte re-review remains
  required before closure.

## Final exact-byte re-review

- HOLD (hydrology/ownership): exact thermal-layer and rollback-owner identity
  remains incomplete; E004/E007/some E011 payloads omit known offending
  context. Ordered production-layer and energy equations are otherwise closed.
- HOLD (Rust correctness, early finding): the independent aggregate-soil check
  sums `theta_m` but omits production residual/frozen storage, rejecting valid
  nonzero-residual profiles hidden by the zero-residual fixture.
- Disposition: all three findings accepted for immediate bridge/test correction.

## Final exact-byte finding remediation at `26e34e024`

- PASS: real-LSE/real-hydrology integration, 16/16, including a nonzero
  residual-water production aggregate and independent poisons for extra,
  nonfinite, reordered, missing, duplicate and forged receiver identity.
- PASS: surface-liquid focused tests, 30/30, including typed owner context in
  canonical failure serialization.
- PASS: affected orchestrator check and strict all-target Clippy.
- PASS: formatting and diff hygiene.
- Static: the bridge accepts an independently frozen receiver-expectation
  object; the rollback envelope is exactly ordered LSE, hydrology and soil
  thermal, matching the owners this Child-3 bridge constructs.
- Pending: fresh exact-byte hydrology/ownership and Rust correctness review.

## Authorization-overflow release remediation at `93c46d3db`

- HOLD retained: Rust release review found that two finite same-store demands
  could overflow their aggregate and yield a non-proportional authorization.
- PASS: demand accumulation, supply multiplication, proportional numerator,
  division, remainder and allocated-total intermediates now fail closed as
  contextual E003 before any batch or candidate exists.
- PASS: surface-liquid focused tests, 32/32, including distinct-requester
  finite-overflow and adjacent large-finite proportional controls; affected
  strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte reviews.

## Frozen-domain release remediation at `0cb11eb12`

- HOLD retained: release review found that the public bridge rejected only
  selected snow state and admitted represented frost/thaw, frozen layers, and
  snow-retained-liquid-only state.
- PASS: E004 preflight now covers snow runtime/carry, retained snow liquid,
  frost runtime/carry, and any positive production-layer frozen depth/water,
  with exact hydrology owner and OFE/tile/source context.
- PASS: real-LSE/real-hydrology integration, 18/18; surface-liquid focused
  tests, 30/30; affected strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte hydrology/ownership and Rust correctness review.

## Terminal-context remediation at `6a107303c`

- HOLD retained: terminal exact-byte reviews found a later independent thermal
  expectation still named the first LSE receiver, and a missing non-terminal
  rollback row named the shifted following owner.
- PASS: expectation preflight reports soil-thermal plus the exact first
  mismatch (or expected missing row) before callback execution.
- PASS: rollback sequence validation reports the first absent expected owner
  for deletions and retains actual identity for equal-length malformed rows.
- PASS: real-LSE/real-hydrology integration, 18/18; surface-liquid focused
  tests, 30/30; affected strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte hydrology/ownership and Rust correctness review.

## Exact-offender E011 remediation at `75ba70681`

- HOLD retained: both fresh exact-byte reviewers found that E011 substituted a
  convenient first configured receiver/hydrology owner for later thermal or
  rollback offenders.
- PASS: receiver and rollback validators now return the first canonical
  offender's typed owner/OFE/tile context without substituting another row.
- PASS: real-LSE/real-hydrology integration, 17/17, including a second-row
  thermal-tile poison and exact wrong LSE/soil-thermal rollback-owner payloads.
- PASS: surface-liquid focused tests, 30/30; affected strict all-target Clippy;
  formatting; and diff hygiene.
- Pending: fresh exact-byte hydrology/ownership and Rust correctness review.

## Heavy comparator attempt 1

- PASS: formatting and diff hygiene at `a92cd5db5`.
- FAIL: workspace strict Clippy reported 16 `float_cmp` findings in the two
  focused integration targets. Every finding was an intentionally exact scalar
  assertion; each was rewritten as an exact IEEE-754 bit-pattern comparison.
  No approximate comparison, tolerance, constitutive value, or production code
  changed.
- FAIL: the first corrected authority compilation required explicit `f64`
  suffixes on six otherwise ambiguous local poison-vector operands. The
  operands were typed without changing their values.
- PASS: corrected surface-liquid authority 9/9, real-owner integration 8/8,
  and workspace strict all-target/all-feature Clippy.
- Not run in attempt 1: full workspace nextest, doctests, and dependency policy
  were correctly withheld after the first hard failure. They require a fresh
  comparator run against the corrected exact bytes.

The small attempt-1 raw logs are retained under
`artifacts/gate-run-20260814-1/`.

## Heavy comparator attempt 2 at `e82ba462a`

- PASS: formatting, diff hygiene, workspace strict Clippy, workspace doctests,
  and dependency policy. `cargo deny` retained one non-failing unmatched
  `MIT-0` allowance warning.
- FAIL: full workspace nextest stopped fail-fast after 84 passes and one
  failure; 2710 selected tests did not run. The advisory authority-map test
  expected only WAT5 and plant authority on `00_core_frames.rs`, while the
  new exact surface-liquid binding correctly adds
  `SC-SURFACELIQUID-001`. The expected per-path set now includes the new
  authority only for that changed shared path.
- The full-workspace run is not counted as PASS and must be rerun after all
  accepted implementation review findings are corrected.

Attempt-2 raw logs are retained under `artifacts/gate-run-20260814-2/`.

## Heavy comparator attempt 3 at `d2e9cd09e`

- PASS: formatting, diff hygiene, workspace strict all-target/all-feature
  Clippy, workspace doctests, dependency policy, and every focused custody,
  authority, AUTH11 and advisory gate.
- FAIL: the complete full-workspace run executed all 2,783 selected tests;
  2,782 passed and one historical Stage-0 source scan failed. The scan matched
  the generic token `surface_energy` in the independently admitted
  `openwepp_land_surface_energy` crate name even though its protected target is
  `openwepp_meteorology::surface_energy` Stage-0 primitive wiring.
- Correction: narrow only that source-scan token to the exact meteorology
  module path. No primitive-function token, production selector, constitutive
  implementation, model identity, or runtime behavior changed.

Attempt-3 raw logs are retained under `artifacts/gate-run-20260814-3/`.

## Heavy comparator attempt 4 at `74d512f44`

- PASS: `cargo fmt --all -- --check`.
- PASS: `git diff --check`.
- PASS: `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- PASS: `TMPDIR=/tmp/ow-nextest-openwepp-20260814-4 cargo nextest run
  --workspace --profile full`, 2,783/2,783 passed, 33 skipped.
- PASS: `cargo test --doc --workspace`.
- PASS: `cargo deny check`; the retained unmatched `MIT-0` allowance warning
  is non-failing policy output.

Attempt-4 raw logs are retained under `artifacts/gate-run-20260814-4/`.

## Terminal Rust review remediation at `82bfdc3a0`

- HOLD retained: terminal Rust review found that deleting a nonterminal
  independent thermal expectation named the shifted actual receiver instead of
  the missing expected receiver, and duplicated closure tolerance arithmetic
  could overflow its scale to infinity and accept a wrong finite value.
- PASS: ordered expectation comparison is membership-aware before positional
  replacement/reorder attribution; deletion reports the exact missing
  owner/OFE/tile and retains beginning/attempted rollback hashes without
  invoking the callback.
- PASS: one checked unit-aware arithmetic surface now guards resource basis
  conversion, ingress parcel enthalpy and sums, routed-area conversion,
  dedicated store/parcel closure and independent real-receiver closure.
- PASS: large-finite overflow and nonzero-underflow poisons exercise every
  named boundary.
- PASS: orchestrator quick suite, 544/544; real-LSE/real-hydrology integration,
  19/19; affected strict all-feature Clippy; formatting; and diff hygiene.
- PASS: `surface_liquid_owner.rs` is 2,347 lines after cohesive extraction of
  its crate-local tests; no nonexempt 3,000-line file was introduced.
- Pending: fresh exact-byte Rust and hydrology reviews.

## Terminal arithmetic-precedence remediation at `3b9e5ed13`

- HOLD retained: both exact-byte re-reviewers found that checked-close
  arithmetic failure (`None`) was collapsed into ordinary E010/E011 closure
  mismatch, and receiver depth/enthalpy aggregation retained unchecked
  division/addition.
- PASS: every checked-close caller now distinguishes arithmetic indeterminacy
  (contextual E003) from an ordinary finite mismatch (E010/E011).
- PASS: production-lane/OFE depth, infiltration/retained enthalpy, receiver
  state joins, aggregate layers, mixed debit and cap-rate basis use checked
  arithmetic with canonical precedence.
- PASS: public producer and independent receiver overflow/underflow poisons
  assert exact context and rollback hashes; finite mismatch controls retain
  their original closure codes.
- PASS: orchestrator quick suite, 544/544; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- PASS: affected production files remain below 3,000 lines.
- Pending: fresh exact-byte Rust and hydrology re-review.

## Per-source enthalpy and routed-context remediation at `636dd36be`

- HOLD retained: closure3 reviews found that final E010 compared per-source
  mass but not per-source enthalpy, routed failures could report origin rather
  than destination context, and store arithmetic was duplicated between
  preflight and final validation.
- PASS: expected/actual `(source parcel, basis OFE)` keys compare both mass and
  enthalpy, followed by independent OFE-total comparison through one shared
  disposition helper.
- PASS: same-OFE cross-parcel enthalpy substitution fails E010; per-source
  comparison-scale overflow fails E003.
- PASS: routed E003/E010 context resolves the exact destination identity and
  retains rollback hashes; one shared checked store projection serves
  preflight and final closure.
- PASS: orchestrator quick suite, 552/552; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- PASS: every edited Rust file remains below 3,000 lines.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Independent partition and recipient-identity remediation at `c3fdeca50`

- CRITICAL HOLD retained: closure7 review found expected infiltration,
  retention and routed residual still consumed actual receipt disposition/mass,
  making coordinated owner/recipient swaps self-fulfilling.
- HOLD retained: exact current/recipient tile identity was incomplete, routed
  descendants retained pre-route kind, and raw Q was not joined to mass and
  specific enthalpy.
- PASS: expected partition replays chronological WB14 from frozen raw sources,
  immutable WB14/beginning-continuation operands and independently reconstructed
  pre-ingress stores/capacities with zero receipt access.
- PASS: routed residual and multi-hop flow derive from that partition with area
  conversion; descendants become `UpstreamRunon` before canonical sorting.
- PASS: join identity binds owner, source, origin/current/recipient store and
  full recipient identity, OFE, kind, support and disposition; raw Q and T/h
  identities fail E003 before mixing.
- PASS: coordinated owner/disposition swap, cross-tile retention, routed kind,
  mass and raw-Q poisons fail with exact context and rollback hashes.
- PASS: orchestrator quick suite, 559/559; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Window/disposition and independent routing remediation at `b5453e7d8`

- HOLD retained: closure6 review found join keys omitted support-window and
  disposition identity, expected downstream routed support was read from the
  actual receipt, and support/mass/temperature domains missed global E003
  precedence.
- PASS: exact join identity is source, basis OFE, support start/end and
  disposition; canonical contribution ordering matches production semantics.
- PASS: expected outlet/routed runoff, including multi-hop support, mass and Q,
  is derived only from frozen upstream sources and configuration route topology
  with area conversion; actual receipts project separately.
- PASS: partial-support two-hop positives, route disposition/mass drift and
  cross-window Q/T swap poisons distinguish E010; exhaustive support, negative
  mass and temperature poisons return contextual E003 before E009/E010.
- PASS: orchestrator quick suite, 558/558; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; check; strict Clippy; formatting; diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Chronological mixing and support remediation at `c4114fc8c`

- HOLD retained: closure5 reviews found independent closure computed one
  whole-OFE mixture instead of chronological `h_mix,b`, hardcoded local source
  support to `[0,1800)`, and coupled exact identity to caller tile order.
- PASS: frozen operands preserve actual support endpoints and canonical sorted
  source identity independent of complete unique caller order.
- PASS: closure partitions each OFE at every local/routed support boundary,
  reconstructs raw segment mass/Q, computes checked `h_mix,b` for nonzero
  windows, attributes every source segment with that mixture, then closes per
  source, OFE and interval raw-to-attributed totals.
- PASS: partial-overlap unequal-temperature multi-boundary vectors and reversed
  caller order produce the same accepted result; wrong support and interval-wide
  mixture poisons fail closed with exact context.
- PASS: orchestrator quick suite, 556/556; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Canonical mixed-enthalpy and source-identity remediation at `e19bcdbcf`

- CRITICAL HOLD retained: closure4 reviews found production had replaced the
  required interval-wide `h_mix = sum(Q) / sum(m)` with source-specific
  temperatures, changing accepted infiltration, retention, runoff and routing
  energy for mixed-temperature inputs.
- HOLD retained: frozen zero-source identity/cardinality was not validated and
  multi-tile OFE aggregate failures fabricated first-tile context.
- PASS: producer checked-sums raw interval mass/enthalpy, applies the canonical
  zero-supply branch, derives one `h_mix`, and assigns it to every attributed
  infiltration/retained/runoff/routed parcel.
- PASS: independent operands preserve raw source mass/Q and exact identity,
  including zero rows, then reconstruct post-mix per-source `mass * h_mix` and
  raw-total-to-mixed-total energy separately.
- PASS: deletion, rekey, duplicate and kind-swap poisons fail closed; unequal
  source temperatures and source order produce identical accepted mixed state.
- PASS: OFE-only aggregate failures carry owner/OFE and typed absence for
  tile/surface/source when a unique identity cannot be proven.
- PASS: orchestrator quick suite, 554/554; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; check; strict Clippy; formatting; diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Multi-record precedence and attribution remediation at `ee240618c`

- HOLD retained: Rust closure review found that the local arithmetic preflight
  invoked a short-circuit full closure validator, so an earlier finite E010
  could hide a later-store E003; non-receipt E009 context also fell back to the
  first configured store.
- PASS: a dedicated exhaustive arithmetic-only preflight scans every store,
  source parcel, receipt, route conversion, routed aggregate and OFE aggregate
  without stopping at finite closure mismatch.
- PASS: producer comparison is structural in canonical order across
  transaction, beginning/ending records and continuations, receipts, ledgers
  and WB14, with no first-record fallback.
- PASS: multi-store and multi-OFE poisons assert exact later E003/E009 context
  and rollback hashes; independent E010 remains a distinct final control.
- PASS: orchestrator quick suite, 546/546; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- PASS: affected production files remain below 3,000 lines.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Aggregate projection and deletion remediation at `86ddb8aa2`

- HOLD retained: closure2 reviews found that preflight accumulated per-OFE
  enthalpy without performing the checked aggregate comparison, and positional
  producer localization misattributed nonterminal deletions to shifted rows.
- PASS: one shared parcel/routing projection drives both exhaustive E003
  preflight and final E010 closure, including checked per-key and per-OFE
  aggregate comparisons.
- PASS: membership-aware sequence/map attribution covers state records,
  continuations, receipts, ledgers and WB14; upper/middle deletions report the
  missing expected identity while replacements/reorders retain actual identity.
- PASS: combined multi-parcel aggregate-scale E003 poisons outrank concurrent
  E009 and earlier finite E010 with exact context and rollback hashes.
- PASS: orchestrator quick suite, 548/548; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- PASS: ingress tests were extracted mechanically; every edited Rust file is
  below 3,000 lines.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Final ingress-precedence remediation at `47f959b43`

- HOLD retained: both final exact-byte reviewers found that independent E010
  closure validation preempted immutable producer attribution/routing failures
  canonically assigned to E009.
- PASS: ingress candidate validation now runs bounded E003 arithmetic/domain
  preflight, one immutable E009 producer reconstruction and field comparison,
  then independent E010 closure.
- PASS: the wrong-infiltration-recipient poison is restored to
  E009/`IngressCandidate`; separate public E003, E009 and E010 poisons assert
  exact available identity and rollback hashes.
- PASS: orchestrator quick suite, 544/544; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- PASS: affected production files remain below 3,000 lines.
- Pending: fresh exact-byte Rust and hydrology re-review.

## Persistent endpoint join remediation at `862f26bb7`

- HOLD retained: closure8 hydrology review found the receipt-free replay
  computed authoritative final stores and cumulative WB14 continuation values
  but discarded them before comparison with persistent ending/restart state.
- PASS: independent projection now retains every configured ending store and
  every OFE continuation, including day, next interval, cumulative supply,
  cumulative infiltration and accepted transaction identity.
- PASS: those values join directly to the actual ending owner with exact
  cardinality/order/OFE/store identity before recomputed digest and full strict
  state validation.
- PASS: coordinated poisons cover wrong store with correct receipts, cumulative
  drift, cadence and transaction drift, missing/duplicate/reordered/wrong-OFE
  continuations, forged self-consistent producer store operands and forged
  digest.
- PASS: orchestrator quick suite, 560/560; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Partition taxonomy and routed-order remediation at `6e203beec`

- HOLD retained: closure8 Rust review found malformed frozen partition
  membership could be reported as arithmetic E003 before immutable E009,
  cumulative infiltration lacked complete supply/capacity preflight, and the
  mixed-kind routed ordering seam lacked a nondegenerate vector.
- PASS: exhaustive arithmetic/domain scanning precedes membership-aware E009;
  missing, duplicate, reordered and replacement rows retain exact available or
  missing identity, while combined poisons prove E003 before E009 before E010.
- PASS: frozen cumulative infiltration is bounded by both cumulative supply and
  WB14 infiltration storage capacity, including zero-supply and distinct
  positive-supply/zero-capacity poisons.
- PASS: closure uses shared named water-density, liquid-heat-capacity and
  reference-temperature constants without consuming producer results.
- PASS: mixed canopy throughfall/drainage with unequal temperature/area and
  downstream local overlap proves route-kind conversion, chronological
  mixtures and caller-order invariance.
- PASS: orchestrator quick suite, 562/562; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Canonical identity, exact-vector and ending-context remediation at `aacf181d7`

- HOLD retained: closure9/10 reviews found ending-state aggregate and structural
  failures could fabricate first-OFE context, canonical parcel order and source
  ID remained duplicated, and the mixed-route test did not freeze exact
  attribution/remainder outputs.
- PASS: one canonical five-field parcel-order key drives production, frozen
  identity and independent projected ordering; one constructor drives local
  and condensation source IDs while allocation arithmetic remains independent.
- PASS: owner/configuration/digest/full-state errors carry aggregate typed
  transaction/owner context; store and continuation membership/order failures
  carry exact missing, extra, replacement or reordered store/OFE identity.
- PASS: the unequal-area mixed-kind/downstream-overlap fixture exercises
  nonzero infiltration, retained surface water, routed and outlet runoff and
  freezes every receipt identity, window, mass, mixture temperature and
  enthalpy bit plus ending stores and WB14 continuations.
- PASS: orchestrator quick suite, 562/562; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.

## Cardinality-aware ending-context remediation at `2dfd0af64`

- HOLD retained: closure10 Rust review found equal-length store/OFE replacement
  reported the missing expected key instead of the available forged row, and
  the tests did not bind the complete context/rollback matrix.
- PASS: shorter actual sequences report missing expected identity; longer
  sequences report the first excess/duplicate actual identity; equal-length
  replacement or reorder reports the first actual mismatched row.
- PASS: first/middle/last missing, appended duplicate, forged replacement and
  pairwise reorder cases cover stores and continuations with exact E010 phase,
  transaction, owner, applicable/absent identity and rollback hashes.
- PASS: orchestrator quick suite, 562/562; real-LSE/real-hydrology integration,
  19/19; custody authority, 9/9; strict Clippy; formatting; and diff hygiene.
- Pending: fresh exact-byte Rust and hydrology closure review.
