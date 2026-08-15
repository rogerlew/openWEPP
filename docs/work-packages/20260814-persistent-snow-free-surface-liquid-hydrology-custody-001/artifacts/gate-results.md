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

## Authority-impact binding correction

- FAIL retained: the first current-base science-admission run rejected the new
  extracted `surface_liquid_ingress_tests.rs` because it lacked a current SC
  impact-map binding.
- PASS: impact-map generation 26 binds both extracted crate-local custody test
  modules and the integration authority contract to `SC-SURFACELIQUID-001`.
- PASS: `check_science_contract_admission.sh --base-ref af9a989... --worktree`
  returned `A0_ADMITTED contracts=46 science_surfaces=16`.
- PASS: authority-suite anti-evasion.
- PASS: AUTH11, 3/3.
- PASS: SC-SURFACELIQUID unit-compliance lint.

## Line-count inventory reconciliation

- HOLD retained: closure12 hydrology review found `runoff.rs` appeared twice
  with conflicting incomplete and WARN dispositions.
- PASS: the obsolete duplicate row is removed; one authoritative WARN row
  records current count, rationale and follow-on split intent.

## Exact-head heavy-gate attempt at `4c7b64237`

- PASS: formatting, diff hygiene, science admission, anti-evasion,
  `SC-SURFACELIQUID-001` unit compliance, the corrected 28-test focused
  custody/LSE suite, and the corrected 58-file package Markdown lint.
- FAIL retained: strict workspace Clippy rejected the 110-line
  `independent_real_receiver_equations_reject_layer_and_enthalpy_poisons`
  integration test. The test is now decomposed into three cohesive poison
  helpers without changing its assertions.
- FAIL retained: the full workspace run completed 2,807 tests with 2,797
  passing and 10 failing. All ten failures read compile-time
  `CARGO_MANIFEST_DIR` paths from deleted terminal-review worktrees through
  stale shared-target integration binaries; seven belonged to the advisory
  linter authority suite and three to AUTH11. The standalone AUTH11 retry
  reproduced the same stale-binary failure.
- Ran: a fresh external target directory rebuilt the affected binaries from
  the current checkout after the test decomposition. Strict focused Clippy
  passed, AUTH11 passed 3/3, and the focused custody/LSE suite passed 28/28.
  No tolerance, production behavior, scientific equation, or authority
  surface changed.
- Pending: commit the test-only decomposition and rerun the complete heavy
  command set on exact committed bytes using a fresh external target.

## Invalidated fresh-target heavy run and terminal review

- INTERRUPTED retained: a new external-target heavy run began at
  `87b187b19`. Strict workspace Clippy completed successfully. The formatting
  and diff-hygiene wrapper commands were malformed and exited 127; neither is
  counted as evidence from this attempt. The full suite was then stopped when
  fresh independent review accepted material source defects. All partial logs
  are preserved and are not terminal PASS evidence.
- HOLD retained: the Rust reviewer found checked-arithmetic underflow in
  proportional authorization, an incomplete public receiver validator, and
  noncanonical receiver/configuration/restart error payloads.
- HOLD retained: the hydrology reviewer independently found generic receiver
  mutation failures without canonical code, context, or rollback hashes.
- PASS after remediation: orchestrator quick suite, 564/564; focused custody
  and unified LSE/real-hydrology suites, 29/29; strict affected-crate Clippy;
  formatting; and diff hygiene.
- Pending: fresh exact-byte dual review and complete heavy rerun.

## Second exact-byte review remediation

- HOLD retained: the `2b713d659` reviews found canonical-last underflow,
  attachment/serialized-byte context loss, receiver E003 precedence,
  unframed receiver hashes, generic unified public errors, and incomplete
  terminal evidence.
- PASS: every proportional row now executes checked arithmetic before canonical
  remainder assignment; both caller orders reject the tiny-positive case as
  contextual E003.
- PASS: frame attachment and noncanonical configuration/restart failures retain
  exact canonical code, phase, available identity and rollback hashes.
- PASS: unified entry/protocol failures are contextual E002/E005/E006; receiver
  numeric preflight precedes E011; all receiver hashes use one tagged,
  length/cardinality-framed encoder with collision poisons.
- PASS after integrated remediation: orchestrator quick suite, 566/566;
  focused custody and unified LSE/real-hydrology suites, 33/33; strict affected
  Clippy; science admission; formatting; and worktree plus terminal-diff
  hygiene.
- Pending: commit exact bytes, fresh dual review, then complete heavy rerun.

## Exact-byte review at `5d298ca1c` and final focused remediation

- HOLD retained: fresh hydrology and Rust reviews independently found that
  standalone finalization did not seal the exact three-owner rollback set,
  frost fine/shadow membership was not reciprocal, and unified E002 preflight
  omitted configured request/source mapping and complete attempted-input
  provenance. The Rust review also found two review files with a blank EOF.
- PASS: standalone finalization now requires exactly one canonical LSE,
  hydrology and soil-thermal rollback row and resolves thermal failure context
  only from an applicable owner row.
