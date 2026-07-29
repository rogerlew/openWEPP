# Prospective Science Review A

Status: `PASS — FINAL PROSPECTIVE RE-REVIEW`

Evidence class: `Static: package, contract, baseline-lineage, and accessible
primary-source review; no result-bearing execution`

## Scientific assessment

The central scientific adjudication is sound:

- evergreen live-foliar stock plus longevity can support, at most, a
  conditional gross annual foliage-production estimate; it does not determine
  deposited needle dry mass or deposition timing;
- branch turnover, attached-dead storage, in-canopy loss, branchfall, and
  same-day ground deposition are distinct, and current aggregate structural
  biomass cannot replace the branch/crown/stand operands used by Lim et al.;
- collector-measured, tissue-separated oven-dry material per horizontal ground
  area is an authoritative external surface-boundary quantity at its measured
  temporal support; and
- the existing CP-GSI02 leaf debit and the proposed external needle/wood
  boundary inputs must remain separate internal and external source classes.

The rejected predictive needle and fine-wood formulas should remain rejected.
The proposed boundary interface can be a legitimate implementation target, but
the present packet does not yet authorize contract amendment or production
work.

## Findings

### PRA-001 — `HIGH` — primary-source identities and claim anchors are not auditable

Two citations are assigned to the wrong works:

1. The Kloeppel, Harmon, and Fahey chapter is Chapter 5 of *Principles and
   Standards for Measuring Primary Production*, pages 63–81. The cited DOI
   `10.1007/978-1-4020-8506-2_12` identifies a methane-flux chapter in a
   different book. Use the authenticated Forest Service record
   `https://research.fs.usda.gov/treesearch/28767` and, if a DOI is retained,
   `10.1093/acprof:oso/9780195168662.003.0005`. Its relevant foliage-production
   and resorption qualification is on approximately pages 69–71.
2. Keane's *Surface fuel litterfall and decomposition in the northern Rocky
   Mountains, U.S.A.* is USDA Forest Service RMRS-RP-70 with DOI
   `10.2737/RMRS-RP-70`
   (`https://research.fs.usda.gov/treesearch/29449`). The cited
   `10.1016/j.foreco.2008.07.033` identifies a beech-knot/branch-occlusion
   article. The stated foliage range `0.057–0.144 kg m^-2 yr^-1` also is not the
   full Table 2 range; the table contains values outside both endpoints.

The remaining rows provide broad citations but generally no immutable source
identity or exact page/table/equation anchors. That is insufficient provenance
for new canonical contract authority.

Exact correction:

- correct both bibliographic identities and remove or correct the Keane range;
- add an authenticated source manifest with title, authors, year, DOI or
  authoritative repository URL, immutable checksum, local/remote object
  identity, and exact page/table/figure/equation for every admitted claim;
- anchor White's longevity/turnover values, Bernier et al.'s collector,
  dry-mass, component, and temporal-reporting statements, Keane's collection
  interval/component definitions, and Lim et al.'s main-text and supplement
  claims separately; and
- rerun the admission decision against those exact passages. No production
  constant may be admitted from a citation summary.

### PRA-002 — `HIGH` — exact-date forcing, temporal support, and completeness are conflated

Bernier et al. and Keane support ground-collected dry-mass fluxes at collection
or reporting intervals. They do not turn an interval total into a known
same-day deposition. Keane, for example, used monthly and later semiannual
visits; Bernier et al. recommend reporting annual dry-mass inputs. The packet
correctly prohibits interpolation and annual-to-daily division, but then
authorizes only an exact-date daily schedule and allows
`prescribed_from_cited_source` transformations without defining what temporal
transformation is scientifically admissible.

The sparse-series rule is also incomplete. First/last nonzero entries cannot
unambiguously define a completeness interval; a one-entry series would have
one-day support. One shared support cannot express different needle and
fine-wood coverage. Most importantly, an omitted date is not an authoritative
zero for interval-observed data.

Exact correction:

