# Prospective Science Review B

Status: `PASS`

Evidence class: `Static`

Review scope: prospective review of the source-admission matrix and proposed
authority law against ADR-0042, SC-PLANT-001 CP-GSI02, SC-RESIDUE-001, and the
identified primary-source records. No production implementation or runtime
validation was reviewed.

## Disposition

`HOLD`

The proposed separation between predictive canopy generation and prescribed
ground-boundary forcing is scientifically sound. In particular, the package
correctly rejects evergreen foliar turnover as equivalent to needle
deposition, rejects branch turnover as equivalent to fine-woody deposition,
and does not debit externally supplied deposition from a modeled canopy pool.

Implementation must not proceed from the current candidate, however. The
candidate does not yet preserve the measurement support of interval
litter-trap observations, distinguish missing material from measured zero, or
bind enough material and authority metadata to make a schedule scientifically
auditable. Its conservation topology and ADR-0042 claim tuple also require
correction before contract amendment.

## Findings

### B-01 — Interval measurements do not authorize exact-date deposition

Severity: `BLOCKING`

The admitted Bernier and Keane measurement designs establish dry mass
collected over an observation interval. They do not establish the exact day
on which each item reached the ground. The candidate boundary instead requires
exact daily deposition, treats omitted dates inside a series as zero-event
days, and permits a `prescribed_from_cited_source` method without an admitted
temporal allocation law. Assigning an interval total to the collection date,
or distributing it across days, invents timing that can change decomposition,
cover, residue depth, frost, and erosion results.

Exact correction:

1. Restrict a daily `measured` schedule to observations whose acquisition
   method genuinely resolves daily ground deposition.
2. Represent interval observations with explicit `support_start`,
   `support_end`, tissue-specific interval mass, and collection-method
   metadata. Do not execute them as daily forcing until an admitted temporal
   allocation law exists.
3. Alternatively, require a pre-transformed daily schedule to cite the exact
   primary temporal authority and identify the transformation algorithm,
   version, source-object checksum, and transformed-object checksum.
4. Never infer an observed zero from an unsampled date. A zero-event day is
   admissible only inside an explicitly declared, exhaustive daily observation
   support.

### B-02 — Missing, not represented, not applicable, and measured zero collapse

Severity: `BLOCKING`

Both numeric tissue fields are required on every entry while the only proposed
statuses are `complete` and `not_represented`. That makes a missing or
unrepresented tissue appear numerically as zero. In addition, the first and
last events do not define observation support: the first or last observed
nonzero deposition may occur after or before the actual sampling boundary.

Exact correction:

- Give needle and fine-woody material independent payloads and independent
  statuses, including at least `complete`, `not_represented`, and
  `not_applicable`.
- Prohibit a numeric series for `not_represented`; require one for `complete`;
  and define `not_applicable` through typed vegetation/material applicability,
  not operator assertion alone.
- Preserve explicit measured zeros only as values inside a complete,
  exhaustively observed temporal support.
- Declare support start, support end, calendar, and sampling resolution for
  each tissue. Do not derive support from the first and last event.
- Fail closed or disclose source incompleteness for every simulation day not
  covered by a tissue's declared support, including calendar and leap-day
  mismatches.

### B-03 — Authority and material identity are under-specified

Severity: `BLOCKING`

`source`, `version`, `checksum`, and a two-value `method` enum do not preserve
the conditions under which an external flux is authoritative. The proposed
needle field does not identify the admitted plant material; the fine-woody
field does not define its maximum diameter or inclusions. “Oven-dry” is
asserted but its method is not bound. A checksum of only the transformed
schedule cannot authenticate the source object or the transformation.

Exact correction: require typed metadata for:

- full source identity, source-object URI or repository path, access/version
  date, digest algorithm and digest, and exact page, table, field, or method
  anchor supporting the admitted claim;
- transformation identity, algorithm/script version, transformation inputs,
  source-object digest, and transformed-schedule digest;
- dry-mass method, including drying temperature, duration or constant-mass
  criterion, and any moisture or carbon conversion, which must itself be
  authorized;
