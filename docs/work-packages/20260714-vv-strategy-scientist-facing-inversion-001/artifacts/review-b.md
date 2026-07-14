# Independent Review B

Evidence class: **Static + Ran**

Static: Reviewed the package, strategy, dossier standard, ADR-0017, ADR-0028,
correctness-authority model, applicable agent instructions, package evidence,
navigation changes, and current SNOTEL authority without consulting Reviewer
A's artifact.

Ran: Executed scoped Markdown lint, local-link resolution, spelling previews,
diff-integrity checks, Rust-touch checks, and a credential/private-path scan.

Recommendation: **HOLD**

## Findings

### B-01 — High, blocking: baseline dossiers can bypass claim-bearing review

The standard starts the workflow by defining an "initial disposition" before
evidence inventory (`docs/standards/scientific-assurance-dossier.md:142-149`). It
then exempts baseline dossiers from steps 4 through 6, which include frozen
criteria and independent scientific/evidence review
(`docs/standards/scientific-assurance-dossier.md:150-163`). The strategy's first
roadmap phase nevertheless permits baseline dossiers to publish `SUPPORTED`
states from existing evidence
(`docs/governance/openwepp-verification-validation-strategy.md:258-265`).

Retaining an historical result is not the same as qualifying a newly bounded
public use. Mapping evidence into a new dossier, claim envelope, or
`SUPPORTED[_WITH_LIMITATIONS]` disposition is itself a claim-bearing assessment
unless an existing reviewed verdict has exactly the same use, version, quantity,
scale, regime, criteria, and limitations. As written, the exception can turn
integration, authority, or campaign evidence into a new positive public verdict
without the independent review required by the strategy at
`docs/governance/openwepp-verification-validation-strategy.md:224-226`. That
conflicts with the package's no-new-verdict boundary
(`docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/package.md:48-56`).

Required disposition:

- default an unassessed dossier to `NOT_ASSESSED`, not an unconstrained
  "initial disposition";
- state that a historical verdict may be carried forward only when its bounded
  claim and independent review are demonstrably identical and current;
- treat every new mapping or use qualification as a new assessment; and
- exempt retrospective baselines only from impossible prospective
  preregistration, while requiring disclosure of post hoc criteria/bias and
  independent dossier review for every positive public disposition.

### B-02 — High, blocking: the deferred-tooling audit kernel lacks immutable result binding

The strategy requires negative and superseded evidence to remain discoverable
(`docs/governance/openwepp-verification-validation-strategy.md:99-104`) and calls
Markdown plus retained manifests sufficient for the initial audit basis
(`docs/governance/openwepp-verification-validation-strategy.md:160-164`). The
standard requires hashes only as an alternative for inputs that cannot be
tracked (`docs/standards/scientific-assurance-dossier.md:104-107`). Outputs,
logs, figures, and failed or superseded runs need only be "retained"
(`docs/standards/scientific-assurance-dossier.md:110-119`).

That is reproducibility metadata but not an evidence-integrity seal. A result,
plot, failure log, or transformation can change in place while the dossier and
its disposition remain textually unchanged. Deferring a crate, schema, database,
or provenance service is reasonable; deferring content identity for
claim-bearing evidence is not. This leaves `VVINV-006`'s minimum audit kernel
incomplete and weakens the promised auditability for both humans and agents.

Required disposition: require a lightweight tracked manifest for every dossier
that records stable relative paths, SHA-256 (or an explicitly named equivalent)
for all claim-bearing inputs, transformation code/configuration, outputs, logs,
figures, reviews, and material failed/superseded evidence; exact source,
executable, and assessment identities; and an explicit unavailable/external
state with access posture. Bind the dossier version to that manifest. This can
be authored manually and does not make generalized tooling a publication
prerequisite.

### B-03 — Medium, blocking: the standard is active before its activation gates pass

The new standard declares `Status: Active`
(`docs/standards/scientific-assurance-dossier.md:1-4`), the strategy says the
public dossier standard is active
(`docs/governance/openwepp-verification-validation-strategy.md:3-9`), and the
standards index publishes it as an Active normative standard
(`docs/standards/README.md:6-20`). The package is still `ACTIVE`, requires every
exit criterion to pass, and has not completed dual review and verification
(`docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/package.md:109-125`).
The gate artifact truthfully records that gate as `NOT RUN`
(`docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/artifacts/gate-results.md:3-18`).

