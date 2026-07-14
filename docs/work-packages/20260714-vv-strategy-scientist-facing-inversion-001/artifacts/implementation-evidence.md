# Implementation Evidence

Status: `INITIAL-IMPLEMENTATION-COMPLETE`

## Outcome

Static: The active strategy now treats public scientific assurance dossiers as
the primary V&V product. Its roadmap publishes an honest baseline before new
campaigns, runs claim-driven science before generalized infrastructure, and
requires recurring demonstrated need before a crate, database, provenance
export, or broad freshness system is justified.

Static: The companion dossier standard converts that posture into an immediately
usable three-layer document: decision summary, scientific evidence, and
reproducibility/audit. It permits honest `NOT_ASSESSED`,
`INSUFFICIENT_EVIDENCE`, and `NOT_SUPPORTED` publication.

## Complexity Reduction

Ran: `git show HEAD:<strategy> | wc -l -w -c` measured the prior strategy at
`672` lines, `4930` words, and `37804` bytes.

Ran: `wc -l -w -c` measured the terminal revised strategy at `370` lines,
`2315` words, and `18789` bytes. The strategy is about half the prior word and
byte size.

Ran: The new dossier standard is `239` lines, `1493` words, and `11176` bytes.
Together the progressively disclosed strategy and standard remain below the
prior monolith at `3808` words and `29965` bytes. The review-driven increase
adds baseline-review and immutable content-binding controls rather than future
platform machinery.

## Preserved Scientific Controls

Static: The revision retains:

- bounded, intended-use-specific claims;
- distinct verification, validation, uncertainty, comparative, review, and use
  qualification dimensions;
- verification of algorithm, numerical solution, production consumer, and
  output lineage before a validation claim closes;
- comparator-as-flag posture under ADR-0017;
- forcing reconstruction and operational lanes under ADR-0028;
- calibration/evaluation independence and leakage controls;
- uncertainty, variability, regime stratification, scale, and extrapolation;
- visible negative and superseded evidence;
- independent review and finding disposition; and
- explicit prohibition of whole-model validation claims.

## Scientist-Facing Changes

Static: The strategy now begins with the questions a hydrologist, soil
scientist, researcher, or practitioner should be able to answer. It surfaces
the existing five-climate SNOTEL example and explicitly limits that example to
its snow quantities. It acknowledges that the integrated campaign is primarily
integration/release verification rather than broad empirical validation.

Static: The dossier standard requires named data, observed-variable and regime
coverage, calibration role, figures and residuals, uncertainty, failed regimes,
practical interpretation, and reproducibility. Internal identifiers, hashes,
and test counts are placed in the audit layer rather than at the start of the
scientific narrative.

## Scope Truthfulness

Static: No observational dataset was newly selected or admitted, no campaign
was executed, and no scientific disposition was strengthened. No executable,
test, science-contract, fixture, release-gate, or `.rs` file was touched.
