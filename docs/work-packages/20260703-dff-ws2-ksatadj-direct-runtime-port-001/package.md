# DFF-WS2 — `ksatadj` Effective-Conductivity Re-Port (SUBHYD, direct runtime)

Status: **SCAFFOLDED (scope + entry gate), 2026-07-03 — awaiting execution.**
Campaign: [disturbed-forest-fidelity](../../planning/disturbed-forest-fidelity-strategy.md)
WS-2. Governing authority:
[`SC-SUBHYD-001`](../../specifications/science-contracts/contracts/SC-SUBHYD-001.md)
`INV-SUBHYD-032` + `BR-SUBHYD-KSATADJ-EXECUTE/GUARD`;
[ADR-0024](../../decisions/0024-reference-implementation-intent-authority.md)
(source-intent authority). Depends on WS-1 (native forest `lanuse`, merge-ready)
for disturbed-soil validation inputs. Owner: Claude Code (scaffold); Codex
authors the kernel re-port. This is a **re-port**, not a new derivation.

## Objective

Re-implement the ratified `ksatadj` disturbed-soil effective-conductivity model
in the **`direct_runtime` production lane** (WB14 conductivity formation), so that
for `ksatadj = 1` soils the infiltration/runoff conductivity is formed from the
`SC-SUBHYD-001#INV-SUBHYD-032` source-intent algorithm rather than base/frost
conductivity alone. Keep **frost on** (`ksflag = 1`); do **not** carry the legacy
`ksflag = 0 → frost off` non-ag workaround. Lift the `BR-SUBHYD-KSATADJ-GUARD`
governance `HOLD` ("until the source-intent operand lineage is implemented") by
implementing that lineage.

## The record this rests on (grounded)

This is **not** a from-scratch build. The `ksatadj` physics was implemented,
contract-ratified, and corrected in the symbol-map lane, then deleted wholesale
with that lane — the direct lane never got the port.

- **Parsed:** `DisturbedPolicy` (`V9002` / `V9003` / `V9005`) carries the policy
  operands (`ksatadj`, `luse`, `stext`, `ksatfac_mm_h`, `ksatrec_per_day`,
  `burn_code`, `lkeff_mm_h`, `uksat_mm_h`) in
  `crates/openwepp-input-contract/src/parsers/soil.rs` — still parsed at HEAD.
- **Contract-complete (no amendment needed to author the physics):**
  `SC-SUBHYD-001` fully specifies the model — `INV-SUBHYD-032` (hard-fail), the
  source-intent algorithm (§"Reference-Intent `ksatadj` Effective-Conductivity
  Authority"), `BR-SUBHYD-KSATADJ-EXECUTE` (operand set) / `-GUARD` (typed
  hard-fail **or** governance `HOLD`), boundary exports (`Keff_ksatadj` →
  `wb14_soil_conductivity_m_s` / `wb14_effective_conductivity_m_s` when
  `ksatadj = 1`), and the conformance-vector obligation. WS-2 **implements** this;
  the only contract edit expected is updating the `HOLD` disposition once the
  lineage lands.
- **Deleted kernel is git-recoverable (the re-port reference):** the corrected
  kernel lived at
  `crates/.../hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`,
  deleted in `a381702b` ("Delete symbol-map kernel boundary runtime", 2026-06-30).
  Recover it as the port reference:
  `git show a381702b^:crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`.
- **Source-intent authority (ADR-0024):**
  `wepp-forest_260430_baseline/src/infpar.for:237-260,286-296,606-648` +
  `input.for:467-473,592-623,748-928` (`dac3c950…`), captured as
  `REF-SUBHYD-KSATADJ-INTENT` in `SC-SUBHYD-001`.
- **Prior-art work-packages (read before executing):**
  `20260618-refimpl-intent-authority-ksatadj-subhyd-001` (ADR-0024 ratified),
  `20260618-refintent001-ksatadj-satfrac-defect-closure-001` (the `sat_frac`
  correction), `20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001`,
  `20260525-mofe13-ksatadj-three-regime-kernel-alignment-001` (the 9001/9002/9003
  regimes).

## The source-intent algorithm (from `INV-SUBHYD-032`, corrected)

For `ksatadj = 1`, over the top two tillage layers:

