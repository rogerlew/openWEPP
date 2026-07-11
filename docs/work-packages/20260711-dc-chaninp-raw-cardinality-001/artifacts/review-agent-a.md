# Review agent A

Status: pre-implementation PASS; final review pending
Evidence mode: Static and Ran

Initial `GO-WITH-AMENDMENTS` findings: define `INV-CHN-013`; separate direct
legacy ordering from inferred exact-arity/noncollapse policy; narrow generic
truncation/default wording; scope D-CHN-003 for negative counts; populate gate
evidence. All were accepted/fixed. Reviewer verified the 99-ID fixture count,
pinned baseline HEAD, exact red tests, A-H taxonomy, and returned
`PREIMPLEMENTATION PASS` with no blocker.

## FINAL REVIEW — 2026-07-11

Status: **HOLD**

Evidence mode: Static and Ran as labeled below.

### Findings

1. **High — A-FINAL-001: the non-waivable A-H obligation gate is not
   closed.** Static: `artifacts/obligation-to-test-map.md` still labels the map
   `active`, leaves D partial and F partial, uses generic “existing” bindings
   instead of naming every test, and says no partial/red row may remain at
   terminal disposition. Contract family F requires `NaN` and infinities for
   every real token. The current strict vectors cover `dtchr` with `NaN` and
   negative infinity and `cbase` with positive infinity, not all three classes
   for both real fields. ADR-0021 makes 100% obligation-to-test binding
   independent of the percentage gate. Add the missing exact vectors, replace
   every A-H row with named test functions and `PASS`/reviewed `N/A`, then rerun
   focused coverage and CRAP from that final test/source state.

2. **High — A-FINAL-002: the claimed normalized-list consumer handoff is not
   proved by a real production consumer.** Static: production search finds
   `ichnum_norm` only in the parser; `network_frame.rs` consumes
   `nchnum_norm` into `routing_globals.nchnum` but carries or consumes no
   normalized channel-ID list. The WSHED-W5 test asserts `ichnum_norm` on the
   parser object before frame construction, then proves only normalized count
   consumption. That is valid producer/count evidence, but it does not close
   the package and contract language claiming normalized topology/ID-list
   handoff. Continue in-envelope to make the real frame/downstream selection
   consumer read `ichnum_norm`, or retain a truthful HOLD/narrowed claim at the
   demonstrated consumer boundary; do not record end-to-end list-consumer
   closure from the current test.

3. **High — A-FINAL-003: cover-first and final metric provenance do not meet
   the queue's evidence contract.** Static: the pre-decomposition artifact does
   not record the exact corrected-source hash/snapshot or worktree identity, so
   the reported safety net cannot be independently tied to the source that was
   decomposed. It also states that LCOV preceded two additional tests while
   JSON is the final pre-decomposition capture. The final artifact records
   commands and output hashes but not the required source commit/worktree
   identity, timings, or byte sizes. Preserve or reconstruct the exact
   pre-decomposition source identity if possible, explain which single capture
   supplies each binding metric, and record all required provenance for the
   final rerun. If the pre-decomposition state cannot be proven, the cover-first
   gate remains held rather than inferred from current-source results.

4. **Medium — A-FINAL-004: package evidence is internally stale and terminal
   gates remain pending.** Static: `contract-and-provenance.md` still says
   “awaiting pre-implementation review” despite the recorded PASS;
   `obligation-to-test-map.md`, `review-disposition.md`, `gate-results.md`, both
   verification artifacts, final disposition, and handoff are active/queued or
   absent from terminal evidence. Some are expected to follow this review, but
   the package cannot be dispositioned complete until they are current and all
   accepted final findings are fixed and independently reverified.

### Passing evidence

- Static: pinned `wshinp.for` reads the raw implied-DO ID list before clamping
  `nchnum`; pinned `chnrt.for` iterates only the post-clamp count. The contract
  correctly labels exact-arity/fail-closed behavior as an openWEPP inference.
- Static: the parser correction rejects raw `99+2` as exact line-4
  `CHN-E-002` in both modes, preserves a raw-count-closed 99-ID source view,
  derives the first two normalized IDs for `nchan=2`, and retains ordinary
  compatibility default behavior outside the ratified line-4 class.
- Static: the recorded after hashes match the current parser and focused test.
  Raw after evidence supports 93.452% lines, 97.090% regions, an 80% minimum
  named-function region result, and maximum deduplicated CRAP 16.352.
- Ran: `cargo nextest run --test infile_chaninp_parser_contract` passed 35/35.
- Ran: `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`
  passed 19/19.
- Ran: `cargo fmt --check` and `git diff --check` exited zero.
- Ran: pinned baseline HEAD is
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; all six recorded raw coverage/
  CRAP hashes match their files.

### Final disposition

**HOLD.** The core raw-cardinality parser correction is technically coherent
and the focused suites pass, but current evidence cannot close the mandatory
A-H gate, the normalized ID-list consumer claim, or the cover-first/final-run
provenance requirements. These are current-package findings, not deferrable
follow-ups.
