# PERFIDX02 — Independent Review (Claude Code)

Status: APPROVE — the make-or-break clone-economics gate passed rigorously; one
fixture-coverage item to carry into Stage 3.
Evidence mode: **Static** (code/diff) + **Ran** (IndexedSurface tests; inert-path check)

## The gate that mattered — passed, and measured *honestly*

PERFIDX02's whole purpose (ADR-0022 Amendment 1) was to turn the clone-economics
assumption into a measurement before any authority flip. It did:

- **Real scale, not the 6K microbench.** Measured on real H2637 clone-source
  surfaces with **4,087 present entries**: sparse `Vec<(SymbolId, value)>` clone is
  **69.9× / 54.1×** faster than `BTreeMap::clone` (without/with UI). RSS with the
  tightened registry ~99 MB (vs 427 MB at the 1.7M capacity).
- **Self-caught benchmark artifact.** An earlier run showed single-digit-ns sparse
  clones; Codex **rejected it as invalid** (LLVM optimized the clone away) and
  corrected it with black-boxing + higher repeat counts. That self-correction is
  the difference between a real measurement and a rigged one — it's why the 54–70×
  is credible. (The lower figure vs PERFARCH01's 110× is *expected* — real present
  count + elision-proofed bench.)
- **Sparse chosen with sound reasoning** over the faster compact-value candidate
  (complete shadow rep, carries `SymbolId` for direct id-ordered export, no
  local-id contract needed before authority flip). Compact deferred to Stage 3+ as
  an O(1)-lookup option. I ran the IndexedSurface tests: 3/3 (sorted-order
  round-trip, rejects unknown symbol, writeback state+flux round-trip).

GO is justified: the **dominant** cost (the per-OFE-day clone, per PERFHO01) is a
large, real-scale win in the chosen representation.

## Correctness — verified

- **Shadow is inert in production.** `begin_if_requested` returns `None` without
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`; `observe_clone_source_surface` /
  `validate_shadow_surface` run only `with_state_if_active`, so they're no-ops when
  inactive and never mutate the authoritative surface. Bit-identity is by
  construction, matching `ANCHOR_MISMATCHES=0` / `POST_SHADOW_UI_MISMATCHES=0` /
  `DETERMINISM_MISMATCHES=0`. The +42 s (892 s shadow-on vs ~850 s) is validation
  cost *only when the env var is set* — production unaffected.
- **Shadow equality** mismatch_count = 0 on H2637 (both variants) + OFE1–5 — the
  sparse shadow faithfully mirrors the BTreeMap in id order.
- **No authority flip** (Stage 2 boundary kept). Gates green
  (fmt/clippy/test/deny).

## Registry tightening — sound, with a coverage item for Stage 3

The 1.7M→44,746 reduction replaces the worst-case PL-decomp `1..=366` enumeration
with **reachable** bounds: `cutday_*` to `ncut`, grazing roots to `ncycle`,
inferred from the management params (`symbol_registry_audit.rs`). Completeness held
(0 post-freeze unknowns on the cohort), so it's correct for H2637 + the ladder.

**Carry to Stage 3 (not blocking now):** the tightened registry's correctness now
depends on `ncut`/`ncycle` (and the `.unwrap_or(0)` fallback) correctly bounding
the produced sequence indices **for every config** — notably grazing / multi-cut
managements the H2637+ladder cohort may not exercise. A miss is **fail-closed** (a
typed error / crash), not silent corruption — but it would be a production crash on
an untested config. The registry is currently only built in the env-gated
audit/shadow path, so this isn't load-bearing yet; **before Stage 3 makes the
registry production-active (authority flip), validate the reachable-set logic
across diverse managements (grazing, multiple cuts/cycles, irrigation).**

## Verdict

Approve and land. The migration's central economic question — *does a working-set
store keep the clone a win at real scale?* — is answered **yes** (54–70×), which is
the strongest evidence yet that the ≤10× path is real (the clone is the dominant
lever; the *total* still awaits the Stage 6 re-measure after authority flip +
resolve-once). Stage 3 (`PERFIDX03`) may proceed, gated on the registry
fixture-coverage above. The no-independent-dual-review caveat is addressed here.
