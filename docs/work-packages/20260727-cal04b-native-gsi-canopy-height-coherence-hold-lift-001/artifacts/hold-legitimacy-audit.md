# HOLD Legitimacy Audit

Status: `PASS / TERMINAL HOLD LEGITIMATE`

Evidence class: `Static + Ran`

## Declared Boundary

The remaining failure is not the owned canopy-height defect. It is defect
`ASSURANCE-CANOPY-README-IDENTITY-001`: an admitted source of the
snow/frozen-soil assurance report changed without the governed assurance
lifecycle transaction needed to reconcile its generated identity and review
state. The affected source, report, review lock, and identity transaction are
outside this package's correction authority and intended write set.

## Evidence Proving The Boundary

- The exact-head unfiltered full profile fails on
  `generated identity member changed:
  tests/fixtures/cancov_forest/README.md`.
- The current README SHA-256 is
  `b81fbe2efa5624e5018c18f24c55ada53d7c484ff020b19d6fa1deae8bd1dd7b`;
  `assurance/v2/identity.lock.json` binds
  `703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c`.
- The same mismatch exists at authenticated package base `f4b3db6c`.
  Commit `502dd745` changed the README; its parent version hashes to the value
  still present in the lock.
- The README is dependency `SF-DEP-CANOPY-README` of the
  `snow-and-frozen-soil-process-evaluation` report, whose report and review lock
  are `IN_REVIEW`.
- This package changes neither the README, the assurance report, its review
  lock, nor `assurance/v2/identity.lock.json`.
- The complete frozen CAL-04B native replay passes 12/12, and the exact-head
  non-assurance full profile passes 2,180/2,180. Those results separate the
  corrected production defect from the assurance failure.

## In-Envelope Routes Considered

The package exhausted its authorized route: amend canopy authority, add
contract-derived tests, correct the real native state transaction and
consumers, preserve legacy behavior, replay every frozen native case, and run
focused plus broad non-assurance gates. No remaining failure identifies an
owned production, contract, verifier, or canopy evidence surface.

Changing only the CAL04B verifier, filtering assurance binaries, or treating
the 2,180-test subset as the declared full-workspace gate cannot close the
mandatory unfiltered gate and would be evasion.

## Why The Current Package Cannot Repair It

The generated lock is machine-owned assurance lifecycle state. A manual hash
edit would bypass the transaction and review model. The finite
`rebind-implementation` route adopts implementation-contract surfaces, not an
admitted report-evidence dependency. Reverting the README would silently erase
an earlier research-documentation change. A valid correction must instead
adjudicate the report dependency, review validity, generation identity, and
transaction under separate assurance lifecycle authority.

This is therefore a different process family and contract authority, one of the
explicit legitimate-HOLD boundaries in
`docs/defect_closure_execplans.md`. The next target is the named assurance
defect above, not another diagnostic step. This package remains open; the
CAL-04B prerequisite remains held.
