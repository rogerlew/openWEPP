# openWEPP

> Rust reimplementation of the WEPP (Water Erosion Prediction Project) hillslope and watershed simulation engine.

> **Status:** Pre-alpha. Repository is scaffolding only; no kernels implemented. Kernel cadence is downstream of [wepp-palimpsest](#relationship-to-other-repos) (formerly `wepp-forest`).

> **Provenance policy:** openWEPP is explicitly **not** a clean-room rewrite. See [ADR-0010](docs/decisions/0010-non-clean-room-direct-port-policy.md).

## Overview

openWEPP reimplements the WEPP simulation engine in Rust, taking modernized Fortran-90 kernels from `wepp-palimpsest` as the source of truth for hydrology, soil, plant, and erosion physics. The goal is a kernel-first, process-architected engine with explicit input contracts, parquet outputs, and the trajectory-ownership discipline established by the `wepp-palimpsest` (`wepp-forest` becomes `wepp-palimpsest`)  WB-xx kernelization program.

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
- HBP (hillslope binary pass) shard production and consumption per the wepp-palimpsest contract
- Parquet output via the wepppy / wepppyo3 interchange schemas
- Three executables: single-hillslope CLI, watershed CLI, replay / debug CLI

### Out of scope

- GUI / web frontends (wepppy)
- GIS preprocessing, DEM delineation (wepppy)
- Climate generation / cligen (wepppy)
- Run orchestration, NoDb state model (wepppy)
- WEPP single-storm simulation modes (`ss`, `ss_batch`)
- Legacy stdin-driven `.run` CLI compatibility
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
| `wepp-palimpsest` (was `wepp-forest`) | Authoritative F90 kernel source; oracle for parity validation; science contracts; HBP format spec |
| [wepppy](https://github.com/rogerlew/wepppy) | Consumer / orchestrator; provides GIS, climate, run state |
| `wepppyo3` (in wepppy) | Defines parquet interchange schemas openWEPP emits |
| `openWEPP` (this repo) | Rust simulation engine |

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
│   ├── specifications/    # Science contract registry pointer (sourced from wepp-palimpsest)
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
- wepp-palimpsest — kernel source-of-truth and oracle
- [wepppy](https://github.com/rogerlew/wepppy) — consumer / orchestrator
