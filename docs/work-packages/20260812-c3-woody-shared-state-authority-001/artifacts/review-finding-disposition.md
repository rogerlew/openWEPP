# Review Finding Disposition

Status: `executing / historical rereviews preserved / collision remediation reviews GO`

Evidence mode: `Static`

All first-review findings are accepted; none is deferred or rejected.

| Finding | Correction | Status |
|---|---|---|
| canonical digest/oracle mismatch | V8 binds injective typed-line encoding; oracle and definition implement it; V3 source retains predecessor JSON/empty-field digest | corrected / PASS |
| ambiguous leaf N | displayed leaf N alone supplies positive-LAI `Nleaf_area`; zero-LAI and donor-N poisons added | corrected / PASS |
| generator published false checks | generator raises before write unless every check is true | corrected / PASS |
| tautological migration/digest evidence | actual two-stratum/two-occupancy migration, 151 whole-state scalar mutations, exact digest-cycle injection, and identity/lineage poisons added | corrected / PASS |
| weak multi-owner migration | two simultaneous invalid source strata report deterministically and return no candidate | corrected / PASS |
| undefined area-cache tolerance | exact left-to-right IEEE-754 bit equality selected | corrected / PASS |
| non-injective path/string framing | UTF-8 length-prefixed lowercase hex components selected | corrected / PASS |
| incomplete schema domains | exact phase enum and GSI `[0,1]` added | corrected / PASS |
| zero-C/positive-display-N fixture | valid zero branch now zeroes displayed C/N; separate executed rejection poison added | corrected / PASS |

Additional rereview findings for strict bool/u128/pending-transfer validation,
distinct V4 configuration rebinding, exact stratum/occupancy membership,
transfer lineage, and canonical single-LF definition bytes were accepted,
corrected, and independently verified PASS.

Reviewer B's later exact-byte rereview found that the positive fixture used the
unsupported receiver spelling `litter_metabolic` and that the independent
validator accepted arbitrary donor/receiver strings and zero transaction or
proposal identities. This HIGH finding is accepted. The positive fixture now
uses the imported typed receiver `metabolic`; validation admits exactly the six
`MaterialDonorClass` and four `MaterialReceiverClass` serde identities, requires
positive nonzero transaction/proposal `u128` values, and executes poisons for
unsupported donor, unsupported receiver, zero transaction, and zero proposal.
Reviewer A then required canonical contract/definition binding and Reviewer B
required exact-width upper-bound poisons. Both findings were accepted and
corrected: V8 and the definition bind positive `u128` transaction and positive
`u64` proposal IDs plus exact typed enum sets; `2^128` and `2^64` poison vectors
reject. Both final rereviews returned GO against definition `571bac78...`,
fixture `6862b507...`, and generator `5ac8dfea...` with no unresolved material
finding.

## Post-release accepted finding

The implementation review subsequently demonstrated that the released oracle
represented an occupancy as the flattened key `stratum_id@tile_id`. This is
accepted as a material authority/evidence defect: arbitrary admitted UTF-8 IDs
make that representation non-injective, and it does not reproduce the existing
production structural encoder.

The remediation changes no constitutive science and does not edit production
Rust. V8 now binds a structural occupancy array sorted by the typed UTF-8 pair;
the oracle, fixture, and definition expose independent length-framed identity
components, exact whole-state preimage bytes, delimiter-collision and arbitrary
UTF-8/control vectors, duplicate-pair rejection, lane-order normalization, and
identity-field mutation digests. No part of the finding is deferred or
rejected. Fresh independent science rereviews A and B are GO with no unresolved
material finding against definition `8ace38d1...`, fixture `3072226f...`, and
generator `422f0a6f...`. Reviewer A first found and preserved a material HOLD
because the V4 hashing helper sorted but did not reject duplicate structural
pairs. The finding was accepted; the canonicalizer now rejects before digest
and the fixture executes the V4 duplicate-state poison. Terminal verification
remains pending.
