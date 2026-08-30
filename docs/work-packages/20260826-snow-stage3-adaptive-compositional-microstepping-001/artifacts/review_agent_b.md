# Review B — ownership, hydrology, and restart

Status: **HOLD**

Evidence mode: `Static + Ran`.

Snapshot reviewed: commit `2a9ca2d845bb4f128441ab01f79b341033a31c7d`
with a dirty shared worktree. At inspection time `git status --porcelain` reported
196 tracked changes and 86 untracked paths. This review is read-only except for
this artifact and does not treat the tracked-only Git diff as the complete
terminal diff.

## Scope and static evidence

- Complete-owner comparison and accepted receipt authority:
  `canonical_owner_bytes.rs`, `snow_stage3_v11_attachment_receipts.rs`, and
  `snow_stage3_v11_adaptive_execution.rs`.
- Terminal liquid custody, hydrology topology, and rollback:
  `snow_stage3_v11_terminal_boundary_receiver.rs`,
  `direct_runtime/surface_liquid_zero_duration_snow.rs`, and
  `direct_runtime/stage3_committed_publication.rs`.
- Restart V3 archive/prefix/custody/publication authority:
  `snow_stage3_v11_restart.rs` and the persisted-restart V2/V3 projections and
  poison fixtures.
- Canonical authority and package closure artifacts, including the exact
  60-second owner amendment, current gate ledger, disposition, line-count
  governance, owned-file manifest, and terminal-diff reconciliation.

Static findings on the physical path:

1. The WB14 cross-factorization exception is narrow. Only the WB14 child
   ordinal and digest-keyed per-OFE receipt-map ordering are classified as
   cross-path `ReceiptLineage`; stable non-WB14 receipt membership/order remains
   a compared discrete surface. Both direct and composed trials still seal
   their complete exact discrete digest and surfaces into the comparison
   receipt, and the selected composed path retains its exact child receipts.
2. Accepted positive-support liquid is derived from accepted support outputs,
   not a trial projection. The receiver binds the beginning clock owner set,
   output-set digest, exact first-hop mass and enthalpy bits, full surface/LSE
   beginning and ending states, typed receipt-chain set, event ordinal/context,
   and ending owner set. The supplemental V3 custody validator reconstructs
   these operands independently and rejects missing, duplicate, route,
   disposition, LSE, and closure substitutions.
3. Zero-duration routing constructs a candidate state, requires the complete
   configured destination-tile partition, checks exact fractions and capacity,
   admits only forward OFE routes, applies the OFE-area basis conversion, and
   rejects incomplete/unresolved topology before returning the candidate.
4. Adaptive direct/composed trials and the outer parent execution operate on
   cloned owners. Failure injections after ledgers, receipts, subslab events,
   terminal receiver, and final owner join return before the candidate is
   installed, preserving rollback. Terminal receiver internals may advance
   candidate clock/parent objects before their last check, but those objects
   remain transaction-local to this outer isolation boundary.
5. V3 archive admission validates the complete manifest count/root/order,
   content digest and length, owner/clock/sequence continuity, resident-tail
   bound, custody supplement, publication prefix, and canonical re-projection.
   V3 uses the dedicated active-publication-tail authority rather than the V2
   full-history restore, and the nonzero archive fixture rejects an empty
   publication-rotation substitution. V1/V2 wire versions remain additive and
   unchanged.

## Findings

### B-01 — BLOCKER: required V3 restore ordering is not implemented literally

`DirectSnowStage3V11AttachmentRestartV3::restore` validates external archive
evidence, but then calls `restore_active_base_v3`, which reconstructs the active
receipt state and calls
`restart_authority_restore_accepted_publication_active_tail_canonical_bytes_v3`.
Only afterward does it install the archived prefix, support-liquid custody
supplement, and publication-rotation V3 payload. This is fail-before-return and
the final joins are strong, but it does not satisfy the package direction that
the archived prefix and publication-rotation V3 authority be installed and
validated before any active-tail reconstruction. Reorder the candidate build,
or obtain and record an explicit authority disposition that the current
validate-first/final-join sequence is equivalent to that requirement.

### B-02 — BLOCKER: no reconciled exact terminal diff or owned-file manifest

`terminal-diff-reconciliation.md` and `owned-file-manifest.md` are still
`queued`/`not-run`, while the shared tree includes both tracked and untracked
production files. Therefore the package has not shown that every terminal file
is in scope, reviewed, or covered by the selected gates.

### B-03 — BLOCKER: exact-current critical regression has no completed PASS

The gate ledger's only completed full-workspace critical row is the historical
pre-repair failure (`8ec6202e-fafa-454a-8fc9-f9f2e621d149`: 3,465 passed, 107
failed, 10 timed out). A replacement exact-current full-workspace run was still
compiling/running during this review and had no result. Focused restart and
custody passes are useful but cannot replace the package's critical terminal
gate.

### B-04 — BLOCKER: line-count closure is false on the exact tree

The package line-count artifact is still queued and reports only the split
persisted-restart files. Direct counting found the touched, non-generated
`crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs` at 3,012
lines. Repository governance requires refactor before closure for a 3,000+
nonexempt Rust file; no approved exception, owner, or sunset plan is recorded.
Multiple other touched Rust files are above the 2,000-line WARN threshold and
also need the promised rationale/split disposition.

### B-05 — BLOCKER: package topology checkpoint and closure artifacts remain open

`package.md` still marks Checkpoint D (joint multi-lane execution/event groups)
incomplete and Checkpoints E/F incomplete. The 2026-08-28 performance amendment
pauses seasonal/archive optimization, but does not relax joint-lane topology,
custody, review, verification, terminal reconciliation, or atomic cutover.
All other independent review and both terminal-verification artifacts were
still queued at inspection time.

### B-06 — BLOCKER: evidence artifacts contradict the amended terminal record

`gate-results.md` and `disposition.md` record the owner-amended canonical
one-day PASS, whereas `owner-amendment-60-second-floor.md` still says full
qualification is blocked and the latest one-day run failed at unresolved
liquid. Historical failures may remain, but current/superseded status must be
reconciled so the owner amendment has one unambiguous terminal disposition.

No additional source-level custody, accepted-receipt, terminal receiver,
rollback, or topology defect was identified in the reviewed paths.

## Ran evidence

- `nix develop --command cargo check -p openwepp-hillslope-orchestrator --lib`
  — PASS on the captured review snapshot (2.66 s).
- Scoped `git diff --check` over the package and reviewed owner/restart files —
  PASS.
- Exact changed/untracked Rust line-count scan — FAIL for closure:
  `v11_covered/open_snow.rs` is 3,012 lines; the scan also found numerous
  2,000–2,999-line touched files requiring WARN disposition.
- Read-only inspection of
  `/tmp/adaptive_microstep_amendment/full-workspace-final.log` and live
  processes — replacement full-workspace nextest was in build/run state, with
  no terminal result available. It is not counted as PASS evidence here.

Focused tests were not launched by this reviewer because the exact-current
full-workspace run and other canonical gates already occupied the shared cargo
target. The package ledger's focused PASS rows were inspected as retained
evidence, not relabeled as reviewer-run evidence.

## Terminal disposition

**HOLD.** The reviewed physical custody and accepted-path implementation is
substantively fail-closed, but GO is not truthful until B-01 through B-06 are
closed on one frozen exact tree and the resulting terminal artifacts and
critical gates are reconciled.
