# Disposition

Status: `HOLD-INTVAL-REL-001`

Evidence class: **Ran + Static**.

The intended release-isolation correction is implemented and proven across the
full workspace. The package cannot claim terminal PASS because the mandatory
exact release command failed on independent protected authority provenance
before remaining authority, binary/staging/lint, and stability lanes.

Close `INTVAL-AUTH-PROV-001`, rerun the exact pinned-input release command from
the beginning, and only then restart integrated validation from Phase 0. No
partial result from this attempt may satisfy either terminal release or
integrated-validation acceptance.
