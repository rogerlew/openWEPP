# disposition

Status: M-D complete; package remains open for M-E contract/test scaffolding

Evidence mode: Ran + Static

## Disposition

Increment M-D is complete as a design-only architecture increment. It produced
`mofe-per-ofe-state-architecture.md`, which defines the target
`PerOfeDailyWaterBalanceCollection`, selects per-OFE lane iteration over the
existing scheduler phase graph, maps legacy `irs`/`rochek` continuation
obligations, names the contract amendments needed in M-E0, and breaks M-E into
measurable sub-increments.

No production code, science contract, or test was edited for M-D. The next
lawful increment is M-E0 contract/test scaffolding for real per-OFE dynamic
state. Do not publish per-OFE WAT rows by splitting the current aggregate WB13
row.

## M-C2 disposition

Increment M-C2 was executed end-to-end through the scoping, comparison, and
evidence boundary and is held. The existing MOFE hourly carry arrays are real
transfer state, but they are not per-OFE daily WB output state. Current
writeback remains aggregate scalar maps, and current WB13/WAT publication still
emits one `OFE=1` row/day with `UpStrmQ=0` and `QOFE=Q`.

No production code, science contract, or test was edited for M-C2. Implementing
per-OFE daily rows from the available aggregate WB13 state would be surrogate
physics. The next lawful implementation path must design and contract real
per-OFE dynamic state before M-F can publish it.

Local comparisons were run without the comparator subagent under explicit
operator direction because GPT-5.3-Codex-Spark weekly quota was exhausted.

## M-C disposition

Increment M-C was executed end-to-end through the current boundary and is held.
The H1-H36 runner execution gate remains green, and the single-OFE anchor stayed
byte-identical to M-B. Local comparison was run without the comparator subagent
under explicit operator direction because GPT-5.3-Codex-Spark weekly quota was
exhausted.

M-C publication closure is not complete. The current implementation still emits
single-row canonicalized aggregate WAT rows for every hillslope. All 29
multi-OFE surfaces fail the M-C row-shape, downstream `UpStrmQ`, and `QOFE != Q`
publication gates.

No production code was changed for M-C. Implementing the requested WAT shape
from the currently available aggregate daily state would require surrogate
per-OFE hydrology rows. That is not allowed by this package's correctness
posture.

## M-B disposition

Increment M-B retired the hydrology execution blocker, but it is not full
closure acceptance. The M-A valid-input blocker is retired: the full
arboreal-dendrite H1-H36 cohort now completes, including all 29 multi-OFE
surfaces that previously failed before WAT publication.

M-B implemented contract-pinned separated upstream surface/lateral carry,
current saturation carry addback, stale aggregate carry purge, and targeted
fail-closed guards. Focused M-B tests, full H1-H36 execution, single-OFE
byte-identical anchors, full workspace tests, clippy, fmt, and cargo-deny were
green at the M-B boundary.

The full three-identity gate is not proven. Published aggregate surfaces
conserve annually at noise on smoke representatives, but transfer and true
per-element identities require per-OFE output and remain blocked by the M-C
publication-state boundary.

Local owcmp comparison was run without the comparator subagent per operator
direction. Execution passed, but semantic comparison remains failed due
structural row-key/per-OFE WAT publication mismatch.

## M-A disposition

Increment M-A is complete as a characterization and scoping increment. It did not edit production code.

The current executable blocker is confirmed: all 29 multi-OFE H surfaces fail in `runoff_reconciliation` before WAT publication. The seven 1-OFE H surfaces complete and publish single-row aggregate WAT outputs.

Legacy calibration is complete for the available H1-H36 WAT files. The expected 15-OFE far-point WAT surface is not present on disk; `pw0.slp` is inventory-only for M-A.

## Next disposition

Execute M-E0: amend the relevant science contracts and install failing
contract-derived tests for the per-OFE daily state semantics declared by M-D.
M-E production code should start only after that authority and red-test surface
exists.
