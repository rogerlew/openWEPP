# MOFE-FARPOINT01 — >10-OFE Routing-Closure Demonstration

Status: **active** — F-A (fixture + baseline + Finding 1) and **F-B**
(frost-overflow double-count defect-closure) complete (operator-directed
2026-06-16: scaffolded + run end-to-end by Claude Code). openWEPP now closes the
three conservation identities on the H2637 19-OFE substrate. Optional F-C
(legacy-vs-openWEPP closure contrast) + the `watpdg` branch-out remain. See
`artifacts/`.

Package type: validation/demonstration work package

## Objective

Demonstrate openWEPP's inter-OFE water-routing **conservation closure holding
at high OFE counts (>10 OFEs)** — the regime where legacy WEPP has known
water-balance defects (operator knowledge; corroborated by the wepppy MOFE
closure-audit triage). This is the rung's **differentiating result**: openWEPP
exceeding, not matching, the legacy ceiling, recorded as carry-forward
`MOFE-GT10-FARPOINT-CLOSURE` at MOFE01 closure.

MOFE01 proved water-routing closure on the `arboreal-dendrite` 1–5-OFE ladder
(legacy-clean there, so legacy was a usable flag). The >10-OFE domain is
**unreachable on that substrate's hillslope cohort** (`pw0` is the watershed
profile, not a hillslope run), so this is a separate package on a high-OFE
substrate.

## Included scope

- Select/identify a high-OFE (>10) hillslope substrate (a real run, or a
  constructed many-OFE profile with provenance).
- Run the MOFE01 per-OFE routing on it; evaluate the three conservation
  identities (per-element, transfer, **hillslope-total** — the M-I-a
  independent identity) at the noise floor per OFE count through >10.
- Measure legacy's own closure on the same high-OFE substrate (the
  comparator-trust curve continued past 10): demonstrate that legacy's
  water-balance defect appears while openWEPP's closure holds.
- Record the exceed-the-ceiling result with paired evidence.

## Excluded scope / protected boundaries

- No comparator-match tuning (ADR-0017); acceptance is openWEPP's own
  conservation closure, legacy is the flag.
- No new routing physics — this validates the MOFE01 implementation at scale;
  any defect found becomes a defect-shaped follow-on.
- Depends on MOFE01 M-I (the hillslope-total identity must exist to evaluate
  at >10 OFEs).

## Acceptance / exit criteria

- The three conservation identities close at noise on a >10-OFE substrate.
- Legacy's per-OFE-count closure defect is measured and contrasted (openWEPP
  holds where legacy breaks), with truthful evidence labels.
- If openWEPP also breaks at high OFE count, that is a defect-shaped finding,
  not a package failure.

## Dependencies

- MOFE01 (`20260612-mofe01-inter-ofe-routing-closure-001/`) — the routing
  implementation + M-I hillslope-total identity.
- `docs/ROADMAP.md`, `AGENTS.md`, `docs/codex_exec_plans.md`, ADR-0011/0017.
- High-OFE substrate: **H2637 (19 OFEs)**, wepp-forest WB-05A in-repo inputs;
  legacy comparator `wepp_260606_hill`.

## Increments & Findings

- **F-A — Fixture + legacy baseline + Finding 1** (complete; `fixture-and-baseline-evidence.md`).
  Selected H2637 (19 OFEs, in-repo provenance); produced a clean `wepp_260606`
  baseline (both `wepp_ui` variants, 0 non-finite); confirmed the documented
  far-point signatures (QOFE/Q = OFE ordinal; OFE19 q-cap with/without-ui
  contrast). Running openWEPP on the substrate **surfaced a defect**: the
  per-element WB13 conservation gate fail-closes at OFE5 on a frost day
  (residual ≡ `frost.runtime_watbtm_m`), deterministic across both variants.
- **F-B — Frost bottom-overflow (`watbtm`) double-count defect-closure**
  (DC-ExecPlan; `dc-execplan-frost-overflow-double-count.md`). Closes Finding 1
  contract-first: `watbtm` was counted on the per-element inflow adjustment
  **and** the `Dp` outflow; SC-WATBAL-001 amended (v162) to exclude it, code
  corrected, contract-derived regression added. Disposition: `disposition.md`.
- Branch-out: `watpdg` (upper overflow) symmetric inflow/outflow treatment is an
  open question pending a `watpdg>0` fixture (see DC-ExecPlan §2 negative boundary).
