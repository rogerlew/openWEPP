# Line-Count Governance

Evidence label: Ran.

Command:

```sh
wc -l crates/openwepp-runner/src/bin/openwepp-snowbench.rs tests/integration/snowdensity05f_melt_closure_handoff.rs tests/integration/snowdensity03_physics_bulk_offline_contract.rs
```

Scaffold outcome:

```text
  259 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
  119 tests/integration/snowdensity05f_melt_closure_handoff.rs
  146 tests/integration/snowdensity03_physics_bulk_offline_contract.rs
  524 total
```

Disposition:

- No touched `.rs` file is at or above the 2000-line WARN threshold.
- No 3000-line blocker exists in the declared write set.

After implementation:

```text
649 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
```

The touched production/test host remains below the 2000-line WARN threshold.
