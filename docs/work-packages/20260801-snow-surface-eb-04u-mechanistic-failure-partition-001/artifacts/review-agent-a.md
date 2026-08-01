# EB-04U Independent Science And Mechanism Review — Agent A

Evidence class: `Static + Reused Ran + Ran review-only reconstruction`.

Verdict: `HOLD` pending disposition of three closure-blocking findings.

## Scope And Checks

I reviewed the package, generated manifests and protocols, EB-04T failure
attribution, the earlier post-partition residual decomposition, ADR-0042,
`SC-SNOWFREEZE-001` observation/rubric authority, and `SC-SNOWENERGY-001`
mass/energy authority. I did not execute the model or inspect new candidate
results.

A separate Python read of the EB-04U and EB-04T JSON confirms that all 16
`(lane_id, cell_id)` identities are present exactly once with no missing or
extra identity. The cohort counts independently reconstruct as `9/2/5`, and
the successor counts reconstruct as EB-04V `9`, EB-04W `5`, and EB-04X `2`.
The fifth EB-04W row relative to the older four-row residual cluster is the
Niwot peak-depth failure; EB-04U appropriately retains its explicit density
coupling rather than representing it as a pure mass-timing diagnosis.

The evidence-role decision is conservative and valid: every observation
already used to select the EB-04U population, operators, or mechanism routes is
`DIAGNOSTIC_ONLY`. A post-hoc year split cannot restore independence. Later
years unavailable at design time can support temporal out-of-sample claims;
they do not by themselves establish spatial/site transferability.

The operand audit independently reads 24 rows: 16 `AVAILABLE_DIRECT`, one
`AVAILABLE_DERIVED`, one `AVAILABLE_SEMANTIC_PROOF_REQUIRED`, one
`PARTIAL_AMBIGUOUS`, and five `MISSING_REQUIRED`. Its explicit treatment of
fresh-snow density, phase partition, redistribution, density tendencies,
snow-canopy storage, and daily canopy state is scientifically appropriate.

## Findings

### EB04U-A-001 — Major — Seasonal memberships are contradictory and candidate-dependent

`seasonal-phase-protocol.md` defines accumulation through the *earlier* of the
modeled and observed peaks, then says observed and modeled phases are computed
separately and that a modeled peak may not define the observed boundary. Those
rules cannot all hold. Dry-settling and wet-compaction membership also depends
on modeled liquid/melt state that has no equivalent observed classification.
If each candidate supplies its own phase membership, candidates can be scored
on different dates and can change their apparent KGE simply by moving a date
between phases. This prevents a like-for-like efficacy comparison and weakens
the prospective freeze required by acceptance criteria 3, 5, and 8.

Required disposition: freeze a nonoverlapping primary evaluation frame before
candidate execution. At minimum, observed-anchored accumulation/peak/ablation
windows must remain common across candidates. Candidate-derived dry/wet regime
labels may be reported as diagnostic conditioning, but must not silently
change the dates entering the primary efficacy operator. Define the handling
of peak ties, missing peaks, transition-window width, and overlapping regime
labels, or make each item an explicit successor prerequisite before a
result-bearing run.

### EB04U-A-002 — Major — Stage 3 energy is not the causal melt-energy ledger

`failure-mechanics-matrix.csv` names shortwave/longwave energy, turbulent
exchange, rain heat, and premature melt as competing explanations for the
melt-out rows. `operand-lineage.csv` then marks
`surface_energy_components` available using only Stage 3 shortwave, longwave,
latent, conduction, and total-surface-energy fields. Canonical
`SC-SNOWENERGY-001` explicitly preserves the CoE melt boundary: Stage 3 energy
changes cold content and does **not** convert positive excess to melt. The
listed Stage 3 fields therefore cannot independently attribute CoE melt, and
the lineage omits the CoE melt-driver terms as well as rain-heat/sensible-heat
operands named by the failure matrix. It also omits the cold-content,
refreeze-energy, exported-cold-content, and unused-positive-energy operands
needed for the contract's `INV-SNOWENERGY-019` reconstruction.

Required disposition: split the lineage into (a) Stage 3 cold-content/vapor
energy, with all required reconstruction operands and the explicit
no-energy-balance-melt boundary, and (b) authoritative CoE melt drivers and
melt/rain-energy terms. Mark absent terms `MISSING_REQUIRED` and carry them
into EB-04W admission. Until those operands exist, EB-04W may distinguish
pre-peak mass deficit from post-peak removal using mass ledgers, but it cannot
claim an independently reconstructed energetic cause of premature CoE melt.

### EB04U-A-003 — Major — Efficacy admits arbitrarily small directional changes

The materiality section correctly says exact direction is descriptive and
binds material change to accepted TOL-010/011 rubric bands or a prospectively
authoritative uncertainty threshold. The mechanism-efficacy predicates do not
consistently apply that rule: EB-04V need only “improve” a KGE component in
both bias strata, EB-04W need only reduce a failure count, and EB-04X may
“preserve or improve” open geometry. The protocol does not state the minimum
number/fraction of independent records or sites that must show a material
benefit, so a single band-crossing cell plus arbitrarily small directional
movement elsewhere could satisfy the prose. This leaves the primary promotion
predicate materially under-frozen despite acceptance criterion 8.

Required disposition: define efficacy explicitly in terms of material
band/authority-threshold crossings, specify the cohort aggregation and minimum
replication/site-spread rule, and preserve both retained density bias
directions separately. If those numeric/fractional requirements cannot be
chosen without future authority or data, label them as mandatory successor
pre-result prerequisites and state that EB-04U admits diagnostic operator
development only, not a result-bearing efficacy study.

## Positive Conclusions

- The exact `9/2/5` partition is supported and does not overstate unique
  causality.
- EB-04V correctly requires process-specific density tendencies and a direct
  or disambiguated fresh-snow-density surface before process selection.
- EB-04W correctly keeps forcing/phase/redistribution ownership separate from
  openWEPP snow-process ownership and prohibits physics compensation for a
  forcing-owned deficit.
- EB-04X correctly requires open-lane SWE–density–depth closure before a
  hardwood-minus-open canopy residual can identify interception or longwave.
- The retained observations are correctly limited to diagnosis; authoritative
  implementation remains possible under ADR-0042, but promotion requires new
  prospectively independent evidence appropriate to the claim.

No proxy physics, calibration, promotion, or runtime change was found in the
reviewed EB-04U write set.
