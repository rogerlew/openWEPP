# Required Reading Map

Status: `frozen for kickoff / execution not started`

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
