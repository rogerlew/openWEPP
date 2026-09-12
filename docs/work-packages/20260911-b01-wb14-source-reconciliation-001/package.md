# B01 WB14 bounded source reconciliation

Status: SCAFFOLDED; execution NOT STARTED. Cadence repair, wider B01 modeling,
production and release remain HOLD.
Preparation source: `16cc53dbb17a0fad371ebeea36939d98c65ad36a`.

## Objective and authorization

Produce an independently reviewed source-provenance and test-scope decision for
resuming [verified cadence repair](../20260911-b01-wb14-verified-cadence-repair-001/package.md).
The predecessor stopped because its exact working145 recipe reproduced only
925 of 927 required file hashes. This package assesses the available source and
prepares a concrete revised-baseline proposal if exact recovery remains unavailable.
It does not adopt that proposal or resume runtime correction.

The owner requested this scaffold following Astra's recommendation to reconcile
source provenance and harness scope before further cadence work. Current authority
covers preparation of this record and locator/catalog updates only. An instruction
to execute this package adopts the bounded assessment below. Even after execution,
adopting a revised baseline or expanding cadence test-write scope is a separate
owner decision over the completed, reviewable proposal. The predecessor's exact
working145 requirement and historical HOLD are not rewritten by this package.

## Evidence and authority

Read the predecessor's recovery stop, [mismatch evidence](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/recovery-mismatch.json),
[full build145 manifest](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/build145-execution-identity.json),
[actual-byte verification](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/parent-actual-byte-verification.json)
and its linked reconstruction recipe/logs. Retain these identities without revision:

| Item | SHA-256 |
| --- | --- |
| Expected working145 927-file map | `b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f` |
| Available partial-source 927-file map | `c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e` |
| Expected `precedence_tests.rs` | `730f9e36c83837251b080ede62069acabef5e19b828c59d1a5fad2a087ca4b10` |
| Available `precedence_tests.rs` | `982dac38c5b45b60eeb6db22a9ad839037359ff3fbc38fae11c10b069d434d76` |
| Expected `raw_hash_tests.rs` | `9b0c40bcada9706a100714fe7b9913bc016124b68be8f5e0f8716efae857d2ad` |
| Available `raw_hash_tests.rs` | `041c0df1739385995e5cc6c241510523e0e4c74bedfba34bf152c501461e53fd` |

Both test paths are under
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/`.
Partial source is retained at
`/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs`.
Neither an expected hash nor agreement on the other 925 entries reveals the
missing bytes or proves their semantic equivalence. Compare available variants
only against explicitly identified available variants; never claim an exact
missing-versus-available diff without recovering both versions.

Pin governing authority separately from runtime source at execution adoption.
For interpretation of the affected guards and acceptance, read applicable
science-contract instructions and the predecessor's selected canonical sections:
SC-SURFACELIQUID-001 `INV-012/013/014/026/035`, `OBL-C-016/C-025`, ordered guard
table and parent/child schema; SC-COUPLEDTIME-001 `INV-005/006/024/031`,
`OBL-014`; SC-SNOWENERGY-001 `INV-083/087`, `OBL-C-055`. Expand dependencies when
the two tests or consumer/harness inventory expose another affected obligation.
Package evidence does not replace canonical authority. The predecessor's
contract-local change-log finding remains an authority-maintenance follow-up;
this package does not amend science or adjudicate it away.

## Proposed execution envelope

Astra owns assessment, integration, custody and disposition under
[bounded execution](../../standards/bounded-agent-execution.md). An assigned
implementer owns detailed source/test inspection and deterministic evidence
scripts. Use explicit read/write bounds and no nested delegation. Retain two-cycle
or 60-active-minute internal reassessment; continue only with a concrete supported
in-scope action. Historical recovery failures/time remain in the predecessor.
New package accounting never makes exact working145 recoverable or resets its
consumed allowance.

Allowed writes on execution: this package's record and artifacts, locator/catalog
status, and new source copies/evidence scripts under the proposed durable root
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/`. Inspect that
destination before creating anything; preserve existing unique state. Use repo
`.venv/bin/python`. Keep the predecessor's partial source, manifest, original
cadence patch and logs immutable. Do not create/change branches. Carry scoped
evidence commit/push permission from the predecessor and verify publication
before claiming remote custody; this grants no production-source publication.

Source snapshots may be reconstructed or copied into new directories for
inspection, with exact input identities, safe paths and logged operations.
An independently named revised-source proposal may retain the available two
files verbatim. Do not author replacement Rust, alter test assertions, apply
the cadence candidate, format source, or manufacture bytes to meet a hash.
No test harness may be added or replaced here.

