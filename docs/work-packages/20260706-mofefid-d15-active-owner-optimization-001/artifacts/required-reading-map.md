# Required Reading Map

Status: **EXECUTED (D15A-S0)**.

Evidence mode: Static (documents read in this session at the revisions below).

Base state: `main` at `9f536aadffc4488d676b1d28663ef67ce4db7e94` (scaffold
commit, child of the required base
`94a7ac3aff003a89328701e4a6daf3abd98c8fe3`), working tree clean. `origin/main`
at `94a7ac3a`; the local HEAD is the required base plus this package's own
scaffold. The D15 blocker-resolution
package (`20260706-mofefid-d15-blocker-resolution-001`) is merged in this
history.

| Document | Revision read | Load-bearing content for this package |
|---|---|---|
| `AGENTS.md` | `9f536aad` | Validation gates (fmt/clippy/nextest full/deny), truthfulness (`Ran:`/`Static:` labels), consumer-path closure rule pointer, no-branch rule, subagent authorization wording. |
| `docs/work-packages/AGENTS.md` | `9f536aad` | Gate evidence non-deferral; Consumer-Path Closure Rule (producer source, in-memory state/frame, runner handoff, downstream call site, output/API surface, negative old-path proof); Conservation/Publication Acceptance Rule (operand-lineage table before production edits); subagent delegation wording; dual review + disposition + verification + line-count governance. |
| `docs/specifications/science-contracts/AGENTS.md` | `9f536aad` | Contract-first sequencing: amend canonical `SC-*` before contract-derived tests and production code. |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | rev 26 (`9f536aad`) | Full read. Activation preconditions consolidated in `INV-OFEROUTE-012` (rev-26 update): remaining gates are the activation wiring itself (runtime closure hard-fail, DC01-disable, routed producer into the D13 consumer), rev-21 friction-operand consumer proof on the real active path, and the post-D10B endpoint-timing refresh. Seam bindings: source = `wb14_hourly_excess + ui_SCrunf` per hour with the recorded `/3600` helper; `ui_LfCrf` stays subsurface; outlet `latqcc` bypasses the router and enters closure ONLY as `latqcc_mm / 1000 × A_outlet_m2`; hourly lane required (daily lanes fail closed). `INV-OFEROUTE-009`: DC01 daily-lump admission and routed hourly runon must never both feed the same active lane (typed hard fail). `INV-OFEROUTE-010`: default/off byte identity. `INV-OFEROUTE-011`: CLOSED (no open D-val blocker). D12 limb closure `1.0e-12 m`; D13 routed-shape unit-sum/fail-closed consumer rules. |
| `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1, §7 | `9f536aad` | D9-D15 sequence state; D15A is the queued hold-lift for the two remaining blockers (timing regression `91.59 s`, absent active owner); D16 stays blocked. Open decision 2: exact opt-in selector + output-publication scope + active consumer proof are package-local to D15A. |
| `docs/ROADMAP.md` §M (queue row M) | `9f536aad` | This package is the next actionable Lane D item; D16/default promotion blocked until opt-in activation evidence exists. |
| D10B package + artifacts | `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001` | GAP-005 RESOLVED (scheme source-corrected; true-celerity CFL; conservative bin handoff; 19-OFE cascade conservation identically zero). D14 endpoint refresh required after its celerity/handoff changes — the refresh (blocker package) measured the `3.06x` regression this package must optimize/adjudicate. |
| D11 packages (`20260705-…-d11-friction-operand-authority-001`, `20260706-…-d11-gap007-dynamic-friction-closure-001`) | artifacts read via delegated audit (recorded in `active-owner-architecture.md`) | Rev-20/21 operand sources for the Lane D shadow (static `routing_coefficients`; live `I_h = wb14_hourly_rainfall_m/3600`; post-growth `LAI`; typed-management `canhgt`). Active path must consume the SAME rev-21 operand path with the same fail-closed guards, with real-consumer proof. |
| D12 package (`20260705-mofefid-d12-melt-limb-hourly-shape-001`) | as above | Source-authorized hourly routed-melt limb; H2637 uniform-fallback days reduced to 6 no-authorized-source residuals; uniform fallback stays diagnostic-only and cannot carry activation evidence. |
| D13 package (`20260705-mofefid-d13-routed-hydrograph-erosion-shape-001`) | as above | `DirectErosionHydrographShapeAuthority::RoutedHydrograph` consumer landed with unit-sum/fail-closed validation; D15 owns the PRODUCER FLIP: routing must supply `routed_hydrograph_runoff_fraction` on active lanes. |
| D14 package (`20260705-mofefid-d14-laned-runtime-profile-optimization-001`) | artifacts read directly (`baseline-timing.md`, `slot-timing-evidence.md`, `optimization-disposition.md`) | Optimization structure this package mirrors; persistent slot diagnostics (`ofe_routing::profile`, `OPENWEPP_LANED_SHADOW_PROFILE=1`); D14 optimized budget ~`29.9 s` wall at `10,334,879` steps; bit-identity preservation witness pattern (identical trajectory counters + SHA256 of protected outputs). |
| D15 preflight + rerun + blocker-resolution artifacts | `20260705-mofefid-d15-opt-in-production-activation-001`, `20260706-…-rerun-001`, `20260706-…-blocker-resolution-001` | Blocker package `hold-legitimacy-audit.md` + `timing-refresh.md` read directly: HOLD-A (no active production owner: shadow-only collector, DC01 still feeding production, erosion still `Dc01SourceShape`, no live closure hard-fail) and HOLD-B (`91.59 s` user / `1:31.67` wall; `16,936,089` steps vs D14's `10,334,879`; `solver_cfl_ns` 64.7 s vs 17.6 s). Exact timing commands and fixture-preparation recipe (19 `routing_coefficients` insertions, the `laned_shadow_h2637` native patch) reused for this package's baseline. |

Subagent-requirement note: this session exposes no `comparator_suite_runner` or
`timing_comparator` agent type (available types: general-purpose, Explore,
Plan, claude, statusline-setup). Per the package's subagent clause, that
session-level block is recorded here and heavy H2637 endpoint/profile timing
runs locally with the exact recorded commands (package governance permits local
execution when dispatch is unavailable). Read-only authority/evidence audits
are delegated to `Explore`-class subagents under the package's explicit
subagent authorization.
