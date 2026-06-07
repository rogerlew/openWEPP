# FDHP01 Kickoff — frost depth heat-flow parity (single-OFE)

Execution mode: package-end-to-end (Defect-Closure ExecPlan)

Autonomy: execute end-to-end — M1 scope/ownership, contract amendment, red/green tests,
pre-impl gate, heat-flow implementation, validation, dual review/verification,
disposition, handoff — without asking for direction on intermediate steps. A declared
phased boundary is permitted; a proxy tweak is not. Ask only if the heat-flow authority is
under-specified for a needed decision (then amend the contract).

## Item 1 — close defect `FDHP01-FROST-DEPTH-HEATFLOW-001`

Replace openWEPP's freeze-index frost-depth **proxy** (`frdp = 0.20·clamp(−mean_temp/6)`,
capped 0.20 m, `03_kernel_support_00:3290-3335`) with the energy-balance **heat-flow**
depth model the contract already mandates (`SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`/`-012`;
legacy `frostn` lineage; CRM Ch. 3.8 Eq. [3.8.1]–[3.8.4]; Dun et al. 2010), on the
frost-active single-OFE substrate `/wc1/runs/al/algebraic-radium` (`ksflag=1`). Close
`GAP-SNOWFREEZE-002`. Conservation must still close.

This is the next ROADMAP item (re-sequenced ahead of MOFE): settle the vertical frost
mechanism on single-OFE before routing. FDMC01 sized the proxy as **materially off** —
depth capped 200 mm vs legacy 240–503; depth correlation 0.13; frozen duration +258 days
(the proxy ratchets via `max(prior,…)`, thaws only when `tmin>0`, over-persists). Close
both the depth-cap and the duration/ratchet errors.

Primary surfaces: `03_kernel_support_00_support_helpers.rs` (proxy `:3290-3335`,
`qsrf`/`quf`/resistance `:3455-3490`, kfactor); `03_kernel_support_01_kernel_phases.rs`
(frost→conductivity `:345/3925/4672`, harmonic `:2112`); `constants.rs`
(`WB14_FROST_MAX_DEPTH_M`, `FROST_RUNTIME_FREEZE_INDEX_SCALE_C` — retire); runner
`mod.rs` (publish `frdp` per the FDMC01 caveat).

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

## Acceptance authority + constraints

- Conversion rule: in-envelope (proxy) + `INV-SNOWFREEZE-006`/`-012` + CRM/Dun authority
  ⇒ MUST land. May not HOLD because the port is large — scope it, land the gap-closing
  phase.
- `wepp_260606_hill` is a FLAG that depth/duration fall in the heat-flow envelope
  (ADR-0017), NOT a match target. Acceptance = contract-correct heat-flow behavior.
- **Conservation must still close** (rung-1 identity incl. `frozwt`, + totalwatsed3 audit).
- **Do not** regress frost activation (FQ-4 — the gate stays), change the kfactor
  conductivity magnitude (legacy-faithful), touch forest `ksatadj`, ET/runoff/p11, snow
  magnitude, or MOFE/17-OFE. **Single-OFE only.**

## Required reading

- `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/package.md`
- FDMC01 package + artifacts (the sized gap + metrics to close)
- FQ-4 package (activation — must stay non-regressed)
- `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`, `docs/ROADMAP.md`
- ADR-0011/0017/0018, `docs/defect_closure_execplans.md`, `AGENTS.md`
- `SC-SNOWFREEZE-001.md` (`INV-SNOWFREEZE-006`/`-012`/`-013`, `GAP-SNOWFREEZE-002`),
  `SC-WATBAL-001.md`
- Legacy `/workdir/wepp-forest_260430_baseline/src/frostn.for` (+ `frzng`/`frznw`/`frsoil`);
  CRM Ch. 3.8; Dun et al. 2010
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; substrate
  `/wc1/runs/al/algebraic-radium/wepp/runs/` (single-OFE, `ksflag=1`)
