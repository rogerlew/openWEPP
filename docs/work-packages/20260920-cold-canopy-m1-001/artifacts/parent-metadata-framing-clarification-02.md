Reviewer: /root/metadata_correctness, same independent correctness conversation.

Yes. A provider-owned domain under the narrow SC-VEGETATION private diagnostic authority is compliant without changing SC-COUPLEDTIME’s closed tag list.

The controlling distinction is:

- SC-COUPLEDTIME lines 118–123 assign calendar semantics to provider authority; coupled time checks the supplied receipt identity and chronology.
- Its closed tags define identities constructed by coupled time, including `parent-interval`.
- The private provider may construct its own receipt identity with the same `OPENWEPP_CANONICAL_FRAMED_SHA256_V1` byte convention and a provider-owned domain name.
- Coupled time then consumes only that resulting 32-byte value through its existing `calendar_receipt` or `forcing_receipt` field. The `parent-interval` framing and field order remain unchanged.

The correction should explicitly say the provider domains are SC-VEGETATION diagnostic domains, never coupled-time domains, and are not additions to or aliases for the coupled-time closed tag set. Raw SHA-256 remains appropriate for source-file content hashes because those are content digests, not receipt identities. The provider receipt identities themselves require framing.

This stays within the adopted envelope. It does not require a coupled-time contract amendment and must not repurpose `parent-receipt` or any other existing coupled-time tag.
