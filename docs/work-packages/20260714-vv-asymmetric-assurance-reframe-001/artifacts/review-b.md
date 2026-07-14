# Reviewer B: Epistemic V&V Precision

Status: `HOLD`

Static: Independently reviewed `package.md`, the active kickoff prompt, required
reading map, implementation evidence, canonical V&V strategy, dossier standard,
bibliography entries `R-114` through `R-125`, changed navigation descriptions,
the controlling comparator and observed-data decisions, and the correctness
authority model. Reviewer A artifacts were not read.

Ran: Checked the cited primary records for NRC RG 1.203, NASA-STD-7009B, EPA
environmental-model guidance, ASME V&V 20, the Sandia reports, VERA-CS, Nearing,
Wang et al., PROV-O, RO-Crate, and the Oreskes abstract. Ran scoped Markdown
lint, local-link resolution, terminology/control assertions, docs-only status,
`git diff --check`, Rust-file census, and a high-confidence security scan.

Review disposition: **HOLD**. Two closure-blocking precision findings require
current-package remediation. The overall asymmetric architecture is strong and
the remaining controls are preserved.

## Findings

### B-01 — High — An unverified result can be misclassified as empirical contradiction

The strategy correctly requires the relevant result surface to be verified
before a claim-bearing corroboration status is published
(`openwepp-verification-validation-strategy.md`, lines 134-146). The dossier
standard's terminal fallback is weaker: when *any* mandatory check fails, it
allows an "applicable fail-closed or less favorable status" and only expressly
holds publication of a *favorable* empirical characterization
(`scientific-assurance-dossier.md`, lines 298-301).

That wording permits a reader to assign
`CONTRADICTED_WITHIN_TESTED_DOMAIN` after a mandatory verification failure. A
mismatch from an unverified algorithm, consumer path, unit conversion, or output
lineage cannot yet contradict the model's bounded representational claim. It is
negative implementation evidence or an unevaluable empirical result. Because a
published contradiction can establish a nonuse domain or block a declared
release purpose, its one-way force needs a stronger evidence prerequisite, not
a weaker one.

Required remediation: replace the ambiguous fallback with dimension-specific
rules. A failed, blocked, or unrun mandatory verification obligation must retain
its own `FAIL`, `BLOCKED`, or `NOT_RUN` status and prevent a claim-bearing
`CORROBORATED_WITHIN_TESTED_DOMAIN`, `MIXED_EVIDENCE`, or
`CONTRADICTED_WITHIN_TESTED_DOMAIN` characterization for the affected result
surface. Record the mismatch as visible negative implementation evidence and
use `NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE` for the empirical assessment
until the surface is verified. Separately state that a well-founded
contradiction on a verified surface requires
`CONTRADICTED_WITHIN_TESTED_DOMAIN` and narrows or rejects the affected claim.

Closure mapping: `VVASYM-003`, `VVASYM-007`, and `VVASYM-009`.

### B-02 — Medium — The nuclear contrast overstates characterization of the operating envelope

The nuclear precedent is otherwise carefully bounded, and bibliography entry
`R-114` accurately limits RG 1.203 to a precedent for closable requirements,
graded assessment, applicability, uncertainty, configuration control, and
independent review. The strategy nevertheless describes a designed installation
as being held within a "fully characterized operating envelope"
(`openwepp-verification-validation-strategy.md`, lines 101-106).

RG 1.203 exists in part because experimental distortion, scaling, model bias,
and plant-analysis uncertainty remain material inside a declared regulatory
context. "Fully characterized" therefore grants nuclear practice a certainty
that the cited source does not claim and that the surrounding strategy
explicitly rejects.

Required remediation: replace "fully characterized operating envelope" with
language such as "declared licensing basis and controlled operating envelope,"
or state explicitly that the installation is engineered and the envelope is
defined while residual uncertainty remains assessed. Preserve the existing
point that openWEPP has neither a regulator's authority nor a controlled
site-specific licensing context.

Closure mapping: `VVASYM-008`.

## VVASYM Assessment