- horizontal collector or ground area basis, spatial support, plot/site/OFE
  binding, collection interval, temporal support, calendar, and units;
- needle/foliar material definition, including species or functional type and
  whether evergreen broadleaf material, cones, bark, and miscellaneous litter
  are excluded; and
- fine-woody material definition, including diameter threshold, twig/branch
  classes, bark treatment, and exclusions.

`prescribed_from_cited_source` must reject undocumented transformations, not
merely state that openWEPP does not endorse them.

### B-04 — Configuration obligations are biologically overbroad

Severity: `BLOCKING`

`evergreen_fraction > 0` does not establish that the vegetation produces
needles; evergreen broadleaf systems are possible. Aggregate structural
biomass does not identify a fine-branch pool or prove that a fine-woody
deposition boundary is complete. The proposed tests would therefore confuse
model-envelope declarations with biological presence.

Exact correction:

- Bind source obligations to a typed represented-material and vegetation
  functional-class declaration.
- Add `not_applicable` and define the evidence needed to select it.
- Do not infer a needle obligation from evergreen fraction alone or a
  fine-woody obligation from aggregate structural biomass alone.
- Require the forcing applicability record to match the simulated material,
  species or functional type, stand/site, spatial support, and temporal
  support.
- Keep predictive needle and fine-wood publication unavailable until their
  own required canopy/branch state and admitted transformations exist.

### B-05 — Conservation topology is ambiguous

Severity: `BLOCKING`

The candidate says the combined source is applied once and rejects duplicate
application to surface, interrill, and rill stores. SC-RESIDUE-001 currently
projects the litter source into the surface-residue state and the interrill
and rill ground-residue representations. Those projections can be correct
parallel area-specific representations, but the current wording can also be
read as either prohibiting required projections or authorizing three additive
global masses. “Applied once” is not yet an auditable conservation law.

Exact correction:

1. Add an explicit topology table from one authenticated source-event ledger
   through each surface, interrill, and rill projection, with units and area
   bases.
2. State which stores are parallel representations and therefore must never
   be summed as independent additions to global mass.
3. Extend operand lineage for `N_ext` and `W_ext` through all SC-RESIDUE
   destinations and real cover, depth, frost, and erosion consumers.
4. Define an independently reconstructable daily closure that includes
   external boundary influx, internal leaf debit/credit, decomposition,
   removal, and remaining stock.
5. Test the single ledger handoff and every required projection separately,
   including a negative test for pre-addition plus downstream re-addition.

The decision not to debit `N_ext` or `W_ext` from an unmodeled canopy pool is
correct, but those operands must be recorded as external boundary influx in
the closure.

### B-06 — The proposed ADR-0042 claim tuple is invalid and misleading

Severity: `BLOCKING`

`PRESCRIBED_BOUNDARY_ONLY` is an implementation mode, not an ADR-0042
identifiability status. More importantly, supplied daily boundary values are
exogenous observations or prescriptions, not calibrated process parameters.
`CALIBRATION_READY_DATA_LIMITED` therefore overstates the result unless the
package identifies an actual estimable parameter, objective, and evidence
path.

Exact correction:

- For the implemented prescribed-boundary interface, use
  `science_implementation_status = IMPLEMENTED`; use ADR-0042's canonical
  not-applicable values for calibration evidence and identifiability if the
  contract admits them; and record
  `implementation_mode = PRESCRIBED_BOUNDARY_ONLY` separately.
- If ADR-0042 does not admit not-applicable values, amend or adjudicate the
  status vocabulary before claiming implementation rather than inventing a
  third-axis value.
- Keep predictive needle deposition and predictive fine-woody deposition as
  separate claims with `AUTHORITY_MISSING`, `NOT_CALIBRATION_READY`, and
  `NOT_ASSESSED`, using the exact canonical enum spellings.
- State explicitly that implementing a forcing interface does not implement
  source biology or validate a predictive flux law.

### B-07 — Primary-source anchors are not yet auditable

Severity: `MAJOR`

