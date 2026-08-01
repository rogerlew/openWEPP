# Independent Governance / Prospective-Design Review B

Evidence class: `Static + independently reconstructed Reused Ran`.

Verdict: `HOLD_PENDING_AMENDMENTS`.

## Scope Reviewed

Independently reviewed the EB-04U package contract, kickoff and reading map,
ADR-0042, the canonical testing strategy, `SC-SNOWFREEZE-001` rubric authority,
the campaign and root roadmaps, the package catalog, the EB-04S retained
adjudication and frozen decision protocol, the EB-04T attribution and criterion
fitness, the package-local generator, generated machine-readable artifacts,
prospective protocols, readiness/admission matrices, figures and sidecars,
gate claims, and the exact current write set. Reviewer A's artifact was not read
before this review.

## Independent Checks

- The package self-check passes, including rejected-signature and duplicate-row
  controls.
- All five manifest input hashes match their current read-only sources.
- Independent manifest reconstruction confirms 16 unique failure identities in
  the exact `9/2/5` cohort split.
- All 16 failure rows and all ten observation-lane assignments are
  `DIAGNOSTIC_ONLY`; no current record is assigned independent-validation or
  calibration authority.
- The manifest records zero model subprocesses, zero new candidate results, and
  no promotion authorization.
- The current diff is confined to the package tree and the three declared
  roadmap/catalog paths. No source, test, contract, fixture, observation,
  retained-output, EB-04S, or EB-04T edit was found.
- The roadmaps preserve EB-04S nonpromotion/default-off and describe EB-04U as
  a new diagnostic study cycle rather than a retry of the sealed factorial.

## Findings

### B-01 — Seasonal phase operators are not fully frozen

Severity: `major anti-leakage`; closure disposition required: `accepted and
fixed` or package `HOLD`.

`seasonal-phase-protocol.md` leaves result-sensitive choices open. Accumulation
ends at "the earlier of modeled or observed seasonal peak," while the next
paragraph says phases are computed separately for observations and each model
cell. Dry settling depends on an undefined "material" liquid or melt signal,
and peak transition is an unspecified "window." The protocol says exact window
widths remain existing rubric definitions, but `TOL-SNOWFREEZE-010/011` and the
EB-04R observation protocol do not define a peak-transition window. EB-04R does
define snow presence, persistent disappearance, and tie handling, but EB-04U
does not carry those numerical rules into this phase protocol.

These unresolved choices can change which days enter a process-conditioned
diagnostic after candidate behavior is visible, contrary to the package's
claim that operators are frozen. Resolve the separate-versus-earlier-peak
contradiction and either freeze deterministic phase boundaries, thresholds,
window widths, and tie/missing rules now, or explicitly classify each
unresolved operator as a pre-result successor prerequisite and narrow the
`PROSPECTIVE_DESIGN_COMPLETE` claim accordingly.

### B-02 — The one-attempt stop-loss lacks an auditable candidate identity

Severity: `major governance`; closure disposition required: `accepted and
fixed` or package `HOLD`.

`prospective-decision-protocol.md` allows one result-bearing attempt per
"materially new, authority-backed candidate," but it does not define candidate
identity or what makes a candidate materially new. That leaves the experiment
budget vulnerable to relabeling a parameterization, selector, equation detail,
or evidence subset as a new candidate after an unfavorable result. Freeze an
identity rule over the authority/formulation, executable and selectors,
parameters, forcing, population and evidence roles, observation operators, and
decision thresholds. A subsequent attempt should require a prospectively
documented mechanism/authority change, not merely a new name or tuned value.

### B-03 — Readiness status conflates openWEPP process and wepppy forcing ownership

Severity: `moderate authority boundary`; closure disposition required:
`accepted and fixed`.

The calibration-readiness matrix gives one `NOT_IMPLEMENTED` science status to
"Redistribution/forcing correction." Wind redistribution is a candidate snow
process, whereas precipitation/phase forcing correction belongs at the
openWEPP/wepppy boundary and may be outside openWEPP implementation scope. The
combined row obscures which implementation is absent, which authority remains
unreconciled, and who owns the next action. Split the row into process and
forcing-provider obligations (or explicitly mark the out-of-repository side
`NOT_APPLICABLE` for openWEPP) while retaining `NONIDENTIFIABLE` for the current
evidence. This will make the ADR-0042 status fields agree with the otherwise
careful ownership language in the EB-04W admission row.

### B-04 — Historical and current evidence-role counts are imprecisely labeled

Severity: `minor claim boundary`; closure disposition required: `accepted and
fixed`.

The three figure sidecars call the 40 audited retained cells
"independent-validation" cells even though EB-04U deliberately reassigns all
of their observations to `DIAGNOSTIC_ONLY`. The gate table also calls all 16
failure rows "lane records," while the evidence-role manifest contains ten
observation lanes. Describe the 40 cells as retained EB-04S cells that were
originally assigned independent validation but are diagnostic-only for
EB-04U–04X, and distinguish 16 failure rows from ten observation lanes. This is
wording only, but the distinction is central to the anti-leakage claim.

## Governance Assessment

The central evidence-role decision is appropriately conservative: prior years
and lanes cannot regain independence through a post hoc split, and new-site,
future-year, or genuinely sealed evidence must be frozen before result-bearing
execution. The package also correctly separates missing data from science
authority under ADR-0042, keeps forcing-owned deficits out of snow-physics
tuning, treats missing operands as successor prerequisites rather than inferred
causes, and admits no production amendment or promotion factorial.

The materiality fallback to accepted `TOL-SNOWFREEZE-010/011` rubric bands is
defensible and avoids deriving thresholds from EB-04S/04T effects. Component
efficacy, adjacent-process noninferiority, interaction reporting, and hard
mass/energy/lineage prerequisites are directionally sound. The two major
findings above concern whether those rules are operationally prospective and
retry-resistant, not the `9/2/5` partition or the diagnostic-only evidence-role
decision.

After B-01 through B-04 are corrected and explicitly dispositioned, this review
supports proceeding to terminal verification with a `PASS` design verdict. No
model rerun, coefficient fitting, contract amendment, or change to EB-04S
nonpromotion is requested.
