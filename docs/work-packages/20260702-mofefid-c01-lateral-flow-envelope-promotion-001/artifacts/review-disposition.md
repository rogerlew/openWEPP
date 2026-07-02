# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-02). Outcome: conditional hold —
one substantive over-claim + two governance items. All **accepted**.

| # | Finding | Disposition | Action |
|---|---|---|---|
| C01-CX-001 | Over-claims H2637 "inside all four tiers" before ENV-T/ENV-E event-tier evidence exists; C01's own scope gives the verdict run to C03 | **accepted (substantive)** | `envelope-derivation.md` and `package.md` corrected: C01 evaluated only the two ANNUAL tiers (ENV-Y in-band Y≈0.67, ENV-ET in-band ET≈863 mm/yr, DC01 the cause of the move); the two EVENT tiers are explicitly deferred to C03 with no in-band claim. The ratified contract text (`INV-SUBHYD-033`) never asserted H2637 in-band, so no contract edit needed — the over-claim was in the package artifacts only. |
| C01-CX-002 | "verification debt" wording ambiguous after contract promotion | **accepted** | Clarified in-place (below): the envelope BANDS are ratified authority; the debt is only that individual anchor NUMBERS are page-cited from agent extraction and C03 re-verifies each against its source before the verdict consumes it. The band derivation does not depend on any single unverified digit (annual anchor triangulated ×4). |
| C01-CX-003 | Missing package-local record of the external-authority guards | **accepted** | Guards run first-hand at disposition (Ran, main checkout): `tools/release/check_authority_suite_antievasion.sh` → `PASS`; `cargo nextest --test auth11_required_suite_obligation_guards_contract` → 2/2. Recorded here and in `guard-log.md`. |

Codex independently ran both guards green; this disposition reproduces that
first-hand. No production code/tests/contracts changed by the disposition
beyond the artifact corrections above.

## CX-002 clarification (canonical wording)

"Verification debt" refers ONLY to the provenance of individual observed
anchor numbers (page-cited from research-agent extraction, not yet
independently re-read digit-by-digit). It does NOT qualify the ratified
status of the envelope: `INV-SUBHYD-033`'s four tiers and their bands are
ratified external authority as of rev 13. C03 re-verifies each cited number
against its source before issuing a magnitude verdict; because the annual
band is triangulated across four independent sources, no band edge depends
on a single unverified digit.
