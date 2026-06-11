# FDHP01 Kickoff — frost depth heat-flow parity (single-OFE)

Execution mode: package-end-to-end (Defect-Closure ExecPlan)

Subagent authorization (REQUIRED, not optional): this prompt explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`
(gpt-5.3-codex-spark) for all heavy batch/closure/comparator runs —
`cargo test --workspace`, clippy/deny closure loops, `owcmp` comparator suites,
and the 43-prefix `algebraic-radium` population validation runs — and to
review/verification subagents for the dual review/verification artifacts.
**Do NOT run heavy batch/closure work on the parent model** unless the subagent
is unavailable, in which case record command-level evidence as justification.
`comparator_suite_runner` returns compact metrics + log paths only (no
source/contract edits). See `docs/standards/prompt-wording-guidance.md` §4a.

Autonomy: execute end-to-end — M1 scope/ownership, contract amendment, red/green tests,
pre-impl gate, heat-flow implementation, validation, dual review/verification,
disposition, handoff — without asking for direction on intermediate steps. A declared
phased boundary is permitted; a proxy tweak is not. Ask only if the heat-flow authority is
under-specified for a needed decision (then amend the contract).

## Item 1 — close defect `FDHP01-FROST-DEPTH-HEATFLOW-001` end-to-end

Replace openWEPP's freeze-index frost-depth **proxy** (`frdp_m` ratchet over
`freeze_index = clamp(−mean_temp / FROST_RUNTIME_FREEZE_INDEX_SCALE_C)`, capped
`WB14_FROST_MAX_DEPTH_M = 0.20 m`) with the energy-balance **heat-flow** depth model the
contract already mandates (`SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`/`-012`; legacy `frostn`
lineage; CRM Ch. 3.8 Eq. [3.8.1]–[3.8.4]; Dun et al. 2010), on the frost-active
single-OFE substrate `/wc1/runs/al/algebraic-radium` (`ksflag=1`). Close
`GAP-SNOWFREEZE-002`. Conservation must still close.

This is ROADMAP queue item 1 (re-sequenced ahead of MOFE): settle the vertical frost
mechanism on single-OFE before routing. FDMC01 sized the proxy as **materially off** —
depth capped 200 mm vs legacy 240–503; depth correlation 0.13; frozen duration +258 days
(the proxy ratchets via `max(prior,…)`, thaws only when `tmin>0`, over-persists). Close
both the depth-cap and the duration/ratchet errors.

Primary surfaces (re-verified 2026-06-10, post-REFACTOR015/019–021 module split; locate
by symbol, not stale line numbers):

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  — **the proxy-replacement seam**: `freeze_index`/`thaw_index` proxy + `frdp_m` ratchet,
  hourly `qsrf_w_m2`/`quf_w_m2` flux block, `resolve_frozen_soil_kfactor`,
  `resolve_active_frost_coupling`/`compute_active_frost_coupling`.
- `.../hydrology/03_kernel_support_00_support_helpers.rs` — frost runtime state/symbols
  (`frdp_m`/`tfrdp_m`, `FROST_RUNTIME_FRDP_M_SYMBOL`,
  `FROST_RUNTIME_FREEZE_INDEX_SCALE_C` lives here post-refactor).
- `.../hydrology/kernel_phases_mod/` — frost→conductivity consumption
  (`hydrology_phase_infiltration_evap.rs`, `hydrology_phase_runoff_reconciliation.rs`),
  harmonic-mean conductivity (`hydrology_phase_plant_percolation.rs`).
- `.../constants.rs` — `WB14_FROST_MAX_DEPTH_M` (retire the cap with provenance).
- `crates/openwepp-runner/src/hillslope/` publication helpers — **publish `frdp`** to the
  WAT/output surface per the FDMC01 caveat.

## Milestone 1 first (reproduce + scope + ownership)

1. Reproduce the FDMC01 baseline single-OFE (proxy capped 200 mm, over-persisting) vs
   legacy heat-flow.
2. **Scope the heat-flow port**: full `frostn`/`frzng`/`frznw` layered energy balance
   (Dun-2008 fine sublayers) vs a faithful energy-balance subset that closes the gap;
   localize the proxy-replacement seam. Declare a phased boundary if the full port exceeds
   one package — but the landed phase MUST close the depth+duration gap (cap retired,
   ratchet gone), not tweak the proxy.
3. Ownership: `INV-SNOWFREEZE-006`/`-012` already mandate heat-flow; the proxy is the
   openWEPP divergence (`GAP-SNOWFREEZE-002`). In-envelope openWEPP defect.

## Contract-first sequence (hard order — no kernel code edits before 1–3 complete)

1. Amend `SC-SNOWFREEZE-001` (heat-flow depth implementation spec; close/restate
   `GAP-SNOWFREEZE-002`).
2. Contract-derived red tests.
3. Pre-implementation contract gate evidence.
4. Production correction (heat-flow model replaces the proxy; publish `frdp`).

No silent defaults, no unbounded clamping, no canonicalize-and-proceed on domain
violations — typed fail-closed errors/guards unless bounded normalization is explicitly
contract-authorized.

## Comparator execution

- This prompt's subagent authorization covers `comparator_suite_runner` for
  context-heavy `owcmp` comparisons.
- Discover suites with `tools/owcmp/owcmp manifest list`; prefer a manifest under
  `tools/owcmp/suites/` plus `tools/owcmp/owcmp env --manifest <path>` before running.
- The runner returns only compact metrics and artifact paths (command, exit code,
  verdict, pass count, first divergent key, focus-column metrics, `summary.json`,
  `summary.md`, `command-log.json`, log/report dirs). Do not paste raw per-hillslope
  reports into chat. Commit `summary.json`/`summary.md`/`command-log.json` by default.
- `wepp_260606_hill` is a FLAG (ADR-0017), not a match target.

## Acceptance authority + constraints

- Conversion rule: in-envelope (proxy) + `INV-SNOWFREEZE-006`/`-012` + CRM/Dun authority
  ⇒ MUST land. May not HOLD because the port is large — scope it, land the gap-closing
  phase.
- Acceptance = contract-correct heat-flow behavior; comparator confirms the envelope.
- **Conservation must still close** (rung-1 identity incl. `frozwt`, + totalwatsed3 audit).
- **Do not** regress frost activation (FQ-4 — the gate stays), change the kfactor
  conductivity magnitude (legacy-faithful), touch forest `ksatadj`, ET/runoff/p11, snow
  magnitude, or MOFE/17-OFE. **Single-OFE only.**
- Line-count governance: 2000+ line `.rs` is WARN; 3000+ non-exempt requires refactor
  before closure.

## Required reading

Maintain `artifacts/required-reading-map.md` as a living artifact (tiers, rationale,
read timing).

Reading budget (local-repo pre-edit reads, Core + Conditional): ~111,000 bytes →
**OK** (`<=400000`, thresholds per
`docs/standards/kernel-work-package-preparation.md`). On-demand SC contracts are
large (`SC-SNOWFREEZE-001` ~150 KB, `SC-WATBAL-001` ~326 KB) — load phase-locally,
section-targeted, not as pre-reads.

Core (always, before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/package.md`

Conditional (triggered — all apply to this package):
- `docs/defect_closure_execplans.md` (DC-ExecPlan)
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  `docs/specifications/science-contracts/index.md` (contract + kernel authority edits)
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` (legacy parity scope)

On-demand (phase-local, touched mechanisms only):
- `SC-SNOWFREEZE-001.md` (`INV-SNOWFREEZE-006`/`-012`/`-013`, `GAP-SNOWFREEZE-002`),
  `SC-WATBAL-001.md` (closure surfaces)
- ADR-0011/0017/0018; `docs/ROADMAP.md`;
  `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`
- FDMC01 package + artifacts (the sized gap + metrics to close); FQ-4 package
  (activation — must stay non-regressed)
- Legacy `/workdir/wepp-forest_260430_baseline/src/frostn.for` (+ `frzng`/`frznw`/
  `frsoil`); CRM Ch. 3.8; Dun et al. 2010
- `docs/prompt_templates/owcmp-comparator-runner-guidance.md`; comparator
  `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; substrate
  `/wc1/runs/al/algebraic-radium/wepp/runs/` (single-OFE, `ksflag=1`)
