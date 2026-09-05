# Inherited worktree and runtime inventory

Status: `PHASE 0 COMPLETE`

Evidence mode: `Static + Ran`

## Frozen execution intake

Ran on 2026-09-01:

- `git rev-parse HEAD` ->
  `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`;
- tree -> `4869ab62c6b0d110e5014109132aa676daab7f0e`;
- branch `main` exactly aligned with `origin/main` (`+0/-0`);
- tracked worktree: zero staged and zero unstaged paths;
- untracked: four unrelated files under `tmp/pdfs/r156-review/`:
  `page-20.png`, `page-21.png`, `page-22.png`, and `r156.txt`.

The untracked review scratch is outside this package and must be preserved.

The predecessor's reported 90 modified and 98 untracked entries describe the
pre-checkpoint r151 tree. They are not the live intake manifest. Commit
`a28c55c2d` checkpointed a 257-path superset relative to parent `a6cbc9402`
(100 modified, 157 added, 61,942 insertions, 6,055 deletions). The interval
contains 78 hillslope-orchestrator paths, 14 LSE paths, 13 restart paths, nine
runner paths, one vegetation path, nine integration tests, three changed
canonical contracts, predecessor evidence, and this replacement scaffold.

## Preserved r151 evidence

Ran: `/tmp/wghl_001d_v57_64m_r151.log` still exists and has SHA-256
`d4a26e0194a769c1303cc7500ea254d2a9dbcdaa08e05f65188e4ba07ea27252`,
matching the predecessor disposition. It records the unoptimized test-profile
command, exit 101, wall 309.55 s, user 309.20 s, peak RSS 442,368 KiB, and a
typed failure at exact support `1800..1860 s` inside the frozen
temperature-primary safeguarded solve. It is historical failure evidence, not
release performance or exact-current-source qualification. No day, width
distribution, accepted/rejected count, or final ledger completed.

## Real runtime path

Static call path:

1. `openwepp-cli-hill` builds `HillslopeRunRequest` and selects only
   `DirectProductionExecutor`.
2. Runner `execute_direct_publication_stream` loads the exact Stage 3 owner
   seed and calls the atomic Stage 3 day-preparation/publication stream.
3. The scheduler prepares and commits the Stage 3 candidate through
   `snow_stage3_v11_attachment_runtime`.
4. Covered support enters adaptive Stage 3, the terminal/ordinary covered
   subslab, `v11_vegetation_consumer`, native vegetation V11, and
   `v11_covered/open_snow.rs`.
5. Accepted support history is the sole input to
   `stage3_committed_publication`; WAT5, erosion, water ledger, archive, and
   runner outputs consume those accepted frames.

Native vegetation and ET are present in the V11 occupancy solver, and the
production Stage 3 input binds non-CoE physics with CoE boundary operands set
to zero. The proof fixture must seed the exact frozen-litter V4 owner; otherwise
older LSE variants remain production-reachable.

## Phase-0 consumer defect

The accepted Stage 3 production path does **not** currently execute Lane D.
Runner Lane D modules/configuration are test-only, production rejects every
`OPENWEPP_LANED_*` selector, the successful Stage 3 runner summary sets
`laned_active: None`, and `stage3_committed_publication` never calls the
per-OFE kinematic-wave router. The orchestrator contains a real Lane D branch,
but the production runner cannot select it and Stage 3 committed publication
bypasses it.

This is `THROUGHPUT-001` consumer-path scope, not a legitimate external HOLD:
the package explicitly includes real runner and Lane D adoption. The exact
write set in `package.md` owns the direct correction. Existing shadow tests,
surface-liquid transfer traces, and counter-only evidence cannot carry the
claim.

## Protected paths

- Preserve native V11 vegetation/ET and frozen-litter V4 execution.
- Preserve non-CoE Stage 3 owner/custody and accepted-only publication.
- Preserve Lane D per-OFE topology, ordering, transfers, and ledgers.
- Do not edit wepppy, create a branch, or modify the unrelated `tmp/` files.
