# Review Finding Disposition

Evidence: `Static + Ran`

Both independent reviewers return `PASS` on the corrected tree.

| Finding | Disposition |
| --- | --- |
| The initial proposal suspended exchange when only the lower thermal volume was below `1 kg m^-2`. | Accepted and corrected to the exact `_calc_layers.c` semantics: total-pack `<= 1 kg m^-2` suspends before partition; lower-volume `< 1 kg m^-2` collapses to one volume and continues exchange. |
| The collapse path initially conflicted with the ordinary `0.25 m` active-volume maximum. | Accepted. `SC-SNOWENERGY-001` now states the bounded collapse exception and tests preserve the ordinary resolved-pack rule outside that branch. |
| Native threshold equality could be lost after conversion to mass units. | Accepted. Branch comparisons use the authoritative native SWE threshold `0.001 m`, with next-down, equality, and next-up tests. |
| Persistent refrozen-liquid state was overwritten during projection. | Accepted and corrected from assignment to additive accumulation; nonzero liquid/refrozen conservation twins cover the path. |
| The replay initially lacked sufficient source-day and execution identity to support the closure claim. | Accepted. Replay v2 binds the source boundary window, rejected processing day, binary, source diff, tool, manifest, fixtures, runfiles, runner, and traces, with locking and post-run hash verification. |
| Line-count and calibration records became stale after the final implementation. | Accepted and reconciled to two authoritative branches, four diagnostics, and implementation status `IMPLEMENTED`. |
| The contract reference named a nonexistent Binding Exposure Index identifier. | Terminal-verifier finding accepted and corrected to `SNOWENERGY-EB04C-THERMAL-DOMAIN`. |
| Replay evidence could imply the later-rebuilt target binary still had the replay hash. | Terminal-verifier finding accepted. The evidence now says replay-captured binary and explains the pre/post-run hash binding. |
| The security note described all replay outputs as target-local. | Terminal-verifier finding accepted. Transient outputs remain under `target/`; the accepted JSON report is package-local. |
| `GAP-SNOWENERGY-008` and lifecycle catalogs still described runtime evidence as pending. | Terminal-verifier finding accepted and resolved by the 22/22 replay, review, verification, and terminal gates. |

No finding was deferred, no numerical guard was weakened, and no calibration
coefficient or phase/mass ownership rule changed. Both terminal verifiers
rechecked the technical tree and returned `PASS`.
