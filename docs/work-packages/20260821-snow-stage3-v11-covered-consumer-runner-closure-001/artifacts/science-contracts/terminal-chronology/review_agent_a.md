# Terminal chronology coordinated contract review A

Evidence: `Static`

Verdict: `HOLD`

1. `CR-A-001` (`critical`): terminal snow--soil physics is not independently
   reproducible. Define accepted substep integration, endpoint handling, soil
   evolution, equations, and authority/evidence.
2. `CR-A-002` (`critical`): event-at-start contradicts positive-support,
   terminal-receipt, and nonempty-ledger requirements. Define a canonical
   zero-support disposition.
3. `CR-A-003` (`critical`): snow-only physical mutation is not distinguished
   from the coupled clock/chronology transition. Define both exact mutation
   sets.
4. `CR-A-004` (`major`): new Binding Exposure Index rows are missing or
   orphaned and use an invalid lifecycle value.
5. `CR-A-005` (`major`): new invariant tables omit authority/evidence columns
   and the additions are not integrated across state, branch, guard, alias,
   unit, tolerance, test, gap, and obligation sections.
6. `CR-A-006` (`major`): canonical snow-owner V4 lacks exact field order,
   widths, byte order, enum tags, map ordering, hash/domain, and parcel fields.
7. `CR-A-007` (`major`): released predecessor lifecycle is ambiguous after
   whole-contract draft downgrades.
8. `CR-A-008` (`major`): new snow/freeze failures are not mapped to exact typed
   errors.

Recommendation: correct every finding and repeat independent review before
promotion or production edits.

## Final re-review

Evidence: `Static`

Verdict: `GO` to mandatory verification/gate stage after CR-A-001 through
CR-A-008 were closed. This verdict made no runtime or exact-head claim.

The post-disposition verification then found two residuals and returned
`HOLD`: terminal mass/energy predicates used the tolerance-family identifier
as though it were a scalar, and final review evidence had not been retained in
the package. The contracts now name `a_terminal_mass=1e-9 kg m^-2` and
`a_terminal_energy=1e-6 J m^-2` in the affected predicates, the structural
test asserts both components, and this section retains the review chronology.
A final narrow verification after those corrections returned `GO`: the
structural test binds both exact SnowEnergy predicates to their unit-bearing
components, the SnowFreeze parcel equation remains asserted, and the retained
review chronology is complete. Static only; no runtime or exact-head claim.