```
avsat    = (st_1 + st_2)/tillay(2) + avsm15                  ! total water + residual
caps     : if avsat > avpor          -> avsat = avpor*0.98
           if avsat >= avpor*avcpm    -> avsat = avpor*avcpm*0.99
sat_frac = min( avsat / (avpor*avcpm), 1.0 )                 ! rock-corrected denominator
  9001   : keff = ... (exp(sat_frac/ksatrec)-1) + Klower     ! ksatfac/ksatrec recovery
  9002+  : keff = (ks * 3.6e6) * sat_frac^(2*lambda + 3)     ! Saxton-Rawls exponent
  9003   : keff = max(keff, lkeff)   (when lkeff > 0)        ! hydrophobicity floor
```

**The historical defect the conformance vectors must guard (do not re-introduce):**
the pre-correction kernel formed `sat_frac = theta_sum/ul_sum` (storage over
summed upper-limit). The source-intent `sat_frac` uses the **rock-corrected
`avsat/(avpor*avcpm)`** denominator — these differ, and `SC-SUBHYD-001` requires a
conformance vector where they diverge.

## Entry gate — RESOLVED at scaffold time (see `artifacts/entry-gate.md`)

A HEAD code-state verification (2026-07-03) resolved the entry gate; the executor
starts from a clear runway:

1. **`ksflag → frost` coupling: already DECOUPLED — nothing to remove.** `ksflag`
   is parsed and carried per-lane (`soil.rs:216`, `lane_setup_helpers.rs:118`)
   but has **no consumer**; frost activation keys off the frost file's `wint_red`
   (`00a_snow_frost_authority_impl.rs:347-350`,
   `04_snow_frost_irrigation.rs:164`), not `ksflag`. FQ-4 already closed this.
   Frost stays **on** independent of `ksflag`. **WS-2 has no frost-decouple work.**
2. **Projection: `ksatadj` reaches a typed struct but is DEAD.**
   `runtime_inputs/02_soil_slope.rs:76-79,244-297` projects
   `TypedSoilWb11RuntimeProjection { ksatadj, ksatfac_mm_h, ksatrec_per_day,
   lkeff_mm_h }` — but **no runtime code reads these four fields** (whole-tree
   search: hits only in the parser + projection). `uksat_mm_h`, `burn_code`,
   `texid_enum`, `luse`, `stext` are parsed but **not** projected; there is no
   `keffflag`/`sat_frac` runtime symbol. WS-2 Increment-1 extends this projection
   (add `uksat`, `solwpv`, top-two layer operands) and gives the fields a consumer.
3. **Direct WB14 conductivity site (port insertion point):**
   `DirectProductionInfiltrationAuthority::inputs`
   (`00_builders_and_authority.rs:3291-3316`) forms `effective_conductivity_m_s`
   from **frost + base top-layer only** (`frost_infcap` → frost-seeded
   `effective_conductivity_m_s` → `layers.first().conductivity_m_s`), consumed by
   the Green-Ampt solver (`direct_runtime/runoff.rs:1595-1636`). `ksatadj` is
   genuinely absent. WS-2 overwrites this with `Keff_ksatadj` when `ksatadj = 1`.
4. **Deleted kernel** recovered as the port reference
   (`a381702b^:…/02_ksat_adjustment.rs`, 677 lines) — the contract is authority
   where it differs.
5. **Contract lifecycle note:** `SC-SUBHYD-001` is `in_review`; `GAP-SUBHYD-002`
   (openWEPP runtime-field aliases not yet fixed) is **non-promotable** — WS-2
   must land the `Keff_ksatadj → wb14_effective_conductivity_m_s` alias map, which
   helps clear it.

## Validation anchor (WS-2 fixture)

`tests/fixtures/disturbed_burn/forest_high_severity_loam/` — real wepppy
hillslope **313** (`honeyed-marathoner`), a **`ksatadj = 1`** /
`forest high sev fire` / `loam` burned forest (9002 policy row, `lkeff = 0.1`,
`keffflag = 1`), 6 sim-years. This is the missing `ksatadj = 1` input: the prior
fix was **byte-inert on H2637** (`ksatadj = 0` there). It **runs end-to-end**
through the production CLI (exit 0 → HBP/loss/wat), giving WS-2 a live
`ksatadj = 1` baseline to re-anchor against. (`p313.cli` was `rad`-clamped with
`tools/clamp_cli_radly.py` to the sunmap potential — this old CLIGEN output
predated the generator clamp; see the fixture `manifest.md`.)

## Scope / increments (staged, contract-first)