- PASS: persisted and runtime frost containers now require exact reciprocal
  fine/shadow membership, ordering and count while retaining empty/empty as the
  valid initial structural state.
- PASS: unified preflight now validates complete configured source binding
  before every E003 check and callback, hashes all ingress, WB14 and soil-source
  operands, and records the computed beginning snapshot in rollback evidence.
- FAIL retained: the first combined focused run exposed one stale internal
  frost fixture with fine rows but no shadow row. The fixture was corrected to
  canonical reciprocal structure; the validator was not weakened.
- FAIL retained: first broader sealing and unified integration attempts exposed
  stale expected error and snapshot assertions. They were corrected to the
  canonical missing-owner E011 context and independently computed beginning
  snapshot, then reran successfully.
- PASS on the combined worktree: focused orchestrator 145/145; unified
  real-hydrology/LSE integration 52/52; custody authority 10/10; affected
  strict Clippy; formatting and diff hygiene.
- Pending: freeze these exact bytes and obtain fresh independent dual review
  before any heavy terminal execution.

## Exact-byte review at `73299b981`

- HOLD: both fresh reviewers found that a soil request can name one OFE while
  its source map selects another configured OFE lane sharing the same layer ID.
- HOLD: both fresh reviewers found that standalone sealing checks an LSE
  rollback row for equality but does not bind it to the actual beginning LSE
  digest.
- HOLD: the Rust review found source-map, winter-domain and exact-one public
  failures that retain only a request-batch attempted hash after the complete
  ingress/WB14/soil-map envelope is available.
- PASS but non-dispositive: unified integration 52/52, custody authority 10/10,
  selected orchestrator 145/145, formatting and diff hygiene.
- Pending: remediate every finding, add the cross-OFE, forged-LSE,
  attempted-hash-sensitivity and frost-cardinality poisons, then repeat fresh
  dual exact-byte review. Heavy execution remains illegitimate.

## `73299b981` HOLD remediation

- PASS: soil requests now bind request OFE to the exact configured production
  lane index, lane ID and layer membership before E003 or callback execution.
- PASS: standalone finalization's sole public constructor consumes complete
  independent receiver expectations and binds the exact LSE, hydrology and
  soil-thermal beginning digests; equal-but-forged LSE rows fail E011.
- PASS: one complete attempted-input hash frames request, ingress, WB14,
  soil-source and actual-snapshot identity and is threaded through source-map,
  winter, exact-one and authorization failures.
- PASS: persisted and runtime frost indices outside production layer
  cardinality fail E003 before unsupported-domain E004.
- PASS on the combined worktree: selected orchestrator 145/145; unified
  integration 54/54; custody authority 10/10; four affected crate checks;
  strict orchestrator all-target/all-feature Clippy; formatting and diff
  hygiene.
- PASS: cohesive extraction leaves every governed Rust file below 3,000 lines.
- Pending: freeze exact bytes and repeat fresh dual review. Heavy execution
  remains blocked until both reviews return PASS.

## Extracted sealing authority binding

- FAIL retained: the first science-admission run rejected the new
  `finalization_sealing.rs` because the impact map lacked an exact current
  contract binding.
- PASS: generation 34 atomically binds the extracted sealing module to both
  SC-SURFACELIQUID-001 and SC-LANDSURFACEENERGY-001.
- PASS after correction: science admission reports 46 contracts and 28 science
  surfaces; anti-evasion passes; AUTH11 passes 3/3; SC-SURFACELIQUID-001 unit
  compliance passes.

## Exact-byte review at `e33f4cdd4`

- PASS: fresh hydrology/science/ownership review found zero unresolved material
  finding and reran the focused, authority and strict affected gates.
- HOLD: fresh Rust review found arbitrary callback errors escaping without the
  canonical public failure envelope, independently knowable receiver
  expectations reaching the callback before validation, and attempted hashes
  omitting receiver expectations plus the caller expected snapshot.
- PASS but non-dispositive: 86 focused surface-liquid tests, 54 unified
  integration tests, 10 custody authority tests, AUTH11 3/3, strict Clippy,
  formatting, anti-evasion and diff hygiene.
- Pending: remediate all three findings and repeat fresh dual exact-byte review.
  Heavy execution remains blocked.

## `e33f4cdd4` HOLD remediation

- PASS: complete receiver expectations are validated before authorization or
  callback; invalid LSE, hydrology, soil-thermal lineage and topology produce
  contextual E011 with callback count zero.
- PASS: unified attempted-input framing v3 binds every raw receiver-expectation
  field, ordered topology/layers, its canonical digest, the actual snapshot and
  the caller expected snapshot. Field-by-field mutation poisons distinguish
  every attempted input.
- PASS: callback-returned Identity, Operand, Bound, Unsupported, LandSurface
  and existing SurfaceLiquid variants emerge from the public boundary as
  canonical ResourceCandidate failures with actual beginning and complete
  attempted hashes; available lower context is preserved and unavailable row
  identity is not fabricated.
