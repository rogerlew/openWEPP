# LANED-T3-AGG — Review disposition

Status: **EXECUTED** (2026-07-07). Both Codex lanes returned
**GO-WITH-AMENDMENTS** with **no High findings**
(`review-codex.md` / `review-qa.md`). Every finding is ACCEPTED; all but one
are FIXED in this disposition (the one exception is a deliberate delegation,
below). Evidence mode: **Ran** for all fixes and re-gates (commands + results
in the updated `gate-results.md`).

## Code lane

**C-M1 (Medium) — hybrid API does not locally enforce hour-aligned bins.
ACCEPTED, FIXED.** The reviewer's failure scenario is genuine: the mask's
bin-start source sample is exact only when no bin can straddle an hourly
source transition, and only the active runtime's 900 s cadence guaranteed
that. Fix: `route_single_ofe_hybrid` now FAILS CLOSED
(`DegenerateConfiguration`) unless `sample_dt_s` partitions
`SEAM_SECONDS_PER_HOUR` exactly (cascade.rs, guard placed with the other
window-shape checks). Retained vector:
`hybrid_rejects_cadence_that_does_not_partition_the_seam_hour` — the exact
reviewer scenario (`sample_dt = 1000`, window `4000`, source turning on in
hour 1) now rejects. The guard is pass-through at the production cadence:
the post-fix H2637 aggressive rerun is BIT-IDENTICAL (parquet `a5fb9233…`,
outlet `371322.66377028974`). Contract: the rule is named in the rev-30
changelog and the new Branch-and-Guard selector row.

**C-L1 (Low) — all-dry sub-noise carry drop undocumented. ACCEPTED, FIXED
(document + pin, not guard).** Disposition rationale: failing closed on a
`<= 1e-21 m²` attribution sliver on an all-zero series would make near-zero
days fragile for exactly the class this package un-blocked; the drop is
bounded by the declared floor and touches no mass ledger. Fix: the
`dispose_terminal_carry` doc now states the bounded all-dry drop explicitly;
retained vector `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`
pins it (bins untouched, never published negative); the rev-30 changelog
sub-noise sentence is corrected to the precise semantics (backward
absorption from trailing positive bins; bounded all-dry drop).

Adversarial-question dispositions 1-8: concurred as written; no further
action beyond C-M1/C-L1. The reviewer's confirmation that the over-counting
variant has exactly one non-test consumer and that downstream
handoff/erosion guards remain intact matches the execution-time audit.

## QA lane

**QA-M1 (Medium) — gate commands not in canonical closure form. ACCEPTED,
FIXED (rerun, not reconciled-by-argument).** Both canonical forms were
re-executed on the post-fix tree: `cargo clippy --workspace --all-targets
-- -D warnings` → PASS; `cargo nextest run --workspace --profile full` →
`1426/1426 passed (4 slow), 4 skipped` (verbatim in `gate-results.md`,
which now marks the pre-review non-canonical runs as superseded).

**QA-M2 (Medium) — rev-30 authority concentrated in the changelog.
ACCEPTED, FIXED.** Promoted into the normative surfaces, preserving the
EXPERIMENTAL/UNRATIFIED posture in each: (a) a Branch-and-Guard row for the
hybrid selector (revs 28-30: aggressive mask, hour-partition fail-closed,
cross-span deficit carry, material-deficit fail-closed, sub-noise
disposition, composition-scoped solver variant); (b) a Test-Vector
Obligations row for the rev-30 deficit-carry vector family; (c) a Binding
Exposure Index row (`OFEROUTE-HYBRID-IMPLICIT-STEPPING`,
`unpromoted-binding`, routed to `science-review-follow-on` with the parent
package's ratification gates named as the promotion preconditions).

**QA-M3 (Medium) — stale-binary caution should be durable. ACCEPTED,
PARTIALLY FIXED + DELEGATED.** Fixed: the rule (build with
`-p openwepp-runner --bins`, verify mtime/hash; plus the
`p2637.run.toml` hardcoded-output-path trap) is promoted into the H2637
timing recipe (`20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md`
Environment section) — the artifact every timing worker is pointed at.
DELEGATED: adding the rule to `docs/work-packages/AGENTS.md` /
`crates/AGENTS.md` is deliberately left to Codex (AGENTS.md maintenance is
Codex-owned per the role boundary in `CLAUDE.md`); this is a standing
requested amendment, not a dropped one.

**QA-L1 (Low) — stale strict-rule comment on `hybrid_implicit`. ACCEPTED,
FIXED.** The `DirectLanedActiveConfig.hybrid_implicit` doc now states the
rev-30 aggressive rule and the deficit carry (production-code comment edit
under this package's operator-granted end-to-end authorization).

**QA-L2 (Low) — line-count governance not recorded. ACCEPTED, FIXED.**
Recorded in `gate-results.md`: `cascade.rs` 1247 / `kinematic_wave.rs` 1858
/ `laned_active.rs` 892 — all below the 2000-line WARN; `kinematic_wave.rs`
is approaching it and is noted for the next split pass.

## Post-fix verification (Ran)

- Focused: 6/6 `rev30_deficit_carry_tests` (incl. the two new pins),
  2/2 `hybrid_rejects*`.
- `cargo fmt --check` PASS; clippy `-D warnings` PASS; canonical full suite
  `1426/1426`.
- H2637 on the rebuilt, hash-verified binary: plain parquet `21c54bf2…`
  unchanged; aggressive parquet `a5fb9233…` and books bit-identical to the
  pre-fix record (the review fixes alter no behavior on any previously
  defined input — the C-M1 guard only rejects previously-undefined
  cadences).

## Standing items after this disposition

1. Codex-owned: promote the evidence-build rule into AGENTS.md (QA-M3
   remainder).
2. Parent-package gates unchanged: Case-4 hybrid oracle ladder +
   fidelity-tolerance ratification (rev 28/30 remain EXPERIMENTAL).
3. The new Tier-3 lever: implicit solve-cost reduction (backlog note).
