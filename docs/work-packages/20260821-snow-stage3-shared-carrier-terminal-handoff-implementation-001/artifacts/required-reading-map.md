# Required Reading Map

Status: `INTAKE COMPLETE / implementation write set frozen`

The package kickoff Core set was measured on 2026-08-20 from the current
working tree with:

```text
wc -c <Core paths>
```

Total: `678907` bytes. Disposition: `WARN` under
`docs/standards/kernel-work-package-preparation.md` (`OK <=400000`, `WARN
>400000 and <=800000`, `REQUIRES-JUSTIFICATION >800000`). No Core file is over
800000 bytes, so no heavy-file justification is required.

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11927 | Repository instructions and protected boundaries. |
| `docs/work-packages/AGENTS.md` | 26367 | Work-package lifecycle, gates, evidence, and review rules. |
| `docs/specifications/science-contracts/AGENTS.md` | 5599 | Contract-authoring and binding rules. |
| `docs/work-packages/20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001/package.md` | 15411 | Scope, write-set, chronology, and exit criteria. |
| `docs/standards/testing-and-gate-strategy.md` | 22200 | Current validation and campaign gate policy. |
| `docs/standards/kernel-work-package-preparation.md` | 15309 | Required-reading, prompt, and implementation preparation authority. |
| `docs/standards/prompt-wording-guidance.md` | 10508 | Prompt wording and autonomy requirements. |
| `docs/specifications/science-contracts/index.md` | 13673 | Contract registry and release status. |
| `SC-COUPLEDTIME-001.md` | 53438 | Coupled-time support and custody authority. |
| `SC-LANDSURFACEENERGY-001.md` | 73840 | LSE support and energy authority. |
| `SC-SNOWENERGY-001.md` | 135595 | Snow energy, liquid, and event authority. |
| `SC-VEGETATION-001.md` | 234588 | V11 canopy state and surface authority. |
| `SC-VEGETATIONTRANSACTION-001.md` | 40810 | Vegetation transaction and rollback authority. |
| Child 2C `final-disposition.md` | 1384 | Released authority and verification disposition. |
| Child 2C `worker-handoff.md` | 1618 | Implementation boundary and handoff. |
| Historical Child 1 `package.md` | 16640 | Consumed HOLD evidence and negative boundary. |

The abbreviated contract labels in the table refer to
`docs/specifications/science-contracts/contracts/`.

## Conditional intake consumed

The actual consumer trace and owner/restart map read these paths before the
implementation gate. Byte counts are from the current worktree:

| Path | Bytes | Intake use |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 86938 | Ordinary `DirectFrameExecutor` publication loop and commit order. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` | 125285 | `DirectRunFrame`/`DirectDayFrame` owner seed and day commit. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | 67072 | Existing R4G CoE snow coupling boundary; preserved. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` | 52558 | Snow lane state and legacy operand ownership. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs` | 132157 | Stage 3 evaluation/persistent state; not reused as production carrier authority. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` | 77282 | Confirms the existing Stage 3 path is evaluation/shadow only. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs` | 25521 | Historical terminal numerics; not the Child 2C receipt path. |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | 122457 | Negative proof: V11/LSE stack is isolated shadow state with no production commit. |
| `crates/openwepp-hillslope-orchestrator/src/v11_vegetation_consumer.rs` | 1560 | Existing adapter boundary and default-off selector posture. |
| `crates/openwepp-coupled-time/src/{error.rs,event.rs,clock.rs,support.rs,transaction.rs,restart.rs}` | 94006 | Chronology, owner candidate, receipt, restart, and error extension seam. |
| `crates/openwepp-vegetation/src/v11.rs` | 99966 | Complete V11 owner manifest and staged transaction custody. |
| `crates/openwepp-land-surface-energy/src/{support.rs,transaction.rs,solver.rs}` | 209437 | LSE minimum support and potential/final owner semantics. |
| `crates/openwepp-persisted-restart-v1/src/{vegetation_v11_v3.rs,checkpoint.rs,canonical.rs}` | 83548 | V3 canonical restart and complete-owner admission seam. |

The conditional intake total is 1,188,787 bytes and is not a Core budget.
The nearest-instruction lookup was run with `tools/agents/find-agents --for`
for each proposed source/test/artifact path. Applicable chains were root
`AGENTS.md` → `crates/AGENTS.md` for Rust, root → `tests/AGENTS.md` for
integration tests, and root → `docs/work-packages/AGENTS.md` for artifacts.
No nearer nested file was present.

## Conditional and on-demand

Actual scheduler, owner, restart, LSE, snow, liquid, hydrology, soil-thermal,
BGC, publication, and nested instruction files are conditional on the intake
path proof. Legacy baselines and comparator/observed-data materials are
on-demand only when a declared evidence obligation requires them. The map must
be amended with exact paths and byte counts before those files are used as
implementation authority.
