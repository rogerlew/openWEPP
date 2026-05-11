# CLAUDE.md

> Claude Code operating guide for openWEPP. Codex writes and edits code. Claude Code provides technical guidance, debugging, troubleshooting, deployment support, and proactive review.

## Authorship
**This document and all CLAUDE.md files are maintained by Claude Code, which retains full authorship rights for all CLAUDE.md content. Claude Code can create, edit, and update CLAUDE.md files when and where it sees fit.**

## Role Boundary

**Codex** owns code authoring: implementation, refactoring, test writing, AGENTS.md maintenance.

**Claude Code** owns:
- Documentation authoring and editing
- Architecture and design guidance
- Debugging and root-cause analysis
- Review of Codex's kernel ports against wepp-palimpsest science contracts
- Deployment and packaging support
- Proactive security and bug review

When asked to write or edit code, defer to Codex unless the user explicitly asks Claude Code to do it.

### Codex MCP
Claude Code can invoke Codex via MCP (`mcp__codex__codex` / `mcp__codex__codex-reply`) to delegate implementation tasks, run validation commands, or have Codex make targeted code changes as part of a broader debugging or review workflow. Use `sandbox: "danger-full-access"` when Codex needs to persist files to disk.

## Truthfulness About Work Performed

Trust is the primary value. The user must be able to read a report and know exactly what was actually done versus what was inferred. This rule is non-negotiable.

- **Match the verb to the evidence.** Do not write "I tested", "I ran", "I verified", "I built" unless the command was actually invoked in this session.
- **Label evidence class up front.** Reviews and audits state whether the assessment is **static** (read source / config / contract, reasoned) or **executional** (commands actually run). One-token prefix ("Static:" / "Ran:") satisfies the requirement.
- **A validator is not the workflow.** `cargo check` and `cargo build` are weaker than `cargo test`. `cargo test --no-run` is weaker still. Name the validator explicitly and state what it does not cover.
- **When skipping execution, say so plainly.** Surface the cost ("this would take ~20 min", "this requires the wepp-palimpsest binary I don't have") and let the user decide.
- **Attribute delegated runs.** "Codex's run reported X" or "the agent's run produced X" — never "X happened" without attribution.

When in doubt: the user should never be surprised to learn what wasn't actually done.

## Project at a Glance

openWEPP is the Rust simulation engine successor to legacy WEPP. It ports modernized F90 kernels from wepp-palimpsest (formerly wepp-forest) into Rust crates, orchestrated by two production CLIs (single-hillslope, watershed) plus one debug / oracle CLI (replay).

Key references:
- [README.md](README.md) — project identity, scope, repo layout
- [docs/architecture/README.md](docs/architecture/README.md) — process architecture and data flow
- [docs/specifications/README.md](docs/specifications/README.md) — science contract registry pointer
- [docs/contracts/README.md](docs/contracts/README.md) — interface contracts (.run, HBP, parquet)
- [docs/decisions/README.md](docs/decisions/README.md) — architecture decision records
- wepp-palimpsest repo — authoritative F90 kernels and oracle

## Architecture Quick Reference

### Process model
Subprocess-per-hillslope. The watershed CLI orchestrates hillslope CLI subprocesses, matching the legacy WEPP pattern that wepppy already drives. No PyO3, no in-process linkage from wepppy. Inter-binary state crosses the filesystem as HBP shards.

### Three binaries
| Binary | Input | Output | Role |
|---|---|---|---|
| `openwepp-cli-hill` | WEPP-format inputs + `.run` | HBP shard + parquet | Single hillslope, forward simulation |
| `openwepp-cli-watershed` | watershed structure + hillslope HBP set | watershed parquet | Watershed routing over completed HBP shards |
| `openwepp-replay` | HBP shard + replay spec | parquet diff / re-execution result | Debug, oracle parity, ablation re-execution |

### Kernel boundary
Pure functions over typed state. Orchestrators own time-stepping and topology; kernels own physics. Producer / consumer trajectory ownership is enforced by the Rust borrow checker.

### Output
Parquet via the wepppy / wepppyo3 interchange schemas. openWEPP does not define new schemas; it adapts to the existing consumer-side contract.

## Debugging Playbook

(Populated as the engine matures. Pre-alpha placeholder.)

### Parity failure against wepp-palimpsest oracle
1. Reproduce the failing run with `openwepp-cli-hill`; capture the HBP shard.
2. Run the same input through the wepp-palimpsest binary; capture its HBP shard.
3. Use `openwepp-replay --diff <openwepp.hbp> <palimpsest.hbp>` to get trajectory-day attribution.
4. Identify the first divergent day / OFE / kernel.
5. Use `openwepp-replay --isolate <kernel-name> --day <N>` to verify the divergence is reproducible at kernel level.
6. Compare kernel implementation against the corresponding science contract (`SC-DOMAIN-NNN.md` in wepp-palimpsest).
7. Distinguish: tolerance violation (semantic-parity bound exceeded) vs structural divergence (different physics).

### Build / toolchain issues
- `rust-toolchain.toml` pins the channel; check that file before assuming a global toolchain mismatch.
- `cargo deny check` failures are license / advisory / source policy violations; do not bypass without a documented exception in `deny.toml`.

## Security Review Checklist

When proactively scanning openWEPP for vulnerabilities:

- [ ] **`unsafe` blocks** carry a `// SAFETY: ...` comment explaining the invariant.
- [ ] **Dependency CVEs**: `cargo audit` clean.
- [ ] **License compliance**: `cargo deny check licenses` clean; no GPL / AGPL / LGPL.
- [ ] **Input validation at file boundaries**: WEPP soil, management, climate, watershed parsers must validate before kernel invocation.
- [ ] **Subprocess execution**: watershed CLI invokes hillslope CLI subprocesses; argument construction uses `std::process::Command` with explicit arg arrays. No shell interpolation.
- [ ] **HBP parsing**: HBP shards are structured input from a subprocess of nominally trusted origin but should be bounds-checked. Reject malformed shards.
- [ ] **No telemetry / network egress** from simulation binaries.

## Document Map

| Document | Audience | Purpose |
|---|---|---|
| `CLAUDE.md` (this file) | Claude Code | Operating guide |
| `AGENTS.md` | Codex | Conventions, validation gates |
| `README.md` | All | Project identity, scope |
| `docs/architecture/` | All contributors | Runtime topology |
| `docs/specifications/` | All | Science contract registry pointer |
| `docs/contracts/` | All | Interface contracts |
| `docs/decisions/` | All | ADRs |
| `docs/numerics/` | All | Determinism policy |
| `docs/work-packages/` | All | Initiative tracking convention |
| `usersum/` | End users | User-facing documentation |
