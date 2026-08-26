# Contract implementation evidence

Status: complete.

Static: `SC-VEGETATION-001@29` now preserves the exact V9 descriptor as
generation-host provenance and admits active equivalence only for the sole
`libcrypto.so.3` mismatch under `INV-VEGETATION-133` / `VEG-E-133`. It requires
loaded-provider identity, SHA-256 known-answer and streaming proof,
all-other-runtime exactness, protected-byte immutability, and complete-output
byte equality. The lifecycle index records v29 without weakening v28.
