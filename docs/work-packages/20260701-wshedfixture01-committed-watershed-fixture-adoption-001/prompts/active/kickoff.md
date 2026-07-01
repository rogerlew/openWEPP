# WSHED-FIXTURE01 Handoff Prompt

Scope: local repository fixture-adoption task for openWEPP; flat-file
reads/edits only except local validation commands; no external connectivity.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md`
sequentially through disposition. Do not stop at analysis unless hard-blocked.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/ROADMAP.md` watershed runtime queue
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
  sections 5.5 and W1A
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/package.md`
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/artifacts/required-reading-map.md`

Conditional:

- `/home/workdir/wepppy/docs/work-packages/20260701_wshed_fixture01/package.md`
  and `/home/workdir/wepppy/tests/topo/test_wshed_fixture01.py` if readable in
  the current environment. Use them as orientation only; they do not close the
  openWEPP committed-fixture gate.
- `/wc1/runs/ca/carnivorous-adobo/wepp` if readable. Treat it as source
  substrate only; do not make `/wc1` the persistent fixture path.

On-demand:

- Existing openWEPP fixture/test patterns discovered with `rg --files tests`.
- Adjacent parser/runfile tests only if touched.

Required-reading budget: `OK`; map:
`docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/artifacts/required-reading-map.md`.

Files:

- `tests/fixtures/watershed/**`
- focused fixture-contract test files in the existing matching openWEPP test
  location
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Task: execute WSHED-FIXTURE01 end-to-end. Adopt a committed openWEPP fixture
derived from `/wc1/runs/ca/carnivorous-adobo/wepp`, exercising a 32-hillslope
watershed, and prove the recurring gate reads the committed openWEPP path.

Constraints:

- ADR-0032 canonical benchmark/ratification mode is `strict-committed-fixture`.
- Do not close on a `/wc1`-only, `/tmp`-only, scratch-only, or wepppy-only
  fixture.
- Do not silently substitute a different fixture. If the wepppy fixture or
  available substrate is not the 32-hillslope carnivorous-adobo-derived
  watershed required by openWEPP, record the mismatch and close
  `EXECUTED-HOLD` unless explicit architecture/user direction changes scope.
- Keep fixture files minimal and auditable. Do not commit generated bulky caches
  or transient outputs.
- Record source substrate path/date, hillslope count, topology summary,
  required input/runfile inventory, adopting package, and intended scope.
- Add or update a focused test/contract check that fails if it reads `/wc1` or
  wepppy instead of `tests/fixtures/watershed/...`.

Subagent requirement: REQUIRED for package review/verification. This prompt
explicitly authorizes subagent spawning/delegation to `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` for read-only fixture discovery,
review, and verification; outputs: compact findings with file/path references
and any command evidence; write access: read-only. If session-level tool policy
still blocks spawning, ask for one-time authorization before spawning, or record
the block and run equivalent local review only if package governance allows
local substitution.

Validation:

- Run the focused fixture-contract test you add or update.
- Run scoped docs lint for this package and touched docs.
- Run `git diff --check`.
- Do not run full Rust workspace gates unless production Rust is touched.
- If full-suite validation is skipped, say so plainly in
  `artifacts/disposition.md`.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, review/verification disposition, roadmap,
work-package README, and final disposition for all completed phases.
