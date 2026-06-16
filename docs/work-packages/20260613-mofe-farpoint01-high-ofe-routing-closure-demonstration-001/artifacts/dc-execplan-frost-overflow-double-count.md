# DC-ExecPlan — Frost bottom-overflow (`watbtm`) double-count in the per-OFE WB13 internal frost adjustment

> A Defect-Closure ExecPlan ([authoring guide](../../../defect_closure_execplans.md),
> [ADR-0018](../../../decisions/0018-defect-closure-execplans-conversion-rule.md)).
> Increment **F-B** of FARPOINT01. Authored + executed by Claude Code under explicit
> operator direction (2026-06-16) to run this package end-to-end.

Status: **CLOSED — landed contract-first correction** (terminal state 1)
Evidence mode: Ran + Static

## 1. Objective

Close the per-element WB13 storage-identity fail-closed observed on the H2637
19-OFE substrate: the conservation gate rejects a **correctly-conserving** run
because the identity's *inflow* accounting double-counts the frost fine-layer
bottom-overflow term `watbtm`, which is simultaneously (and correctly) counted as
a storage *outflow* in `Dp`.

This is a defect in the conservation **check's** accounting, not in physics: the
simulation conserves (`watbtm` correctly exits via deep percolation); the gate
raised a false positive.

## 2. Correction Authority Envelope

- **Defect ID `FARPOINT01-WB13-FROST-OVERFLOW-DOUBLECOUNT`.** Observed failure:
  `HS-SIMPIPE-E-001 per-element storage identity residual 8.231171… mm exceeds
  tolerance 1e-11; ofe=5 [sim_day 3324, 1996-02-06]`, reproduced byte-identically
  on H2637 `with_ui` and `without_ui` (see `fixture-and-baseline-evidence.md`).
  `residual ≡ frost.runtime_watbtm_m` (diff 4.6e-8 = print precision).
- **In-scope write-set:**
  - Contract: `SC-WATBAL-001.md` — the MOFE01 M-E4-REDO "named internal frost
    adjustment" formula (item 2, the `… + watpdg + watbtm` text) and its
    `INV-WATBAL-096` reference, only as needed to remove `watbtm`.
  - Source: `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
    — `internal_wb13_frost_internal_adjustment_m` (the `+ watbtm` term, line ~432).
  - Tests: an inline `#[cfg(test)]` regression in that file; identity-level
    coverage in `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs`.
- **Allowed edit class:** remove the frost **bottom**-overflow term from the
  inflow-side internal frost adjustment so `watbtm` is counted once (outflow `Dp`
  only). No physics, no published-flux, no fail-closed-guard loosening.
- **Acceptance:** (a) the contract-derived regression fails before / passes after;
  (b) `openwepp-cli-hill` on H2637 (both variants) runs to completion with the
  per-element + hillslope-total identities closed; (c) MOFE01 arboreal-dendrite
  closure preserved (regression suite green; `watbtm≈0` there so the change is a
  no-op on that substrate); (d) all AGENTS.md validation gates green.
- **Negative boundary / branch-out — frost *upper*-overflow `watpdg`.** `watpdg`
  sits symmetrically in the same formula and is also present in the identity's
  outflow (`frost_upper_overflow_mm`), so it *may* be double-counted too — but it
  is `0` on the only reproduced failing day, so it is **unverified**. Per gate 1
  (reproduction) this package does **not** modify `watpdg` handling on
  speculation. If a `watpdg>0` day reproduces the same signature during the H2637
  re-run (or later), it is the same envelope (shared authority/write-set/
  validation) and is closed here; otherwise it routes to a named follow-on with a
  `watpdg>0` fixture as its reproduction. Physical distinction kept open: `watbtm`
  is a *terminal downward* outflow (→ deep perc); `watpdg` is *upward → surface*
  and may be recycled within the OFE, which could make its inflow-side treatment
  legitimate.

## 3. Conversion rule (restated)

If a reproducible root cause is established inside this envelope and the corrected
behavior is supported by canonical `SC-*` authority, the package **must** proceed
through contract amendment → contract-derived tests → pre-implementation gate →
production correction → validation → disposition. It may not close as HOLD for
"further investigation possible."

## 4. The seven-gate bar

1. **Reproduction** — ✅ H2637 both variants, byte-identical residual ≡ `watbtm`.
2. **Mechanism** — ✅ `watbtm` counted on inflow (`internal_wb13_frost_internal_adjustment_m`,
   `… + watpdg + watbtm`) **and** outflow (`Dp = D + watbtm`); a named double-count,
   not a variable to inspect.
3. **Ownership** — ✅ both surfaces (contract item 2 + line 432) are in-write-set.
4. **Authority** — ✅ SC-SNOWFREEZE-001:872–886 authoritatively routes `watbtm`
   into WB13 `Dp` (deep-perc outflow; FDHP01 C1b, contract-tested
   `fdhp01_c1b_wb13_dp_publication_includes_frost_bottom_overflow`). A storage
   outflow may not also be an inflow adjustment. SC-WATBAL-001 item 2 is the
   inconsistent text and is the one amended.
