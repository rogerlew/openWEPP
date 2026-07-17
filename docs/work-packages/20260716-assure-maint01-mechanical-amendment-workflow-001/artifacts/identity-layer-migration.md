# Identity Layer Migration

Evidence class: Ran

The migration retained the frozen Git catalog digest as genesis, installed
generated source and review locks, replaced hard-coded reader governance blocks,
and completed the final projection/schema transition. `verify-generation`
passed from frozen base
`15763d7f6d5d4125333d9b7583424c714f5f5ea4` to active generation
`1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`
through 17 explained transitions.

The final transition admitted the four standalone generated-artifact schemas,
recalculated both review locks, invalidated the pre-final snow review-entry
binding, and mechanically reentered the same pending-review decision against
the corrected content-review subject. It did not create a finding disposition,
scientific approval, reproduction approval, steward approval, realization,
release transfer, or public report.

Current layer behavior is executable:

- science binds typed scientific records plus identified dependency, result,
  and research-object bytes;
- communication binds exact authored manuscript and supplement directives;
- attribution binds reader-facing principal names, affiliations, and authorship;
- governance binds authority-relevant report roles and principal eligibility,
  excluding bibliographic-only principal-version increments;
- finding, approval, realization, and transfer roots are derived from immutable
  events and exact predecessor sets;
- stale event/root bindings fail validation and publication;
- generated locks cannot manufacture human authority.

Production `validate --all` and anchored `verify-generation` both passed after
the transition. The tracked `usersum` tree remained byte-identical to the
frozen base.

All one-time migration commands and the old-algorithm loader were then retired.
The ordinary typed implementation-rebind command reports `changed: false`
against the final generation, proving the current locks bind the current
implementation without retaining a dual identity parser.
