# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-02, hold). Codex ran fmt, the
migrated producer test, the totalwatsed3 consumer test, the retained-frame
projection test, both full suites, and read-only parquet/hash checks. Three
candidates **accepted**:

| # | Finding | Disposition |
|---|---|---|
| B02-CX-001 | `SC-WATBAL-001` INV-098 and `SC-SYSTEM-001` INV-031 still hard-fail downstream `QOFE == Q`, contradicting `INV-RUNOFFPART-032` | **accepted.** Both anti-clone invariants reconciled: the `QOFE == Q where slplen != totlen` rejection is **superseded** (SC-WATBAL rev 100, SC-SYSTEM rev 100) — it was an anti-clone *proxy*, and `QOFE == Q` is now the canonical published convention (already required by the ratified MOFE04 canonicalized policy, SC-WATBAL WB13 items 1/6). Per-OFE genuineness is still enforced by the surviving anti-clone evidence (hydrology-vector distinctness, raw local-runoff distinctness, active surface-handoff, lane-local lineage). |
| B02-CX-002 | Gate evidence scratch-only; reviewer's reconstruction shows 87,791 changed rows, not 53,298 | **accepted + reconciled.** `gate-log.md` committed. Both numbers correct against the same pre-B02 baseline: **53,298** = material (`|Δ|>1e-9`) changes (triply consistent: = rows with pre-B02 `QOFE!=Q` = rows with `runoff>0 on OFE>1`); **87,791** = bit-level changes (adds 34,493 sub-ULP near-zero-runoff rows). Baseline dependency documented (vs pre-DC01 the count is 219,529). |
| B02-CX-003 | `QOFE == Q` producer-computed but not guarded; a retained-frame test passes with `Q != QOFE` | **accepted.** Added `validate_publication_qofe_equals_q` (bit-identical `QOFE == Q`) at the publication boundary — called per-row in the retained validator and on the streamed sample rows. Two retained-frame test fixtures that carried `Q != QOFE` (03_tests.rs:961, :1103) corrected to `QOFE == Q`, and the r6a consumer assertion updated (10.0 → 12.5). The guard now rejects any Q!=QOFE publication row regardless of construction path. |

Post-disposition gates (Ran): clippy `-D warnings` 0; orchestrator 148/148;
runner 101/101; authority guards below.

## Re-check disposition (Codex re-review, 2026-07-02)

Both partial closures completed:

| # | Residual | Action |
|---|---|---|
| CX-001 | Guard-map rows + addendum text still carried the old `QOFE = runoff * efflen / slplen` / reject-`QOFE == Q` language | Fixed **all active text**: `SC-SYSTEM-001` guard-map row (185) + addendum item 5 (§842-845) + a stray `and and` typo; `SC-WATBAL-001` guard-map row (494) + addendum item 7 (§2187). All now state `QOFE == Q` as canonical (MOFEFID-B02 / INV-RUNOFFPART-032) with the retained OFE-local basis feeding only byte-invariant runvol/peak. Revision-history rows (SC-SYSTEM 993, SC-WATBAL 2460) left as historical record. |
| CX-003 | Guard covered retained frames + streamed summary sample rows, but not every production streamed row | Guard moved to `observe_row` — the complete per-streamed-row path — so **every** WAT row is checked before write, not just samples. Verified end-to-end: H2637 streaming run (235,961 rows) exit 0 with the guard active; `H.pass` byte-invariant. |

Post-recheck gates (Ran): clippy `-D warnings` 0; runner 101/101; H2637
streaming endpoint exit 0, `H.pass` byte-invariant. CX-002 was already
closed at re-check (reviewer reproduced the 53,298/87,791 split).
