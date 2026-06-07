# Claude Code Review — FQ1 Soil Corrected-Layer Coverage Closure

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the Milestone-1 localization, the
`02_soil_slope.rs` diff, the `SC-SOIL-001` v23 amendment, the validation ledger,
disposition, and handoff. The p1–p43 reruns and 6-control non-regression are
Codex's `Ran` evidence, attributed.

Verdict: **Approve.** This is a model DC-ExecPlan: it localized the mechanism,
confirmed ownership against the legacy comparator, landed a correct, contract-first,
guard-preserving fix, validated it (HS-RUNTIME-E-062 37/43 → 0/43), and held at a
genuinely out-of-envelope boundary (the p11 percolation residual) with a
defect-shaped handoff.

Reviewer correction up front: I initially read `executed-hold-boundary` as "held
without fixing" (a possible conversion-rule violation). That was wrong — the soil
fix **landed** (`02_soil_slope.rs` + `SC-SOIL-001` v23). `hold-boundary` here is a
legitimate terminal state: in-envelope defect fixed, held on a *new* out-of-envelope
blocker.

---

## F1 — Exemplary execution; the conversion rule worked (primary)

- **Localization (M1):** the mechanism is precisely pinned —
  `map_corrected_layer_runtime_symbols_to_parser_layers` built the normalized
  corrected grid from `0` in fixed `WB13_PROFILE_LAYER_THICKNESS` steps (9×200 =
  1800 mm) and required every parser layer fully covered, so a valid profile whose
  deepest parser layer ends at 2000 mm exposed an 1800→2000 mm tail gap.
- **Ownership (M1, Ran):** legacy `wepp_260606_hill` completes both a blocked soil
  (p1) and a control (p8) — so the soils are valid and this is an openWEPP defect,
  not invalid input. Correct ownership call.
- **Fix (verified in the diff):** for the deepest normalized corrected layer only,
  when the parser profile bottom exceeds the normalized bottom, extend that
  interval to the parser bottom using the deepest corrected lineage. This is the
  standard WEPP behavior (the bottom soil layer extends to the profile bottom),
  using **real** deepest-layer properties — not zero-fill, not a guard relaxation.
- **Guard preserved AND tested:** the protected boundary held — a non-monotone
  (invalid) profile still fails closed (`fq1_mapping_still_fails_closed_for_
  nonmonotone_parser_layer`), and a control shape within normalized depth is
  unchanged (`fq1_runnable_control_mapping_within_normalized_depth_is_unchanged`).
- **Contract-first, scope-disciplined:** `SC-SOIL-001` v23 `INV-SOIL-017` encodes
  the deepest-layer extension with authority (`REF-SOIL-LEGACY-WB11`,
  `REF-SOIL-CH7-POR`, `REF-SOIL-PHYS-BOUNDS`), is `hard-fail`, and explicitly keeps
  the WB11/WB18/WB19 hydrology seed grid on `INV-SOIL-015` (the fix touches only the
  parser-layer diagnostic/constitutive symbols, not the seed grid) and preserves
  fail-closed for invalid layers. That is the right narrow scope.
- **Validated:** 43 run, 42 rc=0 + WAT/HBP, **0 `HS-RUNTIME-E-062`** (from 37); the
  6 controls all rc=0 with WAT/HBP (non-regression).

This is exactly what ADR-0018 intended: diagnose → confirm ownership → land the
contract-first fix → validate, no grind.

## F2 — The residual p11 is a legitimate out-of-envelope boundary (correctly handled)

After the soil fix, p11 runs further and fails at `HKERNEL-WB11-PERC-E-003`
(`percolation_deep_seepage`, 1990 day 162; guard terms `infiltration=0`,
`slflag=1`, `kslast=2.8e-9`, `ui_bdrkth=10`, `invalid_layers=none`). That is a
percolation/deep-seepage defect, outside the declared soil parse/runtime envelope —
FQ-1 correctly did **not** normalize or bypass it, and handed it off defect-shaped
(`FQ1-P11-HKERNEL-WB11-PERC-E-003-J162`). The soil block had been masking it (p11
never reached percolation before), so FQ-1 *unmasked* a pre-existing defect by
fixing the soil — which is the expected, healthy consequence of unblocking.

Note for the handoff: `HKERNEL-WB11-PERC-E-003` is the **same error family** as the
WBVAL05 J-95 percolation defect (and WBVAL03's), and the `slflag=1` / tiny `kslast`
suggests a restrictive-layer deep-seepage conductivity domain. The p11 follow-on
should consume the WBVAL05 percolation lineage (WB18 consume-vs-recompute, the
negative-SWE/snow routing) rather than start cold.

## F3 — Rung-2 status after FQ-1

FQ-1 removed the population-scale soil blocker: the frost substrate is now 42/43
runnable (from 6/43). The next follow-ons stand:
- **FQ-2** (frost-closure ledger fix) — still needed; the prior `frost-break` is a
  tool artifact.
- **FQ-3** (Ep/Q/Er/Interception = 0 characterization) — now assessable on 42
  hillslopes; still the likely deeper openWEPP gap.
- **FQ-4** (frost activation) — on the clean substrate.
- **p11 percolation** — small out-of-envelope follow-on (WBVAL05 family).

---

## Recommendation

Approve and commit (soil fix + `SC-SOIL-001` v23 + tests). No rework. The soil
blocker is closed correctly and conservatively (real-property extension,
guard-preserved, contract-backed, narrow scope). Proceed to FQ-2/FQ-3 on the now
42/43 substrate; route p11 to a small percolation follow-on that reuses the WBVAL05
lineage.