- PASS on the combined worktree: selected orchestrator 145/145; unified
  integration 57/57; custody authority 10/10; four affected crate checks;
  strict orchestrator all-target/all-feature Clippy; formatting and diff
  hygiene.
- PASS: every governed Rust file remains below 3,000 lines after extracting the
  360-line unified public-boundary poison matrix.
- Pending: freeze exact bytes and repeat fresh dual exact-byte review. Heavy
  execution remains blocked until both reviews return PASS.

## Exact-byte review at `fc65b2819`

- HOLD: both reviewers found that raw surface-liquid callback failures retain
  an earlier phase or empty context rather than the known ResourceCandidate
  transaction boundary.
- HOLD: the Rust reviewer found nested land-surface taxonomy collapsed to E003
  and receiver expectations missing the configured infiltration thermal layer
  before callback execution.
- HOLD: both reviewers found unified v3 overwriting complete raw malformed
  configuration/state attempted evidence.
- HOLD: the Rust reviewer traced the taxonomy drift to three duplicated
  translation tables and requested one boundary-aware canonical mapping.
- PASS but non-dispositive: selected orchestrator 145/145, full orchestrator
  library 600/600, unified integration 57/57, custody authority 10/10, strict
  affected Clippy, formatting and diff hygiene.
- Pending: remediate all findings and repeat fresh dual exact-byte review.
  Heavy execution remains blocked.

## `fc65b2819` HOLD remediation

- PASS: raw and canonical surface-liquid callback variants are rebound to the
  public ResourceCandidate phase with the known transaction, actual beginning
  and complete attempted hashes while preserving applicable lower row context.
- PASS: one centralized boundary-aware taxonomy maps all 19 materially distinct
  nested land-surface/shadow classes to canonical E001/E002/E003/E004/E010/E011.
- PASS: unified attempted framing joins raw invalid configuration/state bytes
  with the v3 envelope; distinct stale-digest finite and nonfinite mutations do
  not alias.
- PASS: exact configured infiltration thermal layers are validated before
  authorization/callback; wrong-first, deleted and replaced layers retain
  callback count zero.
- FAIL retained: the first focused run passed 60/61 because the state poison
  reached the expected-snapshot E002 branch; full state validation was placed
  after request identity so raw state evidence is joined without weakening
  precedence.
- FAIL retained: the next run exposed request-identity-before-nonfinite-state
  precedence when state validation was too early; the final ordering preserves
  request identity first and the raw state-attempt join.
- PASS on final worktree: unified integration 61/61; custody authority 10/10;
  complete orchestrator library 600/600 in 146.948 seconds; strict affected
  Clippy; formatting and diff hygiene.
- PASS: every governed Rust file remains below 3,000 lines; `mod.rs` is 2,896.
- Pending: freeze exact bytes and repeat fresh dual exact-byte review. Heavy
  execution remains blocked until both reviews return PASS.

## Exact-byte review at `fb89e5a55`

- HOLD: both reviewers found full configuration domain validation preceding
  request identity, allowing configuration E003 to mask canonical request E002.
- HOLD: the Rust reviewer found the centralized taxonomy still semantically
  incomplete because combined topology and water identity/bound enum variants
  require prose inference. Real open-trial domain, D/A/F, closure and missing-
  authorization constructors are misclassified.
- PASS but non-dispositive: selected orchestrator 145/145; unified integration
  61/61; focused integration/authority 78/78; complete orchestrator library
  600/600; strict Clippy; formatting and diff hygiene.
- Pending: restore complete identity-before-domain ordering and replace prose-
  based taxonomy with typed LSE error classes covering every production
  constructor, then repeat fresh dual exact-byte review. Heavy gates remain
  blocked.

## `fb89e5a55` HOLD remediation

- PASS: configuration schema/identity preflight again precedes complete request
  E002; full numeric configuration validation follows identity checks while its
  raw attempt is framed-joined with unified v3.
- PASS: LSE now owns typed `TopologyErrorClass`, `WaterErrorClass` and exhaustive
  `LandSurfaceEnergyErrorClass`; every production constructor is migrated and
  mixed numeric/bound/cardinality guards are split without prose matching.
- PASS: real production vectors map open-trial topology to E003, empty topology
  to E005, D/A/F to E006, pre-ingress source closure to E010 and missing
  authorization to E005. Every enum semantic subclass is covered.
- FAIL retained: first integration rerun passed 61/62 because distinct stale-
  digest NaN configuration attempts still aliased; joining the raw snapshot
  attempt with v3 corrected the evidence.
- FAIL retained: one compile retry exposed missing re-exports for the new real-
  vector helpers; explicit orchestrator re-exports corrected it.
- FAIL retained: first strict Clippy retry found a `map(...).unwrap_or_else(...)`
  form; it was replaced with `map_or_else` without semantic change.
- PASS on final worktree: LSE 28/28; unified integration 62/62; custody
  authority 10/10; orchestrator library 600/600 in 145.035 seconds; strict LSE
  and orchestrator Clippy; formatting and diff hygiene.
