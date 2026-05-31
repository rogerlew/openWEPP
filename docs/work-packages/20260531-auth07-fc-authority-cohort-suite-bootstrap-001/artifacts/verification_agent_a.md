# AUTH07 Verification Agent A

Status: completed  
Evidence mode: Static + Ran

## Scope
1. Verify AUTH07 integration target registration and execution.
2. Verify AUTH07 fixture lock/provenance sidecars and hash checks.

## Verification results

1. Verified `Cargo.toml` registration:
   - `auth07_fc_authority_cohort_contract`
2. Verified test target:
   - `tests/integration/auth07_fc_authority_cohort_contract.rs`
3. Verified test execution:
   - `cargo test --test auth07_fc_authority_cohort_contract`
   - pass
4. Verified AUTH07 fixture root has:
   - `fixtures.sha256`
   - `fixtures.provenance.yaml`
5. Verified lock manifests pass `sha256sum --check --strict`.

## Result
- pass
