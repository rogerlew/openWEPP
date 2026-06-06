# Disposition

Status: complete

Evidence mode: Static + Ran

Disposition: accepted

Summary:

- ADR0017 is ratified as accepted governance.
- ADR0016 and the decisions registry record the accepted amendment.
- Canonical governance docs encode comparator-as-flag adjudication,
  like-for-like unit/lineage gates, independent correctness authority,
  `HARNESS-SURFACE-MISMATCH`, and owned `HOLD`.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039` and
  `SC-WATBAL-001#INV-WATBAL-087` are active canonical contract authority.
- HPHYS0296-0298 stale three-verdict invariant/obligation wording was amended
  to use the ADR0017 peer taxonomy.
- Contract-derived Rust test is registered and passes.
- Dual reviews and dual verifications were performed; all findings were
  accepted and addressed.

Ran gates:

- `cargo fmt --check`: PASS.
- `cargo test --test hphys0313_snowpack_settling_carry_recursion_contract -- --nocapture`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: PASS.
- `git diff --check`: PASS.
- `cargo test --test adr0017_comparator_distrust_ratification_contract -- --nocapture`: PASS.
- `markdown-doc lint --path docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`: PASS.
- `markdown-doc lint --path docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001`: PASS.

Residuals:

- The untracked `docs/backlog/20260605-snow-code-deferred-science-review.md`
  file is explicitly excluded from this package.
- Follow-on reclassification work is needed for open HPHYS0298-0313 snow/`RM`
  and water-balance ledger rows under ADR0017.
