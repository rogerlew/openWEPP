# Required Reading Map

Status: `INTAKE-SCAFFOLD / required reading not yet consumed by an implementation run`

The package kickoff Core set was measured on 2026-08-20 from the current
working tree with:

```text
wc -c <Core paths>
```

Total: `677448` bytes. Disposition: `WARN` under
`docs/standards/kernel-work-package-preparation.md` (`OK <=400000`, `WARN
>400000 and <=800000`, `REQUIRES-JUSTIFICATION >800000`). No Core file is over
800000 bytes, so no heavy-file justification is required.

## Core

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 11927 | Repository instructions and protected boundaries. |
| `docs/work-packages/AGENTS.md` | 26367 | Work-package lifecycle, gates, evidence, and review rules. |
| `docs/specifications/science-contracts/AGENTS.md` | 5599 | Contract-authoring and binding rules. |
| `docs/work-packages/20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001/package.md` | 13952 | Scope, write-set, chronology, and exit criteria. |
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

## Conditional and on-demand

Actual scheduler, owner, restart, LSE, snow, liquid, hydrology, soil-thermal,
BGC, publication, and nested instruction files are conditional on the intake
path proof. Legacy baselines and comparator/observed-data materials are
on-demand only when a declared evidence obligation requires them. The map must
be amended with exact paths and byte counts before those files are used as
implementation authority.
