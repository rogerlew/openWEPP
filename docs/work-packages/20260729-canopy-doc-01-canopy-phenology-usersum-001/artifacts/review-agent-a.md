# Independent Scientific and Coefficient Review A

Evidence class: `Static`

## Scope

I independently reviewed the terminal CANOPY-DOC-01 draft against the package
contract, the active native-forest management schema and projection path,
`SC-PLANT-001`, `SC-RESIDUE-001`, the retained CAL-04B/CAL-05/CAL-07 evidence,
the admitted litter-source literature, and the `usersum` authoring guide. This
review did not rerun empirical analyses or executable model tests.

## Findings

### A-01 — HIGH — The immediate-residue coefficient inventory omits active user fields

The public litter/decomposition table lists only `decomposition.oratea` and
`decomposition.orater`, then describes residue mass-to-depth conversion as a
derived lineage
([`usersum/openwepp-canopy-phenology.md`:183](../../../../usersum/openwepp-canopy-phenology.md)).
The authority ledger similarly substitutes a synthetic
`<derived residue mass-to-depth conversion>` row
([`coefficient-authority-ledger.csv`:18](coefficient-authority-ledger.csv)).

That is not the exact active native input inventory required by the package.
`PlantScenario::NativeForest` exposes `cf` and `diam` as user fields
(`crates/openwepp-management-schema/src/lib.rs:96-107`). The real native
initial-seed path uses `cf` to invert declared interrill/rill cover into residue
mass and as the daily residue cover factor; it uses `diam` to select the legacy
mass-to-depth conversion (`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs:382-407,1600-1649`).
The conversion is also conditioned by native initial-state fields including
`inrcov`, `rilcov`, and the surface-residue seed. Consequently, `cf` directly
controls erosion-facing residue cover and participates in the frost depth
lineage, while `diam` can change the frost-facing depth for the same residue
mass.

This omission leaves users without units, hard domains, effect directions,
range authority, observation targets, or calibration cautions for two active
controls that are inside the package's “immediate residue behavior” and
user-configurable mass-to-depth scope. It also makes the prose at lines 190-194
sound as though the conversion has no user-facing operands. Add exact `cf` and
`diam` entries to the public guide and authority ledger, explain the relevant
initial-state operands without misclassifying them as calibration
coefficients, and replace the pseudo-field row with the actual derivation
lineage.

### A-02 — HIGH — The coefficient ledger does not satisfy the mandated per-field contract

The ledger header
([`coefficient-authority-ledger.csv`:1](coefficient-authority-ledger.csv))
has no equation-location field and no explicit effect-direction field. Some
effects are embedded inconsistently in `model_role` or `warning`, and source
paths name whole contracts or packages rather than the governing equation or
invariant. This does not meet the package requirement that every coefficient
record its “effect direction” and “equation location.” The deficiency is
especially visible for `growth.bbb`, `growth.hmax`, both decomposition
constants, and the derived depth row.

Add explicit ledger columns and populate them for every row. Suitable equation
anchors include the GSI indicator and 21-day laws; `f`, `Bf`, and `LAI`; the
`Cc=max(Cs,1-exp(-bb*Bf))` cover law; the
`Hc=(1-exp(-bbb*(Bs+Bf)))*hmax` height law; the Chapter-9 first-order
decomposition recurrence; the residue cover inversion/forward law; and the
mass-to-depth lineage. A coarse contract filename alone is not an equation
location.

### A-03 — MEDIUM — The decomposition primer is too thin to cover the promised process chain

The litter section
([`usersum/openwepp-canopy-phenology.md`:183](../../../../usersum/openwepp-canopy-phenology.md))
states that larger rate constants accelerate “environmentally modified” loss
and that depth is derived from mass, but it never explains the first-order
residue recurrence, the temperature/moisture modulation, the exponential
mass-to-cover relationship, or how material diameter enters mass-to-depth
conversion. These are the causal links a reader needs to understand why
litter can simultaneously affect erosion cover and the frost thermal boundary.
The current prose is coefficient annotation rather than the package's
referenced science primer on decomposition, residue cover, and residue depth.

Add a compact process paragraph or subsection. It should distinguish surface
and root pools; explain that source and decay are separately identifiable only
with repeated input and stock observations; and connect current residue mass
to erosion-facing cover and frost-facing depth without turning inherited WEPP
coefficients into universal forest parameters.

### A-04 — MEDIUM — The numeric forest-litter fallback has no reader-facing source

The guide publishes the implementation rule that recurring native litter with
configured `oratea=0` uses `0.5 yr^-1`
([`usersum/openwepp-canopy-phenology.md`:187](../../../../usersum/openwepp-canopy-phenology.md)),
but the References section contains no Olson first-order decomposition source
or forest-floor persistence/turnover source supporting that rate class. The
source-and-claim map also has no row for the fallback
([`source-and-claim-map.md`:7](source-and-claim-map.md)). Keane (2008a, 2008b)
supports tissue-separated interval litterfall observations, not this exact-day
fallback rate.

Bind the claim map and public text to the published decomposition authority
used by `SC-RESIDUE-001` (including Olson's first-order model and the admitted
forest-litter evidence). State clearly that `0.5 yr^-1` is a narrow runtime
fallback for recurring native litter, not a calibrated value, typical range,
or recommended user setting.

### A-05 — LOW — One bibliography entry is not APA-style

The References section uses “Lim, H., et al.”
([`usersum/openwepp-canopy-phenology.md`:307](../../../../usersum/openwepp-canopy-phenology.md)).
`et al.` is appropriate for an in-text citation but not an APA-style reference
entry. Replace it with the complete author list (subject to the APA author-count
rule) from the authenticated article metadata. The other reviewed DOI/title
identities agree with the admitted source ledgers.

## Positive assessment

The weather-to-GSI description, signed-latitude photoperiod lineage, 21-day
real-sample warm-up, evergreen/deciduous realization, same-day leaf mass
closure, and post-phenology consumer ordering agree with `SC-PLANT-001`. The
six Hubbard Brook timing ranges are correctly scoped as a correlated accepted
ensemble rather than independent ecological bounds. The mature-LAI interval is
properly labeled a source interval, not a physiological bound. The calibration
sequence preserves observation scale, identifiability, source/decay
separation, independent transfer without refit, and the no-downstream-
compensation rule. Harvard non-transfer and the tropical dry-forest stop-loss
are stated qualitatively without duplicating assurance result tables.

## Disposition

`HOLD`

A-01 and A-02 violate explicit coefficient-completeness exit criteria. A-03
and A-04 leave the required decomposition/residue science primer and numeric
fallback claim incompletely supported. Correct and independently verify all
accepted findings before CANOPY-DOC-01 closes.
