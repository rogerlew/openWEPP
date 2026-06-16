# Disposition — FARPOINT01 F-B (frost bottom-overflow `watbtm` double-count)

Status: **CLOSED 2026-06-16 — landed contract-first correction** (DC-ExecPlan
terminal state 1). Defect `FARPOINT01-WB13-FROST-OVERFLOW-DOUBLECOUNT` removed.

Evidence mode: **Ran** (all gates + H2637 both variants) + **Static** (provenance,
contract reconciliation). Authored + executed by Claude Code under explicit
operator direction to run this package end-to-end.

## Outcome

The per-element WB13 conservation gate fail-closed on the H2637 19-OFE substrate
(OFE5, 1996-02-06, residual ≡ `frost.runtime_watbtm_m` = 8.231 mm) because the
named internal frost adjustment double-counted the frost **lower** overflow
`watbtm`: it was added on the per-element **inflow** side *and* counted as a
storage **outflow** in `Dp`. Per SC-SNOWFREEZE-001, `watbtm` is owned by the
`Dp` deep-percolation outflow lineage; the inflow-side count was the defect.
This was a bug in the conservation **check's** accounting — the simulation
already conserved (`watbtm` exits via `Dp`); the gate raised a false positive.

## What landed

| Surface | Change |
|---|---|
| `SC-WATBAL-001` (v161→**v162**) | M-E4-REDO item 2: removed `watbtm` from the named internal frost adjustment formula (`… + watpdg`); clarified overflows are egress not retained storage; cited SC-SNOWFREEZE-001 `watbtm→Dp` ownership; change-log + registry `last_reviewed` synced. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs:432` | Dropped `+ watbtm` from `internal_wb13_frost_internal_adjustment_m`. `watbtm` remains read + bounds-checked. Commented with contract cite. |
| same file (inline `#[cfg(test)]`) | `farpoint01_internal_frost_adjustment_excludes_watbtm_lower_overflow` — contract-derived regression. |

## Validation (Ran)

- **Regression RED→GREEN**: pre-fix the adjustment returned `0.0169067` (the
  double-counted value); the test asserts `0.0086756` (watbtm excluded) →
  FAILED before, `ok` after. Anti-tautology: the wrong (`+watbtm`) formula is a
  strictly different value.
- **AGENTS.md gates** — `cargo fmt --check` clean; `cargo clippy --workspace
  --all-targets -D warnings` clean; `cargo test --workspace` exit 0 (176 suites
  `ok`, 0 `test result: FAILED`); `cargo deny check` advisories/bans/licenses/
  sources ok.
- **No regression** — MOFE01 lib suite 14/14; FDHP01 C1b `Dp`-includes-`watbtm`
  publication tests 2/2 (outflow ownership preserved). The fix is a no-op where
  `watbtm≈0` (the arboreal-dendrite ladder), so MOFE01 closure is preserved by
  construction.
- **Behavior-level acceptance (the FARPOINT01 demonstration)** — `openwepp-cli-hill`
  on H2637, **both `wepp_ui` variants**, with the fix:

  | variant | exit | elapsed | wat rows | max OFE | fail-closed |
  |---|---|---|---|---|---|
  | without_ui | 0 | 1016 s | 235,961 | 19 | none |
  | with_ui    | 0 | 1035 s | 235,961 | 19 | none |

  The per-element + hillslope-total identities are **hard fail-closed gates**, so
  exit 0 across 235,961 rows × 19 OFEs × 34 years (1987–2020) is closure proof
  for the full substrate. Before the fix both variants aborted at OFE5/day-3324.
  This is the differentiating FARPOINT01 result: **openWEPP's three-identity
  closure holds at 19 OFEs, exceeding the legacy ≤10-OFE ceiling.**

## Seven-gate bar — all satisfied

Reproduction ✓ · Mechanism ✓ · Ownership ✓ · Authority (SC-SNOWFREEZE-001) ✓ ·
Safety (gate corrected, not loosened) ✓ · Testability (RED→GREEN) ✓ ·
Validation (fail-closed → completes; residual → <1e-11) ✓.

## Branch-out resolved empirically

`watpdg` (upper overflow) sits symmetrically in the same formula. The negative
boundary held: it was **not** modified on speculation. The H2637 re-run (both
variants, full 34 years) produced **no `watpdg` failure**, so the `watbtm`-only
fix was sufficient and `watpdg` remains a documented follow-on (needs a
`watpdg>0` fixture to reproduce, if it ever does) — see `worker-handoff.md`.

## Independent / dual review note

This package was run solo by Claude Code at operator direction. The contract
amendment, the line-432 correction, and the test are open for the normal
independent (Codex) review obligation; nothing here is gated on that review for
the closure claim, which rests on the four green AGENTS.md gates + the H2637
behavior-level acceptance above.
