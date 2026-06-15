# CQR04 Routing Equivalence

Ran before refactor:

```text
cargo test --test ws11_channel_routing_physics_equivalence_contract
cargo test --test ws10_watershed_kernel_contract
```

Results before refactor:

- `ws11_channel_routing_physics_equivalence_contract`: 44 passed.
- `ws10_watershed_kernel_contract`: 5 passed.

Ran after refactor:

```text
cargo test --test ws11_channel_routing_physics_equivalence_contract
cargo test --test ws10_watershed_kernel_contract
cargo test --workspace
```

Results after refactor:

- `ws11_channel_routing_physics_equivalence_contract`: 44 passed.
- `ws10_watershed_kernel_contract`: 5 passed.
- `cargo test --workspace`: passed.

Static preservation checks:

- No science-contract files changed.
- No parser, runner, output writer, or non-target production file changed.
- Routing constants, thresholds, guard IDs, and symbol names were preserved.
- `clippy::too_many_lines` suppressions were removed from the target file.

Disposition: routing equivalence evidence pass for package scope.
