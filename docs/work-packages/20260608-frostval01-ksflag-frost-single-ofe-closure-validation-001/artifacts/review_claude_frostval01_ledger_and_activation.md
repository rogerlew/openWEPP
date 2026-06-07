# Claude Code Review — FROSTVAL01: broken closure ledger, real activation question

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: mixed. **Static:** read the package, activation/closure ledgers,
disposition, handoff. **Ran:** verified actual precipitation — `p8.cli` annual
total (`awk` sum) and the openWEPP WAT `P` column (`duckdb` on
`/tmp/frostval01/full/out/p8.wat.parquet`), plus loss.json cross-checks.

Verdict: **`executed-hold` is correct, the package discipline held, and
HS-RUNTIME-E-062 is correctly identified as the dominant blocker — but the
frost-closure ledger is defective, so the "6/6 frost-break" verdict is a
validation-tool artifact and must be withdrawn.** The credible real frost finding
is the *activation* one, and it is now stronger than the package could conclude.

---

## F1 — Discipline held; the dominant blocker is correctly named (positive)

Milestone 1 did its job: it required proving frost activation before trusting any
closure-under-frost result, the package held rather than declaring a trivial pass,
and `HS-RUNTIME-E-062` (37/43 single-OFE hillslopes blocked before hydrology — a
soil runtime/lineage-coverage gap) is correctly named the top-priority unblock.
That is the right rung-2 posture, and the soil-coverage follow-on is real.

## F2 — The frost-closure ledger is broken; "6/6 frost-break" is an artifact (key finding)

The closure ledger reports `Inputs ≈ 5–13 mm/year` for the 6 runnable hillslopes
(e.g. p8 ≈ 10 mm/yr), with ΔStorage swinging ±130 mm and residuals to ~134 mm.
That is physically impossible, and I verified why:

- `p8.cli` (PRESTON MN, gridmet) annual precipitation = **911 mm/yr**
  (6379.6 mm / 2557 days).
- openWEPP's own WAT output `P` column for p8 = **911 mm/yr** (`duckdb` sum =
  6379.6 mm / 2557 days). **openWEPP's precipitation is correct.**

So the ledger's ~10 mm "Inputs" is a **closure-ledger aggregation bug**: openWEPP's
WAT P is correct *and complete* (6379.6 mm = full climate, P>0 on 1149 days), and
the ledger's `inputs ≈ 9 mm/yr` / `outputs ≈ 8 mm/yr` are ~1% of openWEPP's actual
term sums — it sums fluxes over a wrong tiny day-set while ΔStorage is full-scale,
so the residual is dominated by ΔStorage and is meaningless. Therefore the
`frost-break` classification for the 6 runnable hillslopes is **invalid as a
conservation verdict** — it is the validation tool measuring wrong.

Scope of that conclusion (correction): this means the specific `frost-break`
residual is a *ledger* artifact and openWEPP's **precip forcing is correct**. It
does **not** mean openWEPP is clean on this run — see F2b. An earlier wording of
this review said "not an openWEPP defect," which over-generalized from one correct
number to the whole run; that is wrong.

The package applied its symptom-existence discipline to *activation* but not to its
own *closure ledger*: inputs of 10 mm against 911 mm of real precip is an obvious
tool error that should have been caught before any `frost-break` verdict. **Withdraw
the `frost-break` classification**; fix the ledger to read the actual WAT `P` and
re-derive. (Given rung-1 closes to ~1e-6 on a comparable setup, the real residual
on these 6 may well be small — unknown until the ledger is fixed.)

## F2b — Real openWEPP output anomalies on this run (not ledger artifacts)

Pulling openWEPP's actual p8 WAT term sums (full run, `duckdb`) shows genuine
openWEPP behaviors that the "not an openWEPP defect" framing wrongly waved away:

| term | mm/yr | term | mm/yr |
|---|---:|---|---:|
| P | 911 (✓ correct) | Es | 698 |
| RM | 901 | Dp | 89 |
| **Q** | **0** | latqcc | 93 |
| **Ep** | **0** | **Er** | **0** |
| **Interception** | **0** | Irr | 0 |

