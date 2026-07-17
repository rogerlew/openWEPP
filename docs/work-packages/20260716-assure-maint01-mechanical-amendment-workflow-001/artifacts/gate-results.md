# Focused Gate Results

Evidence class: Ran

- `cargo fmt --check`: PASS.
- `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile assurance-amendment`: PASS, 45/45
  selected tests; 53 tests and 182 binaries skipped by the pinned profile.
- Current production generation migration: PASS, generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`.
- `target/release/openwepp-assurance verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4`: PASS, 17 transitions.
- `target/release/openwepp-assurance amend rebind-implementation --all
  --check`: PASS and reports `changed: false` after the final implementation
  rebind.
- Both production reports validate, build, and check together: PASS.
- Final-binary receipt-runner campaign: PASS, ten current and twenty scaled
  trials; see `performance-evidence.md`.
- Release binary SHA-256:
  `010cf889644f8c921bcac204cf09330e908709e29b905d83e44240184ebd9c66`.
- Protected `usersum` diff from the frozen base: zero.
- Touched module line counts: `amendment.rs` 2,397;
  `amendment_support.rs` 92; `fixture.rs` 220; `identity.rs` 1,420;
  `transaction.rs` 745; `publication.rs` 2,867. All remain below the 3,000-line
  hard gate and have explicit extracted support, identity, fixture, and
  transaction modules.
- Changed-file Markdown lint: PASS. Whole-tree Markdown lint reports 15
  preexisting broken-link errors outside this package's changed files.
- `git diff --check`: PASS.

Final delegated closure: PASS. Heavy Run 5 passed formatting, workspace Clippy,
full nextest run `959f93c0-a975-472d-8ee9-a8e8bb6d29e0` with 2,072/2,072 tests,
dependency policy, and fresh adjudicated CRAP at 2 raw / 2 adjudicated / 0
actionable. All 14 touched assurance production files have zero actionable
rows. The source-bound manifest is
`7227650f30319b95c279367c384bd8bed2af40840a124b37041fa19270b41784`.

Both post-heavy terminal verifiers passed. They independently checked the exact
source manifest, anchored generation, identity/lifecycle and approval/release
boundaries, final performance arithmetic, complete forged-receipt rejection
matrix, focused/full gate separation, protected science projection, and zero
public-surface change. See `terminal-verification-a.md` and
`terminal-verification-b.md`.
