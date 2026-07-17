# Repeat Terminal Verification A

Evidence class: `Static` and `Ran`

Disposition: `HOLD`

The terminal source retains the published GSI and FAO-56 equations, typed
failure posture, strict calendar admission, exact bounded FIFO implementation,
and process-kernel-only integration hold. The first-admission vector is now
explicit, and the public integration test does restore `history()` plus
`last_date()` after more than 21 admissions and compare the restored and
uninterrupted paths through the next forcing day. However, that restart vector
does not restore heterogeneous GSI history, so the original terminal finding
is not fully closed.

## Finding

### High — The positive restart vector restores only a homogeneous zero FIFO

`crates/openwepp-plant-phenology/tests/restart.rs` varies temperature and VPD
over ordinal days 1 through 25 at 44 degrees north, then restores the retained
state and admits day 26. Under the generalized parameters, FAO-56 photoperiod
for those dates ranges from approximately 8.777 to 9.381 hours through day 25
and is approximately 9.417 hours on day 26. Every value is at or below the
10-hour inactive threshold. Consequently `iPhoto=0`, every retained `iGSI=0`,
and the continuation result is also zero despite the varying forcing fields.

The test therefore proves public-API restoration of a full date-anchored FIFO
and deterministic continuation only for a degenerate homogeneous history. It
does not prove that ordered heterogeneous members survive serialization,
restoration, subsequent oldest-member eviction, and mean reconstruction. This
does not satisfy the initial Verifier A requirement for a heterogeneous FIFO or
the disposition's corresponding closure statement.

Required disposition: revise the public integration vector so the retained GSI
history contains multiple distinct values and the next admission is nonzero.
For example, retaining the existing forcing progression while using zero
latitude would remove the inactive-photoperiod degeneracy. Assert before
restoration that the history has multiple distinct bit patterns, assert the
restored `history()`, `last_date()`, and `sample_count()`, then compare result
fields and final state bits after the next consecutive forcing day. Refresh the
focused, full-workspace, and fresh adjudicated-CRAP evidence made stale by the
test change before verification repeats.

## Checks

| Check | Result |
| --- | --- |
| Explicit first admission | PASS: unit test asserts sample count 1 and independently expected GSI 0.05. |
| Public restore after more than 21 days | PARTIAL: restore occurs after 25 days and preserves the date anchor, but all 21 retained values are zero. |
| Exact continuation comparison | PARTIAL: result and state equality are asserted through day 26, but only on the same all-zero path. |
| Original Review A findings | PASS: revision metadata, three-nontrivial-indicator product, heterogeneous 20/21/eviction means, and ordinary-latitude daylight anchor are present. |
| Original Review B findings | PASS except restart-vector closure: guard/alias maps, inference labeling, chronology, backlog alignment, terminal evidence, and scope boundary are present. |
| JSON identities | PASS: both labels in `heavy-gates.md` match their named target artifacts. |
| Production source identity | PASS: current source matches all three fresh source manifests. |
| Scope containment | PASS: no production crate depends on or consumes the new process-kernel crate. |
| Focused package Nextest | PASS: 13/13 across two binaries, run `c308c3c1-ea5e-4d03-84b7-94027ba6e1c4`. |

## Identity Evidence

- Current `crates/openwepp-plant-phenology/src/lib.rs` SHA-256:
  `53c50514fb13881983737f24125f0216aff45fb46b0dfb2a0c6a97b58e7c4243`.
  This matches the source entry in the before, after, and final source
  manifests.
- Workspace CRAP JSON SHA-256:
  `93c85e3c8c710e7460f612ee114a53dbf373bfc9bc11df94c0580c38800a01de`.
- Adjudicated report JSON SHA-256:
  `1e5c8f9993710298065fda356fd31532e1706ff4360ca80dd6b4f5e286b1fe38`.
- Workspace LCOV SHA-256:
  `9116d344505807cb892b93f1a2431de87d57fd46ca1b7841a95a9d002df8453b`.
- Final source-manifest SHA-256:
  `2b85ea2fa1bad15763a2719f3b1553ec1e77fad4dcb77e97325f4a876f749937`.
- Adjudication-registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The full workspace coverage workflow was not rerun. The retained evidence was
checked read-only, as requested. The maximum supportable claim remains
`PASS-PROCESS-KERNEL`, but this terminal snapshot remains held until the
nondegenerate public restart vector and refreshed evidence pass repeat
verification.