- **Ep = 0** — zero plant transpiration over 7 years on a **Corn** site; all ET is
  soil evaporation (`Es`). Strong anomaly: the PMET/plant-transpiration path looks
  inactive on this run.
- **Q = 0 / QOFE = 0** — zero runoff over 7 years on a 911 mm/yr site.
- **Er = 0, Interception = 0** — canopy/residue evaporation and the interception
  flux both zero on a cropland site.

These are openWEPP's own outputs, not ledger artifacts, and are real signals to
investigate (with `wepp_260606` as the activation/behavior flag per ADR-0017).
They are mostly ET *partition*/runoff *magnitude* questions — they may or may not
break conservation — but they mean this run is **not clean** and the frost rung is
entangled with them. (`Ep`/`Q` = 0 would also be consistent with a plant/PMET or
runoff path not engaging; needs the comparator flag and a telemetry check, same
caution as F2.)

## F3 — The activation finding is the credible real defect, and is now stronger

On the 6 runnable hillslopes: `frozwt = 0` all days and ksflag on/off flux deltas
= 0. The package correctly left this "undetermined" (frost-inactive could mean "no
freezing conditions"). But F2 removes one alternative and strengthens it: the water
is real (911 mm/yr precip, verified), and **PRESTON MN (43.67°N, 286 m, Minnesota)
freezes hard every winter** — so frost *should* produce frozen soil water there. A
genuine `frozwt = 0` with real water at a freezing site points at **frost not
activating in openWEPP** (the standard `ksflag` path is honored as a value but
produces no frost state). Two confirmations needed before calling it:

1. **Verify the activation telemetry is not itself a tool bug** (same class of
   error as F2): confirm `frozwt`/frost-depth are actually being read, not silently
   zero from a mis-keyed query.
2. **Use the comparator as the activation flag** (ADR-0017 permits this for the
   "should frost fire here" question): does `wepp_260606` produce `frozwt > 0` on
   the 6 at PRESTON MN? The package ran the comparator but does not report its
   frost state on these prefixes — that is the cheapest decisive check and belongs
   in the activation ledger.

If both confirm, the rung-2 frost DC-ExecPlan target is "openWEPP standard `ksflag`
frost does not activate," not "frost breaks closure."

## F4 — Reviewer correction (truthfulness)

In the course of this review I first read `p8.loss.json` `precipitation_mm = 0.0`
and nearly concluded openWEPP was applying ~0 precip. Cross-checking the
known-good WBVAL01 runs showed `precipitation_mm = 0.0` there too — it is an
**unpopulated loss.json field**, not a dry-run signal. Retracted. The decisive
evidence is the WAT `P` column (911 mm, correct), not loss.json. (Separately:
openWEPP's `loss.json precipitation_mm` being always-0 is a minor reporting gap
worth noting, but it is not a forcing defect.)

---

## Recommendation

Keep `executed-hold`. Re-shape the follow-on queue:

1. **HS-RUNTIME-E-062 soil-coverage DC-ExecPlan** (unblock 37/43) — agreed, top
   priority; without it the frost rung cannot be validated at population scale.
2. **Fix the frost-closure ledger** (Inputs bug: read WAT `P` = 911 mm, not 10 mm)
   and re-derive closure on the runnable cohort. Until then, **withdraw "6/6
   frost-break"** — it is a tool artifact, and openWEPP's precip output is verified
   correct.
3. **Confirm the activation defect** (telemetry sanity-check + `wepp_260606`
   frost-flag on the 6). The `frozwt = 0` at a freezing site with real water is the
   credible rung-2 frost finding; promote it to a frost DC-ExecPlan if confirmed.

Net: FROSTVAL01's headline "frost-break" verdict is a broken-ledger artifact and
should not be carried forward, and openWEPP's **precip forcing is correct**. But
this is not a clean run: it surfaced the soil-coverage blocker (HS-RUNTIME-E-062,
37/43), a credible frost non-activation, and real zero-term anomalies in openWEPP's
own output (`Q`, `Ep`, `Er`, `Interception` all 0 — F2b). "Not an openWEPP defect"
was the wrong summary; the correct one is "the *ledger's* 10 mm is not openWEPP's
fault, but openWEPP has real signals here to pursue."
