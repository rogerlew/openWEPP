# openWEPP Observability Subsystem Assessment

Date: 2026-05-20
Evidence class: `Static`

## Decision

Do not carry forward `wepp_observe.on`, `wepp_observe_frost.on`, or
`wepp_observe_wb05e_target.dat` as parser sidecar compatibility surfaces.

These files were ad-hoc debug toggles in `wepp-forest` and are replaced by a
first-class observability subsystem in openWEPP.

## Rationale

1. File-presence debug toggles are brittle and not composable across modules.
2. They couple debug behavior to cwd side effects rather than typed run/debug
   intent.
3. They do not provide a stable contract for kernel-level stimulation or
   repeatable developer workflows.

## Required Subsystem Capabilities

1. Kernel stimulation without end-to-end runs.
2. Typed debug scenarios with explicit scope (`kernel`, `phase`, `surface`).
3. Deterministic replay hooks for fixed windows and fixtures.
4. Structured traces/events (not ad-hoc text probes).
5. Guard/invariant violation capture with contextual state slices.
6. Low-overhead off-path mode for production parity runs.

## Minimum Architecture Slice (OBS01)

1. Debug intent contract
   - typed configuration object passed through orchestrator (no cwd sentinel files)
2. Kernel harness interface
   - invoke single kernels or short pipelines over fixture state
3. Structured telemetry sink
   - event schema for per-step/per-kernel traces
4. Replay bridge
   - deterministic window replay from captured state snapshots

## Parser/Sidecar Governance Impact

1. `wepp_observe*` files are marked `unsupported` in parser sidecar registry.
2. Parser contracts should not include `SC-INFILE-OBSERVE-FLAGS-*` surfaces.
3. Observability is governed by subsystem architecture/work-package policy,
   not by parser-sidecar compatibility contracts.
