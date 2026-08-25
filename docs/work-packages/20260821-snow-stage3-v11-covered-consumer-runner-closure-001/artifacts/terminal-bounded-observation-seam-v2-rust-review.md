# Terminal bounded observation seam V2 Rust/API review

Recommendation: **HOLD / NO GO-to-evidence**

Evidence class: `Static`. Independent review verified the four candidate hashes
and nine pinned source hashes. It made no edits and did not consult the
numerical reviewer.

## Findings

1. **Blocker:** the named fixture and setup helper are private in a sibling test
   module outside the write set; the proposed capture test cannot reuse them.
2. **Blocker:** no viable path carries a construction-time ingress projection
   to `CarrierHook`. Signature/field approaches require additional callers or
   literals outside the write set, while forbidden global/callback approaches
   are not alternatives.
3. **High:** hook and DTO definitions remain prose, so lifetimes, precise
   source types, visibility, ownership and compilation cannot be reviewed.
4. **High:** WB14 and surface witnesses have no distinct executable predicates.
5. **High:** CaptureEvidence names `CoveredTerminalJointTrialStateV1`, while
   the live generic chain carries `Option<CoveredTerminalJointTrialStateV1>`.
6. **Medium:** the snapshot does not disposition nested vectors and optional
   terminal model state in `DirectSnowStage3PersistentState`.
