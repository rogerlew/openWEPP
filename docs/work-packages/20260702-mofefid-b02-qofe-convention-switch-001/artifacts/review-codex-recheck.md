# Codex Re-Check - B02 Review Disposition

Review date: 2026-07-02
Reviewed head: `9b2e9e0c` on `worktree-mofefid-b02-qofe`
Scope: re-check of the disposition for `review-codex.md`. No production code,
tests, or contracts were modified.

## Evidence Classes

Static:

- Read `artifacts/review-disposition.md` and `artifacts/gate-log.md`.
- Inspected the `0110de00..9b2e9e0c` diff for `SC-WATBAL-001.md`,
  `SC-SYSTEM-001.md`, `04_direct_publication.rs`, and `03_tests.rs`.
- Searched the affected contracts for surviving pre-B02 `QOFE` rejection text.
- Inspected the streaming publication path and output summary validation.

Ran:

- `cargo test -p openwepp-runner r6a_direct_projection_consumers_read_publication_frame_operands -- --nocapture`
- `cargo test -p openwepp-runner cqr_row7_retained_publication_frame_validator_covers_count_and_identity_guards -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator r7d4_publication_qofe_equals_q_with_independent_runvol_basis -- --nocapture`
- `cargo fmt --check`
- `/home/workdir/openWEPP/.venv/bin/python` read-only scratch WAT parquet count check:
  `material_abs_gt_1e-9 = 53298`, `bit_changed = 87791`,
  `new_qofe_ne_q_bits = 0`.

## Findings

### B02-RC-001 - Accepted Candidate - CX-001 remains partially open in active guard/addendum text

Evidence class: Static.

The main invariant rows were corrected:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:286`
  now states public `QOFE` equals public `Q`.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:148`
  now supersedes the old `QOFE == Q where slplen != totlen` rejection.

However, active guard-map/addendum text still carries the old rule:

- `SC-WATBAL-001.md:494` still defines the guard as proving public
  `QOFE = runoff * efflen / slplen` and still holds when downstream multi-OFE
  `QOFE` aliases cumulative `Q`.
- `SC-WATBAL-001.md:2187-2190` still says M-F-REDO2 acceptance must publish two
  baseline-authoritative public normalizations, with `QOFE` on `slplen`.
- `SC-SYSTEM-001.md:185` still asks for public
  `QOFE = runoff * efflen / slplen` evidence, despite the parenthetical
  superseding only the alias rejection.
- `SC-SYSTEM-001.md:841-844` still says manifests and consumers must reject
  downstream `QOFE == Q` aliases where `slplen != totlen`.

These are not just historical changelog rows; they are active guard/addendum
authority. The typo at `SC-SYSTEM-001.md:148` ("and and") is minor, but the
active stale guard text means the original contract-contradiction finding is
not fully closed.

Disposition recommendation: update the remaining WATBAL/SYSTEM guard-map and
addendum rows to the same post-B02 convention as the corrected invariant rows:
public `QOFE == Q`, retained local-length basis only for `H.pass.runvol`/peak,
and anti-clone proof from hydrology-vector/raw-runoff/transfer lineage.

### B02-RC-002 - Accepted Candidate - CX-003 remains partially open for streamed WAT rows

Evidence class: Static + Ran.

`validate_publication_qofe_equals_q` is a good guard, and it is called for every
row in the test-only retained-frame validator at
`crates/openwepp-runner/src/hillslope/04_direct_publication.rs:785-813`. The
fixtures that previously carried `Q != QOFE` were corrected, and the retained
projection tests pass.

The production streaming path still does not validate every WAT-bound row:

- `04_direct_publication.rs:74-86` observes each row, runs
  `require_direct_publication_output_family_authority_row(row)`, then converts
  and buffers the WAT row. That authority-row guard does not check `Q`/`QOFE`.
- `04_direct_publication.rs:859-879` checks `QOFE == Q` only on the summary
  sample rows (`first_row`, `last_row`, and `hbp_sediment_row`) after streaming.
  A middle streamed WAT row with `Q != QOFE` can still bypass this validation.

This is not a reproduced H2637 output defect; the producer-side B02 test and
scratch H2637 output both show `QOFE == Q`. It is still a publication-boundary
closure gap because `INV-RUNOFFPART-032` says every WB13 row, and the
disposition says no construction path can bypass the guard.

Disposition recommendation: call `validate_publication_qofe_equals_q(row)?` in
`DirectPublicationStreamingSink::observe_row` before `summary.observe(row)` and
before WAT buffering, then add a negative regression with a non-summary row
where `Q != QOFE` to prove the stream fails closed.

## Closed Checks

- B02-CX-002 is closed. The committed `gate-log.md` explains the material vs
  bit-level count distinction, and I reproduced the key numbers from scratch:
  `53,298` material changes, `87,791` bit-level changes, and bitwise
  `QOFE == Q` on all B02 H2637 WAT rows.
- The direct producer implementation remains directionally correct; the
  producer test passed.
- The updated retained-frame fixtures pass, and `cargo fmt --check` passed.

## Re-Check Outcome

Not merge-ready yet. The substantive B02 direction remains correct, and CX-002
is cleanly closed. CX-001 and CX-003 are only partially dispositioned: active
contract guard/addendum text still contradicts B02, and the production streamed
publication path still does not validate `QOFE == Q` on every emitted WAT row.
