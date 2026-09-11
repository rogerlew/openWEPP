# Prompt wording guidance

Use package.md as the execution contract and continuation record. A kickoff can
simply say: "Execute the active bounded checkpoint in
docs/work-packages/<id>/package.md." Do not duplicate
governance, acceptance, reading inventories or handoff state in another file.
The task's actual scope controls external actions; do not invent blanket network
restrictions or magic authorization phrases.

## Task instructions
State the objective and authorized boundaries plainly. Name relevant authority
sections and selected checks in package.md. Continue through implementation,
correction, validation and disposition within the active checkpoint bounds in
[bounded-agent-execution.md](bounded-agent-execution.md). Package-wide objectives
do not authorize unlimited correction loops. Preserve acceptance when pausing.
Use [the Pro role](chatgpt-pro-role.md) and
[external kickoff](../work-packages/templates/chatgpt-pro-kickoff.md) for the
owner's manual exchange; external proposals need owner adoption.

Common review selection and delegation authorization live in
../work-packages/AGENTS.md. Bounded work has one independent reviewer; consequential
work two, with reviewer-owned fix verification. Separate runner/verifier agents
are optional unless a specific risk or release requirement needs them.
Assign concrete scope and write limits; no mandatory kickoff phrase or new
confirmation when existing user/instruction authorization is sufficient.

## Scientific authority
Prompts reference, rather than restate, applicable science-contract instructions,
canonical SC authority, pinned-baseline provenance, typed guards, no surrogate
production physics, actual consumer proof and independent conservation evidence.
Read affected mechanisms, definitions, guards and cross-cutting dependencies.
Links alone do not mandate full-catalog reading. Missing authority expands
inspection and blocks unsupported implementation; compact reading never waives
an applicable invariant. The science-contract guide owns reading selection.

For DC work the package states the defect and correction envelope. Continue an
authority-backed in-scope correction within checkpoint bounds; explain a pause
or legitimate blocker in the same
record. Freeze scientific predicates/data roles before result-bearing execution.

## Context measurements
Reading and token measurements are optional unless requested. No mandatory
reading-map artifact, per-role budget report or overrun disposition.
When measuring, distinguish bootstrap, later expansion and repeated exposure;
include all observed delegates. File bytes are not delivered tokens, quota or
workflow-total cost. Unobserved telemetry remains UNOBSERVED.
tools/agents/context_report.py is an optional read-only aid, not an admission
mechanism; read its tool instructions only if using it. Compare like tasks and
label structural estimates explicitly. Do not create measurement work to certify
ordinary package completion.

## Role configuration
Use the explicit routes in bounded-agent-execution.md and .codex/config.toml.
Check live capability before delegation; role files can override spawn settings.
Do not claim an active session changed model because configuration was edited.
No silent fallback or routine model-setting report.
