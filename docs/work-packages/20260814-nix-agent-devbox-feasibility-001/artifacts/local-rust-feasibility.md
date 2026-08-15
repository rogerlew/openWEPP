# Local Rust Feasibility Intake

Status: `Ran / provisional ow-dev-01 evidence / no cross-machine claim`

Date: `2026-08-14`

Commit under test: `f069772f2` plus the documented ownership-claim correction.
The worktree was dirty only for developer-environment documentation and helper
changes; no Rust source or Cargo manifest changed.

All arms used the pinned Nix Rust/Cargo `1.95.0`, locked Cargo dependencies,
NVMe-backed isolated targets, and `/usr/bin/time`. These are feasibility probes,
not the final repeated comparison matrix.

## Native Cargo Incremental

Command: `cargo check --workspace`.

| Arm | Wall | User | System | Max RSS |
| --- | ---: | ---: | ---: | ---: |
| Empty isolated target | 18.31 s | 83.40 s | 9.28 s | 501,956 KiB |
| Immediate no-change repeat | 0.08 s | 0.06 s | 0.02 s | 60,824 KiB |

The cold arm included remaining Cargo crate downloads. The warm result proves
the expected single-worktree edit-loop floor but is not an edit/recompile test.

## sccache

All sccache arms set `CARGO_INCREMENTAL=0` and `RUSTC_WRAPPER=sccache`.

| Arm | Wall | Rust hits | Rust misses | Result |
| --- | ---: | ---: | ---: | --- |
| First isolated-target fill | 20.47 s | 0 | 118 | cache populated |
| Second fresh target | 20.55 s | 0 | 118 | no cross-target reuse |
| Same-target clean rebuild | 5.68 s | 118 | 0 | 100% hit rate |
| Normalized fresh-target fill | 20.51 s | 0 | 118 | cache populated with `SCCACHE_BASEDIRS` |
| Normalized third target | 20.46 s | 0 | 118 | no cross-target reuse |

The first attempted normalized fill attached to an already-running server whose
configuration lacked `SCCACHE_BASEDIRS`; that arm was rejected. The server was
stopped, a separate normalized cache was populated, and the third target above
was the decisive fresh-target proof.

## Provisional Decision

- Keep Cargo incremental compilation as the default for active worktrees.
- Keep sccache installed but opt-in; do not set `RUSTC_WRAPPER` globally.
- Do not sacrifice per-agent target isolation to obtain same-target cache hits.
- Retain sccache for later release/clean-rebuild experiments or revisit only if
  Cargo/sccache path identity changes.

## Release Linker Intake

Command:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

The mold arm used
`RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"`. Empty-target results:

| Linker | Wall | User | System | Max RSS | Binary size |
| --- | ---: | ---: | ---: | ---: | ---: |
| Nix/GNU default | 84.10 s | 520.00 s | 13.64 s | 1,417,176 KiB | 11,932,992 B |
| Clang + mold | 85.73 s | 518.78 s | 13.05 s | 1,413,560 KiB | 12,438,824 B |

A source-mtime-only touch forced the runner bin to rebuild against warm
dependency targets without changing tracked bytes:

| Linker | Wall | User | System | Max RSS |
| --- | ---: | ---: | ---: | ---: |
| Nix/GNU default | 29.51 s | 139.57 s | 1.47 s | 1,390,160 KiB |
| Clang + mold | 29.54 s | 140.75 s | 1.48 s | 1,388,340 KiB |

There is no observed mold benefit for this target; Rust/LLVM code generation
dominates. Keep mold available but opt-in.

## Cargo Job-Count Intake

Fresh isolated `cargo check --workspace` targets with downloads already warm:

| Jobs | Wall | User | System | Max RSS |
| ---: | ---: | ---: | ---: | ---: |
| 8 | 18.18 s | 72.48 s | 8.05 s | 501,936 KiB |
| 12 | 17.91 s | 79.82 s | 8.76 s | 502,172 KiB |
| 16 | 17.57 s | 83.48 s | 9.29 s | 502,684 KiB |

Sixteen was fastest in this single-job probe, but only 1.9% faster than 12.
Do not select the host default until the realistic concurrent workload measures
interactive latency and sustained thermal behavior. Twelve remains the leading
provisional concurrency-safe candidate.
