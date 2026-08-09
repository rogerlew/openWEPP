# Primary Rust Correctness Review

## Findings

No open findings.

### Resolved — Lifecycle identity and public-side-effect proof

Paths: `tests/integration/assurance_v2_publication_contract.rs:531` and
`tests/integration/assurance_v2_publication_contract.rs:546`

The previously reported medium finding is closed. The assertion now requires
exact equality with the selected report's governed diagnostic,
`report '<REPORT_ID>' is DRAFT; publication requires APPROVED`; a different v2
validation error that merely mentions `DRAFT` can no longer pass. The test also
captures the seeded `usersum` file tree before publication and compares the
complete captured tree after rejection. This subsumes the former catalog-only
check and detects creation or mutation of public publication files. The direct
empty-snapshot-root assertion remains and excludes both a content-addressed
snapshot directory and the `receipts/` directory.

Finding disposition: `accepted-and-fixed`. No further code change is requested.

## Residual Risk And Missing Tests

Evidence class: `Static` re-review plus inspection of the executing agent's
reported `Ran` isolated PASS after the finding fix. This reviewer did not start
a competing test process while the delegated full-workspace run was active.

- Static production ordering continues to support the diagnosis:
  `crates/openwepp-assurance/src/v2/publication.rs:640` validates root
  confinement before context loading; line 644 calls `validate_publishable`;
  lines 845-850 reject the report lifecycle before approval validation; and the
  first snapshot, receipt, and public writes occur only in finalization at
  lines 715, 733, and 742. No production assurance Rust is changed by this
  package.
- The in-repository `TMPDIR` result is a correct fail-closed confinement
  rejection, not a production defect. With `Scratch::new` deriving roots from
  `std::env::temp_dir()`, an in-repository temporary root violates the
  unrelated-root invariant. The external-scratch PASS is consistent with the
  inspected control flow.
- The Rust test change remains portable within the implementation's existing
  Unix-only publication contract. The literal
  `/home/workdir/openwepp-task-tmp` is host-specific operational guidance;
  another checkout must use and record a writable canonical scratch root
  outside its repository.
- The full-workspace run had not completed at re-review time. Its terminal
  result and the package's independent verification gates remain package
  closure evidence, not open Rust correctness findings in this reviewed diff.

## Disposition

**APPROVED — no correctness blockers.**

The exact amended diff preserves assurance fail-closed semantics, uniquely
binds the DRAFT lifecycle rejection, and proves absence of public publication
file, snapshot, and receipt side effects. No arithmetic, science-contract,
serialization, error-taxonomy, portability, or duplicated-production-logic
regression was introduced. Retaining the diagnostic enhancement is
appropriate, and the decision not to modify production confinement logic is
correct.
