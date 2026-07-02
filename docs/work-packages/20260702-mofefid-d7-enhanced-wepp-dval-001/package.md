# MOFEFID-D7 — Lane D D-val: Reproduce Papanicolaou Enhanced-WEPP

Status: **EXECUTED — REVIEW-READY** (2026-07-02)

**Execution outcome (per-case verdicts; see `artifacts/execution-report.md`):**
Case 1 bare **PARTIAL** (steady magnitude reproduces `NS_trace` 0.868, but the
rising-limb shape gate fails ~40%); Case 2 isolated **operand-limited** (NS
0.45); Case 3 vegetation **does-not-reproduce** + S0 magnitude caveat (NS 0.54);
Case 4 shock **operand-limited** (corrected `I=0`: timing/rise reproduce at
k_o~200, peak −20%, NS ~0.30 — the earlier solver-lag `GAP-OFEROUTE-004` was
WITHDRAWN as a forcing-bug artifact). **Zero cases cleanly reproduce.** Deliverables: committed
copyright-safe harness (`ofe_routing::dval` + `examples/dval_case` +
`tools/dval/compare_dval.py`), S0 cut-point map, S1 operands, `SC-OFEROUTE-001`
rev 8 (`INV-OFEROUTE-011` partial, GAP-004 withdrawn, skin-convention audit
open; Zone taxonomy deferred). Zero clean reproductions — per the success criterion (*truthful per-case verdicts, not all
four pass*), D7 is executed. Shadow-first preserved; no production wiring.
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D,
validation stage. Contract: `SC-OFEROUTE-001` (`INV-OFEROUTE-011` acceptance;
`INV-OFEROUTE-002` I-unit convention). Owner: Claude Code (scaffold);
executor TBD. Activation: **analysis only — no production wiring**; the
routing subsystem stays shadow-first throughout.

> This package is authored as a **scaffold**: objective, honest scope, staged
> plan, acceptance model, and governance. It deliberately does **not**
> prescribe implementation architecture — the executor (Codex, or Claude if
> directed) owns how the runner/harness is built. A feasibility spike
> (`artifacts/feasibility-findings.md`) grounds the framing; read it first.

## Objective

Assess whether openWEPP's OFE routing (D3 friction + D4 solver + D5 cascade +
D6 infiltration) **reproduces the published enhanced-WEPP hydrographs** of
Papanicolaou et al. 2018 for the four validation cases — the D-val evidence
`SC-OFEROUTE-001#INV-OFEROUTE-011` assigns here.

