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
