# Required Reading Map

Status: `current / V5 capped-pass implementation continuation`

Evidence mode: `Ran + Static`

Ran `wc -c` from `/home/workdir/openWEPP` after expanding the package. Core is
exactly `528317` local bytes, disposition `WARN` under the canonical thresholds.
The 392,536-byte work-package catalog dominates the total but remains Core
because it governs current package/campaign lifecycle and successor ordering.

## Core

| Bytes | Path | Why required before edits |
|---:|---|---|
| 11,927 | `AGENTS.md` | Repository authority, safety, science, and validation invariants. |
| 20,921 | `docs/codex_exec_plans.md` | Self-contained living-plan requirements. |
| 26,367 | `docs/work-packages/AGENTS.md` | Package execution, evidence, delegation, and closure rules. |
| 392,536 | `docs/work-packages/README.md` | Current campaign/package lifecycle and dependency ordering. |
| 15,309 | `docs/standards/kernel-work-package-preparation.md` | Mandatory pre-implementation preparation and scaffold gate. |
| 22,200 | `docs/standards/testing-and-gate-strategy.md` | Intent, exact-diff, A0/A1/A3, Critical, and evidence lifecycle. |
| 23,570 | `docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/package.md` | Executable objective, architecture, milestones, write set, and gates. |
| 5,599 | `docs/specifications/science-contracts/AGENTS.md` | Canonical science implementation rules. |
| 5,165 | `crates/AGENTS.md` | Rust crate-local implementation rules. |
| 4,723 | `tests/AGENTS.md` | Contract/integration-test rules. |

## Conditional

- Read the science-contract authoring procedure, kernel profile, and contract
  index only if an authority defect is discovered. Contract edits remain out of
  scope; the trigger stops affected production work pending separate authority.
- Run `tools/agents/find-agents --for <every intended write path>` and read each
  nearest nested instruction file before editing that path.

## On Demand

- Load the exact `SC-*` contract sections and authority-package ledgers for the
  milestone being implemented.
- Load existing crate source/tests/Cargo manifests only when freezing or editing
  the implicated interface.
- Load RHESSys/GIS2RHESSys source only for explicit offline mapping provenance.

## 2026-08-11 Remediation Continuation

Ran instruction discovery for `Cargo.toml`, `Cargo.lock`, both new crates,
kernel-contract, hillslope-orchestrator, `tests/integration`, and this package.
Applicable instructions are `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
and `docs/work-packages/AGENTS.md`. Read the complete `SC-VEGETATION-001` and
`SC-BIOGEOCHEM-001` contracts plus the predecessor equation, numerical,
parameter, ownership, vector, definition, oracle, and model-selection artifacts
before remediation edits.

External checkouts, ignored source PDFs, and generated command output are
excluded from this local Core byte total and become process-triggered evidence.

## 2026-08-12 V2 Stage-B Intake

Instruction discovery was rerun over the complete intended write set and again
resolved `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`, and
`docs/work-packages/AGENTS.md`. Read the complete hold-lift guidance and the
released Stage-A contracts, V2 definition, oracle, topology vectors, ownership
contract, final reviews, terminal verifications, and worker handoff. The frozen
authority predecessor is commit
`817b082d01d194cde61b1cf284bd85e40e44afc9`.

## 2026-08-12 Increment 2A

Ran instruction discovery for `crates/openwepp-vegetation`,
`tests/integration`, and this package tree. Applicable instructions remained
`AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`, and
`docs/work-packages/AGENTS.md`. Re-read the package, testing/gate and kernel
preparation standards, `SC-VEGETATION-001` v6 occupancy amendment and
invariants 073--079, `SC-VEGETATIONTRANSACTION-001`, the numerical tolerance
surface, and the current configuration/state/interception/transaction code
before editing.

## 2026-08-12 Increment 2B Authority Audit

Instruction discovery for vegetation source, integration tests, and this
package again resolved root, crate, test, and work-package instructions. Read
the complete E01--E15 and V2 occupancy sections of `SC-VEGETATION-001`, the
equation/parameter/numerical/vector authority artifacts, the independent Python
calculator, V2 model definition, current radiation/photosynthesis/energy/
hydraulics/numerics/interception/config/state/column/transaction source, and
the historical coupled implementation at commit `02631ae92`. The audit result
is recorded in `potential-pass-hold-legitimacy-audit.md`.

## 2026-08-12 V3 Implementation Continuation

Instruction discovery over the complete intended write set again resolved
`AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`, and
`docs/work-packages/AGENTS.md`. Re-read the package and applicable root, crate,
test, work-package, and science-contract instructions. Read the complete V7
amendment, V3 definition, state/migration selection, potential-pass selection,
failure contract, vector ledger, independent fixture, final science reviews,
terminal verifications, and worker handoff. The frozen V3 authority predecessor
is commit `94a4c99dc1228aa0399c01f4cc9590742960028f`.

## 2026-08-13 V4 Shared-State Runtime Continuation

Instruction discovery over the implementation package, vegetation crate,
hillslope diagnostic, integration tests, and diagnostic fixtures resolved
`AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
`tests/fixtures/AGENTS.md`, and `docs/work-packages/AGENTS.md`. Read
`SC-VEGETATION-001` v8 and the complete V4 authority package definition,
vectors, reference calculator, state-schema/migration selection, review
disposition, review artifacts, and gate history. Also read the V3 runtime
configuration/state/migration, shared C/N, request/capped seams, and public
transaction code before editing. Current executable authority is
`OPENWEPP_C3_WOODY_V4`, SHA-256
`8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`.
V1/V2/V3 definitions and checkpoint evidence remain historical read-only
inputs; the V4 authority package's final heavy/terminal closure remains
pending and is not borrowed as implementation-package evidence.

## 2026-08-13 V5 Capped-Pass Runtime Continuation

Instruction discovery over the implementation package, vegetation crate,
kernel transaction contract, integration tests, and diagnostic fixtures again
resolved `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
`tests/fixtures/AGENTS.md`, and `docs/work-packages/AGENTS.md`. Read
`SC-VEGETATION-001` v9 and the complete V5 authority definition, cap selection,
operand lineage, vector ledger, generator, verifier, review disposition, both
science reviews, both terminal verifiers, gate history, and worker handoff.
The frozen authority predecessor is commit
`b7e6f08b655452c5c59a498ac9becd1439dd21ef`. Exact V5 identities are
definition `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`,
vectors `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`,
and generator `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`.
V1--V4 definitions, HOLDs, reviews, and checkpoint evidence remain immutable
historical inputs.
