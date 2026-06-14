# MOFE-FARPOINT01 — >10-OFE Routing-Closure Demonstration

Status: queued (follow-on from MOFE01 M-H closure; operator-directed 2026-06-13)

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
- High-OFE substrate (to be identified).
