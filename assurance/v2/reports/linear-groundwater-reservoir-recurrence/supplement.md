# Supplement: Daily Linear Groundwater-Reservoir Recurrence

## S1. Scope and source status

This supplement belongs to the internal ASSURE-04A fixture source. It preserves
the evidence map accepted in ASSURE-02 while the v2 source and identity contract
is implemented. It neither authorizes publication nor updates the assessed
scientific evidence.

## S2. Claim-to-evidence map

| Claim | Question answered | Primary evidence | Limit |
| --- | --- | --- | --- |
| `GW-P01` | What daily recurrence and coefficient authority were assessed? | `SC-GWBASEFLOW-001`, Srivastava et al. (2013), `GW-METHOD-AUTHORITY` | Linear daily routine only |
| `GW-P02` | Does the implementation debit prior-day exports before calculating current-day exports? | implementation source, focused timing test, `GW-METHOD-ANALYTICAL` | Exact tested recurrence and admitted domain |
| `GW-P03` | Does the implementation reproduce the independent two-day values within the coded allowance? | `GW-RESULT-TWO-DAY`, `GW-METHOD-ANALYTICAL` | Synthetic daily vector; no field-performance inference |
| `GW-P04` | Does the implementation enforce the admitted coefficient and over-export boundary? | science contract, negative guard test, `GW-METHOD-DOMAIN` | No independent scientific upper bound for either coefficient |
| `GW-P05` | Does the retained production ledger close under independent reconstruction? | `GW-RESULT-H2637`, `GW-METHOD-LEDGER` | One 731-day, 19-OFE case; no convergence or accuracy inference |
| `GW-P06` | Do generated groundwater fluxes reach the intended production consumer without `cbase` substitution? | `GW-DEP-CONSUMER-PROOF` and `GW-METHOD-CONSUMER` | Named publication and watershed branches only |
| `GW-P07` | Are generated groundwater fields excluded from the active surface-runoff source? | `GW-DEP-ROUTER-EXCLUSION-PROOF` and `GW-METHOD-CONSUMER` | Named active source builder only |
| `GW-P08` | Were the twelve declared implementation and evidence paths unchanged at ASSURE-02 intake? | `GW-METHOD-CURRENCY`, ASSURE-02 exact path comparison | Twelve named paths only; not a release transfer |
| `GW-P09` | Do the focused current-tree recurrence, guard, authority, threshold, and HBP-consumer tests pass? | `GW-METHOD-CURRENT-TESTS`, ASSURE-02 seven-test run | Focused confirmation; not full workspace or fresh H2637 |

## S3. Analytical vector

Inputs are `S0 = 10.0 m3`, daily recharge `[2.0, 4.0] m3`,
`kb = 0.10 d^-1`, and `ks = 0.05 d^-1`. Independent application of the
recurrence gives day-one storage and exports `(12.0, 1.2, 0.6) m3` and day-two
storage and exports `(14.2, 1.42, 0.71) m3`. The retained result object records
the exact binary64 maximum residual and the coded allowance.

## S4. H2637 operands

The retained result object records initial storage, cumulative recharge,
cumulative exports, terminal pre-export storage, terminal-day exports, both
independent residuals, and both allowances. Terminal-day operands come from the
run manifest, not the latest runoff-event record.

## S5. Identity and reproducibility notes

- `SC-GWBASEFLOW-001` identifies the formulation and coefficient authority.
- The assessed software realization is Git commit
  `de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.
- The ASSURE-02 current-tree record binds the focused seven-test confirmation
  and independent two-day arithmetic.
- The H2637 conservation record and arithmetic log bind the retained
  production operands.
- `GW-DEP-CONSUMER-PROOF` and `GW-DEP-ROUTER-EXCLUSION-PROOF` bind the exact
  accepted positive consumer-path and active-router negative proofs.
- The accepted ASSURE-02 prototype is retained as the exact agent-assisted
  source predecessor. Its tool/model configuration was not retained; that
  provenance gap is explicit and blocks review entry for this fixture.
- Structured records identify these sources and results; this manuscript and
  supplement remain the canonical scientific explanation.

## S6. Publication boundary

No scientific or publication review lock exists for this fixture. Its
publication and export permissions are false, it has no public path or release
snapshot, and it cannot appear in the public `usersum` catalog. A future report
must be revised, reviewed, and promoted through ASSURE-04D and ASSURE-05.

## S7. Authorship and agent-assistance state

Codex is the disclosed architecture-fixture author. A human report lead and
scientific approver are unassigned. ASSURE-02 internal coding-agent review is
not external scientific peer review. The typed authorship and agent-assistance
records make these facts executable review-entry blockers; they are not
reader-facing substitutes for the positive evidence presented in the
manuscript.
