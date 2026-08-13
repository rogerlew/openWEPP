# Required Reading Map

Status: `current / V3 implementation continuation`

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
