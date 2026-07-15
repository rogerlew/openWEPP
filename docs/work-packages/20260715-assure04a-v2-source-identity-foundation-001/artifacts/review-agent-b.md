# ASSURE-04A Review B

Status: PASS after two remediation cycles

Evidence class: Static + Ran

Reviewer: independent coding-agent reviewer B (Aquinas)

## Initial Finding Set

1. **High — relation closure was global rather than family-specific.** A claim
   `method_id` could name an existing dependency ID and pass because all
   logical IDs shared one lookup set. Every relation must resolve against its
   declared record family.
2. **High — nested schema drift was self-consistent but nonexecutable.** The
   report schema could rename unit `definition` to `meaning`, refresh its
   catalog hash, and pass because only top-level schema fields were compared
   with Rust types.
3. **Medium — schema version constants were not bound.** A companion could
   change the declared `schema_version` or `contract_version` constant to `2`,
   refresh its hash, and pass admission even though the typed loader requires
   version `1`.

## Initial Evidence

Reviewer B demonstrated each finding with an adversarial fixture against the
real loader. The review return itself encountered a platform response error,
but the complete finding text and reproductions were delivered to the parent
before termination. Disposition: **HOLD** pending remediation and an
independent rerun.

## First Re-review

All three original adversarial cases were closed. Reviewer B independently
observed typed errors for cross-family method IDs, nested schema drift, and
schema-constant drift. Focused fmt, Clippy, 24/24 quick Nextest (run
`339e9b66-4ea1-4b92-8d0f-514cdf27196d`), named/all CLI validation, protected
hashes, Draft 2020-12 execution, and line counts passed.

One **Medium** gate blocker remained: the loader accepted bound version
`00.1.0` and ID `-leading-punctuation` after hash reconciliation even though
the companion grammars reject both. Disposition: **HOLD** pending lexical
parity and loader-level regression vectors.

## Terminal Re-review

Disposition: **PASS**. Reviewer B confirmed that all five reconciled-hash
adversarial cases now fail closed: wrong-family relationship, nested schema
drift, schema-constant drift, leading-zero version, and leading-punctuation ID.
Focused fmt and Clippy passed; quick Nextest passed 24/24 (run
`e649c8c0-affd-4682-8f6a-a0fddcea7161`); named/all CLI identity, ASSURE-03
zero-public check, protected hashes, `git diff --check`, and line-count
governance passed. No new actionable finding was reported.