No Rust builds, test execution, authentic runs, saved-checkpoint resume,
performance/warm/seven-day runs, restart repair or production integration.
No equations, solver, thresholds (including B01 0.1/0.12 mm SWE), tolerances,
features/defaults/cfg/test gating, manifests, lockfiles, dependencies, toolchains,
schemas, required-case bindings or canonical contracts may change. Static Cargo
target/module inspection is allowed; inventory is not a compiled test listing.

Stop dependent assessment on unexplained source drift beyond the two known
paths, inaccessible necessary primary evidence, contradictory science authority,
or a need to edit a forbidden surface. Preserve results and return the specific
decision. The already known absence of the two historical files does not prevent
the authorized revised-source assessment; new missing evidence required to
support that assessment does. Do not repeat the exhausted archive/Git search
without a newly identified source and a specific recovery hypothesis.

## Checkpoints and required deliverables

1. **Reconcile available bytes.** Rehash all 927 actual entries against the
   original manifest and reproduce the two mismatches. Verify retained R0/final016
   and correction145 inputs by their identities and evidence; distinguish
   re-executed reconstruction from reused historical stage checks. Record
   symlinks, missing/extra paths and build/test inputs outside the 927-file map.
   A match on that map alone is not a complete workspace/input identity.
   If fresh evidence recovers the historical files, verify their exact hashes
   and the entire reconstructed map before proposing the exact-recovery route.
2. **Assess the two tests.** Trace each available file's provenance through
   retained patches and available Git versions. Bind actual test names, enclosing
   modules, Cargo registrations, feature/ignore conditions, fixtures, assertions,
   error-precedence and raw-hash obligations, and production paths exercised.
   Assess risks to cadence, complete-owner rollback, receipt authentication,
   serialization and independent operands. Label unavailable historical changes
   UNKNOWN, not formatting-only or behavior-preserving. Explain whether and how
   fresh contract-derived tests could supply sufficient prospective coverage;
   do not claim they restore historical evidence or implement them here.
3. **Resolve harness scope prospectively.** Inventory the real ignored
   fresh-process prefix/cadence harness across reconstructed source, exact module
   registration and retained references. Give its source, fully qualified test
   name, process entrypoint, fixture lineage, features and proposed explicit
   ignored-test command. Separate the exact day-4/interval-22/transaction-255
   boundary from generic cadence tests. Trace actual ingress/WB14 and receipt
   custody, split/replay and complete-parent coverage. If missing or outside
   prior write bounds, identify the evidence gap and propose the minimum exact
   test-path/scope amendment for owner adoption. Do not substitute the unrelated
   terminal-event cadence test or claim a static name executes successfully.
4. **Prepare one source decision.** Recommend either verified exact recovery,
   a distinctly named revised baseline, or rejection/HOLD. A revised proposal
   must contain recoverable source bytes/base-plus-patches, full actual input
   identity, the two explicit substitutions, unchanged production-file proof
   relative to the retained recipe, unknowns and their impact, and a concrete
   amendment to the predecessor's source/test envelope recorded here only.
   Include any required expanded harness paths and commands. If unchanged
   production or sufficient test authority cannot be established, recommend HOLD.
5. **Review and preserve.** Correct findings within these bounds, retain raw
   comparisons and attributable review in this record, reconcile the terminal
   diff, and verify committed/remote evidence and necessary recovery bytes
   (including any required LFS content). Report local-only or missing custody
   explicitly. Return the concrete owner decision; do not resume cadence work.

## Acceptance and validation

Assessment success means a supported, independently reviewed recommendation,
including a negative recommendation if warranted. It never means cadence repair
PASS, source adoption, production readiness or empirical validation.

| Requirement | Acceptance evidence |
| --- | --- |
| Source identity | Actual-byte 927-file comparison, complete relevant workspace/input inventory, preserved historical identities and source-custody proof; no unnamed substitution. |
| Semantic claims | Primary source locations and available-version diffs; unavailable historical semantics explicitly UNKNOWN with consequence assessed. No unsupported equivalence claim. |
| Test authority | Registered target/module/feature/fixture/assertion lineage for the two files, touched canonical obligations and concrete coverage gaps. No weakened or removed requirement. |
| Harness | Verified static entrypoint/registration and proposed command, or a precise absence/out-of-scope finding plus proposed amendment. No helper-only or authentic-resume substitute. |
| Decision | One exact-source/revised-source/HOLD recommendation; proposed revised identity and scope are reviewable, but not adopted. Every material unknown receives an explicit disposition. |
| Preservation | Any exact/revised source proposal can be reconstructed from retained available inputs. A negative HOLD recommendation instead preserves the inspected input identities, negative evidence and custody needed to reproduce its assessment; it need not fabricate a new source proposal. Original partial source and hashes remain intact. Remote claims follow actual retrieval. |