- PASS: every changed Rust file remains below 3,000 lines.
- Pending: freeze exact bytes and repeat fresh dual exact-byte review. Heavy
  execution remains blocked until both reviews return PASS.

## Exact-byte review at `3ac61997d`

- HOLD: both reviewers found the complete E002 envelope still split around
  configuration/state E003 validation for ingress, source-map, outer-
  transaction and expected-snapshot identity.
- HOLD: both reviewers found standalone sealing accepts an entirely empty
  request/authorization/finalized-use protocol through vacuous set equality.
- HOLD: the Rust reviewer found real LSE negative D/A/F and condensation-credit
  operands construct domain E003 rather than bound E006.
- RISK accepted for explicit disposition: typed public error variant shapes are
  source-breaking, and thermodynamic constants remain duplicated across LSE
  and direct ingress.
- PASS but non-dispositive: LSE 28/28; unified 62/62; custody authority 10/10;
  orchestrator 600/600; strict Clippy; formatting and diff hygiene.
- Pending: remediate all findings, disposition both risks, and repeat fresh
  dual exact-byte review. Heavy execution remains blocked.

## `3ac61997d` HOLD remediation

- Ran: `cargo nextest run -p openwepp-land-surface-energy --profile quick
  --no-fail-fast` — PASS, 28/28.
- Ran: `cargo nextest run --test
  land_surface_energy_real_hydrology_shadow_contract --profile quick
  --no-fail-fast` — PASS, 64/64.
- Ran: `cargo nextest run --test
  surface_liquid_hydrology_custody_authority_contract --profile quick` — PASS,
  10/10.
- Ran: affected strict all-target/all-feature Clippy for
  `openwepp-land-surface-energy` and `openwepp-hillslope-orchestrator` — PASS.
- Ran: science admission from `af9a989063aa8751dfadb14c442e1b360653658c`
  — PASS, 46 contracts and 28 science surfaces.
- Ran: authority anti-evasion — PASS; AUTH11 — PASS, 3/3; surface-liquid
  contract unit compliance — PASS.
- Ran: `cargo fmt --all -- --check`, `git diff --check`, and package Markdown
  lint — PASS; Markdown validated 98 files with zero errors and warnings.
- Ran by the remediation worker: orchestrator library — PASS, 600/600; three
  tests exceeded the slow-test threshold and no test failed.
- Preserved retries: the first compile used nonexistent
  `WaterAuthorizationReason::FullDemand` and was corrected to `FullSupply`;
  the first focused run passed 62/64, exposing incomplete raw-plus-unified
  attempted hashing and an accidentally invalid soil-key fixture. Both defects
  were corrected before the 64/64 run.
- Static + Ran: global identity-first preflight, independently expected ground
  D/A/F coverage, and real E003/E006 water-taxonomy vectors close all three
  material findings.
- Risk disposition: the typed public LSE error shape is intentionally
  source-breaking within package authority. The duplicated LSE/direct-runtime
  thermodynamic constants are bit-identical; centralization is deferred as a
  maintenance risk because it would broaden scientific-authority coupling.
- Pending: freeze a clean commit and repeat fresh dual exact-byte review. Heavy
  execution remains blocked until both reviews return PASS.

## Exact-byte review at `85358c9b2`

- PASS: fresh hydrology/science/ownership review found no material finding on
  the exact clean bytes. Its 58 focused integration tests, diff hygiene and
  clean-tree checks passed.
- INTERRUPTED retained: the hydrology reviewer stopped a broader exploratory
  run after 597 passes when three unrelated routing-oracle tests exceeded 60
  seconds. This is non-evidence and does not replace a later comparator run.
- HOLD: fresh Rust correctness review found owner/hash drift in standalone
  receiver E003/E010/E011 and structural sealing, incomplete frost-container
  validation that could misclassify malformed state as E004, and an incomplete
  unified-entry E002 ingress preflight that allowed E003 to mask identity and
  callback execution on invalid input.
- PASS in remediation worktree: complete winter-state structural poisons,
  focused CQR, direct R7G frost 17/17, strict library Clippy, formatting and
  diff hygiene.
- Pending: complete receiver provenance and unified-entry corrections, freeze
  exact bytes, then obtain fresh dual review before any terminal heavy gate.

## `85358c9b2` HOLD remediation

- FAIL retained: the first combined integration run passed 39 rows and failed
  one stale thermal rollback assertion that still expected the hydrology
  digest; five rows were cancelled by fail-fast. The assertion was corrected
  to require the exact thermal-owner digest.
- FAIL retained: the first strict affected-crate Clippy run found two newly
  lengthened explicit scanners and two needless pass-by-value arguments. The
  owner-domain scan and explicit anti-aliasing digest retain narrow documented
  allowances; both arguments now borrow.
- PASS: frozen receiver operands bind hydrology, LSE and soil-thermal beginning
  digests plus every ordered rollback row. One canonical resolver supplies the
  exact unique `(OwnerKind, owner_id)` beginning digest or typed absence for a
  missing/duplicate row across E003, E010, E011 and sealing.
