# SC-WATBAL-001 Provenance Sidecar

Status: Active
Last updated: 2026-06-08
Scope: retained provenance sidecar for `SC-WATBAL-001` consolidation work

## Purpose

This sidecar is reserved for historical or superseded WATBAL narrative once the
core contract Binding Exposure Index proves that binding residue is conserved in
canonical `INV-WATBAL-*` / `OBL-WATBAL-*` rows.

SCSTRUCT02 did not relocate narrative into this sidecar because all unmapped or
undecidable rows require science-review adjudication before sidecar eligibility.
History statements remain non-binding unless cross-referenced from the core
contract Binding Exposure Index to canonical binding IDs.

## SCSTRUCT03 Batch 1 ProfileFC/WP relocated entries

Evidence mode: Static
Status: active provenance; non-binding except through core Binding Exposure Index mappings

## HPHYS0202-PROFILEFC-PROFILEWP-LAYER-AGGREGATION-LINEAGE-CLOSURE-HISTORIC HPHYS0202 ProfileFC/ProfileWP Layer-Aggregation Lineage Closure (Historical)

- status: superseded
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: HPHYS0207, HPHYS0216, HPHYS0216D, HPARITY02 profile-capacity publication lineage closure
- canonical_binding_ids: INV-WATBAL-041, INV-WATBAL-042
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 HPARITY02 profile-capacity publication lineage closure; REF-WATBAL-LEGACY-WATCON; REF-WATBAL-LEGACY-WB13

Early layer-aggregation authority for ProfileFCStore/ProfileWPStore. Live binding residue is conserved by current normalized hydrology seed-grid and runtime storage-lineage invariants plus the retained HPARITY02 core rules.

Relocated narrative:

### HPHYS0202 ProfileFC/ProfileWP Layer-Aggregation Lineage Closure (Historical)

HPHYS0202 amends WB13 profile-storage publication authority so
`ProfileFCStore` and `ProfileWPStore` are simulation-owned layer aggregates
from canonical WB11/WB13 state lineage (`watbal.for`/`watbalprint.for`):

1. `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000` in `mm`.
2. `ProfileWPStore = Σ(thetdr_i * dg_i) * 1000` in `mm`.
3. Required aggregation symbols are per-layer `thetfc_####`, `thetdr_####`,
   and `dg_####` runtime surfaces for `i in [1..nsl]`.
4. Optional adapter seed symbols `wb13_profile_fc_store_mm` and
   `wb13_profile_wp_store_mm` are diagnostic carry surfaces only and must not
   override WB13 publication values. This historical rule is superseded by
   HPHYS0207 depth-authority closure below.
5. Missing/non-finite/domain-invalid layer aggregation symbols hard-fail WB13
   publication under existing guard-family continuity.


## HPHYS0205-CORRECTED-LAYER-AUTHORITY-CLOSURE-HISTORICAL HPHYS0205 Corrected-Layer Authority Closure (Historical)

- status: superseded
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: HPHYS0207, HPHYS0216D, HPARITY02 profile-capacity publication lineage closure
- canonical_binding_ids: INV-WATBAL-041, INV-WATBAL-042
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 HPARITY02 profile-capacity publication lineage closure; REF-WATBAL-LEGACY-WATCON; REF-WATBAL-LEGACY-WB13

Corrected-layer source authority for profile FC/WP publication. Live residue is conserved by normalized hydrology seed-grid and runtime storage-lineage invariants; older diagnostic carry wording is retained as provenance only.

Relocated narrative:

### HPHYS0205 Corrected-Layer Authority Closure (Historical)

HPHYS0205 closes the layer-source ambiguity identified in HPHYS0202 by
requiring the authoritative layer symbols consumed by WB13 publication to carry
baseline-corrected moisture lineage, not raw parser theta inputs:

1. Authoritative `thetfc_####`/`thetdr_####` runtime symbols used for WB13
   profile storage publication must be projected from baseline-authoritative
   soil-correction lineage (`scon` family: rock/entrapped-air adjustment and
   moisture-curve domain corrections), when that lineage is available.
2. WB13 `ProfileFCStore`/`ProfileWPStore` publication remains runtime-owned,
   and this historical layer-aggregation authority is superseded by HPHYS0207
   normalized-profile storage authority below.