| Criterion | Status | Reviewer B assessment |
| --- | --- | --- |
| `VVASYM-001` | `PASS` | The strategy separates software verification, empirical corroboration, and application fitness and names their decision owners. |
| `VVASYM-002` | `PASS` | Quantitative residuals, convergence, uncertainty, and comparator deltas remain evidence; binary acceptance begins only after requirement, metric, tolerance, realization, and failure consequence are declared. |
| `VVASYM-003` | `HOLD` | Agreement is partial and revisable and verified contradictions have one-way force, but B-01 leaves negative empirical classification ambiguous when verification has failed. |
| `VVASYM-004` | `PASS` | Dossiers equip the named application decision owner and do not translate an openWEPP evidence status into site-specific authorization. |
| `VVASYM-005` | `PASS` | Verification and empirical vocabularies are distinct; `SUPPORTED` and whole-model use dispositions are prohibited. |
| `VVASYM-006` | `PASS` | A release is an exact realization with all required verification obligations at `PASS`, carrying an immutable, dated, supersedable-without-rewriting as-of corroboration snapshot. |
| `VVASYM-007` | `HOLD` | Evidence summary, worksheet, and content-bound manifest are present, but the standard's fallback must be repaired per B-01. |
| `VVASYM-008` | `HOLD` | Oreskes and EPA are accurately characterized and the nuclear analogy is generally bounded; B-02 corrects one overstatement. |
| `VVASYM-009` | `HOLD` | Calibration separation, uncertainty, scale, comparator-as-flag, negative evidence, independent review, and audit controls remain present; B-01 is the remaining fail-closed classification defect. |
| `VVASYM-010` | `PASS` | Navigation descriptions are consistent; the scoped documentation checks below pass. |
| `VVASYM-011` | `NOT RUN` | This is the initial Reviewer B gate. Finding disposition, accepted-fix verification, the second independent review/verification, and terminal disposition remain sequenced current-package gates. |
| `VVASYM-012` | `PASS` | Scope is documentation-only; no scientific verdict, application decision, runtime behavior, security-bearing payload, or Rust file changed. |

## Source And Terminology Review

Static: The cited source roles are supportable. Oreskes et al. describe
open-natural-system verification/validation limits, nonuniqueness, and
inherently partial confirmation. EPA explicitly prefers corroboration as a
usefulness rather than truth claim, makes quality application-contextual,
separates calibration from independent corroboration, and treats evaluation and
post-audit as continuing. NASA and ASME preserve declared acceptance criteria,
quantitative comparison, and uncertainty. Sandia and VERA-CS support
hierarchical verification, numerical-error assessment, and standardized
reporting. Nearing and Wang support observation variability, cross-regime
evaluation, calibration disclosure, scale, and tail limitations. PROV-O and
RO-Crate are correctly described as optional future export choices.

Static: Terminology is otherwise consistent across the strategy, standard, and
navigation. Historical names such as "integrated validation campaign" are
explicitly reclassified rather than silently treated as broad empirical
corroboration. Comparator agreement remains a flag, not a target. No generalized
Phase-1 platform prerequisite has returned.

## Ran Evidence

| Check | Result |
| --- | --- |
| `markdown-doc lint` over 14 canonical, navigation, and package files | `PASS`: 0 errors, 0 warnings |
| Independent relative-link resolution | `PASS`: 60 local links, 0 missing |
| Required terminology, preserved-control, and `R-114` through `R-125` assertions | `PASS` |
| Documentation-only status census | `PASS`: 15 changed paths, all Markdown |
| `git diff --check` over tracked package-owned files | `PASS` |
| Tracked Rust diff and package-local Rust census | `PASS`: 0 and 0 |
| High-confidence credential, token, private-key, and private-path scan | `PASS`: no matches |

## Gate Non-Deferral

Static: The package and gate-results artifact truthfully remain active with
review and verification pending. Neither finding may be deferred to a later
dossier campaign or evidence-platform package: both are small, in-envelope
canonical wording fixes required before this package can satisfy its own exit
criteria. Documentation-only CRAP and heavy runner exemptions are legitimate;
they do not waive dual review, finding disposition, accepted-fix verification,
terminal documentation gates, or final disposition.

Final Reviewer B recommendation: **HOLD pending B-01 and B-02 remediation and
the remaining required review/verification sequence.**
