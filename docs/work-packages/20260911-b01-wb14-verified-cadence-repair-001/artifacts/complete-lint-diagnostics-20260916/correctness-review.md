# Correctness review — complete supplemental lint diagnostics

**Reviewer:** `/root/diagnostics_correctness` (Sol/high), an attributable replacement
independent of the collection author, orchestrator, and QA reviewer. This review
does not claim continuity with either prior reviewer conversation.

**Reviewed source:** baseline tree
`114bd2877dd4141ec5023dbd99e1d7c1c97e68dfd75b5fdf6a570350c8f463e5` and
candidate tree `ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592`,
candidate cumulative patch
`4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d`.
Both retain supplemental input custody
`2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082`.

**Evidence class — Static:** I inspected the collection/comparison methods, exact
receipts, raw Cargo JSON and verbose Clippy logs, target artifacts, unmatched
structured diagnostics, relevant dependency source, frozen candidate/base source,
and prior accepted source-relocation evidence. **Ran-readlogs:** the four Clippy
commands and toolchain-identity command were executed by the orchestrator; I
inspected their retained results. I did not execute Cargo, Clippy, Rust tests,
simulation/runtime programs, or scientific workflows.

## Findings

### High — the unchanged original regression and scientific disagreement still block overall recorder acceptance

- **Location:** retained execution-discretion evidence and
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`.
- **Evidence:** this supplemental collection made no Rust or scientific-authority
  change. The accepted prior correctness review records the original regression as
  FAIL at the same preliminary typed adaptive-refinement budget error on baseline
  and candidate. The unresolved outer `4/22` versus nested `0/0` authority
  disagreement also remains outside this diagnostic-only amendment.
- **Disposition:** overall recorder preparation remains **INCOMPLETE / HOLD**. The
  complete lint measurement cannot repair or waive either scientific blocker.

No new high- or medium-severity correctness, safety, security, typed-error,
serialization, numerical, or science-contract defect was found in the supplemental
measurement or newly reached targets.

## Supplemental diagnostic method and attribution

The four receipts preserve the retained wrapper, explicit source manifest,
package, `--all-targets`, `--locked`, JSON messages, and exact feature selections:

- owning crate: `persisted-restart-v1,restart-authority-evidence`;
- runner: `openwepp-runner/test-fixture-authority`,
  `openwepp-hillslope-orchestrator/persisted-restart-v1`, and
  `openwepp-hillslope-orchestrator/restart-authority-evidence`.

Each command appends only `-- --no-deps -D warnings --cap-lints warn` and uses the
baseline or candidate diagnostic target root. No inherited `RUSTFLAGS`, encoded
Rust flags, Rust wrapper, or toolchain override was present. The source trees and
supplemental inputs were unchanged across every command. `environment-continuity.json`
also binds identical `Cargo.toml`, `Cargo.lock`, `flake.nix`, `flake.lock`,
`rust-toolchain.toml`, and `clippy.toml` across baseline, candidate, and the active
Nix environment.

Actual execution confirms the intended cap behavior. All four receipts exit zero,
all Cargo streams end with `build-finished: true`, and all emitted diagnostics have
warning severity. The verbose logs show Clippy driver invocations for every primary
artifact under Rust 1.95.0 / Cargo 1.95.0 / Clippy 0.1.95. The retained strict runs
on these sources remain exit 101 with denied lint errors. Therefore the zero exits
are diagnostic-collection success only; strict Clippy remains **FAIL**.

Target completion is complete and source-bound:

- Each owning stream has six non-fresh primary artifacts: normal and test library
  profiles plus the four declared examples. There are six corresponding primary
  Clippy-driver invocations.
- Each runner stream has 21 non-fresh primary artifacts: normal and test library
  profiles, normal and test profiles for all five binaries, and all nine integration
  tests. There are 21 corresponding primary Clippy-driver invocations.
- Every row in `complete-lint-target-coverage.json` is complete on both sources and
  both package builds finish successfully. Sharing each side's target root between
  its owning and runner commands did not substitute cached primary evidence: all
  required runner artifacts record `fresh: false`.

The four newly reached owning examples and the five runner binaries plus nine
runner integration tests emit no diagnostics on either source. Their absence from
the diagnostic list is backed by their completed primary artifacts and Clippy
driver invocations, rather than inferred from message absence alone.

The capped owning comparison is 2,956 baseline messages versus 2,954 candidate
messages, with 2,944 exact normalized matches. Its six candidate-unmatched groups
(10 messages after multiplicity) are the same six groups exposed and independently
reviewed in the prior strict comparison:

- Two `result_large_err` closure sites retain the same error variants and closure
  behavior. One changed span is formatting-only; the other moves by the in-scope
  recorder-history call-site insertion. Both occur in normal and test library
  profiles.
- `close_deferred_native_v2_soil_at_parent_end_v1` remains a pre-existing
  `too_many_lines` site. Its reported metric changes from 238 to 253 because that
  function was rustfmt-expanded; the prior source evidence proves the complete
  function equal with whitespace removed. This is a changed inherited metric, not
  merely a line-number relocation.
- `execute_covered_real_v11_parent_with_evidence` remains a pre-existing
  `too_many_lines` site. Its metric changes from 663 to 670 because the candidate
  adds the exact seven-line `SnowFreePreChildHistoryV1` argument at the authorized
  recorder seam. Removing that added argument makes the compared region byte-equal.
  The lint remains on the same function and does not identify changed arithmetic,
  guards, units, clamps, domain handling, or solver behavior.
- `run_canonical_covered_physical_prefix_capture` changes from 115 to 116 lines due
  formatting only; the prior evidence proves its whitespace-removed body equal.
- The `exercise_complete_wb14_cadence` forwarding helper retains ten arguments and
  therefore the same `too_many_arguments` diagnostic after moving from line 32 to
  line 187. Its former booleans are now lossless private `Into<...FlagV1>` values,
  which removes the baseline-only `fn_params_excessive_bools` warning. The other
  baseline-only warning, `needless_pass_by_value`, disappears because the recorder
  now consumes the captured context. Neither removal masks an error.

The capped runner comparison is exactly 291/291. Every lint class present in each
prior strict stream remains represented in its corresponding capped stream. The
additional 93 owning-stream and 176 runner-stream messages are compiler warnings
from freshly built dependencies and match exactly between baseline and candidate;
they are not candidate-introduced or newly reached selected-target diagnostics.

I separately reviewed the warnings with plausible safety relevance. The nine
`dangerous_implicit_autorefs` occurrences in each fresh stream come from inherited
`serde_yaml 0.9.34+deprecated` access to `unsafe-libyaml` parser/emitter prefix
fields. At every warned call site, the raw pointer refers to a live pinned owned
allocation, initialization has no competing reference, or the caller holds
exclusive `&mut self`; the warned accesses are read-only. The validity and aliasing
requirements named by the lint are therefore satisfied, and I found no concrete
safety or security defect. The four `arrow-array` `unnecessary_transmutes` preserve
same-width integer bit patterns as `f32`/`f64`; the suggested `from_bits` form is a
clarity improvement, not a behavioral correction. The remaining fresh dependency
warnings are compatibility or maintenance diagnostics. Their inherited status did
not lower this review's safety threshold.

## Residual risk and missing tests

- Strict owning-crate and runner Clippy remain **FAIL, exit 101**. The cap is an
  authorized acquisition mechanism for this bounded comparison, not a green lint
  result or cleanup of inherited debt.
- The diagnostic processor retains complete structured messages and raw rendered
  logs, but exact feature/manifests/toolchain identity comes from receipts and
  `environment-continuity.json`; Cargo compiler-message records do not encode all of
  that identity. I manually matched those fields for all four commands.
- This evidence proves compilation/lint completion for every selected target. It
  does not execute examples, binaries, or tests and does not add runtime, numerical,
  comparator, or scientific evidence. The previously accepted recorder behavior,
  native parity, formatting, and source review remain reused because source and
  relevant inputs are unchanged.
- The deprecated `serde_yaml` dependency and other matched dependency warnings are
  maintenance risk outside this recorder-specific scope. No dependency, manifest,
  feature, or suppression change was authorized or made.
- The original regression remains **FAIL**, and the scientific/authority
  disagreement remains unresolved. Full recorder preparation and cadence remain
  unqualified.

## Verdict

**Supplemental diagnostic collection: COMPLETE. Complete matched diagnostic
criterion: PASS for this bounded observational increment.** Every selected target
and required profile completed under the exact matched configurations; newly
reached selected targets add no diagnostics; runner diagnostics match exactly; and
all owning differences are independently attributable to the already reviewed
recorder delta without a new correctness, safety, security, or science defect.

**Strict Clippy: FAIL. Original regression: FAIL. Complete recorder preparation:
INCOMPLETE / HOLD.**