- PASS: frost validation rejects nonintegral/cardinality-mismatched counts,
  duplicate/reordered/nonpositive indices, missing-first/gapped fine indices
  and undeclared membership before the unsupported E004 branch.
- PASS: the unified input-only identity preflight covers tile, OFE, surface,
  source, ingress mode and WB14 parameter identity before request/winter E003,
  authorization or callback. Mixed request-domain and winter-domain poisons
  return E002 and prove the callback was not invoked.
- PASS on the corrected combined worktree: selected orchestrator library
  145/145; unified integration 46/46; custody authority 10/10; strict affected
  all-target/all-feature Clippy; formatting and diff hygiene.
- Pending: freeze the corrected bytes and obtain fresh dual exact-byte review
  before any terminal heavy execution.

## Post-checkpoint focused authority gates

- PASS at `5d298ca1c`: AUTH11 3/3, authority-suite anti-evasion, SC-SURFACELIQUID
  unit compliance and the four affected crate checks.
- FAIL retained: science admission from `af9a98906` first rejected the extracted
  `surface_liquid_closure_preflight.rs` because the impact map lacked an exact
  current contract binding.
- FAIL retained: the first map correction bound two contracts in one entry;
  admission correctly rejected the non-atomic receiver-failure binding.
- PASS after correction: impact-map generation 33 binds each extracted closure,
  receiver-failure, unified-entry-preflight and extracted test surface to one
  atomic SC contract per entry. Admission reports 46 contracts and 27 science
  surfaces.

## Exact-byte review at `5d298ca1c`

- HOLD: fresh Rust and hydrology reviews independently found incomplete
  standalone rollback-owner sealing, nonreciprocal frost fine/shadow membership
  and incomplete unified source/attempted-input E002 provenance.
- PASS within both reviews: unified integration 46/46, selected library
  145/145, custody authority 10/10; hydrology also ran R7G frost 17/17 and Rust
  ran strict affected Clippy.
- PASS: D/A/F, restart custody, signed condensation, mass/enthalpy joins,
  ingress order, rollback isolation and selector exclusion had no new material
  finding.
- Pending: correct all accepted findings and repeat fresh dual exact-byte
  review before heavy gates.

## Fifth exact-byte review remediation

- HOLD retained: exact `fe6cc4bd5` reviews found unified/global precedence,
  public attempted-hash, caller-order overflow context, restart taxonomy and
  later/cardinality binding/ingress context defects.
- PASS: public request/protocol and receiver validation now observes canonical
  precedence through derived arithmetic. Direct and unified failures carry raw
  beginning/attempt hashes and exact available offender identity.
- PASS: only canonical-key `D_sum` arithmetic remains; restart over-capacity is
  E003; later/excess OFEs report exact identity while missing identity is typed
  absence.
- FAIL retained: an intermediate 69-test filtered run had one stale declared-
  hash assertion; an intermediate 32-test integration run had two stale
  precedence assertions. These were test-expectation defects and are preserved
  here rather than counted as passing evidence.
- PASS after integrated remediation: orchestrator quick suite, 578/578;
  focused authority/unified suites, 42/42; owner tests, 33/33; strict affected
  Clippy; formatting; and diff hygiene.
- PASS: the two growing test surfaces were cohesively split to 2,979 and 2,955
  lines; the extracted context and raw-hash modules are 85 and 100 lines. No
  touched Rust source exceeds the 3,000-line ceiling.
- PASS on the combined final worktree: all four affected-crate checks; strict
  all-target/all-feature Clippy for all four affected crates; science admission
  with 46 contracts and 21 bound science surfaces; authority anti-evasion;
  AUTH11 3/3; SC-SURFACELIQUID unit compliance; formatting; worktree and
  base-relative diff hygiene; and package/contract Markdown lint, 69 files with
  zero errors or warnings.
- Pending: stable commit, fresh exact-byte dual review, then complete heavy
  rerun.

## Exact-byte review at `2e32a8a0e`

- PASS: independent hydrology/science/ownership review found no material
  custody, D/A/F, ingress, receiver, rollback or production-isolation defect.
- HOLD: independent Rust correctness review found an upward one-ULP
  mass/depth/mass full-infiltration round-trip that can produce a negative
  remainder, together with missing independent raw-source mass reconstruction.
- HOLD: the same review found mixed public failures can violate the canonical
  E001 through E011 precedence at unified request/protocol/native-domain and
  ingress boundaries.
- Ran: reviewer-focused surface-liquid orchestrator tests, 70/70; unified
  integration, 32/32; custody authority, 10/10; formatting and diff hygiene.
- Pending: correct both accepted implementation findings, rerun focused gates,
  freeze new exact bytes and obtain fresh dual review before heavy execution.

## `2e32a8a0e` HOLD remediation

- PASS: bit-exact full-infiltration recognition now returns each original raw
  parcel mass instead of round-tripping it through metres. The ordinary
  `0x1.f9e1df20c7aa4p-6` zero/nonzero-store regression proves nonnegative
  partition, bitwise `I+E=X`, and no beginning-store debit.