The admission matrix supplies mostly author/year/DOI-level identity but not
the exact pages, tables, methods, source objects, or immutable local records
that support each admitted operand. The Keane entry is especially ambiguous:
DOI `10.1139/X08-003` identifies the Canadian Journal companion article,
whereas the Forest Service surface-fuel report is RMRS-RP-70, DOI
`10.2737/RMRS-RP-70`. The stated component separation and numerical range
must be anchored to the source that actually contains them. The pinned legacy
topology claim likewise needs file-and-line provenance.

Exact correction:

- Add an authority-source ledger with complete citations, exact
  page/table/section or dataset-field anchors, immutable object identity,
  checksum, access date, and local evidence path.
- Resolve which Keane source supports each admitted method and rate claim;
  cite the companion article and RMRS-RP-70 separately where both are used.
- Add exact pinned-baseline source-file and line anchors for the accepted
  topology.
- Reconcile each admitted claim against that ledger before contract amendment.

## Recommendation

Keep predictive canopy generation on `HOLD`. Retain prescribed
tissue-specific ground deposition as the preferred implementation direction,
but do not authorize production edits until B-01 through B-06 are corrected
and B-07 is made auditable. After correction, repeat prospective science
review before amending SC-PLANT-001 or SC-RESIDUE-001.

## Prospective Re-review

Date: `2026-07-28`

Evidence class: `Static`

Re-review status: `FAIL`

Re-review disposition: `HOLD`

The corrected packet closes the central scientific defects. Interval
litter-trap totals are now explicitly non-executable in the daily interface;
measured and prescribed scenarios have distinct zero semantics; each tissue
has independent support and status; source objects and admitted claims are
anchored; and the residue ledger now treats surface, interrill, and rill as
parallel areal projections rather than three additive global masses.

Three exact-text residuals remain. They are narrow corrections, but the
package requires both prospective reviews to pass before contract amendment,
so contract-first external-boundary work may not yet proceed beyond correcting
the review packet. Predictive needle and fine-woody biology remains
independently held regardless of these interface corrections.

### Finding-by-finding result

| Finding | Re-review result | Basis |
| --- | --- | --- |
| `B-01` | `CLOSED` | `measured_daily` requires exhaustive daily acquisition, interval totals are non-executable, prohibited temporal allocations are explicit, and prescribed scenario zeros are not represented as observations. |
| `B-02` | `CLOSED` | Needle and fine wood have independent `complete`, `not_represented`, and authority-backed `not_applicable` states; only complete material carries a payload; support is explicit and independent of nonzero event dates. |
| `B-03` | `RESIDUAL` | Source, material, dry-mass, spatial, temporal, and digest fields are now present, but the transformation block does not consistently distinguish an original payload from a derived payload. |
| `B-04` | `RESIDUAL` | Typed vegetation, material, site, and OFE fields exist, and invalid biomass proxies are prohibited, but no rule requires the complete payload's material and spatial declarations to agree with the simulated vegetation and active OFE. |
| `B-05` | `CLOSED` | The open-system ledger identifies external influx, internal leaf debit/credit, independent `S`/`I`/`R` recurrences, area-weighted ground state, real consumers, and reconstruction/duplicate-addition tests. |
| `B-06` | `RESIDUAL` | The ADR-0042 triples are corrected and predictive claims remain separate, but the final implementation-mode claim does not cover the admitted measured-daily mode exactly. |
| `B-07` | `CLOSED` | The authority ledger separates the Keane journal article from RMRS-RP-70, supplies authenticated hashes and exact anchors for binding sources, anchors baseline topology by file and lines, and prevents unauthenticated White/Bernier passages from governing production. |

### RB-01 — Transformation metadata has contradictory applicability

Maps to: `B-03`

Severity: `BLOCKING`

The candidate says:

```text
transformation:
  identity: <none for direct daily observations; otherwise required>
  ...
  transformed_digest: <64 lowercase hex>
```

This makes a transformation mandatory for every `prescribed_scenario`, even
when an operator authors the authenticated scenario directly, while appearing
to require a transformed digest even when direct daily observations have no
transformation. The rule therefore cannot unambiguously validate original
versus derived payloads.

