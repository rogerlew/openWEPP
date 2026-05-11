# ADR-0004: Hillslope orchestration is subprocess-per-hillslope

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
Three orchestration models were considered:

- **In-process library linkage** — the watershed binary calls hillslope kernels directly. Fastest per-hillslope; shared address space requires careful state isolation; complicates wepppy integration (wepppy currently shells out to legacy WEPP).
- **PyO3 / FFI from wepppy** — wepppy calls openWEPP kernels via Python bindings. Tightly couples release cadence; complicates Python packaging.
- **Subprocess-per-hillslope** — watershed binary spawns hillslope binary as a subprocess per hillslope; hillslope writes an HBP shard to disk; watershed reads HBP shards. Matches the legacy WEPP pattern and matches wepppy's current call shape.

## Decision
**Subprocess-per-hillslope.** The watershed CLI spawns the hillslope CLI as a subprocess per hillslope. Inter-binary state crosses the filesystem as HBP shards.

wepppy invokes the openWEPP CLIs as subprocesses, matching its existing pattern. No PyO3 bindings.

## Consequences
- Per-hillslope process startup cost is paid per run. Acceptable for the workload (hillslope runtime dominates startup).
- HBP write/read on every hillslope. Disk I/O is a real cost; HBP format choices matter.
- Crash isolation: a single failing hillslope cannot corrupt the watershed orchestrator's state.
- wepppy can replace the legacy WEPP binary with openWEPP's hillslope CLI without changing its orchestration model.
- Argument construction in subprocess calls uses `std::process::Command` with explicit arg arrays. No shell interpolation.
