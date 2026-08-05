# Gate Evidence

Evidence class: **Ran**

## Assurance and rendering

- Exact retained-result reconstruction: PASS, parsed JSON equality for 188
  values; retained SHA-256
  `90cc97ff4893cc45fd478d16358c660a86eb20db3c989088b95758d697c7c0dd`.
- Current inspect: PASS at generation
  `0a63d3fe7d847a0b623c163ac0d83f0ca64a47807b39575d3b0b101f76d50567`,
  lifecycle `IN_REVIEW`, approval lock null.
- Named and all-report validation: PASS.
- Named and all-report planning: PASS.
- Draft-source normalization check before lifecycle entry: PASS with no change.
- Two unrelated terminal all-report builds and checks: PASS, 100 files in
  each tree, byte-identical trees.
- Fresh terminal snow subtree versus tracked review draft: PASS,
  byte-identical.
- Tracked review-draft check: PASS, 98 generated files after review entry.
- Public-report inventory: PASS, zero files.

One first terminal all-report staging attempt seeded only the snow narrative
and correctly failed on the missing hillslope narrative link. The required
three preexisting narrative inputs were then seeded in both disposable roots;
both complete builds and checks passed. This was staging-input correction, not
a source or assurance defect.

## Rust and integration

- Focused new manifest positive/no-op and invalid-source matrix: PASS, 2/2.
- Focused lifecycle-fixture correction after the first heavy run: PASS, 2/2 in
  182.274 seconds.
- `assurance-amendment` profile at terminal generation: PASS, 56/56,
  73 skipped, 16 slow, 365.459 seconds.
- Full amendment integration contract before review entry: PASS, 22/22 with 2
  skipped.
- `cargo fmt --all --check`: PASS.
- warnings-denied workspace Clippy: PASS.
- workspace doctests: PASS.
- `cargo check -p openwepp-assurance --tests`: PASS.

The first `assurance-amendment` terminal attempt exposed two integration tests
that reused the now-`IN_REVIEW` repository fixture and attempted a second
review entry. They were corrected to construct an isolated current-source
pending-review fixture. The focused rerun and complete profile rerun pass; the
failure is not omitted from this record.

The first terminal quick attempt then exposed one real-source contract
assertion that still expected the snow report to be `DRAFT`. The package write
set was amended before changing it to the governed `IN_REVIEW` state. That
stale run ended 159 passed, 1 failed, and 40 skipped in 459.340 seconds. The
focused corrected case passed 1/1 in 25.859 seconds, and both dedicated Rust
reviewers passed the complete test-only delta without findings. The corrected
canonical quick result is recorded below when complete.

## Documentation and repository hygiene

- Package `markdown-doc lint`: PASS, 15 files, zero warnings/errors at the
  pre-terminal-artifact checkpoint.
- Package `markdown-doc validate`: PASS, 15 files, zero errors at the same
  checkpoint.
- American-English `uk2us` comparison over changed report and package prose:
  PASS with no diff.
- `git diff --check`: PASS.
- Rust line count: 2,887 lines, WARN disposition retained below the 3,000-line
  blocker.

Terminal verifier A found one identity-bound README sentence that incorrectly
said every review-draft state was `DRAFT`. The sentence now describes governed
nonpublic `DRAFT` or `IN_REVIEW` lifecycles. Typed implementation rebind
produced terminal generation `0a63d3fe...` without changing the exact snow
report roots or active event. Review rendering remained a 98-file no-op/check
PASS; named/all validation and zero-public inventory pass afterwards.

## Campaign profiles

- Exact-generation terminal quick: PASS, 2,183/2,183, 40 skipped, 56 slow,
  2,275.916 seconds.
- Exact-generation terminal frost: PASS, 358/358, 1,919 skipped, 1 slow,
  532.642 seconds.
- Exact-generation terminal full workspace: PASS, 2,272/2,272, 5 skipped,
  57 slow, 2,691.048 seconds.

The same four profiles also passed before the terminal README correction. The
metrics above are the authoritative reruns against exact generation
`0a63d3fe7d847a0b623c163ac0d83f0ca64a47807b39575d3b0b101f76d50567`.