- **Increment 0 — entry gate:** items above; record in `artifacts/entry-gate.md`.
- **Increment 1 — operand lineage:** form the source-intent operands in the
  direct runtime (top-two tillage-layer `avsat`, `avpor`, `avcpm`, `avthetafc`,
  `avthetadr`, `sat_frac`) from the typed soil runtime state, with typed
  fail-closed guards (`BR-SUBHYD-KSATADJ-GUARD`). Non-aliased: `sat_frac` must be
  the rock-corrected form, not `theta_sum/ul_sum`.
- **Increment 2 — branch evaluator + WB14 wiring:** the 9001 / 9002+ / 9003
  formulas with `mm h^-1 ↔ m s^-1` conversion, overwriting the WB14 effective
  conductivity when `ksatadj = 1`; export `Keff_ksatadj` →
  `wb14_effective_conductivity_m_s`. Frost-on.
- **Increment 3 — conformance vectors:** re-create the non-aliased conformance
  vectors that died with the old kernel — at least one `solwpv ≥ 9002` and one
  where `avsat/(avpor*avcpm)` differs from a storage-over-upper-limit surrogate
  (per `SC-SUBHYD-001` §obligation). Contract-derived, source-intent — **not**
  legacy-magnitude.
- **Contract disposition:** update `BR-SUBHYD-KSATADJ-GUARD`'s `HOLD` to the
  implemented state; record the `INV-SUBHYD-032` closure.

## Guardrails (challenging inherited framings)

- **"Augment for sensible burned outputs" ≠ tune to legacy magnitudes.** The
  source-intent algorithm **is** the authority (ADR-0024). Implement it
  faithfully. If it produces implausible magnitudes (the legacy ~190,000×
  peakflow under hydrophobicity), that is a **WS-3 magnitude adjudication** —
  possibly a contract-governed physical-plausibility bound — **not** a WS-2
  ad-hoc conductivity knob. No provisional/proxy physics in the production path
  (AGENTS.md).
- **Scope honesty — `ksatadj = 1` only.** The prior fix was **byte-inert on
  H2637** (`ksatadj = 0` there): this re-port changes **only** burned/disturbed
  (`ksatadj = 1`) soils and does **not** touch base-conductivity soils. It does
  **not** resolve the FARPOINT01 71% runoff/lateral gap (that is base
  conductivity — a separate arc). WS-2's evidence surface is the WS-3 disturbed
  burn matrix, not H2637.
- **Bit-identity is not the gate.** Outputs change by design for `ksatadj = 1`
  soils; the gates are contract conformance + conservation closure + the
  conformance suites, not pre/post SHA equality.

## Validation

- **Kernel conformance (WS-2 gate):** the non-aliased `INV-SUBHYD-032` conformance
  vectors against `REF-SUBHYD-KSATADJ-INTENT` (source-intent), covering the
  9001/9002+/9003 regimes and the `sat_frac`-vs-surrogate divergence.
- **Conservation:** WB14/WB11 closure holds with the adjusted conductivity.
- **Directional (WS-3 gate, downstream):** the disturbed burn matrix reproduces
  burn-ordering laws (burned > unburned runoff/sediment/peak) with the re-ported
  `ksatadj`, frost-on — proving the decouple is sound. WS-2 delivers the kernel;
  WS-3 gates the pair.

## Sequencing & dependencies

WS-2 kernel work is **independent of WS-1 plumbing** (the source-intent re-port
is soil-side), so it can proceed now; its **validation** needs WS-1's
disturbed-soil inputs — WS-1 is merge-ready (branch
`dff-ws1-inc2-native-forest-lanuse`), and the disturbed `.sol` fixtures
(`tests/fixtures/disturbed_burn/`, `tests/fixtures/cancov_forest/`, 9002 policy
rows) are already present. WS-2 gates WS-3.

## Provenance

- Contract: `SC-SUBHYD-001` `INV-SUBHYD-032`, `BR-SUBHYD-KSATADJ-EXECUTE/GUARD`,
  `REF-SUBHYD-KSATADJ-INTENT`; `ADR-0024`.
- Source-intent: `wepp-forest_260430_baseline/src/infpar.for:606-648` +
  `input.for` (`dac3c950…`).
- Deleted kernel: `a381702b^:…/02_ksat_adjustment.rs`.
- Prior art: the four ksatadj/REFINTENT work-packages listed above.
- Campaign: `docs/planning/disturbed-forest-fidelity-strategy.md` WS-2.
