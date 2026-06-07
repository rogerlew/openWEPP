# FDMC01 — Frost Depth Model Comparator Characterization

Status: complete

Package type: **Characterization** (validation/characterization shape — NOT a
Defect-Closure ExecPlan; this package lands no production or contract change).

## Objective

Size the frost **depth model** gap that FQ-4 left as a Stage-2 carry-forward: compare
legacy `wepp_260606_hill` energy-balance frost depth and frozen-soil duration against
openWEPP's freeze-index depth proxy on the **frost-active** `algebraic-radium`
substrate (`ksflag=1`, where both engines activate frost), to inform the promotion
decision for the backlog item
[`backlog/20260607-frost-depth-model-heat-flow-parity.md`](../../backlog/20260607-frost-depth-model-heat-flow-parity.md).

Deliverable is a **characterization verdict** (proxy crude-but-close vs materially
off, with magnitude + timing evidence), not a fix. No code, no contract amendment.

## Why this substrate (and not arboreal-dendrite)

A frost-*depth* comparison requires frost to be **active on both sides**:

- **`/wc1/runs/al/algebraic-radium`** — `ksflag=1`, 44 hillslopes (43 single-OFE),
  PRESTON MN gridmet daily; the FQ-4 substrate where openWEPP frost engages and legacy
  frost fires. **This is the substrate.**
- **`/wc1/runs/ar/arboreal-dendrite`** is **out of scope** here: it is a forest-soil
  MOFE run with `ksflag` **off** (no standard frost; it exercises the separate forest
  `ksatadj` path). It is a MOFE rung-3 substrate, not a frost-depth substrate.

Use the single-OFE prefixes (frost is a per-column vertical mechanism); the 17-OFE
MOFE hillslope is out of scope (rung-3).

## What "characterization" means here (boundaries)

- **No production edit.** Do not touch the frost kernel, constants, or the freeze-index
  proxy. This package measures; it does not fix.
- **No contract amendment.** `SC-SNOWFREEZE-001` is not edited here. The promotion
  decision (heat-flow parity vs sanctioning the proxy) belongs to the backlog item.
- **Comparator role.** This is characterization, so legacy `wepp_260606_hill` is
  legitimately the *yardstick for sizing the gap*. This does **not** make legacy an
  acceptance target — per ADR-0017 any eventual fix is judged contract-first. State the
  distinction explicitly in the verdict.
- **Protected boundaries carry over:** snow magnitude (Stage-2), forest `ksatadj`
  (separate concern), MOFE/17-OFE (rung-3). Do not touch ET, runoff, or frost
  activation (all closed).

## Tasks

1. **Locate the legacy frost-depth output surface.** Determine where
   `wepp_260606_hill` exposes frost depth (`frdp`) and frozen-soil state — a frost/winter
   output file, the water-balance output, or a debug surface. If legacy does not emit
   `frdp` in a parseable form, record that as a feasibility finding and propose the
   minimal way to obtain it (this is a checkpoint, not a blocker to declare upfront).
2. **Run both engines** on the `algebraic-radium` single-OFE cohort: openWEPP (frost
   depth `frost.runtime_frdp_m`/`dfrost`, `ws_frz`/`frozwt`, frozen-days) and legacy
   `wepp_260606_hill` (frost depth/duration from task 1).
3. **Compare per hillslope:** max frost depth, frozen-days count, onset and thaw
   timing, and the time series shape (legacy heat-flow vs the proxy). Note where the
   openWEPP 0.20 m cap binds vs legacy depths up to 1.0 m.
4. **Assess downstream materiality (bounded):** because the conductivity bite is
   near-total whenever frost exists, the depth/duration delta maps to how *long* the
   bite is active → how much runoff shifts. Characterize this coupling qualitatively
   (full magnitude judgement is post-MOFE per the roadmap).
5. **Verdict** feeding the backlog promotion: proxy crude-but-close vs materially off,
   with the evidence; recommend target (1) heat-flow parity DC or (2) contract
   amendment sanctioning the proxy.

## Acceptance Criteria

- A `frost-depth-characterization-ledger.md` quantifying, per single-OFE hillslope,
  legacy vs openWEPP frost depth (max, time series), frozen-days duration, and
  onset/thaw timing — or a documented feasibility finding if legacy `frdp` is not
  obtainable, with the minimal-acquisition proposal.
- An explicit characterization **verdict** + promotion recommendation for the backlog
  item, with the comparator-as-sizing-not-target distinction stated.
- **No** production, contract, or test-of-production change landed (characterization
  only); analysis tooling/artifacts only.

## Deliverables

- `artifacts/frost-depth-characterization-ledger.md` (per-hillslope depth/duration/timing
  comparison + materiality note).
- `artifacts/frost-depth-characterization-verdict.md` (crude-but-close vs materially-off
  + promotion recommendation).

## Dependencies

- `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md` (the item this sizes)
- `docs/ROADMAP.md` (Stage-2 queue item 2), `AGENTS.md`, ADR-0011/0017
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  (`INV-SNOWFREEZE-006`/`-012`/`-013`, `GAP-SNOWFREEZE-002`)
- FQ-4 package + `artifacts/` (frost activation evidence, the proxy localization)
- Legacy reference: `/workdir/wepp-forest_260430_baseline/src/frostn.for`,
  `frzng.for`, `frznw.for`, `frsoil.for`
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Substrate: `/wc1/runs/al/algebraic-radium/wepp/runs/` (single-OFE prefixes;
  `ksflag=1`)

## Autonomy

Execute the characterization end-to-end (locate legacy surface, run both engines,
compare, verdict) without asking for direction on intermediate steps. This package
**must not** land a production or contract change — if a fix appears warranted, that is
the backlog item's promotion decision, not this package's scope. Ask only if the legacy
frost-depth surface cannot be obtained in the environment.
