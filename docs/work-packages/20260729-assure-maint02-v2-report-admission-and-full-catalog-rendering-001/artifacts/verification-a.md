# Terminal Verification A

Status: complete / pass

Verifier: fresh independent agent `assure_maint02_terminal_a`

Evidence class: Static + Ran

Verified:

- typed admission uses compare-and-swap, isolated candidate validation, atomic
  exchange, installed-state verification, and rollback/recovery;
- a real repeat `admit-report --check` produced a schema-v2 no-op with
  byte-identical pre/post `assurance/v2/**` inventories and no recovery tree;
- schema-v2 receipts require exact old/new root key sets while archived
  schema-v1 receipts remain rootless and canonical;
- admission roots match the installed CAL-09 review lock;
- anchored generation verification passes at 22 transitions and generation
  `30db4a7e6a691601426428b7772e28143ff9fa1bf10dd9d1ae80062d7f0002a2`;
- validation and planning select exactly three current non-fixture DRAFT
  reports and zero public reports;
- stable-preview `check --all` passes;
- CAL-09 approval/release fields remain null and export/vendoring disabled;
- all nine staged CAL-09 SVGs have title, description, and `role="img"` with
  no detected active/external-content pattern;
- retained-SVG sanitizer tests pass 4/4 and receipt v1/v2 schema discrimination
  passes;
- protected surfaces, line-count governance, the 2,161-test workspace log, and
  `git diff --check` pass.

The verifier raised one documentation finding: exact-diff evidence omitted
native-usersum link routing, removal of unsupported pre-review agent metadata,
and semantically equivalent “100%” to “completely” wording. The corrected
artifact also includes verifier B's late `lib.rs` inventory correction. The
verifier rechecked and accepted all dispositions.

Verdict: PASS. No unresolved findings.