Apply [testing strategy](../../standards/testing-and-gate-strategy.md) directly.
This package changes no executable behavior or authority: run diff/whitespace,
local-link, JSON/schema as applicable, deterministic actual-byte/hash and
comparison checks, script syntax and focused script checks for any added evidence
tool. Record exact argv/cwd, source/input identities, exit status and logs; use
`Static:` versus `Ran:` accurately. No Rust validation is claimed or required
solely for this assessment. If work would require changing executable behavior,
stop for scope instead of silently narrowing its validation.

All predecessor runtime acceptance remains required at any subsequently adopted
cadence execution: matched retrospective red/candidate fixtures, all 18 physical
children through 387000 s, ordinary/rollover and compound-poison controls, exact
complete-owner rollback, real fresh-process/split path, independent operand
reconstruction, warnings-denied Clippy, full correctness and applicable A0/A1/A3.
Baseline replacement does not turn historical failed/NOT RUN checks into passes.
The original preimplementation sequencing violation remains explicit; future
tests and gate evidence must precede further production edits. No numerical
calibration is involved; calibration and identifiability are NOT_APPLICABLE.

Two independent execution reviewers are required because a proposed source/test
trust change has consequential unknowns. Correctness/authority owns semantic
claims, guard/coverage authority, historical sequencing and admissible next scope.
QA/evidence owns actual bytes, reconstruction/input identity, harness registration,
publication and anti-evasion. Neither may have authored the reconciliation or
materially designed the revised-source proposal; a prior recovery author cannot
independently accept their own reused recovery claims. Same reviewers verify
fixes. Missing required review prevents assessment acceptance. Scaffold review
does not substitute for these execution reviews or future cadence reviews.

## Current state and next action

Only the scaffold and locator/catalog are prepared. No new recovery, semantic
comparison, harness inventory, baseline adoption, build or test has run under
this package. All execution requirements above are NOT RUN. Next action after
owner execution adoption: assign the bounded actual-byte/provenance inventory,
then proceed through assessment and review to the concrete source/scope decision.

### Scaffold checks and review

Ran from `/workdir/openWEPP`: `git diff --check`, explicit whitespace/conflict
checks on the new untracked record, and `.venv/bin/python` checks of six local
references and all six retained source/file hashes: PASS. Terminal changes are
this record and the two locator/catalog additions only. No runtime or source
reconciliation workflow ran. At scaffold review the changes were local and
uncommitted; the owner subsequently authorized committing and pushing them.

Independent scaffold reviews reused `/root/cadence_correctness` (Sol/high) and
`/root/cadence_implementation` (Terra/medium, scaffold wording QA only), neither
of whom authored this scaffold. The latter explicitly excluded their own prior
recovery evidence and cannot independently accept that evidence during execution.
Correctness identified one medium acceptance ambiguity: an unconditional
reconstructable-proposal requirement could prevent an otherwise valid negative
HOLD assessment. The preservation row now distinguishes those outcomes; both
same reviewers verified the fix. Attributable final responses:

> Correctness: Fix verified. Static source: untracked package blob
> 261d32309cfc8e3450bc7985beae6df412319385, SHA-256
> 61994dfd03db009db806636e0dc861f6cfc25f1e036a9d0834ee70ff59d2d557
> over HEAD 16cc53db.... Line 158 now makes reconstructability conditional for
> exact/revised proposals and gives negative HOLD an explicit preservation/evidence
> posture without fabricating a proposal. Finding closed; final scaffold verdict
> PASS, no blocking findings. Scaffold only; no assessment execution/source
> adoption/runtime approval.

> QA: Static QA: PASS. The corrected preservation row properly makes
> reconstructability conditional on an exact/revised proposal and allows a
> negative HOLD to preserve identities, negative evidence, and custody without
> fabricating source. Prior recovery evidence remains excluded.

These reviews bind the substantive scaffold before this checks/review appendix;
they do not approve the assessment's unexecuted claims. Preparation is complete;
execution remains NOT STARTED.
