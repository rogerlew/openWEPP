# disposition

Status: M-E4 executed-hold (Claude review: identity checks tautological — see m-e4 evidence); package remains open for M-E5

Evidence mode: Ran + Static

## Disposition

Increment M-E4 is complete for its internal WB13 record scope. Multi-OFE
hillslope runs now produce internal per-OFE WB13 daily records from the
persisted OFE lane state. The M-E4 record path checks transfer input identity,
per-element storage identity, and aggregate internal-transfer cancellation
before persistent lane state replacement.

Required H smoke passed for H1/H6/H9/H11 under
`/tmp/openwepp_mofe01_me4_runtime_smoke`. The manifest audit reports internal
record counts equal to `row_count * contributor_ofe_count` for the 5-, 3-, 4-,
and 2-OFE smoke cases and all three residual maxima at `0.0` mm. Local owcmp
was run directly, without the comparator subagent, and command execution passed
for each smoke surface. Semantic comparison still fails at the expected public
aggregate-publication boundary (`semantic_pass_count=0/1` per smoke surface;
focus columns zero diff).

Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 remain byte-identical to M-E2
for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass).

M-E4 preserves public aggregate WB13/WAT publication:
`publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`. The
next lawful increment is M-E5: build public WB13/WAT rows from the internal
per-OFE records and retire the transitional aggregate publication lifecycle.

## M-E3 disposition

Increment M-E3 is complete for its dynamic state persistence scope. Multi-OFE
hillslope runs now carry an `OfeLanePersistentStateSequence` across days behind
the sequential OFE executor. Each daily multi-OFE shadow lifecycle overlays the
current climate surface, seeds WB11/calendar/PL runtime symbols, executes the
ordered OFE lane sequence, and replaces persistent lane state only after the
full sequence succeeds.

M-E3 preserves public aggregate WB13/WAT publication. Smoke manifests for
H1/H6/H9/H11 report `per_ofe_state_policy=persistent-dynamic-state-shadow`,
dynamic per-OFE state flags true, `per_ofe_record_count=0`, and
`publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`.
Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 remain byte-identical to M-E2
for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass).

Required M-E3 H smoke passed for H1/H6/H9/H11 under
`/tmp/openwepp_mofe01_me3_runtime_h1`. Local owcmp was run directly, without
the comparator subagent, and command execution passed for each smoke surface.
Semantic comparison still fails at the expected aggregate-publication boundary
(`semantic_pass_count=0/1` per smoke surface; focus columns zero diff).

Full H1-H36 replay was not rerun after M-E3 runner wiring. The staged M-E3
gate names H1/H6/H9/H11 smoke execution; full-cohort replay under the new
N-lane shadow path is debug-mode expensive and remains M-E6/performance
hardening scope.

The next lawful increment is M-E4: produce authoritative internal per-OFE WB13
records from the persisted lane state so per-element and transfer identities
become measurable. M-E3 must not be treated as publication or identity closure.

## M-E2 disposition

Increment M-E2 is complete for its sequential OFE lane executor scope. It added
ordered same-day OFE lane execution around the existing scheduler phase graph,
explicit `TransferInput` overlay, explicit `TransferOutput` extraction,
downstream area-ratio scaling, stale current-output rejection, finite/nonnegative
transfer validation, overflowed-total rejection, and focused two-OFE synthetic
handoff tests.

M-E2 did not persist dynamic per-OFE state across days, produce per-OFE WB13
records, or flip WAT publication. Final H1-H36 execution passed under
`/tmp/openwepp_mofe01_me2_final`, single-OFE anchors remained byte-identical to
M-E1, and the no-publication-flip audit passed. Local owcmp command execution
passed without the comparator subagent, while semantic comparison still fails
at the expected publication boundary (`semantic_pass_count=0/36`,
`structural_row_key_failures=350720`).

The next lawful increment is M-E3: persist OFE-local dynamic daily state behind
the executor without manufacturing per-OFE records from aggregate WB13/WAT
rows. M-E2 must not be treated as routing identity closure; per-element and
runtime transfer identities remain blocked until real dynamic records exist.

## M-E1 disposition

Increment M-E1 is complete for its data-model shadow-state scope. It added the
typed per-OFE daily water-balance record/collection model, transfer
input/output payloads, static per-OFE lane slices, manifest shadow-state
provenance, and focused tests. It also fixed review findings by separating
static slice count from dynamic record count, making the legacy aggregate
adapter N=1-only, strengthening transfer source/recipient validation, and
removing stale exact `SC-WATBAL-001` version pins in two unrelated authority
tests.

M-E1 did not populate dynamic per-OFE runner records and did not flip WAT
publication. Final H1-H36 execution passed, single-OFE anchors remained
byte-identical, and the no-publication-flip audit passed. Local owcmp command
execution passed, while semantic comparison still fails at the expected
publication boundary (`semantic_pass_count=0/36`,
`structural_row_key_failures=350720`).

At the M-E1 boundary, the next lawful increment was M-E2 executor wiring.

## M-E0 disposition

Increment M-E0 executed the contract/test scaffold end-to-end and is held at
the intended red architecture gate. The three contracts now define the M-D
per-OFE dynamic-state authority:

- `SC-RUNOFFPART-001#INV-RUNOFFPART-029`,
- `SC-WATBAL-001#INV-WATBAL-097`,
- `SC-SYSTEM-001#INV-SYSTEM-030`.

`mofe01_me0_contract_authority_is_present` passes. The full
`mofe01_per_ofe_state_contract` target fails because current production code
lacks structural per-OFE daily state records, transfer input/output payloads,
and publication-policy manifest gates. These failures are the required M-E0 red
tests, not unexpected regressions.

No production Rust implementation path was edited. No runtime comparison was
run because M-E0 changed no runtime behavior and the acceptance boundary is the
contract-derived red test. No comparator subagent was used.

The next lawful increment is M-E1: introduce the per-OFE daily state data model
or explicitly contracted equivalent without synthesizing records from aggregate
WB13/WAT rows.

M-E0 is intentionally not a green/mergeable closure state because the new test
target is normally registered in `Cargo.toml` and fails by design until M-E1.

## M-D disposition

Increment M-D is complete as a design-only architecture increment. It produced
`mofe-per-ofe-state-architecture.md`, which defines the target
`PerOfeDailyWaterBalanceCollection`, selects per-OFE lane iteration over the
existing scheduler phase graph, maps legacy `irs`/`rochek` continuation
obligations, names the contract amendments needed in M-E0, and breaks M-E into
measurable sub-increments.

No production code, science contract, or test was edited for M-D. At the M-D
boundary, the next lawful increment was M-E0 contract/test scaffolding for real
per-OFE dynamic state; M-E0 has since installed that scaffold and preserved the
ban on publishing per-OFE WAT rows by splitting the current aggregate WB13 row.

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

Execute M-E3 per the M-D sub-increment breakdown. Preserve the M-E2
no-publication-flip boundary until real dynamic per-OFE records are populated
and identity tests can measure OFE-local state and adjacent transfer handoff.
