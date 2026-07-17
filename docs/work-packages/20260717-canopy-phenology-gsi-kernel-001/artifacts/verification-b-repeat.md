# Repeat Terminal Verification B

Evidence class: `Static`, `Ran`, and retained `Ran`

Disposition: `HOLD`

Verification subject: terminal-current working tree based on frozen base
`45d49090214b4702d11a04aafe5d5ccade7ba440`. Repeat Verifier B read the
initial Review B, initial Verification B, producer disposition, focused-gate
evidence, and heavy-gate evidence. Repeat Verifier B did not read repeat
Verifier A's output and did not rerun full-workspace coverage.

## Checks Performed

- Inspected both crate test binaries, the complete public state/restoration
  path, the CP-GSI01 contract amendment, and the process-kernel integration
  hold.
- Ran package Nextest with the quick profile: 13/13 passed across two binaries,
  run `ad0ed87c-0d75-4385-98b5-95fd1a9d30cb`.
- Ran strict package Clippy, `cargo fmt --check`, and `git diff --check`; all
  passed.
- Independently evaluated the 25 forcing records used by the new public
  restart test through the contracted GSI equations.
- Recomputed the hashes of the CRAP input JSON, adjudicated report JSON,
  workspace LCOV, final source manifest, adjudication registry, production
  source, and public restart test.
- Confirmed the current production source and restart test exactly match their
  entries in `source-manifest-final.json`.
- Searched Rust production and test sources for reverse consumers. The new
  crate remains unintegrated; no canopy, biomass, litter, snow, ET, erosion, or
  assurance consumer reads its result.

## Initial Review B Closure Audit

| Finding | Repeat verification | Evidence |
| --- | --- | --- |
| `B-01` traceability | `CLOSED` | `SC-PLANT-001` retains Guard Map rows for `INV-PLANT-028..032` and aliases for forcing, parameters, indicators, output, FIFO, and date anchor. |
| `B-02` warm-up authority | `CLOSED` | `INV-PLANT-029` and CP-GSI01 continue to distinguish the published 21-day law from openWEPP's available-real-sample cold-start inference. |
| `B-03` chronology/restart | `PARTIAL / HOLD` | The public API implements anchored restoration and consecutive admission, but the new success vector is degenerate and does not satisfy the required heterogeneous restoration proof; see `VRB-01`. |
| `B-04` contract vectors | `CLOSED` | The unit-test binary retains the three-nontrivial-indicator product vector and independent first/20/21/eviction assertions. The first admission is explicitly checked at `lib.rs:623-625`. |
| `B-05` backlog truthfulness | `CLOSED` | Increment 3 owns fixed-date replacement and consumer integration; selected GSI and FAO-56 choices are recorded as resolved. |
| `B-06` terminal evidence | `CLOSED` for the measured snapshot | Retained evidence records full Nextest 2,085/2,085, workspace Clippy, dependency policy, and fresh CRAP with zero actionable rows. Current source and test hashes match the retained final manifest. |

## Finding

### VRB-01 — High — The restart success path is an all-zero alias, not the required heterogeneous proof

`tests/restart.rs:19-23` does exercise 25 successful public `advance` calls,
and lines 25-38 restore through public `history()`/`last_date()` and admit the
next calendar day. Structurally, this is the requested positive public-API
path. It does not, however, close initial finding `VB-01` as dispositioned.

The test uses ordinal days 1 through 25 at 44 degrees north. Under the
contracted FAO-56 photoperiod equation and the generalized 10-hour inactive
threshold, `iPhoto` is zero on every one of those days. Independent evaluation
therefore gives 25 instantaneous-GSI values equal to zero: one unique retained
value, zero nonzero values, and a retained range of `[0,0]`. Day 26 is also
zero. Varying temperature and VPD in lines 5-6 does not make the resulting FIFO
heterogeneous.

Consequently, the test cannot distinguish ordered-value restoration from an
implementation that loses or reorders retained values, and its next-day mean
is an all-zero alias. This conflicts with the explicit remediation in initial
Verification B and with `disposition.md:28-30`, which says the test reconstructs
a heterogeneous full FIFO. `B-03` and `VB-01` remain open.

Required correction: use calendar and forcing values that produce at least two
distinct retained instantaneous-GSI bit patterns, assert that heterogeneity
before restoration, restore the full public history/date anchor, then admit a
nontrivial next consecutive day and retain bit-identical result and final-state
assertions. Refresh focused, full-workspace, dependency-policy, and fresh CRAP
evidence because the changed test is a hashed measurement input.

## Retained Artifact Identity Audit

The identities in `heavy-gates.md` correctly name and hash the actual final
artifacts:

- workspace CRAP input JSON:
  `93c85e3c8c710e7460f612ee114a53dbf373bfc9bc11df94c0580c38800a01de`;
- adjudicated report JSON:
  `1e5c8f9993710298065fda356fd31532e1706ff4360ca80dd6b4f5e286b1fe38`;
- `target/adjudicated-crap/workspace.lcov`:
  `9116d344505807cb892b93f1a2431de87d57fd46ca1b7841a95a9d002df8453b`;
- final production source manifest:
  `2b85ea2fa1bad15763a2719f3b1553ec1e77fad4dcb77e97325f4a876f749937`;
- adjudication registry:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The current production source SHA-256 is
`53c50514fb13881983737f24125f0216aff45fb46b0dfb2a0c6a97b58e7c4243`,
and the current public restart test SHA-256 is
`0d293a5fb9aaba7ea9a3c7fd00293da16f3835af384d1abb4e05af199f4e3963`.
Both exactly match the corresponding final-manifest entries. The CRAP report
remains fresh, closure-eligible, and `PASS` for this measured snapshot.

## Claim Ceiling And Conclusion

The contract, package, roadmap, source boundary, and reverse-consumer search
all preserve `PASS-PROCESS-KERNEL` as the maximum possible claim. There is no
support for an integrated-canopy, empirical-validation, snow-model, or release
claim.

Repeat Terminal Verification B remains `HOLD`: the first-admission vector,
original `B-01`, `B-02`, `B-04`, `B-05`, terminal identities, CRAP freshness,
and scope ceiling pass, but the public restart vector is degenerate and does
not close the accepted heterogeneous anchored-restoration obligation.
