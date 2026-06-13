# contract implementation evidence

Status: M-C held; M-B contract implementation complete

Evidence mode: Ran + Static

## M-C

No contract edits were made in M-C. The increment reached a contract/design
boundary: current `SC-WATBAL-001` still contains the older MOFE04 single-row
aggregate publication policy, while the staged M-C scope requires per-OFE WAT
semantics or an explicitly contracted equivalent.

Changing that authority without adding a real per-OFE runtime state surface
would only bless surrogate output synthesis, so the increment is held.

## M-B

M-B revised:

- `SC-RUNOFFPART-001` to version 42.
- `SC-WATBAL-001` to version 154.
- `docs/specifications/science-contracts/index.md` review metadata for the touched contracts.

Implemented authority includes separated `UpStrmQ`/`SubRIn` carry, stale aggregate carry purge before MOFE hourly-array execution, positive top-layer saturation excess routing, and the M-B conservation identities.

Validation:
- `cargo test --test mofe01_inter_ofe_route_contract --test wb11_hydrology_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract mofe01_mb -- --nocapture`: PASS.
- `cargo test --workspace`: PASS.
