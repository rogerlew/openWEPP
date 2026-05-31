# AUTH06 Verification Agent A

Status: completed  
Evidence mode: Static + Ran

## Scope
1. Verify AUTH06 integration target registration and execution.
2. Verify sidecar integrity checks for active suites.

## Verification results

1. Verified `Cargo.toml` registration:
   - `auth06_fixture_provenance_hash_enforcement_contract`
2. Verified test target:
   - `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
3. Verified test execution:
   - `cargo test --test auth06_fixture_provenance_hash_enforcement_contract`
   - pass
4. Verified each active suite fixture root has:
   - `fixtures.sha256`
   - `fixtures.provenance.yaml`
5. Verified lock manifests pass `sha256sum --check --strict`.

## Result
- pass
