# openWEPP

> Rust reimplementation of the WEPP (Water Erosion Prediction Project) hillslope and watershed simulation engine.

> **Status:** Pre-alpha. Repository is scaffolding only; no kernels implemented.

> **Strategy policy:** architecture-first with top-down science contracts, plus legacy-comparator investigation lanes. See [ADR-0011](docs/decisions/0011-architecture-first-top-down-science-contracts.md).

> **Provenance posture:** explicitly non-clean-room; legacy source may be read for static analysis and provenance mapping.

## Overview

openWEPP reimplements the WEPP simulation engine in Rust with an
architecture-first approach: typed state, explicit module boundaries,
contract-first interfaces, and deterministic orchestration.

Science behavior is governed by top-down contracts derived from:
- WEPP technical references (including `references/50201000`),
- literature-backed invariants,
- physical/common-sense invariants,
- static legacy code inspection as secondary evidence.

Legacy binary comparison is used as an investigation signal, not a universal
gold oracle.

Comparator confidence tiers:
- higher confidence: single OFE and daily water-balance surfaces
- lower confidence: hourly and watershed surfaces (investigation triggers)

openWEPP is the simulation engine only. GUI, GIS preprocessing, climate generation (cligen), DEM-to-watershed delineation (TOPAZ / WhiteboxTools), and run orchestration remain [wepppy](https://github.com/rogerlew/wepppy) concerns. openWEPP plugs into wepppy as a subprocess-per-hillslope replacement for the legacy WEPP binary, emitting parquet via the existing `wepppyo3` interchange schemas.

### Why Rust

- **Borrow checker enforces trajectory ownership.** The producer/consumer state-ownership rules formalized in the wepp-palimpsest trajectory-ownership contract map directly onto Rust lifetimes and ownership transfer. The compiler enforces what F90 conventions only request.
- **Typestate makes illegal state machines unrepresentable.** WEPP's OFE loops and channel-network traversals are state machines that the wepp-palimpsest fuzzing program has spent weeks debugging. Rust's typestate pattern eliminates a class of bugs at compile time.
- **No implicit numeric promotion.** Silent `f32` / `f64` conversions that legacy WEPP is full of become compile errors.
- **Modern numerics ecosystem.** `ndarray`, `polars`, `arrow`, `parquet`, `rand_chacha` provide a mature, audited foundation that Fortran cannot match.

### In scope

- Hillslope simulation (single OFE and multi-OFE)
- Watershed channel routing and impoundment routing
- WEPP soil, management, climate, and watershed input-file compatibility
- Initial backward-compatibility bridge for legacy stdin `.run` plus `.txt`
  sidecar inputs/flags
- HBP (hillslope binary pass) shard production and consumption per the wepp-palimpsest contract
- Parquet output via the wepppy / wepppyo3 interchange schemas
- Three executables: single-hillslope CLI, watershed CLI, replay / debug CLI

### Out of scope

- GUI / web frontends (wepppy)
- GIS preprocessing, DEM delineation (wepppy)
- Climate generation / cligen (wepppy)
- Run orchestration, NoDb state model (wepppy)
- WEPP single-storm simulation modes (`ss`, `ss_batch`)
- Silent fallback when legacy sidecar inputs are missing or ambiguous
- Sediment routing physics (deferred to the wepp-palimpsest sediment kernelization program)

## Runner and release boundary

openWEPP owns its own launcher boundary through `openwepp_runner` and does not
inherit legacy `wepp_runner` fallback behavior by default.

Release binaries follow `openwepp_YYMMDD*` naming and require JSON sidecars for
all roles (watershed, hillslope, replay). See:

- [docs/contracts/openwepp-runner-contract.md](docs/contracts/openwepp-runner-contract.md)
- [docs/contracts/openwepp-binary-release-contract.md](docs/contracts/openwepp-binary-release-contract.md)

## Relationship to other repos

| Repo | Role |
|---|---|
| `wepp-palimpsest` (was `wepp-forest`) | Legacy implementation surface for static analysis and comparator signals; HBP format reference |
| [wepppy](https://github.com/rogerlew/wepppy) | Consumer / orchestrator; provides GIS, climate, run state |
| `wepppyo3` (in wepppy) | Defines parquet interchange schemas openWEPP emits |
| `openWEPP` (this repo) | Rust simulation engine and top-down contract authority for openWEPP behavior |

## Repository layout

```
.
├── AGENTS.md              # Codex coding agent guide
├── CLAUDE.md              # Claude Code reviewer / debugger guide
├── README.md              # This file
├── LICENSE                # CC0-1.0
├── Cargo.toml             # Workspace root (members empty until first crate)
├── rust-toolchain.toml    # Pinned toolchain
├── deny.toml              # cargo-deny: license allowlist, no viral copyleft
├── docs/
│   ├── README.md          # Doc index
│   ├── decisions/         # Architecture decision records
│   ├── specifications/    # openWEPP science-contract authority and source hierarchy
│   ├── contracts/         # Interface contracts (.run, HBP, parquet schemas)
│   ├── architecture/      # Process architecture, data flow
│   ├── numerics/          # Determinism, RNG, summation policy
│   ├── standards/         # Rust coding, comments, and QA standards
│   └── work-packages/     # Dated initiative tracking convention
├── references/            # Scientific references (tracked bibliography + local cache policy)
└── usersum/               # End-user docs (vendorable into wepppy's usersum)
```

Crate layout under `crates/` is intentionally undecided pre-bootstrap; the first work package that lands a crate establishes the convention.

## License

CC0-1.0. See [LICENSE](LICENSE) and [docs/decisions/0001-license-cc0.md](docs/decisions/0001-license-cc0.md).

Dependency license posture: no viral copyleft (GPL / AGPL / LGPL denied at the `cargo deny` gate). See [deny.toml](deny.toml).

## See also

- [AGENTS.md](AGENTS.md) — Codex coding playbook
- [CLAUDE.md](CLAUDE.md) — Claude Code review / debug playbook
- [docs/README.md](docs/README.md) — Documentation index
- [docs/decisions/0011-architecture-first-top-down-science-contracts.md](docs/decisions/0011-architecture-first-top-down-science-contracts.md) — strategy authority
- [wepppy](https://github.com/rogerlew/wepppy) — consumer / orchestrator
