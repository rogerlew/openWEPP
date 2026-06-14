# WSHED01 Kickoff — increment W-A (characterization + watershed routing scope)

Execution mode: staged increments per `artifacts/watershed-staged-increment-plan.md`.
This kickoff governs increment **W-A** (characterization; **no production
code**). Later increments (W-B impoundment, W-C routing/output, W-D
totalwatsed3 closure) are dispatched by the plan's dispatch line.

Subagent authorization (REQUIRED, not optional): this prompt explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`
(gpt-5.3-codex-spark) for any heavy batch/comparator runs and to
review/verification subagents. W-A is characterization; heavy runs (the
watershed CLI cohort) may apply — do NOT run them on the parent model unless
the subagent is unavailable, in which case record command-level evidence
(`docs/standards/prompt-wording-guidance.md` §4a).

Autonomy: execute W-A end-to-end — characterize and produce the scope
artifact — without asking for direction on intermediate steps. Hard stop: NO
production code edits in W-A; NO wepppy production edits (cross-repo boundary).

## The assignment (W-A)

Per `artifacts/watershed-staged-increment-plan.md` increment W-A, produce the
characterization evidence and `artifacts/watershed-routing-scope.md`, all
`Static:`/`Ran:` and file:line-cited against the **current tree** (read the
lines — the FDHP01-Dh lesson):

1. Run `openwepp-cli-watershed` (`crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`)
   on `/wc1/runs/ar/arboreal-dendrite/wepp` with the closed MOFE01 hillslope
   HBP shards; record the full success/failure chain →
   `characterization-watershed-cli-current.md`.
2. Classify the `jpond=0` blocker: read
   `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
   (`IMP-E-004`) and the CLI wrap (`openwepp-cli-watershed.rs:251`,
   `CLIWAT-E-010`). Is rejecting `jpond=0` (no impoundments) a **parser
   defect** or valid? Confirm against legacy `.imp` handling →
   `impoundment-no-pond-finding.md`.
3. Map the channel-routing (`openwepp-watershed-orchestrator`), watershed
   output schema (`openwepp-watershed-output` + interchange contract), and the
   **totalwatsed3** input expectations
   (`wepppy/wepp/interchange/totalwatsed3.py`,
   `tools/totalwatsed3_daily_closure_audit.py`).
4. Author `watershed-routing-scope.md`: legacy routing authority map, openWEPP
   seam mapping, the totalwatsed3 input contract, the watershed conservation
   identity (independent operands — no 0==0 closure, the M-E4-REDO/M-I
   lesson), red-test definitions, and the W-B/W-C/W-D increment breakdown +
   sizing.

## Gate (Gate Evidence Non-Deferral Rule)

W-A's completion criterion is the characterization evidence + scope artifact,
with current-tree citations and the `jpond=0` finding classified with
evidence. No production edits; W-A legitimately closes `complete` on that
evidence. Do NOT begin W-B under the W-A dispatch.

## Required reading

Maintain `artifacts/required-reading-map.md` as a living artifact.

Core (always, before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`
- `artifacts/watershed-staged-increment-plan.md`

Conditional (triggered):
- `docs/defect_closure_execplans.md`,
  `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  `docs/specifications/science-contracts/index.md`,
  `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

On-demand (phase-local):
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`,
  `crates/openwepp-watershed-orchestrator/`, `crates/openwepp-watershed-output/`,
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `SC-WATBAL-001` + watershed/interchange contracts (section-targeted)
- ADR-0011/0017/0018; `docs/ROADMAP.md` (queue item 1)
- MOFE01 package + `mofe-staged-increment-plan.md` (the HBP shard producer +
  the tautology/clone/hollow-closure failure modes)
- wepppy `wepp/interchange/totalwatsed3.py`,
  `tools/totalwatsed3_daily_closure_audit.py`; WBVAL06/6a notes
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/` (HBP shards, chan.inp, pw0,
  pw0.imp); legacy `/workdir/wepp-forest_260430_baseline/src/` channel/
  impoundment routines (on-demand for the legacy authority map)
