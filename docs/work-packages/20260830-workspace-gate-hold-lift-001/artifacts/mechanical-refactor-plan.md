# Mechanical refactor plan

Status: `AUTHORIZED — PUBLICATION SPLIT COMPLETE; VEGETATION RECONCILIATION PENDING`

Evidence mode: `Static`

Refactor seam: move only the accepted-segment replay loop from
`V11ParentTransaction::restore_with_bgc_scope` into one private helper with a
private replay-state carrier. Keep schema/finalized guards, beginning/staged
validation, complete-owner validation, replay-state initialization, trailing
zero-duration transitions, cumulative reconstruction, terminal equality, and
final construction in their present order in the public method.

Preserved public surface: every exported type, function signature, visibility,
error variant, checkpoint wire, and owner envelope. The helper/state are
private. Preserve sequential floating-point accumulation, BTreeMap iteration,
mutation-set ordering, custody/predecessor checks, and error precedence.

Test-only seams: extract the existing zero-duration transition setup fragments
without deleting or combining assertions; use checked exact rank conversion in
the three-stratum fixture. No lint allowance or threshold/config change is
permitted.

Pre-edit line counts: `v11.rs` 2,880 (`WARN`, below mandatory 3,000);
`v11_bgc_tests.rs` 280. The objective is function-level Clippy closure, not a
large file move; terminal counts and rationale remain required.

Publication split: `stage3_committed_publication.rs` is 2,974 lines before the
amended repair. Mechanically extract its existing `#[cfg(test)]` module body to
`stage3_committed_publication_tests.rs`, retaining the existing tail include and
every test/assertion. Put accepted WAT5 source assembly in the focused
`stage3_committed_publication_wat5.rs` helper. Preserve public signatures; any
new visibility is crate-private and limited to the already sealed accepted
receipt projector used by the independent real-child test. Target all
resulting handwritten Rust files below 3,000 lines and record exact terminal
counts.

Publication terminal split: the pre-extraction source was 2,995 lines after
the authorized source correction. The mechanically separated files are
`stage3_committed_publication.rs` 2,463 lines and
`stage3_committed_publication_tests.rs` 503 lines; the one-line difference is
the path-module declaration replacing the former inline module wrapper. The
focused accepted-source assembler is
`stage3_committed_publication_wat5.rs` 350 lines and the retained tail is 689
lines. Every publication file is below 3,000 lines. The existing tail include
and all extracted tests/assertions remain present.

## WGHL-FULL-001D open-snow structural split

Status: `COMPLETE — MECHANICAL PARITY VERIFIED`

Static: mechanically replace two complete, contiguous, symbol-anchored blocks
in `v11_covered/open_snow.rs` with same-position `include!` seams. Move
`covered_normalized_delta_v1` through
`covered_soil_max_normalized_deltas_v1` byte-for-byte to
`open_snow_convergence_metrics.rs`; retain
`record_covered_limiter_sample_v1` in `open_snow.rs`. Move the attribute and
complete block beginning with `CoveredOrdinaryPhysicalAuthorityV1` through the
inherent implementation ending with `execute_ordinary_physical_reuse`
byte-for-byte to `open_snow_physical_support.rs`. Do not alter the following
`DirectV11ImportedStack::execute_imported_v10_stack` implementation, the v35
control region, `stable_monotone.rs`, formulas, thresholds, guards, error
precedence, or visibility.

Ran: pre-split line inventory is `open_snow.rs` 4,435 lines. The exact planned
blocks are 407 and 1,578 lines, projecting approximately 2,452 lines for the
wiring file and 407/1,578 lines for the two shards. Target: every resulting
file below 2,000 lines except the wiring file, which must remain below the
mandatory 3,000-line threshold.

Ran: expanding both terminal `include!` seams reconstructs all 4,435 pre-split
lines with exact SHA-256
`593efc8ceb7f54c3bf7b4bed965c9b5ea8c48966e41e192b29452ccdc375ef64`.
`cargo fmt --all -- --check` passed; all-target crate `cargo check` passed with
only the pre-existing unused
`exact_floor_terminal_phase_candidate_below_domain_v1` warning relocated
unchanged. V35 authority and production-source binding passed 1/1 each (run
IDs `a964612c-31c1-4cdc-ad0b-336d831adc87` and
`0ca2f02c-3be5-4ce4-ab71-6b73b6e2dade`). Focused ordinary reuse, terminal
reuse, phase-forcing reconstruction, and physical-residual reconstruction
passed 1/1 each (run IDs `3b92af9d-994e-47e3-b949-9c9c4ec1b184`,
`2b4d383d-f542-4747-8aac-36a943ba7aa1`,
`3bee255b-5dcc-4423-b577-c7d1e1355417`, and
`85623a05-471b-46d4-9d82-7623a20ce1f6`). The focused terminal accepted-endpoint
group passed 5/5 (run ID `a4c68273-fd11-4b81-bd93-01027d3794c5`).

## Terminal orchestrator structural splits

Status: `COMPLETE — MECHANICAL PARITY VERIFIED`

Static: mechanically replace two complete contiguous blocks with same-position
`include!` seams. In `snow_stage3_v11_attachment.rs`, move the block beginning
with `fn append_canonical_bytes` through the complete
`impl PreparedStage3V11SupportIdentityV1` byte-for-byte into sibling
`snow_stage3_v11_prepared_support_identity.rs`. In
`v11_covered/execution.rs`, move the block beginning with `One explicit
default-off invocation` through the complete
`shared_carrier_specific_humidity_tests` module byte-for-byte into sibling
`execution_carrier_humidity.rs`.

Static: preserve item order, comments, attributes, signatures, visibility,
tests, formulas, thresholds, diagnostics posture, error precedence, and module
scope. No opportunistic cleanup or active V47 edit is authorized. Pre-split
counts are 3,191/3,105 lines; exact moved blocks are 312/187 lines, projecting
2,880/312 and 2,919/187 lines. Pre-split source SHA-256 values are
`8d718bd4164f0725b8b5f5810f9f90ec838a50aedcb6025286bd8ead0ea8f70a`
and `18cc5eec03340f920fcdc8c17d84ef6b8ac3b087af92908c898543ec170aded8`;
block SHA-256 values are
`522d4d5710cedd382e2a30e62c288c2fb1272bdc374920a9704bddfb73139060`
and `31cfc800bf498a0c456087392a059dcd775e904d62cd7532e01fd777f5e3115c`.

Ran: the exact same-position includes and shards have terminal counts
2,880/312 and 2,919/187 lines. Expanding the attachment and execution include
seams reproduces the exact pre-split source SHA-256 values above; each shard
also retains its exact pre-edit block SHA-256. `cargo fmt --all -- --check`,
`git diff --check`, and all-target orchestrator `cargo check` passed. Focused
prepared-support digest/identity and shared carrier-humidity tests passed 3/3
(Nextest run `4841f7a0-3368-4e45-b80f-73d1bf102e7a`).
