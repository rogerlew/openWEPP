# Local Rust Feasibility Intake

Status: `Ran / provisional ow-dev-01 evidence / no cross-machine claim`

Date: `2026-08-14`

Commit under test: `be5e5f8d1` plus the documented environment-helper changes.
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

## Concurrent Workload Intake

The realistic local mix ran one fresh-target workspace check, two independent
focused checks, and a 50-pass repository scan at the same time. An initial
harness run was rejected because its background-shell scoping made the timing
logs unreliable. The corrected runs were:

| Heavy jobs | Heavy workspace | Kernel (4 jobs) | Vegetation (4 jobs) | Scan x50 |
| ---: | ---: | ---: | ---: | ---: |
| 12 | 20.16 s | 8.88 s | 12.89 s | 0.42 s |
| 8 | 20.74 s | 8.57 s | 12.65 s | 0.41 s |

For comparison, the isolated baselines were 17.91 seconds for the 12-job
workspace check, 4.94 seconds for kernel, and 8.60 seconds for vegetation.
Contention is visible, but the host remains responsive and completes every arm.
Reducing the heavy job from 12 to 8 costs only 0.58 seconds while improving both
focused checks. The repository scan is effectively unchanged.

## Local Concurrency Decision

- Admit only one heavy command per host with `tools/dev/heavy`; a competing
  command exits 75 rather than silently queueing and consuming an agent slot.
- Default wrapped Cargo builds and nextest campaigns to 8 workers/threads while
  focused agents may be active.
- Permit an explicit 16-worker override only for an otherwise idle, exclusive
  heavy run. The single-job probe supports it, but sustained thermal behavior
  remains part of the final cross-machine campaign.
- Keep focused checks isolated at 4 Cargo jobs in the feasibility harness.
- Treat these as safe ow-dev-01 intake defaults, not as evidence that this host
  is faster than `forest`.
