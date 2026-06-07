# Claude Code Review — WBVAL06 Interception-Flux Publication Closure

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the package, the WAT schema diff
(`hillslope_wat.rs`), the runner diff (`hillslope/mod.rs`), and the
`SC-WATBAL-001` v146 amendment including the pre-existing closure conformance
vector. I did **not** run anything; the 22/22 closure (`max 1.04e-6 mm`), the
gates (`fmt`/`clippy`/`test --workspace`/`deny`), and the release rerun are
Codex's `Ran` evidence, attributed. Changes are **uncommitted** — this is the
pre-commit gate.

Verdict: **Approve the diagnosis and the openWEPP-side publication; one
cross-tool closure gap to settle.** The residual was correctly identified as a
WAT *publication-completeness* gap (interception), not a physical leak — the
symptom-existence gate worked. openWEPP writes `.parquet` directly and is
independent of the wepppyo3 interchange (that interchange only parses legacy WEPP
text output), so there is **no schema-inheritance constraint** — openWEPP may
publish the interception flux as it sees fit. The surviving question is the one
that actually matters: **water-balance closure must be auditable from
totalwatsed3**, and WBVAL06 demonstrated openWEPP's *own* identity audit (1e-6),
not totalwatsed3's. See the corrected F2 below.

(Correction, 2026-06-07: an earlier draft of this review framed F2 as an
ADR-0005 wepppyo3-interchange schema-match issue. That was wrong — openWEPP
bypasses the wepppyo3 interchange. The real acceptance is totalwatsed3 WB-audit
closure.)

---

## F1 — Correct diagnosis and a non-fudge, contract-first fix (primary)

The revised Milestone-1 symptom-existence gate did its job: instead of grinding
the 26.79 mm residual as a physical leak, it audited the identity and found the
omitted term. Three points confirm the fix is sound, not a fudge:

1. **`I` is a real, already-computed runtime flux.** The runner publishes it from
   an existing runtime symbol (`require_runtime_surface_scalar_prefer_flux(
   runtime_surface, "I")`, guarded `>= 0`), not from the residual. It was being
   computed all along for the canopy/interception budget; it simply was not
   exposed in the WAT parquet.
2. **`I` was already canonical authority.** `SC-WATBAL-001`'s pre-existing
   conformance vector (item 7) is `wb12_storage_reconciled = wb12_storage_initial
   + wb12_precip_input + S - I - Q - ET - D - Qd` — `I` is a recognized
   Chapter-5 closure term used *internally*. The kernel was conserving with `I`;
   only the WAT *output* omitted it, so any external balance from the published
   columns under-counted outputs by exactly `I`. v146 adds the publication (new
   item 8), schema column, unit-registry alias, and runner guard — contract-first.
3. **It zeroes all 22×5 to 1e-6.** A fudge term would not simultaneously close
   every emitter and year to machine precision; a genuinely-missing real flux
   does. The residual was exactly the unpublished interception flux.

This vindicates the WBVAL01 B1 concern (the residual was an incomplete published
identity, not a leak) and the whole symptom-existence-gate discipline.

## In-envelope and boundary-respecting

- **Publication accounting only** — a new optional output column for an
  already-computed flux; no kernel physics changed.
- **Flux vs storage correctly separated** — `Interception` (flux `I`) is distinct
  from the existing `InterceptionStorage` (carryover); `build_hillslope_wat_row`
  sets `interception: Some(I)` and leaves `interception_storage: None`, and the
  v146 note explicitly avoids "overloading storage semantics." Good — they are
  different physical terms.
- **Snow protected boundary respected** — the residual resolved to interception,
  not snow, so the boundary was never approached; no snow physics touched.
- **Not a silent term-add** — governed by contract text, units, and tests, per
  the package's own protected boundary.

## F2 — totalwatsed3 audit closure (the real acceptance; one gap to settle)

openWEPP writes `.parquet` directly and does not inherit the wepppyo3 interchange
schema, so the only thing that matters is that **WB closure is auditable from
totalwatsed3** (the sole consumer of these interception terms is WB auditing).
Verified 2026-06-07:

- **totalwatsed3** (`wepppy/wepp/interchange/totalwatsed3.py`,
  `docs/dev-notes/totalwatsed-interchange.spec.md`) closes with
  `Precipitation − (Runoff + Lateral Flow + ET + Percolation) − ΔStorage`. Its
  outflows are Runoff/Lateral/ET/Percolation — **no separate interception-flux
  outflow** — and it carries `InterceptionStorage` only as a storage passthrough.

WBVAL06 publishes interception as a *separate* `Interception` flux (`I`) with
openWEPP's published `ET` *excluding* it (per the `SC-WATBAL-001` closure vector
`... + S − I − Q − ET − D − Qd`). The 1e-6 closure WBVAL06 reported is from
openWEPP's *own* identity audit, which adds `I`. totalwatsed3's audit, as written,
does **not** add `I` (and reads `InterceptionStorage = None`), so on openWEPP's
output its residual would reappear at ≈ `+I` (≈26.8 mm). **Closure has been shown
for openWEPP's private identity, not for the totalwatsed3 audit — which is the
stated acceptance surface.**

This is not a schema-inheritance problem; it is a decomposition-alignment one.
Two valid resolutions, the choice is free since WB auditing is the only consumer:

1. **Fold interception into published `ET`** so totalwatsed3's existing closure
   (`P − (Q + Lat + ET + Perc) − ΔS`) closes on openWEPP output with no
   totalwatsed3 change. Simplest; keep `I` as an internal/diagnostic term.
2. **Teach totalwatsed3 to consume the `Interception` flux** as an outflow
   (`P − (Q + Lat + ET + Perc + I) − ΔS`), keeping `I` a first-class auditable
   term. Requires the wepppy-side totalwatsed3 companion update (which WBVAL06,
   being openWEPP-only, did not make).

Either is fine; option 2 preserves interception as a visible audit term and
matches WBVAL06's existing flux column. The gap to settle before declaring rung-1
closed: run the **totalwatsed3** audit on openWEPP's post-WBVAL06 output and show
it closes (which today implies the option-2 companion update, or option 1).

## F3 — Independent review still unmet (carry-forward)

As with SNOWSCI-S1, truly independent dual review/verification requires sub-agent
authorization and was not performed; this review partial-fills the science/diff
gate, and the passing cargo suites are the objective backstop.

---

## Strategic note — rung-1 single-OFE conservation is now essentially closed

Together, SNOWSCI-S1 (the snow signed-melt double-debit, ~72% of the leak) and
WBVAL06 (the interception publication completeness, the remaining ~28%) close the
single-OFE WAT conservation identity to ~`1e-6 mm/year` for all 22 emitters,
years 2..6, on the `indispensable-presenter` validation run **under openWEPP's own
identity audit**. Stated precisely: the single-OFE conservation *mechanism* is
closed for years 2..6; year 1 remains an explicit initial-storage exclusion;
broader confidence wants a second run; and the closure should be reproduced
through the **totalwatsed3** audit (F2) before rung-1 is called done. Subject to
that, rung 1 (single-OFE WB closure) is in substance complete — next rung is
**frost**.

## Recommendation

The diagnosis is right and the openWEPP-side interception publication is fine to
commit (it does not break any consumer — WB auditing is the only one). Before
declaring rung-1 closed, settle F2: run the **totalwatsed3** audit on openWEPP's
post-WBVAL06 output and show it closes — which today means either folding
interception into published `ET` (option 1, no totalwatsed3 change) or the
wepppy-side totalwatsed3 companion update to consume the `Interception` flux
(option 2). No rework of the openWEPP fix itself is needed.
