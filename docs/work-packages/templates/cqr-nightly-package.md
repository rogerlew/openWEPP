# CQR nightly: {{target_module_path}}

Status: QUEUED
Package: {{package_id}}
ExecPlan: docs/work-packages/cqr-nightly-burndown-execplan.md
Nightly batch: {{batch_ordinal}}
Target rank: {{rank}} of {{selected_count}}
Aggregate admission package: `{{aggregate_admission_package}}`
Aggregate scaffold commit: `{{aggregate_scaffold_commit}}`
Aggregate batch manifest: `{{aggregate_batch_manifest}}`
Master ExecPlan: `{{master_execplan}}`

## Objective and scope
Reduce owned eligible production functions above CRAP 30 to <=30 while preserving
runtime/numerical/API behavior. Scope: {{target_module_path}}, focused
characterization tests and this package record/evidence. No science formula,
tolerance, authority, serialization, public output or fail-closed semantic changes.
Preserve expression grouping, accumulation order, short-circuit and error behavior.
Do not touch unrelated dirty work.

## Intended Write Set
- `{{target_module_path}}`
- `{{test_paths}}`
- `docs/work-packages/{{package_id}}/**`
- `docs/work-packages/active.md`
- `docs/work-packages/README.md`

## Authority and acceptance
Use common package governance, the CQR ExecPlan, relevant mechanical/CQR guidance,
ADR-0021 symbol-level eligibility and affected contracts/tests.
Quality evidence: {{quality_evidence_id}} and retained CURRENT intake receipt.
Intake cannot recollect; only typed STALE/INVALID plus the explicit operator
directive authorizes recollection under the existing quality-evidence tool.

Preserve raw/actionable rows with exact symbol, line, source hash, CRAP/CC/coverage,
classification, gate treatment and evidence. E-* and CRAP-above-30 R-INFRASTRUCTURE
remain actionable. R-OBSERVABILITY, R-IRREDUCIBLE-CRAP and X-* exclusions require
two independent reviewers and canonical registry evidence; no globs, filenames,
prior status or "hard to test" exclusions. Otherwise bounded CQR uses one reviewer,
who also verifies fixes. Target-selection reviewers may continue these assignments.

Confirm coverage or add characterization before decomposition. When tests change,
retain ADR-0021 tier, line/region thresholds, per-function floor and complete
obligation-to-test binding. After measurement must demonstrate no owned actionable
CRAP >30. Unrelated workspace quality debt remains observational.

## Specialized aggregate binding
When aggregate admission applies, preserve the existing executable's exact binding
fields, committed aggregate batch manifest and scaffold identities. These are
specialized inputs, not permission to require narrative sidecars.
Commit this package.md with its selected target, acceptance and exact Intended
Write Set before implementation as required by that validator; do not create
prompt directories, reading maps or placeholder reports. Run:

```bash
.venv/bin/python tools/local_ci/check_cqr_aggregate_admission.py \
  --repo . \
  --aggregate-package {{aggregate_admission_package}} \
  --aggregate-scaffold {{aggregate_scaffold_commit}} \
  --module-package docs/work-packages/{{package_id}}/package.md
```

Retain PASS JSON before implementation. Do not retrospectively widen aggregate
bindings. A user prohibition on the required specialized commits blocks this route.

## Current state and evidence
Record selected row/classifications, baseline identity/metrics, characterization,
implementation, after metrics and output identity here. Link raw reports/receipts.
Run focused tests while extracting cohesive blocks. Reconcile terminal diff and
execute selected correctness plus owned metric gates. Critical/campaign/release
boundaries retain full correctness. No default runner agent or line-count gate.

## Review and corrections
Attributable review findings and reviewer-owned fix verification go here.
Two reviewers must accept denominator/exclusion changes; otherwise use the common
risk-based count. No separate target-selection/review/terminal-verifier reports.

## Disposition and recovery
Complete only with owned metrics, behavioral evidence, legitimate selected gates
and required review satisfied. Record exact completion/hold status and commit.
For local target HOLD, preserve evidence and roll back only package implementation
to the known baseline, then record proof, blocker/owner and actionable follow-on.
Global baseline/tooling/overlap/output-identity failures stop the batch.
Never revert unrelated work. Finish the completion/hold commit before the next
target where the specialized CQR batch requires it.