1. Distinguish `MEASURED_BOUNDARY` from `PRESCRIBED_SCENARIO`. An exact daily
   measured value is admitted only when the source resolves deposition to that
   date. A prescribed scenario may carry exact dates and masses, but it is an
   externally asserted boundary condition, not observed natural timing,
   predictive canopy science, empirical calibration, or validation.
2. Add separate per-tissue status and explicit inclusive
   `support_start/support_end` fields. `complete` must assert full event
   accounting over that interval; `not_represented` must remain incompleteness,
   never numerical zero.
3. Permit sparse omitted dates to mean zero only inside an explicitly complete
   exact-daily measured or prescribed support. Interval measurements remain
   interval quantities and cannot enter the daily interface until an
   independently authoritative disaggregation is supplied.
4. Require a simulation claiming tissue completeness to be contained in that
   tissue's support. Outside-support access fails closed; unused entries and
   partial overlap remain reported.
5. Expand provenance to carry tissue definition, maximum woody diameter/class,
   dry/wet basis, horizontal-area basis, original temporal support and units,
   transformation algorithm/version, and original plus transformed checksums.
   Define checksum algorithm and exact checksum scope; a nonempty digest string
   alone is not authentication.

### PRA-003 — `HIGH` — the mass ledger omits the contract's parallel ground-pool topology

The proposed equation and operand table show only the surface-residue stock.
`SC-RESIDUE-001#INV-RESIDUE-020/021` requires the same areal litter source to
enter the surface, interrill-ground, and rill-ground recurrences before their
common decay and cover derivation. Those are parallel area/state
representations, not three independent physical masses that may be summed.

The statement that the source is “not duplicated into downstream residue
pools” is therefore ambiguous, while rejection vector 8 could incorrectly
reject the contract-required projection itself. The current lineage also does
not state the open-system closure that distinguishes internally debited leaf
mass from externally supplied needle/wood mass.

Exact correction:

- write the surface, interrill, and rill pre/post-decay recurrences explicitly,
  including source-before-decay order and authorized losses/actions;
- label the three pools as parallel contract surfaces and prohibit summing them
  into a three-times source or applying the source more than once within any
  one recurrence;
- freeze the system ledger:
  internal `L_leaf` debit equals its residue credit, while
  `N_ext + W_ext` is an external boundary addition and therefore has no modeled
  canopy debit;
- identify which surface stock drives residue partition/depth and which
  interrill/rill states drive cover/erosion; and
- require independent daily reconstruction of each recurrence, external input,
  decomposition/removal loss, and the real depth/frost/erosion consumer chain.

### PRA-004 — `HIGH` — ADR-0042 status and package closure overstate the boundary-only result

`IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED / PRESCRIBED_BOUNDARY_ONLY` is
not an ADR-0042 status triple. `PRESCRIBED_BOUNDARY_ONLY` is a useful claim
boundary, but it is not an allowed `identifiability_status`. Moreover, the
schedule is an exogenous forcing surface, not a calibrated process parameter;
the package excludes empirical calibration and currently defines no
observation/objective operator that could justify calibration-readiness for
the schedule values.

The package objective says it will close recurring needle and fine-woody source
authority, but the reviewed science closes only an input-interface route.
Predictive natural-forest source generation remains authority-missing.

Exact correction:

- freeze separate status rows for predictive needle deposition, predictive
  fine-wood deposition, and prescribed boundary forcing;
- retain the two predictive rows as
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- for the boundary interface, use the exact ADR fields and keep
  `source_mode=PRESCRIBED_BOUNDARY_ONLY` as a fourth, non-ADR claim-boundary
  field. Unless an applicable calibration operator is prospectively defined,
  its calibration and identifiability statuses should be `NOT_APPLICABLE`;
- narrow package completion language to implementation of an authenticated
  prescribed boundary interface plus explicit source-incompleteness
  disclosure. Do not claim that natural recurring needle/fine-wood generation
  has been implemented or made calibration-ready; and
- require the final disposition to retain the predictive authority hold and
  name the additional live-to-litter/timing and branch/deposition science
  needed to lift it.

## Go / hold disposition

`HOLD` before canonical contract amendment, contract-derived tests, or
production implementation. The predictive needle and fine-wood laws remain
correctly rejected and may not proceed.

