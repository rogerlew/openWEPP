# ASSURE-04B Terminal Verification B

Status: PASS; no blocker or new finding

Evidence classes: Static and bounded Ran

Verifier B traced every acceptance row to current implementation, tests,
consumer evidence, protected identities, and the terminal heavy bundle. It
independently rebuilt the real CLI and proved named/all human and JSON
equivalence, repeated byte stability, exact reconstruction of all 48 human node
rows from JSON, zero public reports, no absolute root, and no repository write.

The verifier confirmed shared one/all graph construction, dependency-first
ordering, blocked-before-stale precedence, typed graph failures, selection
isolation, modification-time independence, the ASSURE-04C build/check boundary,
and descriptor-confined reads through both the engine and v2 consumer paths.

All 16 canonical CRAP checksums passed. The live tree matched 225 production
and 432 measurement-input hashes; full Nextest recorded 2,001 passes and three
skips; fresh CRAP recorded 2 raw / 2 adjudicated / 0 actionable. Touched maxima
were 17.0053 (`cli.rs`), 16.4319 (`engine.rs`), 26 (`v2.rs`), 7.8930
(`confined.rs`), and 13.169 (`planner.rs`); `lib.rs` had no measurable row.
Protected hashes, `usersum/**`, line counts, scope, and diff checks passed.

Gate Evidence Non-Deferral and real-consumer rules are satisfied. Verifier B
recommends mechanical package closeout after Verifier A PASS.
