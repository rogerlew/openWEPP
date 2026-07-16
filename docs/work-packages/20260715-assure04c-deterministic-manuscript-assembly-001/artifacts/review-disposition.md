# ASSURE-04C Review Disposition

Status: PASS — all findings accepted, remediated, and independently confirmed

Evidence class: Static disposition with cited ran evidence

| Finding | Disposition | Remediation |
| --- | --- | --- |
| A01/B01/B03 protected creation and symlink/TOCTOU staging | accepted | Authorize the lexical location before creation, hold a no-follow staging-root descriptor, and perform creation/read/write/enumeration/removal/rename through descriptor-relative operations only. Protected nonexistent paths and symlinked components have negative tests. |
| A02/B02 failed-build prior-byte mutation and hidden restoration errors | accepted | Revalidate sources after all temporary output is ready; snapshot prior selected bytes; restore from the snapshot after post-install failure; surface cleanup and restoration errors. Stale input, special temporary path, and pre-existing backup tests prove prior selected bytes remain unchanged. |
| A03/B05 prose migration exceeded mechanical authority | accepted | Removed the added audience and v2-framing prose, restored the ASSURE-04A source identity, initial storage depth, and H2637 allowance semantics, and limited remaining changes to typed substitutions, required version/revision mechanics, figures, and portable links. |
| A04 authored unit suffixes could drift | accepted | Replaced `value` with a single `quantity` directive that emits the retained value and its typed unit symbol together. Inline scientific values no longer carry separately authored unit suffixes. |
| A05 manifest metadata could inject Markdown/links | accepted | Escape Markdown/raw-HTML metacharacters, reject control text and external-link-like metadata, constrain immutable identities, encode DOI URL parentheses, and cover safe HTML rendering plus rejected external-link metadata. |
| A06 actual usersum renderer unproven | accepted | Rendered the retained manuscript, supplement, and injection probe with the exact `cmarkgfm` function imported by the WEPPcloud usersum route; see `usersum-renderer-proof.md`. |
| B04 incomplete quantitative lineage | accepted | Added typed retained result values/bindings for storage depth, recurrence interval, guard coefficients, declared path count, and passing-test count. Replaced the H2637 numeric alternative with qualitative text backed by its visible generated value table. |
| B06 science-contract source not portable | accepted | Staged the digest-bound public-safe `SC-GWBASEFLOW-001.md` research object and linked it from the report and supplement. |
| B07 usersum narrative version mechanics absent | accepted | Added matching `Version 0.1 — 2026-07-15` lines, audience lines, and final revision logs to both narrative documents. |
| A suggestion: incorrect two-day figure caption | accepted | The result-bearing figure now says it compares the maximum implementation residual with the coded analytical-vector allowance; the analytical table retains its own value-comparison caption. |
| A suggestion: stale line-count record | accepted | Renewed `line-count-governance.md`; every touched production Rust file remains below 3,000 lines. |
| A-R1/B-R3 ambient link validation and staging-root identity | accepted | Local-link resolution is now lexical and descriptor-relative; no generated link calls `canonicalize` or ambient `is_file`. Accepted completion reopens the requested pathname no-follow and compares device/inode identity with the held capability. |
| A-R2 preparation cleanup, typed recovery, and rollback evidence | accepted | Current-report temporary/restore paths are cleaned when preparation fails. Recovery failures use a dedicated nested `AssuranceError::Recovery` variant rather than flattening to `Invalid`. A concurrent post-install source-drift integration test proves failure and byte-for-byte restoration. |
| A-R3 remaining prose breadth | accepted | Restored the base H2637 result paragraph structure and reduced reproducibility changes to typed portable links and the package-authorized consumer cross-reference. |
| A-R4/B-R1 raw quantities, numeric row labels, and autolinks | accepted | Literal admission rejects any numeric token followed by a declared unit and rejects Markdown, bare URL, `www`, and email autolinks. Tests prove a refreshed `999.0 m3` claim and bare URL fail. Day rows use nonnumeric ordinal labels; all numeric table cells remain bindings. |
| A-R5/B-R4 audience/version conventions | accepted | Both narratives now use `Version 0.1`, a mandatory audience line, and matching final `0.1` revision-log rows. |
| B-R2 post-install source drift | accepted | Inputs are reverified after installation and immediately before accepted completion. Any drift restores the prior selected subtree from its snapshot; the concurrent marker-based regression test exercises the post-install path. |
| B-R5 stale retained-consumer evidence | accepted | Rebuilt and rechecked ten outputs; renewed the exact manuscript/supplement hashes and actual-renderer byte counts. |
| A-R6/B-R6 `error.rs` omitted from write set | accepted | Amended the package write set, recorded that the review-remediation edit preceded detection of the omission, and added the file to line-count governance. |
| A-R7/B-R7 focused-count artifact stale in reviewer snapshot | accepted | The parent artifact already records 9/9 assembly and 31/31 aggregate focused tests; retained the current parent run ID and requested final reviewer confirmation. |

No finding was rejected, deferred, or assigned to follow-up. After two
technical renewal rounds and one governance-only confirmation, both independent
reviewers returned PASS with no remaining Phase 4 blocker.
