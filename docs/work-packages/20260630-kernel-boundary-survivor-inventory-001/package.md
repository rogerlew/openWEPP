# Kernel-Boundary Survivor Inventory

Status: EXECUTED-COMPLETE-SURVIVOR-CLASSIFICATION

Package id: `20260630-kernel-boundary-survivor-inventory-001`

## Objective

Execute step 1 of the kernel-boundary typing program from the array-native
runtime specification: inventory and classify the remaining symbol-map survivor
surface after ADR-0031 removed public compatibility runtime selection.

## Authority

- [Array-native runtime specification](../../architecture/array-native-runtime-specification.md)
  §8.2 defines the support-boundary sequence and names this step as "Inventory
  and classify survivors".
- [ADR-0031](../../decisions/0031-delete-compatibility-runtime-single-authority-terminal.md)
  authorizes full deletion of the compatibility runtime and defers carrier-type
  removal to a kernel-boundary typing program.
- [ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md) remains
  the typed array-native runtime authority.

## Scope

In scope:

- Static source inventory for remaining references to
  `HillslopeWritebackSurface`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, `SymbolRegistry`, `HillslopePhaseScheduler`,
  `HillslopeDayFrame`, `HotSymbolTables`, indexed writeback carriers, and
  scheduler execution entrypoints.
- Classification of each file-level survivor as executable runtime,
  kernel-boundary interface, diagnostic/trace support, WB13/publication/audit
  support, test support, or genuine serialization/intake adapter.
- Temporary allowlist and deletion/retyping routes for follow-on work.
- Public selector absence check for `--compatibility-runtime` and removed
  compatibility runtime enum variants.

Out of scope:

- No Rust production code changes.
- No deletion of `scheduler.rs`, `day_frame.rs`, carrier types, or tests.
- No typed kernel-boundary API implementation.
- No output identity, RSS, or runtime performance gate.

## Execution

The package ran static scans over:

- `crates/openwepp-runner/src`
- `crates/openwepp-hillslope-orchestrator/src`
- `crates/openwepp-kernel-contract/src`

Artifacts record:

- [Source scan summary](artifacts/source-scan-summary.md)
- [Survivor classification](artifacts/survivor-classification.md)
- [Temporary allowlist and next work](artifacts/allowlist-and-next-work.md)
- [Gate evidence](artifacts/gates.md)
- [Review](artifacts/review.md)
- [Verification](artifacts/verification.md)

## Findings

The remaining support surface is not a single removable runtime blob. It has
six classes:

1. compiled but unreachable scheduler/day-frame runtime;
2. kernel request/writeback interface used by hydrology phase code;
3. diagnostic/trace/shadow helpers;
4. WB13/publication/audit compatibility support;
5. tests that preserve old scheduler and symbol-boundary behavior;
6. genuine lower-level symbol serialization/intake adapters.

The core survivor scan found `1,284` core carrier/runtime matches across `74`
Rust files. The broader symbol serialization scan found `4,137`
`BoundarySymbol`/`BoundaryValue` matches across `84` files. The public
compatibility selector scan under `crates/` and `tools/` found `0` matches for
removed selector tokens.

## Disposition

Result: `EXECUTED-COMPLETE-SURVIVOR-CLASSIFICATION`.

The next package should not start by deleting `scheduler.rs`. It should begin
with the typed diagnostics/trace and typed kernel-boundary API slices identified
in [Temporary allowlist and next work](artifacts/allowlist-and-next-work.md).
Only after live consumers move to typed boundaries should the compiled
scheduler/day-frame/carrier support be deleted.