5. **Safety** — ✅ the gate is **corrected**, not loosened: the identity still
   fail-closes on real imbalance; the only change removes a spurious inflow term.
   No physics, no published `Dp`/storage change, no clamp.
6. **Testability** — ✅ inline unit regression on `internal_wb13_frost_internal_adjustment_m`
   (asserts result excludes `watbtm`); identity test with anti-tautology
   assertion (including `watbtm` reopens residual ≈ `watbtm`).
7. **Validation** — ✅ before/after measurable: H2637 fail-closed → completes;
   residual at OFE5/day-3324 8.231 mm → < 1e-11.

Anti-tautology (gates 6/7, conservation): the regression makes the wrong formula
(`+ watbtm`) produce a *different* value (residual ≈ `watbtm`) than the corrected
one (≈ 0); validation reconstructs the identity from independent operands (RM,
UpStrmQ/SubRIn, the published `Dp` that owns `watbtm`, and `Δ(Total-Soil+frozwt)`),
not by restating the producer formula.

## 5. Milestones

- **M1 Reproduce** — ✅ (F-A). H2637 fail-closed, residual ≡ `watbtm`.
- **M2 Localize to mechanism** — ✅ arithmetic + git provenance + 4-way contract
  reconciliation (see Decision Log).
- **M3 Amend contract** — SC-WATBAL-001 item 2: drop `watbtm` from the named
  internal frost adjustment; cross-reference SC-SNOWFREEZE-001 `watbtm→Dp`
  ownership; change-log entry.
- **M4 Contract-derived tests (RED)** — add regression; confirm it fails on
  current code.
- **M5 Pre-implementation gate** — record contract amended + tests red.
- **M6 Production correction** — remove `+ watbtm` at line ~432.
- **M7 Validate** — AGENTS.md gates; H2637 re-run both variants; MOFE01 suite.
- **M8 Disposition** — terminal state + defect-shaped handoff (`disposition.md`,
  `worker-handoff.md`).

## 6. HOLD-legitimacy conditions

Legitimate stop only if: (a) the H2637 re-run reveals the corrected identity is
still wrong by a mechanism outside this envelope (different process family), or
(b) `watpdg` reproduces and its correct disposition requires SC authority this
envelope does not hold. Neither "inspect further" nor "defer the watpdg fix as a
breadcrumb" is legitimate.

## 7. Decision Log

- **2026-06-16 — `watbtm` is the term, inflow side is the error.** Four-way
  convergence: (i) arithmetic — dropping `watbtm` from the inflow adjustment (keeping
  it in `Dp`) closes OFE5/day-3324 to −2e-8 mm; dropping from both sides does not
  close (it cancels), so the inflow term is the spurious one. (ii) Provenance —
  `watbtm→Dp` landed in FDHP01 C1b (`1ee1e171`, 2026-06-11, contract-tested);
  `watbtm` was added to the inflow adjustment **later** in MOFE01 M-F-REDO-CLONE
  (`6f220bc8`, 2026-06-13). (iii) Physics — `watbtm` = fine-layer bottom overflow
  → leaves downward as deep perc; the storage reconciliation and `Dp` publication
  both treat it as outflow; only line 432 treats it as inflow. (iv) Contracts —
  SC-SNOWFREEZE-001 (`watbtm→Dp`) vs SC-WATBAL-001 item 2 (`…+watbtm` inflow) are
  inconsistent; SC-SNOWFREEZE is the physically-correct, contract-tested authority.
- **2026-06-16 — `watpdg` excluded from this fix.** Unreproduced (`watpdg=0` on
  the failing day); see negative boundary (§2). Handled empirically by the H2637
  re-run.

## 8. Progress

- **M1 Reproduce** ✅ — H2637 both variants fail-closed at OFE5/day-3324,
  residual ≡ `watbtm` (F-A).
- **M2 Localize** ✅ — arithmetic + git provenance + contract reconciliation
  (Decision Log).
- **M3 Amend contract** ✅ — SC-WATBAL-001 v161→v162; `watbtm` removed from the
  M-E4-REDO internal frost adjustment formula; change-log + registry synced.
- **M4 Tests (RED)** ✅ — `farpoint01_internal_frost_adjustment_excludes_watbtm_lower_overflow`
  fails pre-fix (returns `0.0169067`, expects `0.0086756`).
- **M5 Pre-impl gate** ✅ — contract amended + test red recorded.
- **M6 Correction** ✅ — `per_ofe_internal_wb13.rs:432` drops `+ watbtm`.
  Regression GREEN.
- **M7 Validate** ✅ — fmt/clippy/`test --workspace`/deny all green; MOFE01 14/14;
  FDHP01 C1b 2/2; H2637 both variants exit 0, 235,961 wat rows × 19 OFEs × 34 yr,
  no fail-closed. `watpdg` did not reproduce.
- **M8 Disposition** ✅ — terminal state 1; `disposition.md`, `worker-handoff.md`.

Terminal state: **1 — landed contract-first correction validated to remove the
defect.** See `disposition.md`.