- PASS: the external reconstruction independently joins attributed mass to raw
  source-parcel and OFE mass and rejects negative expected partitions; a
  receipt-mass poison fails closure.
- PASS: public unified and ingress validation now explicitly observes E002
  identity, E003 domain, E004 unsupported, E005 cardinality, E006 bounds, E007
  exact-one custody and E008 cadence. Mixed identity/NaN, snow/duplicate,
  duplicate/negative, NaN-interval and unknown-ingress/cardinality poisons
  pass with complete context and hashes.
- PASS on the combined worktree: surface-liquid tests 73/73; unified
  integration 35/35; custody authority 10/10; orchestrator quick 581/581;
  AUTH11 3/3; affected check and strict all-target/all-feature Clippy;
  admission with 46 contracts and 21 science surfaces; anti-evasion; SC unit
  compliance; formatting; base/worktree diff hygiene; Markdown 71 files with
  zero errors or warnings.
- Pending: freeze a new exact commit and obtain fresh independent dual review.

## Exact-byte review at `f249431d4`

- PASS: fresh hydrology/science/ownership review found no material finding and
  ran 73/73 surface-liquid, 10/10 authority, 35/35 unified integration and
  581/581 orchestrator tests.
- HOLD: fresh Rust review found whole-record configuration/state validation can
  emit an earlier E003 before a later required E002 identity failure.
- HOLD: the same review found duplicated checked receiver aggregation between
  precedence preflight and final operand construction.
- INTERRUPTED retained: the reviewer stopped a full-workspace attempt after
  the material HOLD was confirmed; 176 passed, 19 were interrupted and 2,688
  did not run. This is not terminal evidence.
- Pending: remediate both accepted findings, freeze new bytes and repeat fresh
  dual review before any terminal heavy run.

## `f249431d4` HOLD remediation

- PASS: complete configuration and restart identity sets are preflighted before
  record-domain checks, and attachment/authorization outer identities precede
  lower-priority configuration/state validation. Cross-row, reversed-order,
  NaN/overcapacity plus later wrong-identity poisons pass.
- PASS: a single checked receiver fold now constructs infiltration depth plus
  infiltration/retained enthalpy maps for both precedence preflight and final
  operand freezing. Context-specific failures are injected without duplicating
  conversion or summation arithmetic; bit-exact drift and E003 poisons pass.
- INTERRUPTED retained: two accidentally redundant orchestrator quick
  invocations were terminated. They are not evidence; the retained clean run
  passed 587/587.
- PASS on the combined worktree: surface-liquid tests 77/77; unified
  integration 35/35; custody authority 10/10; orchestrator quick 587/587;
  AUTH11 3/3; affected check and strict all-target/all-feature Clippy;
  admission with 46 contracts and 22 science surfaces; anti-evasion; SC unit
  compliance; formatting; base/worktree diff hygiene; Markdown 73 files with
  zero errors or warnings.
- Pending: freeze new exact bytes and obtain fresh independent dual review.

## Exact-byte review at `10b914da1`

- HOLD: fresh Rust review found two exported public seams where configuration
  or state E003 validation can mask caller-supplied E002 identities.
- HOLD: fresh hydrology/ownership review found applicable configured surface
  and source-store identity absent from receiver-closure error context.
- PASS on the reviewed bytes: 97/97 selected library tests, 39/39 unified
  integration tests and 10/10 custody authority tests.
- PASS: both reviews found no other material arithmetic, hydrology, custody,
  rollback or production-isolation defect.

## `10b914da1` HOLD remediation

- PASS: resource and ingress public entry points now perform whole-envelope
  E001/E002 identity preflight before any configuration/state E003 validation;
  seven mixed-poison permutations pass.
- PASS: receiver operands freeze and digest-bind exact configured
  `(OFE, tile, surface, source)` identity and propagate it through applicable
  E003/E010/E011 closure failures without fabricating parcel identity.
- PASS on the combined worktree: 101/101 selected library tests, 39/39 unified
  integration tests and 10/10 custody authority tests.
- FAIL then PASS: strict affected-crate Clippy first identified a test-only
  103-line fixture; the explicit test-fixture lint annotation preserves the
  production limit and the rerun passed. Formatting and diff hygiene pass.
- Pending: commit corrected bytes and repeat fresh independent dual review
  before any terminal heavy execution.

## Exact-byte review at `73f22169a`

- HOLD: both reviews found incomplete nested winter-domain E003-before-E004
  coverage. Rust review also found parcel E010 can stop the later E003 scan and
  derived receiver E003 still carries a hydrology beginning digest.
- PASS on reviewed bytes: unified integration 40/40, custody authority 10/10
  and selected orchestrator 103/103.

## `73f22169a` HOLD remediation

- PASS: one complete production winter validator covers snow, albedo, all snow
  layers/cross-fields, frost scalars/layer shadows/fine layers and runtime
  carries before the unsupported E004 branch.