3. Optional adapter profile-storage diagnostics
   (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`) remain
   non-authoritative for publication in this historical amendment and must not
   disagree with corrected-layer aggregates when both are present and finite.
4. Missing/non-finite/domain-invalid corrected-layer symbols are typed
   publication-domain violations and hard-fail under existing WB13 guard
   continuity.


## HPHYS0206-CORRECTED-LAYER-NORMALIZATION-AND-MAPPING-CLOSURE-HISTORICAL HPHYS0206 Corrected-Layer Normalization and Mapping Closure (Historical)

- status: superseded
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: HPHYS0207, HPHYS0216D, HPARITY02 profile-capacity publication lineage closure
- canonical_binding_ids: INV-WATBAL-041, INV-WATBAL-042
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 HPARITY02 profile-capacity publication lineage closure; REF-WATBAL-LEGACY-WATCON; REF-WATBAL-LEGACY-WB13

Normalized corrected-layer mapping closure for profile FC/WP publication. Current binding residue is conserved by same-grid WB11 seed authority and runtime storage-lineage invariants.

Relocated narrative:

### HPHYS0206 Corrected-Layer Normalization and Mapping Closure (Historical)

HPHYS0206 closes residual authoritative-layer mapping ambiguity by requiring
the corrected FC/WP publication lineage to use the same normalized layer set
as profile-capacity lineage and deterministic OFE-layer mapping semantics:

1. Authoritative `thetfc_####`/`thetdr_####` publication-consumer symbols must
   be derived from corrected layers computed on the same baseline-normalized
   profile layer set that governs `wb13_profile_depth_mm` and
   `wb13_profile_porosity_cap_mm`.
2. Mapping from normalized corrected layers to emitted OFE layer symbols
   (`thetfc_####`/`thetdr_####`) must be deterministic and depth-domain
   complete for each emitted layer interval.
3. Profile-storage diagnostics (`wb13_profile_fc_store_mm`,
   `wb13_profile_wp_store_mm`) remain non-authoritative for WB13 publication in
   this historical amendment and must not override layer-authoritative
   publication values.
4. Missing normalized corrected-lineage inputs, incomplete normalized-layer
   coverage, or non-finite/domain-invalid mapped authoritative layer symbols are
   typed fail-closed boundary violations; raw parser-theta fallback for
   authoritative FC/WP publication symbols is prohibited.


## HPHYS0216-PROFILEFC-LAYER-AUTHORITY-REALIGNMENT HPHYS0216 ProfileFC Layer-Authority Realignment

- status: superseded
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: HPHYS0216D and HPARITY02 profile-capacity publication lineage closure
- canonical_binding_ids: INV-WATBAL-041, INV-WATBAL-042
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 HPARITY02 profile-capacity publication lineage closure; REF-WATBAL-LEGACY-WATCON; REF-WATBAL-LEGACY-WB13

Intermediate ProfileFCStore realignment that restored layer aggregation and demoted wb13_profile_fc_store_mm to diagnostic carry. HPHYS0216D supersedes it with explicit normalized-tail authority; live residue is conserved in current core rules and invariants.

Relocated narrative:

### HPHYS0216 ProfileFC Layer-Authority Realignment

HPHYS0216 closes the `ProfileFCStore` structural split by restoring
baseline-authoritative FC publication lineage from `watbal.for` /
`watbalprint.for` while preserving corrected-layer symbol projection and typed
guards:

1. `ProfileFCStore` publication authority is layer aggregation from emitted
   authoritative symbols:
   `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000`, `i in [1..nsl]`.
2. `wb13_profile_fc_store_mm` remains a diagnostic carry surface and is not a
   publication-driving authority symbol for `ProfileFCStore`.
3. `ProfileWPStore` authority remains `wb13_profile_wp_store_mm` under
   HPHYS0209 unless superseded by later contract amendment.
4. Missing/non-finite/domain-invalid layer aggregation symbols (`nsl`,
   `thetfc_####`, `dg_####`) are typed fail-closed WB13 publication-domain
   violations.
5. Required WB13 profile ordering remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.


## HPHYS0216D-PROFILEFC-LAYER-TAIL-AUTHORITY-RECONCILIATION HPHYS0216D ProfileFC Layer+Tail Authority Reconciliation

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: none; binding residue retained in HPARITY02 profile-capacity publication lineage closure and current invariant mappings
- canonical_binding_ids: INV-WATBAL-041, INV-WATBAL-042
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 HPARITY02 profile-capacity publication lineage closure; REF-WATBAL-LEGACY-WATCON; REF-WATBAL-LEGACY-WB13

Narrative closure record for ProfileFCStore layer plus normalized-tail authority. The binding rule remains in the retained HPARITY02 core section and maps to current seed-grid/storage-lineage invariants; this copied narrative is provenance only.

Relocated narrative:

### HPHYS0216D ProfileFC Layer+Tail Authority Reconciliation

HPHYS0216D closes the residual normalized-tail omission identified after
HPHYS0216 by preserving layer-authoritative FC publication while requiring an
explicit runtime tail-contribution symbol.

1. `ProfileFCStore` publication authority is:
   `Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm`.
2. `wb13_profile_fc_tail_mm` must represent normalized-profile residual depth
   not covered by parser-layer aggregation and must be runtime-owned.
3. `wb13_profile_fc_store_mm` remains a reconciliation/diagnostic profile
   storage surface; it is not direct publication authority, but must reconcile
   with the combined FC publication authority above.
4. Missing/non-finite/negative `wb13_profile_fc_tail_mm` is a typed fail-closed
   WB13 publication-domain violation.
5. Required WB13 profile ordering remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

## SCSTRUCT03 Batch 2 Snow/Melt Comparator Arc relocated entries

Evidence mode: Static
Status: active provenance; non-binding except through core Binding Exposure Index mappings

## HPHYS0298-WATER-BALANCE-DISPOSITION-ADDENDUM HPHYS0298 Water-Balance Disposition Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-073, INV-WATBAL-087
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

HPHYS0298 disposition narrative is retained as provenance. Live residue is conserved by the paired lineage partition gate and ADR0017 comparator-verdict gate.

Relocated narrative:

### HPHYS0298 Water-Balance Disposition Addendum

HPHYS0298's all-window `OPENWEPP-DEFECTIVE` verdict is historical and is
superseded by HPHYS0299 plus ADR0017. The retained water-balance conclusion is
only that downstream WB17, WB18, WB19, WB12, and WB13 compensation is
prohibited while the paired lineage is unresolved. Reusing any HPHYS0298
snow/`RM` verdict now requires `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039` and
`INV-WATBAL-087`: same-quantity unit proof, lineage-stage proof, independent
correctness authority, and a symmetric verdict taxonomy that includes
`HARNESS-SURFACE-MISMATCH`.
The retained unresolved lane is producer-side hourly precipitation-phase partition
evidence anchored to `winter.for:410-412`, not as downstream storage or
water-balance compensation authority.


## HPHYS0299-WATER-BALANCE-UNIT-PROVENANCE-ADDENDUM HPHYS0299 Water-Balance Unit/Provenance Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-074, INV-WATBAL-087
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Corrected hrsnow depth-vs-water-equivalent unit/provenance narrative is retained as provenance. Live residue is conserved by the HPHYS0299 corrected unit/provenance ledger and ADR0017 same-quantity pairing gate.

Relocated narrative:

### HPHYS0299 Water-Balance Unit/Provenance Addendum

Water-balance continuation routing must not treat a depth-vs-water-equivalent
diagnostic mismatch as producer physics evidence. The corrected HPHYS0299
ledger is now required before reusing HPHYS0298 `hourly-forcing` verdicts:
canonical `hrsnow` is snowfall depth, the openWEPP parity summary is
`snow_hourly_snowfall_depth_sum_m`, and `snow_hourly_snowfall_water_equiv_sum_m`
is a derived SWE-related summary for different diagnostics.


## HPHYS0300-WATER-BALANCE-RAW-POST-RAW-ADDENDUM HPHYS0300 Water-Balance Raw/Post-Raw Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-075
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Raw hourly melt/post-raw routed-melt classifier narrative is retained as provenance. Live residue is conserved by the HPHYS0300 raw/post-raw water-balance ledger.

Relocated narrative:

### HPHYS0300 Water-Balance Raw/Post-Raw Addendum

HPHYS0300 water-balance closure is a producer-lineage classifier. It may use
full-suite H1..H39 metrics to quantify continuation impact, but those metrics
cannot authorize WB17, WB18, WB19, or WB13 compensation while raw `hrmlt`,
post-raw `wmelt`, or corrected-depth hourly forcing remains the first
divergent source. Rows lacking term/state evidence remain `HOLD`, not
semantic closure.

The HPHYS0300 evidence gate is bounded. Once paired baseline/openWEPP
term/state evidence isolates a raw-melt or post-raw source to a named
producer-side term/state input with units and source-line provenance, the next
package must either implement the baseline-authoritative producer correction or
record the blocking invariant that prevents it. It must not route the same
isolated source into another diagnostic-only package, and it must keep H39
first-2013 corrected-depth hourly forcing on a separate actionable correction
lane instead of waiting for raw-melt term instrumentation.


## HPHYS0301-H39-RAIN-RELEASE-WATER-BALANCE-ADDENDUM HPHYS0301 H39 Rain-Release Water-Balance Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-076
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

H39 rain-release reconciliation narrative is retained as provenance. Live residue is conserved by the HPHYS0301 residual-rain/release water-balance ledger.

Relocated narrative:

### HPHYS0301 H39 Rain-Release Water-Balance Addendum

HPHYS0301 water-balance continuation supersedes any production edit claim that
compares H39 first-2013 baseline residual rain-on-snow evidence to openWEPP raw
rain. The valid water-balance comparison for that evidence class is baseline
residual `hrrain` against openWEPP released rain plus `snow.post_winter_rain_m`.
When that reconciliation removes the material raw-rain aggregate delta and no
source-line raw forcing defect is proven, H39 first-2013 remains a snow
producer `HOLD` for paired rain-retention/raw-melt/post-raw evidence. It does
not authorize WB17, WB18, WB19, WB13, or forcing-code compensation.


## HPHYS0302-COMPARATOR-SURFACE-WATER-BALANCE-ADDENDUM HPHYS0302 Comparator-Surface Water-Balance Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-077, INV-WATBAL-087
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Comparator-surface audit narrative is retained as provenance. Live residue is conserved by the HPHYS0302 same-surface audit gate and ADR0017 comparator governance.

Relocated narrative:

### HPHYS0302 Comparator-Surface Water-Balance Addendum

HPHYS0302 requires water-balance diagnostics to distinguish publication
surfaces from producer surfaces before assigning residual ownership. `RM` and
`Snow-Water` daily WAT/WB13 comparisons may prove output-surface parity or
residual magnitude, while raw `hrmlt` and post-raw `wmelt` comparisons may
bound aggregate cut-points. They do not authorize WB17/WB18/WB19/WB13
compensation or snow-producer edits. A production correction requires paired
baseline/openWEPP term-state evidence for the melt terms and forcing/state
inputs named in `SC-SNOWFREEZE-001#INV-SNOWFREEZE-033`.


## HPHYS0305-PAIRED-MELT-TERM-STATE-ADDENDUM HPHYS0305 Paired Melt-Term/State Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-078
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Paired melt-term/state instrumentation narrative is retained as provenance. Live residue is conserved by the HPHYS0305 paired melt-term/state gate.

Relocated narrative:

### HPHYS0305 Paired Melt-Term/State Addendum

HPHYS0305 is an evidence gate, not a production-correction package. It may add
diagnostic observe/trace surfaces so fixed-baseline `melt.for`/`snowd.for`
symbols can be compared against openWEPP aliases, but it must keep production
physics unchanged unless paired evidence isolates a named source and a
follow-on package implements the source-owned correction under canonical
contract authority. Required baseline/openWEPP pairings are:

- `amelt`/`bmelt`/`cmelt`/`dmelt` -> `snow_hourly_melt_*_in`
- `hrrain` -> `snow_hourly_rain_m`
- `hrtemp`/`tdpt`/`hrad`/`cloudC`/`vwind` -> `winter_hourly_*`
- `snodpt`/`densgt` -> `snow_hourly_depth_after_m` and
  `snow_hourly_density_after_kg_m3`


## HPHYS0306-BRANCH-ACTIVE-MELT-TERM-OBSERVE-ADDENDUM HPHYS0306 Branch-Active Melt-Term Observe Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-079
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Branch-active observe-domain narrative is retained as provenance. Live residue is conserved by the HPHYS0306 branch-active observe semantics gate.

Relocated narrative:

### HPHYS0306 Branch-Active Melt-Term Observe Addendum

HPHYS0306 closes the HPHYS0305 missing-`amelt` blocker by separating missing
baseline term observations from inactive fixed-baseline melt hours. The
baseline branch-active domain is the set of fixed-comparator observe keys where
`melt.for` reached the paired `amelt`/`bmelt`/`cmelt`/`dmelt` observation after
term computation. The openWEPP branch-active domain is the set of
`snow_hourly_melt_branch_active` keys with `true` values. These domains are
compared before numeric term-state comparisons:

- baseline inactive + openWEPP inactive: skip term/forcing/state comparison;
- baseline active + openWEPP inactive or baseline inactive + openWEPP active:
  route `branch-active-mask-hold`;
- baseline active + openWEPP active: compare paired same-unit
  forcing/state/term surfaces and classify the first source.

No package may convert inactive fixed-baseline hours into zero-valued
`amelt`/`bmelt`/`cmelt`/`dmelt` observations unless a later canonical contract
amendment cites baseline code that explicitly stores such inactive values.


## HPHYS0307-MELT-CALL-BRANCH-ACTIVATION-LINEAGE-ADDENDUM HPHYS0307 Melt-Call Branch Activation Lineage Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-080
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Melt-call branch activation lineage narrative is retained as provenance. Live residue is conserved by the HPHYS0307 branch activation lineage gate.

Relocated narrative:

### HPHYS0307 Melt-Call Branch Activation Lineage Addendum

HPHYS0307 routes branch-active mask gaps by control-flow source, not by
downstream `RM`, `Snow-Water`, `Total-Soil`, or melt-magnitude residuals.
Baseline branch activation uses source-line provenance:

- `winter.for` calls `snowd.for` for every winter hour before writing
  `hrmlt(hour,iplane) = wmelt(iplane)`;
- `snowd.for` initializes no-snow/no-snowfall and freezing daily-mean lanes
  without calling `melt.for`;
- `snowd.for` calls `melt.for` only in the existing-snowpack,
  non-freezing-daily-mean lane when post-input `snodep > 0.0`;
- openWEPP `snow_hourly_melt_branch_active` must represent the analogous
  branch predicate where `compute_simimpl29_melt_hour` is invoked.

The HPHYS0307 ledger must classify each row as `baseline-extra-melt-call`,
`openwepp-extra-melt-call`, matched active-domain source divergence, or
trace-parser conflict. A row with only classification evidence and no
source-line-owned openWEPP defect remains `HOLD`.


## HPHYS0308-BRANCH-EXTRA-STATE-ORDERING-ADDENDUM HPHYS0308 Branch-Extra State-Ordering Addendum

- status: historical
- source_package: 20260608-scstruct03-watbal-bei-science-review-adjudication-001
- effective_date: 2026-06-08
- verdict: binding-exposed
- superseded_by: core WATBAL governance invariants and ADR0017 comparator-governance authority where cited
- canonical_binding_ids: INV-WATBAL-081
- migration_target: none
- provenance_anchors: SC-WATBAL-001 Binding Exposure Index; SC-WATBAL-001 invariants INV-WATBAL-073..081/087; docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md

Branch-extra state-ordering narrative is retained as provenance. Live residue is conserved by the HPHYS0308 snowd branch predicate state-ordering gate.

Relocated narrative:

### HPHYS0308 Branch-Extra State-Ordering Addendum

HPHYS0308 treats branch-extra keys as timestamp-level state-ordering evidence.
The branch predicate may be edited only after the key-level state proves that
openWEPP evaluates the same baseline state on the wrong condition. Otherwise:

- baseline-extra keys where openWEPP snow depth is already zero are
  `snow-state-carry-depletion-hold`;
- openWEPP-extra keys where fixed-baseline has no paired `melt.for`
  observation are `baseline-branch-instrumentation-hold`;
- matched branch-active keys with same-hour `cmelt`/`snodpt` divergence remain
  source-ordering holds;
- aggregate `RM`, `Snow-Water`, `Total-Soil`, or `SoilWaterTotal` residuals
  cannot authorize branch-predicate edits.