**What this validates and what it does not.** The digitized
`Figure_*.xlsx` series are Papanicolaou's **enhanced-WEPP model output** (plus
Original-WEPP, and Observed where present). Reproducing them validates
**method fidelity** — that openWEPP faithfully implements the reference
method the contract is anchored to (`REF-OFEROUTE-FRAMEWORK`). It is **not**
validation against nature: the paper's Ef-vs-observed targets
(0.91/0.75/0.87/0.88) remain their citation, which openWEPP inherits only to
the extent it reproduces their model. This distinction is load-bearing and
must survive into the acceptance language (do not report "validated against
observations").

**Authority split.** D7 has two separate quantities:

- `NS_trace`: openWEPP versus the digitized enhanced-WEPP model trace, computed
  by this package to judge method fidelity.
- `Ef_obs`: the paper's enhanced-WEPP versus observed/measured efficiency,
  cited from Papanicolaou et al. and not recomputed as openWEPP validation
  against nature.

`INV-OFEROUTE-011` evidence updates must preserve this split. A passing
`NS_trace` says openWEPP reproduced the reference method trace; it does not
turn the paper's `Ef_obs` into an openWEPP observed-data verdict.

## Framing — this is an investigation, not a rubber-stamp

> **Execution governs.** The bullets below are the pre-execution *feasibility-
> spike* framing. Where they conflict with the executed result, the status
> banner + `artifacts/execution-report.md` are authoritative. In particular
> the spike's "Iwagaki shock lag" was later found to be a **forcing-bug
> artifact and is WITHDRAWN** (see the corrected bullet).

The feasibility spike motivated an investigation (not a rubber-stamp) scope:

- **Iwagaki (Case 4, shock) — spike saw a ~5 s "lag" that EXECUTION
  WITHDREW.** The spike (and the first execution) fed the lateral-supply rate
  into the skin-term rainfall intensity `I`, but Iwagaki has no rain. With
  `I = 0` corrected, the timing and rise **reproduce** at `k_o ~ 200`; the
  residual is peak magnitude / moderate NS, **operand-limited** on the
  unspecified flume `k_o`. There is **no** shock-capture GAP
  (`GAP-OFEROUTE-004` withdrawn).
- **The figure→case→geometry→unit mapping is not pinned.** Figure_5's
  enhanced-WEPP peak (0.0028 m²/s) is ~24× too large for a 60 mm/h × 7.5 m
  Case-1 plot — it is almost certainly the Walnut Creek *hillslope*, not the
  Case-1 plot. Series carry mixed units (`l/s/ha`, `m³/s/m`, `mm/hr`).
  Comparing before the mapping is proven like-for-like will manufacture false
  agreement or false divergence (the recurring HPHYS comparator-surface
  hazard).

So D7 must be scoped to **produce honest per-case verdicts** — some
reproduce, some are gaps — not to reach a preordained pass.

## Scope (staged; each stage gates the next)

- **D7-S0 — Cut-point mapping (prerequisite).** For each `Figure_*.xlsx`
  column, pin: which validation case / hillslope it is, its geometry, and its
  unit. Prove every comparand is like-for-like (unit + geometry + quantity
  class) **before** any NS is computed. Deliver a mapping table with the
  reasoning; a magnitude sanity check (steady q ≈ excess·L) per column.
  The table must include source file, sheet, column range, series role
  (`Observed`, `Enhanced_WEPP`, `Original_WEPP`), time origin, time unit,
  discharge unit, geometry basis, conversion applied, comparison window, and
  pass/fail disposition. Any unpinned or contradictory field fails S0 closed.
- **D7-S1 — Operand completion.** Cases 1–3 need Green-Ampt soil params
  (`Ks`, `ψ`, `Δθ`) not in the operand set — derive from texture (Rawls et
  al.) with cited sources and a stated uncertainty; Case 4's flume `k_o` is
  unspecified in the paper — source or bound it, and record it as an operand
  gap if unresolvable. A case cannot receive a solver-defect verdict until all
  load-bearing operands are sourced, frozen, or bounded tightly enough that the
  verdict is invariant across the declared bound. Otherwise the verdict class
  is `operand-limited`.
- **D7-S2 — Skin-term unit convention (`INV-OFEROUTE-002`).** *(planned)* Pin
  the `I`/`ν` convention of `f_s = (3393 I^0.407 + k_o)/Re` against a
  fully-known case. **Execution outcome: NOT closed.** The skin term is
  `k_o`-dominated for Cases 1-3 (`k_o = 500`; the `I` term ≈ 60 for Case 1), so
  Case-1 reproduction does not validate the convention, and D7 found + fixed an
  `I`-usage bug (Iwagaki). `INV-OFEROUTE-002` / `GAP-OFEROUTE-002` are corrected
  to **unconfirmed / audit OPEN**; the audit is a follow-up, not closed by D7.
- **D7-S3 — Non-shock validation cases (expected tractable).** Cases 1–3
  exercise grain roughness, isolated roughness, and vegetation patchiness.
  The rise-to-steady cases should align with D4's steady-state evidence; Case 3
  must additionally exercise the D5/D6 vegetation-strip / infiltration-cascade
  composition. Compute `NS_trace` vs enhanced-WEPP only after S0/S1 pass.
- **D7-S4 — Shock case (attribution).** *(planned: attribute the spike's
  Iwagaki lag.)* **Execution outcome: the "lag" was a forcing-bug artifact**
  (lateral rate mis-fed into the skin `I`; Iwagaki has no rain). With `I = 0`
  the timing/rise reproduce at `k_o ~ 200`; the residual peak/NS is
  **operand-limited** on the unspecified flume `k_o`. **No shock-capture GAP** —
  the provisional `GAP-OFEROUTE-004` was **WITHDRAWN**. Two residual solver
  observations (peak `k_o`-noise; internal vs sampled `t_peak` disagree ~9 s)
  are noted for a possible future look, not GAP-promoted.
- **D7-S5 — Verdict + contract disposition.** Per-case NS + verdict; update
  `INV-OFEROUTE-011` evidence (**executed: PARTIAL, not closed** — zero clean reproductions); the provisional shock gap was **withdrawn**, not promoted; and either
  reproduce or explicitly defer the Zone 1/Zone 2 stream-power taxonomy row
  that `INV-OFEROUTE-011` currently names. If taxonomy is deferred, amend the
  contract so D7 does not appear to close an unrun obligation.

Sequencing note: **do not start S3/S4 comparisons before S0/S1 close** — an
unpinned cut-point or a guessed operand invalidates any NS number computed on
top of it.

## Acceptance model (honest, per-case)

A case **reproduces** enhanced-WEPP only when all gates below pass:

1. S0 proves the compared surfaces are like-for-like.
2. S1 freezes or bounds every load-bearing operand.
3. `NS_trace >= 0.85` over a named, justified comparison window.
4. Peak magnitude and time-to-peak are within named, pre-run bands.
5. The case-specific shape co-condition passes:
   - Cases 1–3: runoff initiation, event volume/runoff depth, rising-limb
     timing, and steady or recession segment shape agree within named bands.
   - Case 3: vegetation-strip response is not reduced to a steady-depth proxy;
     the hydrograph must preserve the enhanced-WEPP patchiness signature.
   - Case 4: shock onset, 10–90% rise time, peak timing, and absence of
     oscillatory artifacts agree within named bands.

`NS_trace` is necessary but not sufficient. A high NS with the wrong onset,
modality, shock steepness, or volume is a failing method-fidelity verdict, not a
pass. A case that does not reproduce (Iwagaki today) is a **documented gap with
attribution**, carried as a `GAP-OFEROUTE` or `operand-limited` verdict — never
tuned into a pass. The package succeeds if it delivers *truthful* per-case
verdicts, not if all four pass.

## Copyright governance (binding)

The `Figure_*.xlsx` series are copyrighted and gitignored. They are **read to
compute comparisons, never vendored**:

- Committed tests assert **citable scalar targets** (published peak,
  time-to-peak) with provenance — the C01/C03 anchor pattern — plus the
  openWEPP-computed values.
- The **full-series NS** is computed **offline** and reported in an artifact
  that **references** the source by path + sha256 (Figure_4:
  `2bf68787…d2fe8`; Figure_5: `a58c4e29…d1f96`), not by duplicating the
  series.
- The executor must add a checked-in, copyright-safe extraction/comparison
  script or harness that reads the gitignored workbook from the local
  `references/copyrighted/` cache, verifies sha256, and emits only derived
  scalar metrics/artifacts. The script must not contain or generate committed
  workbook rows, full hydrograph series, or copyrighted table copies.
- No `Figure_*.xlsx` content is copied into the repo or into a test fixture.

## Gates (executor must meet)

- Shadow-first preserved: no production phase wiring; default hillslope path
  byte-flat (`INV-OFEROUTE-010`). D-val is analysis over the existing shadow
  kernels.
- fmt / clippy `-D warnings` / full orchestrator suite / BEI PASS-DEFERRED /
  authority anti-evasion — all green.
- Any contract edit (INV-011 evidence, a new GAP) passes BEI lint and adds a
  revision entry.
- Truthful reporting: `Ran`/`Static` labels; verbs match evidence; no
  "validated against observations"; per-case verdicts state `NS_trace`,
  paper `Ef_obs` citation status, window, shape-gate result, and
  operand-uncertainty.

## Risks / open questions (for the executor to resolve, not assume)

1. Iwagaki `k_o` is unknown — the shock verdict may be operand-limited, not a
   solver defect. S1/S4 must separate these.
2. Green-Ampt params for Cases 1–3 are texture-derived (uncertain) — the
   steady discharge depends on `Ks`; report the sensitivity.
3. *(resolved)* The spike's ~5 s shock lag was a forcing-bug artifact, not a
   solver finding; withdrawn on correction. Residual Iwagaki solver notes
   (peak `k_o`-noise; internal vs sampled `t_peak` ~9 s) remain observations,
   not attributed defects.
4. Figure→hillslope mapping (Fig 5–8 appear to be the Walnut Creek hillslope
   thought-experiments, not the plot cases) — S0 must resolve before use.
5. *(n/a — S4 attributed the shortfall to operands, not the solver, so no
   solver/cascade correction package is opened. The split-rule stands for any
   future shock work if a known `k_o` reveals a genuine solver gap.)*

## References

- `artifacts/feasibility-findings.md` — the grounding spike (this package).
- `docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/`
  — case operands, `source-manifest.md` (sha256), `validation-cases.json`.
- `references/copyrighted/Papanicolaou2018.md` §3.1 (case setups), Table 1
  (operands), §3.1.4 line 205/207 (Iwagaki spec + shock note).
- `SC-OFEROUTE-001` `INV-OFEROUTE-002` / `INV-OFEROUTE-011`.
