# rancor simulation architecture pattern assessment

Evidence: Static
Ran evidence: none

## Transferability assessment

| pattern | evidence | transferability to openWEPP |
| --- | --- | --- |
| singleton simulation runtime | `RancorSim.Instance` singleton and global mutable runtime (`/workdir/rancor/Rancor/Rancor/RancorSim.cs:100`, `:101`) | reject |
| timer-driven server/client loops | `DispatcherTimer`-based tick loop with mutable counter-driven updates (`/workdir/rancor/Rancor/Rancor/RancorSim.cs:178`, `:261`, `:446`) | partial (fixed-step concept only) |
| monolithic unit model with mixed concerns | `UnitModel.Update()` orchestrates physical updates + control + alarms (`/workdir/rancor/Rancor/Rancor/UnitModel.cs:309`, `:321`, `:327`) | reject |
| reflection-based remote mutation | property/method mutation by string and reflection (`/workdir/rancor/Rancor/Rancor/UnitModel.cs:67`, `:84`, `/workdir/rancor/Rancor/WebAPI/Controllers.cs:52`, `:67`) | reject |
| broad runtime mutability surface | large interface with many writable properties/actions (`/workdir/rancor/Rancor/Rancor/IUnitModel.cs:45`, `:90`) | reject |
| messagepack DTO state sync | DTO serialization on sim and unit models (`/workdir/rancor/Rancor/Rancor/RancorSimBase.cs:150`, `/workdir/rancor/Rancor/Rancor/UnitModelBase.cs:147`) | adopt (bounded) |
| fault injection manager | centralized fault overrides and trigger-driven malfunctions (`/workdir/rancor/Rancor/Models/FaultManager.cs:26`, `/workdir/rancor/Rancor/Models/Fault.cs:266`) | adopt with typed constraints |
| API controller write-through to model | Web API directly mutates runtime model on dispatcher thread (`/workdir/rancor/Rancor/WebAPI/Controllers.cs:55`, `:70`, `:92`) | reject |

## Findings
- Static: [DIRECT] Rancor intentionally supports high runtime mutability through string-based property/method dispatch and HTTP endpoints (`/workdir/rancor/Rancor/Rancor/UnitModel.cs:67`, `:84`, `/workdir/rancor/Rancor/WebAPI/Controllers.cs:52`, `:67`).
- Static: [DIRECT] The simulator loop interleaves control logic, faults, logging, and model updates inside one timer callback (`/workdir/rancor/Rancor/Rancor/RancorSim.cs:261`, `:294`, `:295`, `:306`).
- Static: [DIRECT] Client/server state replication relies on messagepack DTOs and periodic polling (`/workdir/rancor/Rancor/Rancor/RancorSim.cs:448`, `/workdir/rancor/Rancor/Rancor/UnitModelProxy.cs:21`).
- Static: [INFERENCE] Rancor's architecture optimizes interactive operator-training flexibility; this conflicts with openWEPP needs for deterministic, contract-validated process simulation.
- Static: [INFERENCE] Transferable elements are limited to structured snapshotting and optional fault-injection scaffolding, provided mutation is strongly typed and compile-time constrained.

## openWEPP disposition
- Static: [INFERENCE] Keep: DTO snapshot/export patterns, explicit event logs, optional fault scenario injection under typed contracts.
- Static: [INFERENCE] Reject: singleton-global mutable model, reflective mutation surfaces, timer/GUI-coupled orchestrator loops, direct web API mutation of kernel state.
