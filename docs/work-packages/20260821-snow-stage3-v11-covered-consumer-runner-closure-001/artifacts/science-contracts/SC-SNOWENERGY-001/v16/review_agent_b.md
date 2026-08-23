# SC-SNOWENERGY-001@16 independent review B

Evidence: `Static:` independent review of the prospective v16 contract,
runtime policy, and contract-derived tests. No Rust or governance command result
is claimed.

Disposition: `HOLD` pending amendment.

Findings:

1. Critical: the proposed tolerance values are under-supported. In particular,
   cumulative mass must use the existing water-equivalent conversion scale,
   density should remain exact, and no new relative terms are justified.
2. High: v16 authority is absent from the primary invariant, tolerance,
   obligation, and Binding Exposure Index maps.
3. High: frontmatter says `in_review / draft` while the contract body still
   says `approved / active`.
4. High: lane receipt V2 is an obligation for a future normative schema, not an
   already defined wire. V1 must be explicitly non-restorable.
5. Medium: exact `settle_day_count` semantics must be stated despite its
   floating representation.
6. Medium: review, verification, strict binding, assurance, and exact-head
   promotion qualification remain incomplete.
