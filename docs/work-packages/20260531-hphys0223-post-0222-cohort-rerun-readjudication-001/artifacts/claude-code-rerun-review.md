# HPHYS0223 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: post-HPHYS0222 cohort rerun / readjudication (measurement package).
Reviewed against the over-drainage analysis in the AUTH05 artifact
`claude-code-fc-authority-worked-example.md`.

(Filename note: the package's `claude-code-review-findings.md` is Codex's
cross-reference to the HPHYS0222 review; this file is the reviewer's findings on
HPHYS0223 itself.)

Evidence: **Static** (read artifact/contract) and **Ran** (command executed by
this reviewer; the 39-hillslope rerun itself was produced by the package).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — The rerun is genuine and the reported numbers reproduce

Ran (reviewer re-count over the 39 per-hillslope semantic reports under
`/tmp/hphys0223_20260531T201410Z/parity/reports/semantic/`). Values match
`hphys0223-implementation-and-test-evidence.md` exactly:

| Column | fail | avg mean-abs-diff |
|---|---|---|
| Dp | 39/39 | 0.325 |
| latqcc | 39/39 | 0.752 |
| Total-Soil | 39/39 | 140.709 |
| SoilWaterTotal | 39/39 | 140.709 |
| ProfileFCStore | 27/39 | 2.053 |
| ProfileWPStore | 1/39 | 0.057 |

The package is the measurement HPHYS0222 deferred; it executes 39/39 and
adjudicates honestly (`HOLD`, "residuals unchanged vs HPHYS0221").

## F-2 — Confirms HPHYS0222 was inert on the monitored residuals

Static + Ran. The residual families are unchanged versus HPHYS0221 — the
HPHYS0222 `solwpv` branch fix did not move `Dp`/`latqcc`/`Total-Soil`/
`SoilWaterTotal`. This closes the measurement gap noted in the HPHYS0222 review
(F-3 there) and corroborates that the branch fix is downstream of the dominant
defect (F-2 there).

## F-3 — The remaining residual is concentrated in soil-water storage

Static. The columns separate into two regimes:
- Near-closed: `ProfileWPStore` (0.057), `Dp` (0.325), `latqcc` (0.752) — all
  sub-mm averages, just over the 0.1 mm absolute tolerance. The WB19
  flux-partition work has effectively closed these.
- Dominant: `Total-Soil` and `SoilWaterTotal` at **140.709 mm** (identical →
  shared `wb11_soil_water` lineage). This is the over-drainage and is now the
  single first-order residual remaining.

## F-4 — ProfileFCStore's small parity gap masks a large physics gap

Static. `ProfileFCStore` shows 2.053 mm mean-abs-diff (27/39), ~2% against
legacy. Per the AUTH05 worked example, the H1 model FC (107 mm) is ~half the
−33 kPa physics authority (223 mm), and legacy (~114 mm) shares the error — so
the ~2% legacy-parity figure understates a ~2× physics error, and the FC thread
is the mechanism behind the Total-Soil over-drainage (F-3): a profile that
relaxes to half its physical field capacity stores roughly half the water.

## F-5 — Scope and gates

Static. The package changes no production code (commit `fe8d344` touches no
`crates/`); it is a rerun/readjudication and governance package. Ran (reviewer,
HEAD `fe8d344`): `cargo fmt --check` → exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0; `cargo test
--workspace` → exit 0. Decision: `HOLD`.
