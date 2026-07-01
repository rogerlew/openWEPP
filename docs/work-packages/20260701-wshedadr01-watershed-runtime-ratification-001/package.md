# WSHEDADR01 Watershed Runtime Ratification

Status: `EXECUTED-COMPLETE-ADR0032-WATERSHED-RUNTIME-RATIFIED`

Date opened: `2026-07-01`

Package type: governance/documentation ratification package.

## Objective

Execute the WSHED-ADR roadmap rung by ratifying the watershed runtime public
entrypoint, `--jobs` default, and canonical sidecar/input-discovery benchmark
mode. The ratification is recorded as ADR-0032 and synchronized into the
watershed runtime architecture specification, decision index, roadmap, and
work-package execution log.

## Rationale

WSHEDARCH01 Revision 4 intentionally left three decisions open before W2/W3
implementation packages could start cleanly:

- whether the new supervisor lives under `openwepp-cli-watershed` or a separate
  production binary;
- what `--jobs` does when omitted;
- which sidecar/input-discovery surface is canonical for future benchmark and
  ratification evidence.

## Included Scope

- Author ADR-0032.
- Update `docs/decisions/README.md`.
- Update
  `docs/architecture/watershed-runtime-architecture-specification.md`.
- Remove WSHED-ADR from the forward roadmap queue and leave the next watershed
  runtime rung queued.
- Record package-local review, gates, and disposition.

## Excluded Scope

- No production Rust edits.
- No CLI implementation.
- No benchmark reruns.
- No fixture capture.
- No `NoEvent` science-contract decision.
- No final runtime-authority claim beyond the governance decisions ratified by
  ADR-0032.

## Intended Write Set

- `docs/decisions/0032-watershed-runtime-ratification.md`.
- `docs/decisions/README.md`.
- `docs/architecture/watershed-runtime-architecture-specification.md`.
- `docs/ROADMAP.md`.
- `docs/work-packages/README.md`.
- `docs/work-packages/20260701-wshedadr01-watershed-runtime-ratification-001/**`.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer` and `rust_qa_reviewer` subagents for
read-only review of ADR-0032 and the synchronized documentation changes;
expected outputs are compact findings with severity and file/path references;
write access is read-only for subagents, with parent disposition recorded in
`artifacts/review-disposition.md`.

## Deliverables

- Accepted ADR-0032.
- Updated decision index.
- Updated watershed runtime architecture specification.
- Updated roadmap queue.
- Package review, gate, and disposition artifacts.

## Exit Criteria

- ADR-0032 selects the public watershed runtime entrypoint.
- ADR-0032 selects the public `--jobs` default.
- ADR-0032 selects the canonical sidecar/input-discovery benchmark mode.
- Architecture spec no longer lists those decisions as open questions.
- ROADMAP no longer carries WSHED-ADR as a forward queue item.
- Dual review findings are dispositioned.
- Docs-only validation is recorded.

## Security and Safety

Docs-only work. No network, branch changes, production services, credentials,
runtime behavior, or serialized output formats are in scope.