This makes the normative activation claim outrun its own evidence. It is a
status-semantics and gate-non-deferral defect even though the package itself has
not claimed `COMPLETE`.

Required disposition: keep the new standard and its index/delivery-maturity
language `Draft` or `Pending Review` until both reviews, finding disposition,
accepted-fix verification, and final package disposition pass. Promote all
status surfaces together as the terminal activation step.

## Scientific Integrity Results

The following review dimensions pass at the current revision:

- **V&V distinction:** verification covers requirements, code, numerical
  solution, production consumption, and output lineage; validation addresses
  real-system adequacy; neither can substitute for the other.
- **Calibration/evaluation separation:** evaluation roles are frozen before
  verdict-bearing work; leakage across shared systems/events is prohibited; and
  post hoc threshold, case, or tolerance changes reset the assessment.
- **Uncertainty and applicability:** material measurement, sampling, forcing,
  parameter, model-form, numerical, transformation, variability, scale, and
  extrapolation effects remain visible; unknown uncertainty is not zero.
- **Comparator posture:** ADR-0017 is preserved exactly as flag-not-target with
  like-for-like quantity lineage and independent authority required for physical
  conclusions.
- **Bounded claims and release language:** dispositions attach to named uses,
  quantities, versions, scales, and regimes. Green CI, conservation, empirical
  fit, or parity cannot independently close a release claim.
- **SNOTEL characterization:** the strategy accurately identifies the five-site
  SWE/depth/density corpus and forcing-robust decomposed rubric, requires a
  current audit before a public dossier, and does not extend the evidence to
  runoff, erosion, growth, routing, or watershed support. The current contract
  also retains bounded/open residual and no-paired-snow limitations, so the
  strategy does not create a new scientific verdict.
- **Authority separation:** dossiers remain evidence/use-qualification records;
  they do not replace canonical `SC-*` authority, ADR-0028's bounded
  observed-data admission conditions, or the correctness-authority model.
- **Security and scope:** reproduction guidance prohibits exposed credentials
  or restricted payloads. No executable, fixture, dataset, contract, release
  gate, or Rust file is owned by this package.

## Exit-Criterion Audit

| Criterion | Status | Review result |
| --- | --- | --- |
| `VVINV-001` | `PASS` | Named audiences and seven practical transparency questions are explicit. |
| `VVINV-002` | `PASS` | Honest baseline dossiers lead the roadmap and include gap/negative states. |
| `VVINV-003` | `PASS` | Verification is mandatory and visible without leading the human summary. |
| `VVINV-004` | `PASS` | Strengths, gaps, and SNOTEL evidence are bounded truthfully. |
| `VVINV-005` | `PASS` | Claim-driven plans and campaigns precede generalized infrastructure. |
| `VVINV-006` | `FAIL` | The human structure is usable, but B-01 and B-02 leave status transition and audit-kernel defects. |
| `VVINV-007` | `PASS` | General schemas, crates, databases, and exporters are not prerequisites. |
| `VVINV-008` | `PASS` | Reviewer reruns confirm lint, links, spelling, and diff integrity. |
| `VVINV-009` | `NOT RUN` | Dual review/disposition/verification is still in progress, as the gate artifact states. |
| `VVINV-010` | `PASS` | Documentation-only scope, no new verdict, low security impact, and no Rust touch are truthful. |

Per the package rule, `VVINV-006=FAIL` and `VVINV-009=NOT RUN` prevent a
`COMPLETE` disposition. B-01 through B-03 require explicit disposition and
accepted-fix verification.

## Reviewer-Executed Checks

| Check | Result |
| --- | --- |
| `markdown-doc lint` over the strategy, standard, indexes, package, and reading map | **PASS**, 8 files, 0 errors, 0 warnings |
| Local relative-link resolution over the principal changed documentation | **PASS**, 59 links, 0 missing |
| `git diff --check` over the package write set | **PASS** |
| `uk2us` preview over the new strategy, standard, package, and indexes | **PASS** for package changes; only unrelated historical catalog suggestions appeared |
| Tracked `.rs` diff and package-local `.rs` census | **PASS**, 0 and 0 |
| Credential/private-path scan | **PASS**, only policy prohibitions matched |

Final recommendation: **HOLD** until B-01 through B-03 are resolved and the
remaining package review, disposition, and verification gates pass.
