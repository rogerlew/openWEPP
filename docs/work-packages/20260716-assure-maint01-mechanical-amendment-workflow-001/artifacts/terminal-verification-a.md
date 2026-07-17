# Terminal Verification A — Identity And Lifecycle

Evidence class: Static and Ran

Current disposition: **PASS**. Attempt 1 was correctly superseded after a
parallel terminal review found an impact-class escalation defect. Attempt 2
verified the correction and all assigned identity/lifecycle boundaries against
the exact source snapshot closed by heavy Run 5. No current-scope finding
remains.

## Attempt 1 — Superseded Pre-Fix Terminal Review

### Static Evidence

- Generated identity is acyclic and layered. `science_root`,
  `communication_root`, `attribution_root`, and `review_governance_root` feed a
  content subject; immutable event IDs then feed finding, approval,
  realization, and release-transfer roots without self-inclusion.
- Identity-lock parsing requires the current
  `openwepp-assurance-generated-identity-v2` algorithm, canonical JSON,
  content-addressed generation ID, a retained frozen-base genesis, and exact
  admitted-file digests.
- Lifecycle requests admit a finite event-type/decision pairing. Approval and
  release events require eligible human principals, complete role-specific
  roots, competence and independence inputs, and exact predecessor sets.
  Steward and transfer authority cannot be recombined across predecessor
  chains.
- `APPROVED` requires a current approval lock. Withdrawal and supersession are
  terminal authored states, invalidate active events, clear publication
  authority, and cannot manufacture approval or transfer authority.
- Approval realization binds complete projected inputs and the embedded
  identity/assembly/lifecycle/planning/publication implementation digest.
  Build/check and publication separately compare exact observed staged bytes;
  altered staged output fails closed.
- `amend rebind-implementation --all` permits preexisting drift only for the
  finite v2 README/schema contract. It recalculates generated locks, does not
  alter authority events, and rejects manuscript or other report-source drift.
- Transaction exchange verifies compare-and-swap, held and staged tree
  stability, external read-set stability, isolated production open/build/check,
  installed receipt equality, and post-exchange generation validity. Typed
  recovery verifies the selected generation before cleanup or restoration.
- One-time migration commands and the old-algorithm loader are absent from the
  production CLI and source. The specification's stale present-tense
  `migrate-identities` statement was found during this review and corrected to
  historical/retired wording at
  `docs/specifications/assurance-amendment-and-identity-workflow.md:558`.
- The final production lock at the time of this attempt named generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`,
  retained frozen base
  `15763d7f6d5d4125333d9b7583424c714f5f5ea4`, admitted 65 sources and two
  review locks, and had one active snow/frost review-entry event with no
  approval or release root.

### Ran Evidence

- `target/release/openwepp-assurance validate --all`: PASS; two reports, zero
  public reports.
- `target/release/openwepp-assurance verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4`: PASS; 17 transitions to the
  then-current generation.
- `target/release/openwepp-assurance amend rebind-implementation --all
  --check`: PASS; `changed: false` on the then-current generated locks.
- Production inspection: groundwater remained `DRAFT`; snow/frost remained
  `IN_REVIEW`; neither had an approval lock.
- Targeted nextest run ID `fdc39d08-0070-429d-a1ae-4f0199a4a0c1`: PASS, six
  identity/lifecycle/recovery tests in 41.013 seconds. It covered current
  generation-chain inspection, finite implementation rebind and strict report
  drift rejection, deterministic terminal lifecycle, supersession, and
  selected-generation recovery verification.
- Targeted nextest run ID `c579a525-9ce2-47cf-a86b-95b0f128fb9f`: PASS, two
  approval/release/publication negative-matrix tests in 93.155 seconds. It
  covered wrong approval bindings, release mismatch, changed schema/catalog/
  dependency bytes, and changed generated output bytes.
- Protected `usersum` diff from the frozen base: zero.

### Closure Blocker

High: the retained fresh CRAP report passed with raw 2, adjudicated 2, and
actionable 0, but its final source manifest recorded
`crates/openwepp-assurance/src/v2/amendment.rs` as `f75eac61...` and
`amendment_support.rs` as `2e5a5ce7...`. The impact-class correction changed
those files to `a609c2a6...` and `a2529855...` after the gate completed. The
heavy report is therefore valid historical evidence but not terminal closure
evidence for the corrected source. A fresh same-runner heavy pass is required.

## Attempt 2 — Post-Fix Terminal Verification

### Static Evidence

- The identity and authority architecture described in Attempt 1 is unchanged:
  current-only identity parsing, canonical content-addressed locks and events,
  layered acyclic roots, exact predecessor sets, finite decision mappings,
  role-eligible human approval/release authority, terminal withdrawal and
  supersession, exact staged-byte checking, strict report drift rejection, and
  selected-generation recovery verification remain operative.
- The impact-class defect is closed. Both changed and no-op
  `rebind-implementation` receipts identify the operation as
  `scientific-full`. A changed receipt carries stable gate ID
  `assurance-implementation-package-v1` and no focused gate argv, so it cannot
  enter the receipt runner's metadata/editorial/governance fast lane. The
  finite rebind surface and strict rejection of manuscript/report drift remain
  unchanged.
- One-time migration commands and the old-algorithm loader remain absent. The
  compatibility section now describes `migrate-identities` as a completed,
  deleted one-time operation rather than a current CLI command.
- Review A's seven accepted findings remain closed: decision-specific event
  authority; implementation plus observed-output binding; operative terminal
  lifecycle; complete steward/transfer bindings; exhaustive report-field
  classification; canonical content-addressed generation transitions; and
  schema admission of generated role/principal events.

### Ran Evidence

- Heavy Run 5 passed the complete required ladder on terminal source:
  `cargo fmt --check`, workspace Clippy with warnings denied, full nextest run
  `959f93c0-a975-472d-8ee9-a8e8bb6d29e0` with 2,072 passed and 5 skipped,
  `cargo deny check`, and fresh adjudicated CRAP.
- The fresh CRAP artifact is closure-eligible and records source-manifest
  SHA-256
  `7227650f30319b95c279367c384bd8bed2af40840a124b37041fa19270b41784`,
  raw 2, adjudicated 2, actionable 0, touched actionable 0, untouched
  actionable 0, and no invalid adjudications. All 234 current production-source
  hashes were independently compared with that manifest after targeted
  verification; mismatches: zero.
- The two raw rows are the preexisting adjudicated
  `MeteorologyError::fmt` and `SymbolAliasRegistryError::fmt` rows, outside the
  touched assurance files.
- `target/release/openwepp-assurance validate --all`: PASS; two reports and
  zero public reports.
- Anchored `verify-generation`: PASS through 17 transitions to generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`.
- Production `amend rebind-implementation --all --check`: PASS, no-op, and
  classified `scientific-full`; generated locks are current.
- Targeted nextest run `d2fc122f-1c23-4ec9-8882-8bbab8daeef3`: PASS, three
  selected implementation-rebind, generation/recovery, and selected-tree
  verification tests in 15.019 seconds.
- No Rust source changed after the Run 5 manifest. Protected `usersum` remains
  byte-identical to the frozen base, and `git diff --check` passes.

### Final Disposition

PASS. The corrected terminal source preserves generated identity and layered-
root integrity, immutable lifecycle decision authority, approval/release
binding, finite implementation rebind behavior, strict report-drift rejection,
migration/parser retirement, recovery safety, and anchored production
generation. Fresh source-bound CRAP is raw 2, adjudicated 2, actionable 0. No
assigned closure blocker remains.