After PRA-001 through PRA-004 are corrected and both prospective reviews pass,
contract-first implementation may proceed only for the exact-date,
provenance-bound prescribed surface boundary interface, with per-tissue
support/completeness semantics and the complete three-surface mass ledger.
That implementation is useful and scientifically legitimate as an external
boundary condition; it does not close predictive native-canopy source physics.

## Re-review — 2026-07-28

Status: `FAIL / HOLD`

Evidence class: `Static: exact corrected-packet review; no result-bearing
execution`

### Finding closure

- `PRA-001`: `CLOSED`. The corrected packet separates the Kloeppel chapter,
  Keane journal article, and Keane RMRS-RP-70 report; supplies correct
  bibliographic identities, authenticated-object SHA-256 values, and exact
  claim anchors; removes the incorrect Keane range; and prevents
  unauthenticated White and Bernier passages from determining production
  operands.
- `PRA-002`: `OPEN — HIGH residual`. The packet now correctly separates
  `measured_daily` from `prescribed_scenario`, prohibits execution of interval
  totals as daily deposition, gives needle and fine wood independent explicit
  supports and applicability/completeness states, and defines mode-specific
  omitted-day semantics. However, the payload still does not carry typed
  original temporal support and original units, as the original finding
  required. `transformation.inputs` is an unconstrained placeholder and is not
  an auditable replacement for those fields. The packet also requires
  `source_digest` and `transformed_digest` to look like SHA-256 values but does
  not define the exact byte object or canonical serialization covered by
  either digest. Consequently an implementation or independent test cannot
  determine whether a digest authenticates the cited source, the forcing
  object, the entries alone, or some other representation.
- `PRA-003`: `CLOSED`. The corrected law gives explicit pre-addition and
  post-decay surface, interrill, and rill recurrences; identifies them as
  parallel areal representations rather than additive global mass; preserves
  the internal leaf debit/credit and external needle/wood influx distinction;
  binds the surface and ground states to their real consumers; and requires
  independent reconstruction of sources, losses, weighted ground state,
  depth, frost, cover, and erosion inputs.
