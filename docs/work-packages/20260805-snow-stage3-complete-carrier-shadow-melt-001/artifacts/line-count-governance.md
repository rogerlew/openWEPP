# Line-Count Governance

Static: current shadow increment.

`runoff_reconciliation.rs` is `3,177` lines after the within-day shadow
implementation, above the `3,000`-line closure threshold. The package remains
executing and cannot close until the shadow solver is extracted into a bounded
submodule (or the file otherwise falls below the threshold) and focused gates
are rerun. This open structural obligation does not change the scientific hold
on persistent cross-day shadow state, thin-pack disposition, and same-substep
liquid routing.
