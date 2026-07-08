# Required Reading Map

## Authority and Maintenance Responsibility

Agents executing or authoring the package have explicit authority and
responsibility to maintain this file as a living control artifact.

Required maintenance posture:

- Keep entries accurate, complete, and current with package scope.
- Add newly required readings immediately when scope expands.
- Re-tier readings when they move between Core, Conditional, and On-demand.
- Remove stale entries only after verifying they are no longer required.
- Record rationale and trigger updates when authority assumptions change.

A stale or incomplete required-reading map is a governance defect and must keep
package disposition in HOLD until corrected.

## Reading Budget

- local_required_bytes_total: `78723`
- threshold_outcome: `OK` for the core pre-edit set; large contracts are
  conditional/on-demand by touched mechanism.
- measurement_method: `wc -c <core files>`
- measured_at_utc: `2026-07-08T18:11:19Z`

The full relevant contract set is intentionally not a mandatory pre-edit read.
The executor must read the target contract before touching that contract or the
mechanism it governs.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `AGENTS.md` | Core | Root governance for package work | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/work-packages/AGENTS.md` | Core | Work-package execution, gate, review, and consumer-path rules | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Contract-first sequencing and science authority rules | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/standards/AGENTS.md` | Core | Prompt and standards governance | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/standards/prompt-wording-guidance.md` | Core | Kickoff prompt wording and subagent authorization requirements | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md` | Core | Execution-order dependency and post-Tier-1 active-router context | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/package.md` | Core | Package-local scope, phases, gates, and exit criteria | Always | Pre-edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md` | Conditional | Confirms Tier 1 completed and names retained active-router behavior | Tier 1 has completed | Phase A | Agent | `2026-07-08T18:11:19Z` | Package must not implement before this is current. |
| `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/worker-handoff.md` | Conditional | Captures Tier 1 follow-ons or holds that affect this package | Tier 1 has completed | Phase A | Agent | `2026-07-08T18:11:19Z` | |
| WSHED-W7 resume final disposition | Conditional | Confirms sediment-active watershed fixture/publication path is available | WSHED-W7 resume has completed | Phase A | Agent | `2026-07-08T18:11:19Z` | Canonical queue source: `docs/ROADMAP.md` Watershed Runtime Performance Queue. |
| WSHED-W7 resume worker handoff | Conditional | Captures watershed publication follow-ons or holds that affect this package | WSHED-W7 resume has completed | Phase A | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract amendment procedure | Any `SC-*` edit | Before contract edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-profile/BEI compliance | Any `SC-*` edit | Before contract edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md` | Conditional | HBP hourly pair schema, parser, and intake authority | HBP writer/parser/intake policy touched | Before mechanism edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | Conditional | Watershed/channel route and hourly-limb authority | Route kernel or watershed consumer touched | Before mechanism edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Conditional | Lane D active ownership and producer authority | Active producer or missing/mixed active policy touched | Before mechanism edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | Conditional | Exported sediment mass and hourly sediment timing authority | Sediment timing or mass consumer touched | Before mechanism edit | Agent | `2026-07-08T18:11:19Z` | |
| `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` | On-demand | Runoff source-shape authority | Runoff partition source-shape changed | Phase-local | Agent | `2026-07-08T18:11:19Z` | Avoid unless scope expands. |
| `crates/AGENTS.md` | Conditional | Rust crate-local guidance | Rust implementation touched | Before Rust edit | Agent | `2026-07-08T18:11:19Z` | |
| `tests/AGENTS.md` | Conditional | Test conventions | Tests touched | Before test edit | Agent | `2026-07-08T18:11:19Z` | |

## Change Log

| UTC | Agent | Change |
| --- | --- | --- |
| `2026-07-08T18:11:19Z` | Codex | Initialized required-reading map from canonical template. |