- `PRA-004`: `CLOSED`. The objective now adjudicates predictive physics and
  boundary forcing separately. The corrected ADR-0042 rows retain
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED` for both
  predictive stages and use
  `NOT_IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE` for the boundary
  interface, with implementation mode outside the ADR triple. The permitted
  completion claim is limited to that interface.

### Required correction for the residual

Revise the complete-payload schema to:

1. require typed `original_temporal_support` and `original_units` for every
   transformed input, including its observation interval or explicit
   exact-daily resolution;
2. define `source_digest` as SHA-256 over a named immutable byte object and
   state how that object is resolved from `source_uri_or_path`; and
3. define `transformed_digest` over a deterministic canonical byte
   representation, including the included fields, excluded digest field,
   encoding, field ordering, number/date representation, and newline rules,
   or bind it to a separately identified immutable transformed file whose
   exact bytes are hashed.

The direct operator-authored `prescribed_scenario` case must also state
whether it is an identity forcing object or a transformation, so that its
source and transformed digests have one unambiguous meaning. No temporal
disaggregation of interval observations becomes admissible through this
metadata correction.

### Re-review disposition

Contract amendment, contract-derived tests, and production implementation
remain `HOLD` until the `PRA-002` residual is corrected and both prospective
re-reviews pass. After that correction, contract-first work may proceed only
for the authenticated exact-date boundary interface with the reviewed support,
applicability, and three-surface ledger rules.

Predictive native-canopy needle and fine-woody deposition remains on
`AUTHORITY_MISSING` hold. Nothing in this re-review authorizes a live-stock
turnover shortcut, a branch-turnover-to-ground-deposition shortcut, or any
other predictive source physics.

## Final re-review — 2026-07-28

Status: `PASS`

Evidence class: `Static: exact final corrected-packet review; no
result-bearing execution`

### Final finding disposition

- `PRA-001`: `CLOSED`. The authenticated source ledger, corrected
  bibliographic identities, exact anchors, and non-authoritative treatment of
  unauthenticated sources remain intact.
- `PRA-002`: `CLOSED`. Each complete payload now carries typed original
  support start/end, `exact_daily` or `interval` resolution, interval
  definition when applicable, and original units. `source_digest` covers every
  raw byte of the immutable object resolved by `source_uri_or_path`.
  `executable_digest` independently covers every raw byte of the named
  executable forcing file.

  The executable object has a specified UTF-8-without-BOM CSV byte grammar:
  exact header and column order, LF line endings including the final LF,
  ISO-form dates, finite nonnegative base-10 daily masses, unique
  strictly increasing dates, and no in-memory reserialization as the
  authenticated object. These rules provide one executable byte object for
  parser and digest tests.

  A directly authored prescribed scenario is now unambiguously an identity
  payload: source and executable paths resolve to the same file, digests are
  equal, original resolution and units are exact daily
  `kg_dry_mass_m2_day`, and no transformation metadata is allowed. A derived
  payload instead requires transformation identity/version, all input
  identities and digests, algorithm authority, and a distinct authenticated
  executable object. The packet continues to prohibit assignment,
  interpolation, uniform division, repetition, extrapolation, or
  climatological wrapping of interval totals. Derivation metadata alone
  cannot authorize interval disaggregation; separately admitted temporal
  authority is mandatory.
- `PRA-003`: `CLOSED`. The parallel surface/interrill/rill recurrences,
  internal/external closure, and real-consumer reconstruction requirements
  remain intact.
- `PRA-004`: `CLOSED`. The separate ADR-0042 rows and boundary-only
  implementation modes remain correct, mutually exclusive, and distinct from
  predictive canopy physics.

All `PRA-001` through `PRA-004` findings are closed. No finding is waived.

### Proceed boundary

Prospective science review A now authorizes contract-first work for only the
authenticated external daily boundary interface:

- operator-asserted exact-date `prescribed_scenario` forcing; or
- genuinely exhaustive, authenticated `measured_daily` forcing.

Proceeding remains subject to the package's second prospective re-review,
canonical contract amendment, contract-derived tests, and pre-implementation
gate before production edits. Implementation must preserve per-tissue
applicability and independent complete support, raw-byte authentication,
fail-closed temporal and spatial binding, external-source labeling, and the
reviewed parallel surface/interrill/rill ledger through real consumers.

Predictive evergreen needle deposition and predictive fine-woody deposition
remain `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`. This `PASS`
does not authorize stock-over-longevity deposition, branch-turnover
deposition, an invented timing distribution, or any other native predictive
litter-source law.

## Independent Terminal Science and Implementation Review A — 2026-07-28

Status: `FAIL / HOLD`

Evidence class: `Static: exact current implementation, contract, test, and
package-evidence review; Ran: focused contract-derived suite, 6 passed, 0
failed`

### Positive determinations

- The implementation does not introduce a predictive needle or fine-woody
  source equation. The ADR-0042 predictive rows correctly remain
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.
- The implemented prescribed identity happy path verifies raw source and
  executable SHA-256 bytes before parsing, requires dry-mass/horizontal-area
  units, checks site and OFE binding, and publishes leaf, needle, fine-woody,
  and total operands separately.
- The runtime forms `Q = L + N + W` once and passes that one source operand to
  the existing decomposition input. The real-run test independently
  reconstructs the no-action surface, interrill, and rill recurrences. The
  external operands have no modeled canopy debit.
- Current touched/new Rust files remain below the 3,000-line closure limit.
  `00_builders_and_authority.rs` is 2,998 lines and therefore remains a
  package-governed warning, not an exemption from future decomposition.

### Numbered findings

1. `TRA-001 — HIGH — typed tissue/material compatibility is not enforced`.
   `validate_payload` requires only that
   `species_or_functional_type` occur in the top-level class list. It does not
   require a complete needle payload to be needleleaf, and it does not reject
   a complete fine-woody payload classified as `non_woody`. Consequently,
   inputs that contradict the tissue named by the field pass the advertised
   material/applicability guard. The vegetation authority record also accepts
   any nonempty `checksum` string without a named immutable object, digest
   algorithm, or byte verification, so the classification that authorizes
   `not_applicable` is not authenticated to the same standard as the forcing.
   Closure requires tissue-specific class guards and an authenticated
   classification identity, with negative tests.

2. `TRA-002 — CRITICAL — interval/derived inputs bypass the reviewed temporal
   authority boundary`. `ForestLitterDerivation.inputs` is an arbitrary list
   of strings rather than authenticated input-object identities and digests,
   and `transformation_authority` is accepted when merely nonempty. Any
   interval source is admitted when any such derivation exists; no guard
   establishes the separately admitted temporal-disaggregation authority
   required by the prospective reviews and `INV-PLANT-039`. Derived source and
   executable objects also need not be distinct. This permits precisely the
   interval-to-daily shortcut the authority packet prohibits. Either remove
   the derived/interval route from this increment or implement the complete
   typed input ledger and fail closed unless a canonical admitted temporal
   authority binding is present. Add acceptance and rejection vectors for
   that route.

3. `TRA-003 — HIGH — source support can contradict executable support`.
   Identity payload validation requires equal path and digest, but it does not
   require `original_observation.support_start/support_end` to equal the
   executable payload support. The same authenticated byte object can
   therefore be declared as a one-day original observation and executed under
   a year-long support claim. Access/version dates are likewise checked only
   for nonemptiness. Identity payloads need exact support equality; derived
   payloads need an authority-backed, tested support transformation.

4. `TRA-004 — HIGH — `not_represented` is numerically executed as zero`.
   When forcing is absent, and when a tissue explicitly declares
   `not_represented`, the runtime returns `0.0`, includes it in the source
   sum, and publishes a numeric zero in the research output. The adjacent
   status string preserves a disclosure label, but it does not satisfy the
   reviewed rule that unrepresented material is incompleteness and never an
   authoritative numerical zero. A numeric zero must remain reserved for an
   explicit supported measured/scenario zero (or an authority-backed
   `not_applicable` neutral contribution). The runtime/output contract must
   preserve the missing value as missing while separately carrying the
   model-envelope completeness state; compatibility execution must not be
   presented as authenticated zero deposition.

5. `TRA-005 — HIGH — required contract vectors and real-consumer closure are
   incomplete`. The six-test contract-derived suite covers contract anchors,
   one prescribed identity success, digest drift, interval-as-measured mode,
   measured-daily row exhaustiveness, and CRLF rejection. It does not test the
   contract-required tissue/class applicability failures, authority and
   derivation provenance, original/executable support conflict, outside-
   support runtime access, site/OFE mismatch, missing versus inapplicable
   versus measured/scenario zero, or negative omitted/duplicate source
   projection. The real-run test reconstructs `Q` and the three no-action pool
   recurrences, but it does not reconstruct the weighted ground state, cover,
   mass-to-depth conversion, or external-source effect at the frost and
   residue-cover erosion consumers. Its erosion assertions cover canopy
   height and canopy cover, not interrill/rill residue cover. The evidence
   artifact merely states that those consumers “remain” downstream and does
   not provide the required producer → state/frame → runner handoff →
   downstream call-site lineage or the negative old-path check.

6. `TRA-006 — HIGH — terminal gate evidence is not exact-head or internally
   reconciled`. The measured-daily exhaustiveness guard and sixth contract
   test landed after the recorded workspace Clippy/full-correctness results.
   `gate-results.md` still reports the contract suite as 5/5, while the exact
   current focused run is 6/6. `line-count-governance.md` still reports 797
   lines for `forest_litter.rs` and 184 for the integration test; the current
   counts are 853 and 196. Markdown validation, exact-diff reconciliation,
   dual terminal gates, and terminal finding disposition are pending, and
   `final-disposition.md` still describes the pre-implementation stop before
   contract and production edits. These stale/pending records cannot support
   terminal closure under the package non-deferral rule.

### Terminal disposition

The exact tree supports only an implementation-in-progress finding. The
prescribed identity happy path and three-pool source recurrence are promising,
and the predictive-physics stop-loss is intact, but findings `TRA-001` through
`TRA-006` are closure-blocking. Package disposition remains `HOLD` until the
authority bypasses, missing-value semantics, contract vectors, real consumer
proof, and exact-head evidence are corrected and independently re-reviewed.

## Final Terminal Re-review A — 2026-07-28

Status: `PASS`

Evidence class: `Static: corrected exact implementation, canonical contracts,
package evidence, line counts, and gate record; Ran: contract-derived suite
and focused real-consumer/source-guard tests`

The immutable terminal `FAIL / HOLD` and `TRA-001` through `TRA-006` findings
above remain the record of the first reviewed snapshot. This section records
the independent disposition of the corrected snapshot.

### Finding closure

1. `TRA-001 — CLOSED`. Vegetation applicability now binds to a confined,
   SHA-256-authenticated canonical classification CSV. The inline class list
   must exactly match that object. Complete needle inputs require a needleleaf
   class, complete fine-woody inputs reject `non_woody`, and contradictory
   `not_applicable` declarations fail against the authenticated classes.

2. `TRA-002 — CLOSED`. This increment no longer admits derived or interval
   objects. Every executable payload requires an `exact_daily` original,
   `derivation: none`, and identical source/executable path and digest.
   Therefore free-text derivation metadata cannot authorize temporal
   disaggregation. A future transformation route requires a separate
   contract-first authority increment.

3. `TRA-003 — CLOSED`. Identity payloads require original and executable
   supports to match exactly. Authority access/version dates are parsed as
   dates, and dry-mass provenance requires a positive duration or explicit
   constant-mass criterion in addition to finite drying temperature.

4. `TRA-004 — CLOSED`. Unrepresented and inapplicable tissue operands publish
   as JSON `null`, while `source_completeness` distinguishes an incomplete
   ledger. The internal additive identity used to preserve compatibility
   execution is not published or claimed as authenticated zero deposition.
   Complete prescribed and exhaustive measured-daily zeroes remain
   distinguishable by status and source mode.

5. `TRA-005 — CLOSED`. The contract-derived suite now contains 16 tests
   covering both admitted modes and the applicable authentication,
   interval/derivation, exhaustiveness, byte grammar, date/mass, class/
   material, support, drying, spatial, path, and status rejection families.
   The real native run independently reconstructs `Q`, all three parallel
   recurrences, weighted ground mass, interrill/rill/composite cover, and
   residue depth. It proves that active erosion reads the exact reconstructed
   interrill/rill cover operands and active frost reads the exact depth. The
   source guard proves one decomposition handoff and no downstream needle or
   fine-woody re-addition.

6. `TRA-006 — CLOSED`. Terminal artifacts now record 16/16 contract tests,
   corrected line counts, successful warnings-denied workspace Clippy,
   the type-size layout guard, and an exact-head full profile with 2,117
   passed tests, 29 profile-declared skips, and 757.402 seconds elapsed.
   Threshold-crossing Rust files were extracted; no touched nonexempt file is
   3,000 lines or longer. The declared write set now includes the root Cargo
   manifest and lockfile.

### Independent execution

Ran:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `16 passed, 0 failed`.

Ran separately:

```text
cargo nextest run -p openwepp-runner \
  native_forest_yaml_executes_through_the_direct_production_consumer

cargo nextest run -p openwepp-runner \
  canopy_phenology_02_real_consumers_share_the_typed_native_state
```

Result: `1 passed, 0 failed` for each focused run.

### Final scientific disposition

No blocking or major finding remains for the authenticated identity-only
external daily boundary. Terminal review A passes the implementation,
authority, conservation, output, real-consumer, ADR-0042 claim-limit, and
line-count gates.

This `PASS` is exclusively for prescribed exact-day or authenticated,
exhaustive measured-daily external boundary input. It does not authorize a
predictive canopy source law, an interval-to-daily transformation, empirical
calibration, independent validation, or a natural-source sufficiency claim.
Predictive evergreen needle and fine-woody deposition remain:

```text
AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED
```

Package completion still requires the other independent terminal re-review
and re-verifications plus final documentation lint after their append-only
artifacts are complete.
