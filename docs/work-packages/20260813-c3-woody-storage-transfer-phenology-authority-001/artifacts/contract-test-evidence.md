# Contract Test Evidence

Ran:

```text
cargo nextest run --test vegetation_boundary_authority_contract --profile quick
Summary: 25 passed, 0 skipped
```

The suite verifies definition and section digests, immutable V1--V6 bytes,
byte-identical oracle regeneration, all six tissue/element owners, event
branches, same-interval exclusion, evergreen and migration semantics, computed
poisons, and rollback hashes.