- PASS: parcel identity-only joins no longer terminate arithmetic preflight;
  an earlier identity mismatch plus later E003 poison reports E003.
- PASS: derived receiver E003 binds LSE/thermal owner rollback hashes, retains
  hydrology hash for hydrology failures and uses typed absence for missing or
  duplicate owner rows.
- FAIL retained: an intermediate design propagated lower-priority E010 and
  failed four focused cases; E003-only propagation was restored.
- FAIL retained: three test-shaping attempts used an incorrect context/detail,
  an incoherent source-domain poison and a nonexistent outlet fixture. They
  were corrected before final evidence and are not product regressions.
- PASS on final combined worktree: selected library 145/145, unified integration
  44/44, custody authority 10/10, strict affected all-target/all-feature Clippy,
  formatting and diff hygiene.
- Pending: commit corrected bytes and repeat fresh independent dual review
  before any terminal heavy execution.

## Exact-byte review at `83e1ee296`

- HOLD: Rust review found partition E009 can preempt a later projection E003,
  and receiver-owned E003 failures use the hydrology beginning digest.
- HOLD: hydrology review found negative/nonfinite snow scalars can bypass the
  positivity-only snow predicate and enter the snow-free path.
- PASS on reviewed bytes: surface-liquid 84/84, custody authority 10/10 and
  unified integration 39/39.

## `83e1ee296` HOLD remediation

- PASS: arithmetic-only closure preflight scans receipt/raw-parent arithmetic
  before partition membership and covers reordered complete partitions.
- PASS: sealed and post-ingress LSE/thermal numeric failures bind the unique
  implicated owner/kind beginning hash or typed absence; attempted receiver-set
  hashes remain exact.
- PASS: all 16 snow-lane/runtime-carry scalars reject negative, NaN and both
  infinities as E003 before positive snow's E004 branch; 80 cases pass.
- FAIL retained: briefly propagating every closure-preflight error caused four
  existing E010 identity joins to preempt producer E009. The accepted design
  propagates E003 only after removing partition E009 from the arithmetic scan.
- PASS on final combined worktree: selected library 106/106, unified integration
  40/40, custody authority 10/10, strict affected all-target/all-feature Clippy,
  formatting and diff hygiene.
- Pending: commit corrected bytes and repeat fresh independent dual review
  before any terminal heavy execution.

## Exact-byte review at `a5c2243e6`

- HOLD: Rust review found ingress-candidate E003 could mask whole-input
  E001/E002, and temporal mass splitting plus replay shared a one-ULP
  nonconservative formula.
- HOLD: hydrology review found sealed nonfinite LSE receiver errors attributed
  to the hydrology owner instead of the recoverable LSE owner.
- PASS on reviewed bytes: WB14 5/5, receiver validation 3/3, custody authority
  10/10 and unified integration 39/39.

## `a5c2243e6` HOLD remediation

- PASS: candidate revalidation shares the complete identity-only ingress
  preflight before independent closure arithmetic.
- PASS: temporal mass assigns the exact parent remainder to the canonical last
  window; raw mass closure reconstructs from frozen parents and rejects the
  old five-ratio and one-ULP child alternatives.
- PASS: sealed LSE E003 uses the exact LSE request owner and applicable
  configured surface/source identity; thermal ownership and exact rollback
  hashes remain intact.
- FAIL retained: an initial comparison made aggregate proportional-mixing mass
  bit-exact and failed two legitimate rounded attribution cases. Exactness was
  narrowed to per-parcel authority joins.
- FAIL retained: an attempted raw-parent-only enthalpy reconstruction failed
  three routed/mixed cases because thermal mixing changes source attribution.
  Independent replay remains the enthalpy authority; frozen parents are the
  raw mass authority only.
- PASS on final combined worktree: selected library 103/103, unified integration
  39/39, custody authority 10/10, strict affected all-target/all-feature Clippy,
  formatting and diff hygiene.
- Pending: commit corrected bytes and repeat fresh independent dual review
  before any terminal heavy execution.

## Exact-byte review at `c9524729a`

- HOLD: fresh Rust review found producer and independent closure assign child
  enthalpies independently as `m*h`, contrary to exact authoritative-parent Q
  plus canonical-last subtraction remainders. A valid vector loses one ULP
  while the existing tolerance masks it.
- HOLD: both fresh reviews found condensation T/h E009 drops exact credit
  identity; Rust review additionally found nonfinite production-lane area is
  attached and later misclassified E002 rather than E003.
- PASS: surface-liquid 77/77, unified integration 37/37, custody authority
  10/10 and diff hygiene on the reviewed bytes. Heavy gates were not run.
- Pending: remediate all three accepted findings, freeze new bytes and repeat
  fresh dual review before terminal heavy execution.

## `c9524729a` HOLD remediation

- PASS: temporal, mixed-source, infiltration/excess and retention/runoff
  enthalpy splits now retain one authoritative parent Q and assign the
  canonical-last exact subtraction remainder. Independent closure reconstructs
  from frozen raw operands through a separate closure-only module.
