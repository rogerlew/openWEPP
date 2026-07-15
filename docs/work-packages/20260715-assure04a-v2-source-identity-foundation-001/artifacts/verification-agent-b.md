# ASSURE-04A Verification B

Status: PASS — renewed terminal verification

Evidence class: Static + Ran

Verification B proved that the tracked report schema requires nullable fields
to be present while plain Rust `Option<T>` admission treated omission as
equivalent to explicit `null`. A scratch fixture removed the first dependency
`immutable_identity: null`, refreshed the manifest hash, and passed the real
named validation route with exit 0.

Affected families are authorship, dependency, research object, review, and
publication. This violates the fail-closed missing-field gate and the required-
field parity contract. Focused formatting, strict crate Clippy, and 24/24 tests
passed on the defective tree, proving the omission vector was missing.

Required disposition: represent missing, explicit null, and present values
distinctly for every schema-required nullable field; add negative omission
vectors across all affected families; rerun focused gates and the complete
terminal five-gate sequence; then renew both terminal verifications.

Other audited admission, identity, confinement, restriction, CLI, dependency,
protected-surface, CRAP, JUnit, and documentation evidence had no additional
blocking finding. The pre-remediation `v2.rs` hash was
`422b62a30e4863122c51898914202d85b6214ab051188829991a787a1d635345`.

## Remediation Re-review

Historical status: PASS for B-T01 remediation; terminal verification was then
renewed below

The verifier reproduced the exact original omission against the amended source.
Removing the dependency `immutable_identity: null`, refreshing the manifest
hash, and running real validation now exits nonzero with a missing required-
nullable-field error.

Static trace covered all 16 schema-required nullable fields: two authorship,
five dependency, four research-object, one review, and four publication fields.
Every semantic branch either requires explicit null or requires a present
value; missing cannot pass either route.

Independent formatting and strict crate Clippy passed. The five-family omission
test passed in Nextest run `c1038acb-f051-43da-8b49-26be83dd1641`. The amended
`v2.rs` SHA-256 is
`886a5693d67ab88b0b0a6901260017eeca636aa7ccad1ad0faed7ccf24104b58`.
The line-count artifact records and dispositions the 2,042-line warning. Fresh
heavy/CRAP evidence and renewed terminal Verification A/B subsequently passed.

## Renewed Terminal Verification

Evidence class: Static + Ran

Renewed Verification B passed with no remaining implementation, security,
schema, identity, CLI, gate, or documentation finding. The exact original
nullable omission now fails closed, all 16 affected fields trace through the
three-state admission type, and the five-family omission regression passes.

Independent formatting, strict focused Clippy, named/all validation,
zero-public check, report-specific fail-closed commands, and whitespace checks
passed. Focused Nextest passed 25/25 in run
`cc9894aa-2638-4774-af7c-3d387ce0bea0`.

The verifier authenticated the amended 1,986-test JUnit, all 16 fresh CRAP
checksums, zero actionable rows, 223 production and 429 measurement-input
hashes, current `v2.rs` and test identities, protected public bytes, preserved
historical bundles, 2,042-line disposition, and closure-summary truth. It did
not rerun the full workspace or CRAP acquisition; it independently verified
their retained evidence and ran the proportionate focused workflow.

## Final Documentation Closure Confirmation

Status: PASS

After the mechanical completion/roadmap/prompt transition, Verification B
independently reproduced the final 27-row status, 27-path set, 3,258-byte
content manifest, and 42,850-byte full-index diff recorded in the heavy-gate
artifact. All six closure-governance hashes matched; the kickoff moved from
`active/` to `archived/` with unchanged content identity.

Package and disposition are complete, ASSURE-04B remains unscaffolded and
unauthorized, and no production, test, Cargo, v2, protected, or `usersum` byte
drifted after the heavy freeze. `git diff --check` passed. No code workflow was
rerun for this documentation-only confirmation.