Exact correction:

- Make transformation presence depend on derivation, not on measured versus
  prescribed mode.
- For an original authenticated payload, require no transformation object and
  prohibit a transformed digest.
- For a derived payload, require transformation identity, version, inputs,
  source digest, and transformed-payload digest in either mode.
- State which digest authenticates the executable entries.

### RB-02 — Complete-payload applicability is declared but not enforced

Maps to: `B-04`

Severity: `BLOCKING`

The candidate carries top-level `vegetation.functional_classes`,
payload-level `material.species_or_functional_type`, and
`spatial_support.site_or_plot`/`ofe_binding`. Its rules constrain
`not_applicable`, but none requires a `complete` payload to match the
represented vegetation or the active lane/OFE. Merely carrying both sides of
the relationship does not bind them.

Exact correction:

- Require every complete payload's material class and species or functional
  type to be compatible with the authority-backed top-level vegetation
  classification.
- Require its site/plot and OFE binding to resolve to the active simulation
  lane and spatial support.
- Fail closed on material, vegetation, site, plot, lane, or OFE mismatch.
- Add those mismatches to the contract-derived rejection vectors.

### RB-03 — The permitted mode claim is incomplete

Maps to: `B-06`

Severity: `BLOCKING`

The authority table admits both `PRESCRIBED_BOUNDARY_ONLY` and genuinely
exhaustive `MEASURED_DAILY_BOUNDARY`, but the “Permitted claim after
implementation” block hard-codes:

```text
implementation_mode = PRESCRIBED_BOUNDARY_ONLY
```

That is exact only for prescribed scenarios and would mislabel an implemented
measured-daily path.

Exact correction: make the post-implementation mode conditional and
exclusive:

```text
implementation_mode = PRESCRIBED_BOUNDARY_ONLY
```

for prescribed scenarios, or:

```text
implementation_mode = MEASURED_DAILY_BOUNDARY
```

for authenticated exhaustive daily observations. The ADR-0042 triple remains
`IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE` for either external-boundary
mode. Neither mode changes the predictive rows.

## Re-review recommendation

Retain the top-level `FAIL / HOLD` until RB-01 through RB-03 are corrected.
After those text corrections, this reviewer finds no remaining scientific
obstacle to contract-first implementation of the authenticated external daily
boundary interface. Predictive evergreen needle deposition and predictive
fine-woody deposition must remain
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED` and outside the
implementation claim.

## Final Prospective Re-review

Date: `2026-07-28`

Evidence class: `Static`

Final re-review status: `PASS`

Final disposition: `PROCEED` for the contract-first authenticated external
daily boundary interface only.

This final disposition supersedes the earlier prospective `FAIL / HOLD`
dispositions after verifying their recorded corrections. The original
findings remain historical evidence of the review sequence.

### Residual verification

| Residual | Result | Exact verification |
| --- | --- | --- |
| `RB-01` / `B-03` | `CLOSED` | Transformation presence now depends on derivation rather than boundary mode. An original payload requires `derivation: none`, identical source/executable files and digests, and no transformation metadata. A derived payload in either mode requires transformation identity, version, all input identities/digests, algorithm authority, and a byte-authenticated executable digest. The immutable UTF-8 CSV grammar and SHA-256 byte scope identify the exact executable entries. |
| `RB-02` / `B-04` | `CLOSED` | Every complete payload must be compatible with the authority-backed vegetation and represented material, and its site/plot/spatial/lane/OFE binding must resolve to the active simulation lane/OFE. Every named mismatch fails closed and appears in the contract-derived rejection vectors. |
| `RB-03` / `B-06` | `CLOSED` | `PRESCRIBED_BOUNDARY_ONLY` applies exclusively to `prescribed_scenario`; `MEASURED_DAILY_BOUNDARY` applies exclusively to authenticated exhaustive `measured_daily`. Both retain the exact ADR-0042 triple `IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE` after implementation. |

### Complete finding disposition

`B-01` through `B-07` are all `CLOSED`. The corrected authority packet now:

- keeps measured interval observations non-executable without separately
  admitted temporal-disaggregation authority;
- preserves independent per-tissue status, support, material, and
  missing-versus-zero semantics;
- authenticates binding source and executable objects with exact anchors,
  identities, and raw-byte digests;
- enforces vegetation, material, site, plot, lane, and OFE applicability;
- preserves one external source ledger through parallel surface, interrill,
  and rill areal projections with independently reconstructable closure; and
- separates external-boundary implementation claims from predictive canopy
  biology and from calibration or identifiability claims.

### Proceed boundary

The package may proceed in its declared order with canonical contract
amendment, contract-derived tests, the pre-implementation contract gate, and
then implementation of the authenticated external daily boundary interface.
This `PASS` does not authorize skipping those gates, does not establish that
the interface is already implemented, and does not authorize an interval
observation as exact-day forcing.

Predictive evergreen needle ground deposition and predictive fine-woody
ground deposition remain on scientific `HOLD` as
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`. Implementing the
external boundary interface must not be described as implementing,
calibrating, identifying, or validating either predictive biological law.

