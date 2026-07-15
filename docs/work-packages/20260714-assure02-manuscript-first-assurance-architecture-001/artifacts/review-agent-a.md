# Review A — Scientific Communication And Pilot Integrity

Review class: internal coding-agent review; not external domain peer review

Evidence class: Static + limited Ran

Verdict: **HOLD**

The architecture materially fixes the v1 audience/headline failure. It makes a
conventional scientific manuscript the public product, separates verification
from empirical corroboration and application fitness, prevents candidate
publication, and preserves rather than diminishes snow/frost science. The hold
is caused by correctable pilot-evidence and manuscript-integrity gaps.

The reviewer checked SHA-256 identities for the groundwater contract,
implementation, H2637 evidence and arithmetic log, and downstream consumer
proof against their current files. The reviewer did not rerun scientific tests
and made no workspace edits.

## Findings

### RA-001 — Major — Claim-evidence record is not independently reproducible

Anchors:

- `artifacts/groundwater-claim-evidence-matrix.md:18-21`
- `artifacts/prototype-linear-groundwater-reservoir-evaluation.md:101,240-258`

`GW-P07` contains a literal `<assessed paths>` placeholder, so the unchanged-
path comparison cannot be rerun. `GW-P08` records test counts without exact
commands, logs, exit records, source root, or hashes. `GW-P05` and `GW-P06`
provide digests and abbreviated artifact names without full repository
locators. The headline surface-router negative proof is mapped to the wrong
M-T2 artifact.

Requested remediation:

- replace the placeholder with the complete ordered path list and exact
  command;
- retain comparison and focused-nextest command output with commit and SHA-256
  identities;
- give full repository paths for every retained artifact and test;
- add a distinct active surface-router negative-proof claim and cite the M-T2
  consumer-proof artifact; and
- make every key finding resolve directly to a claim ID.

### RA-002 — Major — Material formulation-domain boundary is omitted

Anchors:

- Prototype lines 81-97, 118-126, and 212-228
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md:86,139-148,266-267,283-294`

The manuscript gives `kb` and `ks` units of `d^-1` while treating
`Qb_i = kb S_i` and `Qs_i = ks S_i` as daily volumes without stating the
implicit one-day integration convention. It also omits that current openWEPP
admits only nonnegative `ks`, excludes upward lower-aquifer recharge represented
by negative `ks` in the broader lineage, and fails when combined daily exports
exceed accepted storage.

Requested remediation:

- state the discrete daily convention, such as `Q = k S Δt` with `Δt = 1 d`;
- state the admitted `kb >= 0`, `ks >= 0`, and combined-export domain;
- identify negative `ks` or upward recharge as outside current authority and
  requiring separate model/contract work; and
- add the exclusion to Limitations and the claim-evidence matrix.

### RA-003 — Major — Pass conclusions omit acceptance tolerances

Anchors:

- Prototype lines 13-16, 110-116, and 145-178
- `docs/standards/scientific-model-evaluation-report.md:103-107`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:1657-1661`
- `tests/integration/laned_shadow_h2637.rs:655-672`

The report says “exactly to the test tolerance” and that H2637 identities
“close” without stating either acceptance rule. Current tests use absolute
`1.0e-12` for the two-day vector and
`1.0e-9 * max(|storage|, 1)` for each H2637 recurrence identity. Observed
values pass comfortably, but the criterion must precede interpretation. H2637
is a recurrence/publication-ledger reconstruction, not numerical convergence
or solver-error characterization.

Requested remediation:

- state both tolerances, units/scaling, provenance, and rationale in Methods;
- report observed and allowed residual with the result; and
- distinguish analytical code verification and ledger reconstruction from
  numerical-solution convergence.

### RA-004 — Moderate — Public open-research assets have no clear owner

Anchors:

- `docs/governance/scientific-assurance-v2-architecture.md:21-26,69-78`
- `docs/standards/scientific-model-evaluation-report.md:216-231`
- `docs/governance/scientific-assurance-v2-source-build-contract.md:79-90`

The architecture describes the machine bundle as internal and says selected
safe assets may be downloadable, while the report standard requires project-
owned claim-bearing evidence, source rows, figure data, procedures, and
retained objects to be available. The reader path mentions open-research assets
but assigns no publication owner.

Requested remediation: define a mandatory public research-object surface for
safe claim-bearing data, table and figure sources, methods, software and
configuration identities, and reproduction material. Keep review locks,
protected evidence, and build internals internal. Add publication checks proving
required safe assets are present.

### RA-005 — Moderate — Internal vocabulary and observational context

Anchors:

- Prototype lines 13-19, 38-52, 63-70, 118-130, 147-174, and 204-210
- `docs/standards/scientific-model-evaluation-report.md:50-77,182-191`

`OFE`, `HBP`, and `cbase` reach the abstract or main methods without
reader-facing definitions. The tables lack explicit descriptive titles. The
phrase “observational support for the scientific formulation” is stronger than
the Priest River study warrants without saying it was a calibrated,
coupled-model evaluation with forcing and parameter confounding.

Requested remediation:

- define overland-flow element (OFE), hillslope binary pass (HBP), and separate
  channel baseflow parameter (`cbase`) at first use or move serialization detail
  to the supplement;
- give each table a formal scientific title or caption; and
- describe the Priest River evidence as calibration-conditioned, coupled-model
  observational evidence motivating the formulation, not isolated validation
  of the recurrence.

## Positive Assessment

- Verification, empirical corroboration, comparison, release transfer, and
  application fitness remain distinct.
- The report is recognizably manuscript-first and substantially more useful to
  hydrologists and soil scientists than the v1 status-first page.
- The groundwater conclusion is otherwise bounded and repeatedly rejects
  field-validation and site-fitness inference.
- ASSURE-03 and ASSURE-06 preserve snow/frost contracts, datasets, campaigns,
  results, and narrative while retiring only the failed publication
  architecture.
- No citation value checked against the retained Srivastava paper was
  numerically contradicted.
