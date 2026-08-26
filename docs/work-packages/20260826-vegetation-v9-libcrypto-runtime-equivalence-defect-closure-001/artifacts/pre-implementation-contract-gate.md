# Pre-implementation contract gate

Status: passed as an intentional red gate before implementation.

Ran: `check_science_contract_admission.sh --base-ref
f1b0ff9cc40dda5272eaeb87007949af3803a778 --worktree` returned `A0_ADMITTED`
for 49 contracts with authority SHA-256
`a72bfe763938eb5b2471e7ae97b878860218e2e829097201e2174697a45bfbc1`.

Ran: Nextest run `76e61484-e0f3-45ca-beb3-3700b265daf4` executed exactly the
two new V9 acceptance tests before the verifier existed. Both failed solely at
canonicalizing the required
`artifacts/verify_v9_runtime_equivalence.py` path with `NotFound`; compilation
and contract assertions completed. This is the required regression-first proof.

Ran: `cargo fmt --all -- --check` and `git diff --check` passed before the
implementation file was created.
