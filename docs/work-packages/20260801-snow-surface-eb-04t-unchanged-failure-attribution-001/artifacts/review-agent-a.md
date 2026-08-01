# Independent Science/Governance Review A

Evidence mode: `Static + Ran`.

Reviewer role: independent science/governance reviewer A. Review scope was
read-only except for this artifact.

Disposition: `GO_WITH_AMENDMENTS`.

## Evidence Checked

- Ran the package-local rejected-alias/frozen-inventory self-check: `PASS`.
- Independently reconstructed exactly 16 baseline forcing-robust failures with
  signature counts `9/2/2/2/1`; all 64 B/L/S/LS labels remain `fail`.
- Independently confirmed LS primary-error directions: 15 away from the
  observation and one unchanged.
- Confirmed all six recorded input SHA-256 identities against current bytes.
- Confirmed every failed-cell longwave main effect is exactly zero in the
  selected primary metric and all five timing failures occur in `open` SNOTEL
  controls.
- Confirmed 13 of 16 rows have nonzero factorial interaction terms, including
  the two Niwot timing cases with interactions of `+4.0 d` and `-2.0 d`.
- Parsed all three SVGs successfully and confirmed one Markdown sidecar per
  figure.
- Confirmed the analysis launches no model subprocess and preserves EB-04S as
  `CLOSE_NONPROMOTION_EMPIRICAL_RULE`, with no retroactive promotion or default
  activation.

## Findings

### Major — factor-specific identifiability is missing from the criterion conclusion

All five rows classified as target-sensitive timing failures are open-site
SNOTEL controls. The frozen EB-04R protocol expressly states that SNOTEL open
controls cannot identify canopy longwave
(`prospective-decision-protocol.md:100-105`). In the reconstructed primary
metrics, `L == B` for all 16 failures. Consequently, the five timing failures
provide direct evidence about the sublimation-bearing candidate and the LS
bundle's open-control behavior, but they do not provide a target-sensitive
efficacy test of sub-canopy longwave.

The current criterion report asks whether “longwave and sublimation” improve
their target-sensitive signatures and then treats all five timing rows as
directly or jointly sensitive to “the target mechanisms”
(`criterion-fitness.md:7-21`; `scientific-synthesis.md:14-18`). That collapses
two distinct identifiability statements and exceeds the frozen claim limit.

Required amendment: publish factor-specific sensitivity/identifiability in the
JSON, CSV, synthesis, criterion report, process attribution, figure sidecar,
and roadmap/catalog language. State explicitly that the adverse/unchanged
timing evidence is relevant to sublimation and blocks promotion of the combined
LS candidate, while it neither demonstrates nor refutes canopy-longwave
efficacy. Preserve EB-04S nonpromotion.

### Major — the interaction evidence needed to explain non-resolution is omitted

The tool computes an interaction term for every row
(`analyze_failures.py:166-170`), but the synthesis and process-attribution
discussion reduce the response to zero longwave main effect plus dominant
sublimation movement (`process-attribution.md:18-21`). This misses material
structure in the retained factorial:

- Niwot peak depth: S moves the median error 4 days toward observation, but the
  LS interaction adds 4 days, leaving LS unchanged from B.
- Niwot peak SWE: S moves 2.5 days away; the LS interaction recovers 2 days,
  leaving a 0.5-day adverse combined response.
- Eleven other failed rows also have nonzero interaction terms, generally
  mitigating the S-only adverse response.

The omitted interaction is directly responsive to the package objective of
understanding why failures were not resolved. It also prevents the evidence
from being read as a simple causal claim that sublimation alone explains every
LS outcome.

Required amendment: add a factor/interaction summary, including the exceptional
Niwot peak-depth response, to the machine-readable summary and human synthesis.
Expose interaction in a readable figure or table/sidecar. Keep the attribution
associational and do not assign the open-control interaction uniquely to canopy
longwave.

### Moderate — “none is numerically invariant” contradicts the primary metric result

The package and generated synthesis state that none of the 16 failures is
numerically invariant (`package.md:169-171`; `scientific-synthesis.md:7-12`).
However, Niwot peak-depth median offset and its error magnitude are exactly
unchanged between B and LS (`-46.5 d`, `46.5 d`). The separate fact that its
complete metric object differs does not make the selected primary scalar
non-invariant.

Required amendment: say that no complete metric object is byte/value-identical
between B and LS, while 15 selected primary errors move away and one selected
primary error is unchanged. Use that exact distinction in generated prose and
gate evidence.

### Moderate — the 11/5 ownership split needs uncertainty/sensitivity treatment

The 9 density-trajectory rows are well supported as adjacent density debt. The
two depth-SWE rows are less clean: prior decomposition explicitly assigns their
mechanism-class signal as `canopy_snow_interception_or_subcanopy_longwave`, while
also warning that the open member prevents a pure canopy diagnosis
(`post_partition_residual_decomposition.py:305-325`). EB-04T instead puts both
in the adjacent `mixed_indirect` bucket, making the exact `11 adjacent / 5
target-sensitive` split appear more authoritative than its source supports.

Required amendment: label this split as an EB-04T inference and report the
ambiguous boundary. A simple sensitivity statement is sufficient: nine are
clearly adjacent density failures, two depth-SWE failures have mixed
density/geometry/canopy-process ownership, and five open-control timing failures
are sublimation-sensitive. This does not alter the finding that no failed LS
primary metric moves toward observation.

### Moderate — exact-direction counts must not imply material degradation

The frozen diagnostic intentionally uses exact direction and no material-effect
threshold. Several adverse LS changes are very small (for example, Paradise
density KGE error changes by about `0.14%`). The figure sidecars mention the
exact-distance rule, but roadmap/catalog summaries state “15 move away” without
the same qualification.

Required amendment: carry the no-materiality limitation into the synthesis and
roadmap/catalog. Distinguish exact direction from an ordinal regression or a
scientifically material worsening; all labels remained unchanged and EB-04S
reported no new robust failures.

## Governance Assessment

The retained analysis does **not** support retroactively passing EB-04S, changing
defaults, or claiming warm-maritime conifer transfer. The original nonpromotion
decision remains valid under its frozen rule. After the amendments above, the
data support a narrower and more useful conclusion: the total-failure-reduction
gate mixed adjacent-process debt with intervention-sensitive evidence; the
combined candidate nevertheless failed because its open-control,
sublimation-sensitive timing errors did not improve, while the failed-cell set
does not independently adjudicate canopy-longwave efficacy.

Validation is sufficient for a retained-evidence diagnostic after the generated
artifacts are amended and deterministically regenerated. Terminal review,
finding disposition, exact-diff reconciliation, Markdown lint, and dual
verification remain required before package closure.