## Independent Terminal Science and Implementation Review B

Date: `2026-07-28`

Evidence class: `Static + Ran`

Terminal review status: `FAIL`

Terminal disposition: `HOLD`

Reviewed snapshot: the current package/contracts/schema/runtime/output/tests
after exhaustive `measured_daily` enforcement was added. The prospective
review history above remains immutable.

The implementation correctly keeps predictive evergreen-needle and
fine-woody deposition outside the production claim. I found no stock-times-
turnover surrogate, broad biome default, branch-mortality substitution,
pooled-carbon conversion, or canopy debit for the external `N_ext` and
`W_ext` inputs. The ADR-0042 matrix correctly retains both predictive rows as
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`, while the external
boundary rows are `IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE`.

The typed path also verifies the forcing-file digest before parsing, enforces
the canonical CSV grammar, separates tissue statuses, checks functional
class/site/OFE bindings, publishes the three source operands, and sends their
sum through the common surface/interrill/rill decomposition input. The real
fixture independently reconstructs those three parallel recurrences. I ran:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `6 passed, 0 failed`.

Those strengths do not close the following binding findings.

### TB-01 — Authority-backed classification and derivation are not authenticated

Severity: `BLOCKING`

`ForestVegetationAuthority.checksum` is accepted as any nonempty string. It
has no digest algorithm, source-object path, claim anchor, or byte
verification. Both `not_applicable` decisions depend on this record, so a
payload such as the fixture's `checksum: fixture-classification` is treated as
authority-backed without authentication.

Likewise, an interval original becomes executable when `derivation` exists
and `transformation_authority` is merely nonempty. `derivation.inputs` is an
untyped list of strings and does not require each input identity and digest.
This contradicts the reviewed rule that metadata alone never authorizes
interval-to-daily disaggregation and the final prospective claim that every
derivation carries all input identities/digests and an admitted algorithm
authority.

Required correction:

1. Authenticate the vegetation-classification source and exact claim bytes,
   or bind applicability to an admission-controlled authority identifier.
2. Prohibit interval-derived daily execution in this package unless a
   separately admitted temporal authority is represented by a typed,
   verifiable binding; a free-form nonempty string is insufficient.
3. Type and verify every derivation input identity/digest and the exact
   transformation authority/version.

### TB-02 — Support and dry-mass provenance can contradict the executable claim

Severity: `BLOCKING`

The executable `support_start`/`support_end` is not reconciled with
`original_observation.support_start`/`support_end`. Even an identity payload
can therefore declare executable support outside its authenticated original
support while satisfying the same-file/same-digest check. That permits the
unsupported overlap or extrapolation the authority packet explicitly
prohibits.

The `oven_dry` state also accepts a temperature with neither a drying duration
nor a constant-mass criterion. The prospective review required one of those
method endpoints to make dry-mass identity auditable.

Required correction: require identity support to equal original support;
constrain derived support to the admitted transformation without
extrapolation; and require a complete drying endpoint for every accepted
dry-mass state.

### TB-03 — `not_represented` is still published as numeric zero

Severity: `BLOCKING`

`direct_production_tissue_litter` returns numeric `0.0` for
`not_represented`, and the research schema requires the corresponding litter
operand to be a finite number. This conflicts with `SC-PLANT-001` revision 25:
“`not_represented` is disclosed incompleteness, never numeric zero.” It also
allows `total_litter_input_kg_m2` to look complete while one or both applicable
tissues are unknown; no aggregate source-completeness field prevents that
interpretation.

The internal numerical recurrence may use no contribution for an unavailable
optional boundary, but publication must preserve epistemic state. Publish a
nullable/typed unknown operand or an explicit incomplete total/ledger status,
and add a negative vector proving an incomplete source cannot be labeled
source-complete.

### TB-04 — The contract-derived failure suite is materially incomplete

Severity: `BLOCKING`

The six-test suite proves contract anchors, one prescribed identity payload,
wrong executable digest, interval-as-`measured_daily`, missing measured dates,
and CRLF rejection. It does not prove most of the fourteen rejection-vector
families frozen before implementation, including:

- authenticated exhaustive measured-daily acceptance and explicit measured
  zero;
- status/payload contradictions and authority-backed `not_applicable`;
- material, functional-class, site, plot/lane, and active-OFE mismatches;
- malformed dates, negative/nonfinite mass, outside support, leap/calendar
  cases, path escape, and source-versus-executable digest scope;
- identity/derived support and derivation contradictions;
- zero/duplicate recurrence projection, parallel-state summation, external
  canopy debit, and source-completeness mislabeling.

The broad workspace run cannot substitute for contract-derived bindings when
no existing tests exercise these new schema/runtime obligations. Add direct
positive and negative vectors for every applicable family or disposition a
family with exact existing test evidence.

### TB-05 — Real cover/erosion consumer proof stops before the consumer operands

Severity: `BLOCKING`

The runtime fixture proves the external source reaches the post-decay
interrill and rill masses, and static inspection shows residue partition
derives cover from those masses. It also executes erosion. However, the trace
publishes only the erosion canopy-cover operand, and the test asserts only
erosion canopy cover/height. It does not independently reconstruct
`1-exp(-cf*I_next)` and `1-exp(-cf*R_next)`, capture the interrill/rill cover
operands passed to `DirectWave1DailyState`, or prove those exact values are
consumed by the real erosion call.

This is producer-plus-adjacent-state evidence, not the package's required real
cover/erosion consumer closure. Publish/capture the two residue-cover
operands, independently reconstruct them from the authenticated fixture, and
assert the real erosion consumer receives those values. The existing
surface-to-depth and active-frost checks are otherwise credible.

### TB-06 — Terminal gate evidence is not exact-head or internally current

Severity: `BLOCKING FOR CLOSURE`

The exhaustive measured-daily schema/test edit postdates the recorded
warnings-denied Clippy and full-profile evidence. Therefore
`gate-results.md` and `kernel-profile-compliance.md` cannot truthfully call
those runs exact-head for this reviewed snapshot. The contract-test artifact
also still reports `5/5`, and line-count governance reports
`forest_litter.rs` at 797 lines while the reviewed file is now longer.
Markdown validation, dual verification, exact-diff reconciliation, and final
implementation disposition also remain pending; `final-disposition.md` still
describes the earlier authority-synthesis stop before contract/production
edits.

Required correction: after all implementation/review findings are fixed,
refresh line counts and package artifacts, run the applicable exact-terminal-
diff gates, complete both verifications and Markdown validation, and replace
the stale synthesis-only final disposition with a truthful terminal
implementation disposition. Any non-pass current-scope gate retains `HOLD`.

## Terminal recommendation

Retain package `HOLD` and do not describe the authenticated boundary as
closed. TB-01 through TB-05 require implementation/test correction; TB-06
requires fresh terminal evidence afterward. The predictive-science stop-loss
remains correctly applied and must not be loosened while closing these
interface defects.

## Final Terminal Re-review B

Date: `2026-07-28`

Evidence class: `Static + Ran`

Final terminal re-review status: `PASS`

Final terminal disposition: `PROCEED` to re-verification and documentation
closure.

This section supersedes the terminal `FAIL / HOLD` disposition above only
after inspection of the corrected exact tree. The failed review and findings
remain immutable evidence.

### Correction verification

| Finding | Result | Exact verification |
| --- | --- | --- |
| `TB-01` | `CLOSED` | Vegetation functional classes now come from a separately SHA-256-authenticated canonical classification CSV and must exactly match the inline typed list. Needle/fine-wood compatibility is checked against that authenticated class. This increment now rejects every derived payload and every interval original; a free-form transformation citation can no longer authorize execution. |
| `TB-02` | `CLOSED` | Identity source/executable path and digest must match, original resolution must be `exact_daily`, and original/executable support must match exactly. Authority access/version dates are parsed. Every dry-mass state now requires positive drying duration or a nonempty constant-mass endpoint; `dry_to_constant_mass` specifically requires the criterion. |
| `TB-03` | `CLOSED` | Research publication uses nullable tissue operands: only `complete` publishes a number, while `not_represented` and `not_applicable` publish null. `source_completeness` is independently validated as `incomplete` whenever either tissue is `not_represented`; inapplicable tissue remains a valid complete-domain declaration. |
| `TB-04` | `CLOSED` | The contract-derived suite now contains 16 vectors. It accepts authenticated prescribed identity and exhaustive measured-daily inputs, including explicit measured zero, and rejects digest/classification drift, interval or derived execution, missing measured dates, noncanonical bytes, duplicate dates, negative mass, material mismatch, support conflict, incomplete drying provenance, site/OFE mismatch, lexical path escape, and numeric payload on an unrepresented tissue. Runtime/source-guard vectors cover outside-support access, incomplete publication, exact projection, and no re-addition. |
| `TB-05` | `CLOSED` | The real fixture reconstructs `Q`, all three post-decay pools, weighted ground mass, interrill/rill/composite cover, and residue depth from published operands. `DirectErosionDailyConsumers` records the exact interrill/rill covers passed through `DirectWave1DailyState`; `DirectFrostDailyConsumers` records the exact thermal residue depth. The fixture equates those real consumer operands to the independent reconstruction, while a source guard proves the external pair occurs once in the decomposition handoff and has no downstream re-addition. |
| `TB-06` | `CLOSED` | Terminal evidence was refreshed after all corrections: warnings-denied workspace Clippy passed; the type-size guard passed; the exact-head full profile reports 2,117 passed, 29 profile-declared skips, and 757.402 seconds. Line-count governance now records `forest_litter.rs` at 948 lines and extracts all three files that crossed 3,000 lines; no touched nonexempt Rust file remains at or above the blocking threshold. The package write set now includes root Cargo registration/lock changes, and the final disposition is implementation-current pending only re-review/re-verification/docs closure. |

### Independent runs

Ran on the corrected snapshot:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `16 passed, 0 failed`.

```text
cargo nextest run -p openwepp-runner \
  native_forest_yaml_executes_through_the_direct_production_consumer
cargo nextest run -p openwepp-runner \
  canopy_phenology_02_real_consumers_share_the_typed_native_state
```

Result: `1 passed, 0 failed` for each focused run.

`git diff --check` also passed for the declared package implementation and
documentation surfaces.

### Science and closure judgment

No predictive surrogate was introduced. External needle and fine-woody
operands remain authenticated, exact-day, dry-mass ground-boundary inputs
with no modeled canopy debit. Their sum enters the parallel
surface/interrill/rill recurrences once; those are per-area representations,
not three global masses. The output retains tissue status, source mode, and
aggregate completeness instead of converting missing authority to observed
zero.

The ADR-0042 claim boundary remains correct:

- predictive needle deposition:
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- predictive fine-woody deposition:
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- authenticated identity-only external boundary:
  `IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE`.

This review finds no remaining terminal science, implementation,
conservation, consumer-path, test-vector, line-count, or gate-truthfulness
finding. Package completion still requires the already-declared independent
re-verification and final documentation lint; this `PASS` does not pre-mark
those pending gates.
