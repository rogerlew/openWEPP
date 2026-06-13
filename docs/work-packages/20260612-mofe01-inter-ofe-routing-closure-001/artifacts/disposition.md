# disposition

Status: M-B complete; package remains open for M-C publication closure

Evidence mode: Ran + Static

## Disposition

Increment M-B is complete as a hydrology route-closure increment. The M-A valid-input blocker is retired: the full arboreal-dendrite H1-H36 cohort now completes, including all 29 multi-OFE surfaces that previously failed before WAT publication.

M-B implemented contract-pinned separated upstream surface/lateral carry, current saturation carry addback, stale aggregate carry purge, and targeted fail-closed guards. Focused M-B tests, full H1-H36 execution, single-OFE byte-identical anchors, full workspace tests, clippy, fmt, and cargo-deny are green.

Local owcmp comparison was run without the comparator subagent per operator direction. Execution passed, but semantic comparison remains failed due structural row-key/per-OFE WAT publication mismatch. That is the expected M-C closure target.

## M-A disposition

Increment M-A is complete as a characterization and scoping increment. It did not edit production code.

The current executable blocker is confirmed: all 29 multi-OFE H surfaces fail in `runoff_reconciliation` before WAT publication. The seven 1-OFE H surfaces complete and publish single-row aggregate WAT outputs.

Legacy calibration is complete for the available H1-H36 WAT files. The expected 15-OFE far-point WAT surface is not present on disk; `pw0.slp` is inventory-only for M-A.

## Next disposition

Proceed to M-C: per-OFE WAT publication closure. Do not tune hydrology to legacy numeric deltas before row-key/publication semantics are contracted and implemented.