- PASS: the `T=285 K`, `M=0.3`, `I≈0.03` vector proves exact parent/child bits;
  a one-ULP canonical-last poison now fails independent closure as E010. The
  dimensional tolerance is unchanged.
- PASS: condensation temperature and enthalpy E009 failures carry complete
  transaction/owner/OFE/tile/surface/source context and raw beginning/attempt
  hashes. Attachment rejects NaN and both infinities in either lane as E003
  after complete identity preflight.
- PASS on the combined worktree: surface-liquid 80/80; unified integration
  39/39; custody authority 10/10; orchestrator quick 590/590; AUTH11 3/3;
  strict affected Clippy; admission with 46 contracts and 23 science surfaces;
  anti-evasion; SC unit compliance; formatting and exact diff hygiene;
  Markdown 77 files with zero errors or warnings.
- Pending: freeze new exact bytes and obtain fresh independent dual review.

## Exact-byte review at `7b208bb26`

- PASS: fresh hydrology/science/ownership review found no material finding and
  passed 77/77 surface-liquid, 10/10 authority, 35/35 unified real-hydrology and
  4/4 receiver tests.
- HOLD: fresh Rust review found two public cross-input seams where lower
  priority configuration/state domain or protocol cardinality/bounds can mask
  higher-priority request identity or receiver arithmetic failure.
- PASS: the same Rust review confirmed the `f249431d4` whole-record precedence
  and shared receiver-fold findings are corrected.
- INTERRUPTED retained: an accidental broad orchestrator run recorded 584
  passes and three SIGINT results. It is not evidence.
- Pending: remediate category-wide cross-input precedence, freeze new bytes and
  repeat fresh dual review before terminal heavy execution.

## `7b208bb26` HOLD remediation

- PASS: unified execution preflights configuration/state schema and identity,
  request identity, production binding and outer transaction/snapshot identity
  before any configuration/state E003 domain validation.
- PASS: finalization now applies protocol E002 identity, then protocol and both
  receiver sets' E003 arithmetic, then E005 cardinality and E006 bounds.
  Cross-set mixed poisons cover both receiver classes and both state-record
  positions.
- PASS on the combined worktree: new reproductions 2/2; unified integration
  37/37; surface-liquid 77/77; custody authority 10/10; orchestrator quick
  587/587; AUTH11 3/3; strict affected Clippy; admission with 46 contracts and
  22 science surfaces; anti-evasion; SC unit compliance; formatting and exact
  diff hygiene.
- Pending: freeze new exact bytes and obtain fresh independent dual review.

## Fourth exact-byte review remediation

- HOLD retained: exact `dd8127b04` Rust and hydrology reviews found joint
  proportional-supply representability, raw attempted hashing, direct public
  taxonomy/precedence, receiver E010/E011 attribution, later-row context and
  release-evidence defects.
- PASS: SC-SURFACELIQUID-001 v6 freezes a common symmetric binary64 scale only
  for aggregate representational overshoot inside the existing mass envelope.
  The ordinary-scale one-ULP counterexample, reverse caller order, exact `F=A`
  ending, three-equal-demand no-priority and three-distinct finalized-use order
  vectors pass.
- PASS: raw attempted configuration/state/parser bytes are completely framed;
  stale embedded digests cannot alias distinct malformed attempts.
- PASS: direct and unified public paths enforce canonical taxonomy and exact
  offender context. Finite independent receiver equations use E010; atomic
  envelope/cardinality uses E011.
- FAIL retained: one exploratory nextest expression selected zero tests. It is
  not evidence; the corrected owner run passed 30/30.
- PASS after integrated remediation: orchestrator quick suite, 574/574;
  focused authority/unified suites, 40/40; owner tests, 30/30; strict affected
  Clippy; SC unit compliance; science admission with 46 contracts and 19
  surfaces; formatting; and diff hygiene.
- Pending: commit exact bytes, fresh dual review, then complete heavy rerun.

## Third exact-byte review remediation

- HOLD retained: the `0e5262b4b` reviews found canonical-last rounding
  priority, incomplete public canonical error coverage and E003 precedence,
  incomplete attempted-state hashes, a false first-attachment beginning hash,
  and incomplete release evidence.
- PASS: every oversubscribed authorization row now retains the exact checked
  `D_i*S/D_sum` result; the three-equal-demand unit-supply vector is bit-exact
  and caller-order independent.
- PASS: unified snapshot, production binding, request partition, authorization
  ordering and final protocol failures now use contextual canonical errors;
  global numeric preflight establishes E003 precedence.
- PASS: attempted hashes bind the beginning hydrology snapshot and every
  thermal receiver operand. First invalid attachment uses an absent beginning
  hash, while replacement retains the actual beginning hash.
- PASS after integrated remediation: orchestrator quick suite, 567/567;
  focused custody and unified LSE/real-hydrology suites, 36/36; affected crate
  checks; strict affected-crate Clippy; formatting; and diff hygiene.
- Pending: commit exact bytes, fresh dual review, then complete heavy rerun.
