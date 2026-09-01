# V47 atomic complete-owner transaction posture QA review

Status: `APPROVE`

Evidence mode: `Static + Ran`

Reviewer role: independent secondary Rust QA review

## Findings

No remaining V47 closure-blocking finding.

- `MEDIUM`, resolved during review —
  `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution_stack_helpers.rs`:
  the inserted parent-end continuation close initially carried duplicate
  `#[allow(clippy::too_many_arguments)]` attributes and failed
  `clippy::duplicated_attributes`. One duplicate was removed. A targeted
  library Clippy run with `-D clippy::duplicated_attributes` now passes.
- `MEDIUM`, resolved during review —
  `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`:
  the first source-contract assertion matched the ambiguous substring
  `permits inferred numeric adjacency`. It now binds the exact negative
  authority, `Neither posture permits inferred numeric adjacency`.
- `LOW`, resolved during review —
  `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs`:
  the same-ID positive vector initially exercised only the private typed
  posture. It now also executes the ordinary public
  `install_soil_thermal_accepted_v2` path and compares the installed resident
  with the independently constructed accepted resident.

## QA assessment

Static review confirms that the ordinary/public and authoritative-beginning
install paths pass no split authority. Only the authenticated unpublished-
continuation path constructs and passes the explicit native-V2
`PhysicalSoilEnergyTransactionAuthorityV2`. Installation re-authenticates
that authority from the mutually equal vegetation/LSE/BGC source owners and
the continuation/prepared target, validates the accepted resident and its
target-sealed state/layers, and requires the exact authenticated predecessor
join for a split. The posture contains no transaction arithmetic or adjacency
inference.

The focused behavior is substantive:

- the same-ID vector executes the real public install;
- the successor vectors require explicit source/target authority and exact
  predecessor equality, then install the composed second child while retaining
  the outer source owners and the authoritative beginning;
- foreign, swapped, missing-authority/predecessor, and each individual outer-
  owner disagreement fail closed;
- retained V39 vectors exercise continuation/physical-trial substitution,
  support, transaction, receipt-chain, operand/carry, accepted candidate,
  external seal, exact no-op, and authoritative-beginning poisons;
- refusal checks compare pre/post native-V2 canonical owner bytes and source
  transactions, while the production path validates before clone and performs
  only clone-then-replace after complete validation;
- no `DFF_V47` production diagnostic or V47 persisted schema/state was found.

The V47 API is long but explicit: source/target authority construction is
separate from install, the install re-authenticates rather than trusting the
argument, and generic callers cannot opt into split custody. This is preferable
to a boolean or inferred successor mode and remains cohesive with the existing
V39 continuation API.

## Ran evidence

- `nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(/v39_/) | test(/v46_/) | test(/v47_/)'` — Nextest
  `a1cc0069-1758-4bb1-a866-a4aac68d5d4a`, `29/29 PASS`.
- After finding correction,
  `nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(/v47_/)'` — Nextest
  `ac9f04c8-fe9a-4ac7-a2db-3695b59775d3`, `15/15 PASS`.
- After finding correction,
  `nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v47_/)'` — Nextest
  `31dd9f6b-39fe-4346-9250-b922e5cccac0`, `2/2 PASS`.
- `nix develop -c cargo fmt --all --check` — `PASS`.
- `git diff --check` over the six V47 production/test/call-site files —
  `PASS`.
- `nix develop -c cargo clippy -p openwepp-hillslope-orchestrator --lib --no-deps -- -D clippy::duplicated_attributes` — `PASS` for the corrected V47
  lint; the command reports pre-existing warning debt but no denied duplicate-
  attribute error.
- Static diagnostic/adjacency scan — no `DFF_V47`, target/source transaction
  arithmetic, `wrapping_add(1)`, or `saturating_add(1)` in the V47 posture.

## Line-count and non-blocking follow-ups

Ran terminal counts for the V47 files are:

- `v10_soil_thermal_v2.rs`: 2,382 (`WARN`);
- `v10_soil_thermal_v2_tests.rs`: 2,591 (`WARN`);
- `v9_real_consumer_shadow.rs`: 2,599 (`WARN`);
- `v11_covered/owner_finalization.rs`: 2,865 (`WARN`);
- `snow_stage3_v11_adaptive_execution_stack_helpers.rs`: 1,585 (`PASS`);
- `snow_terminal_enthalpy_event_numerics_contract.rs`: 1,443 (`PASS`).

No file reaches the 3,000-line closure boundary. The existing V43 exact-move
intent for the V2 unpublished fixed-point/continuation source and tests remains
binding. Before either additional warning file reaches 3,000 lines, extract
the deferred native-V2 continuation consumer/finalizer blocks from
`v9_real_consumer_shadow.rs` and `owner_finalization.rs` into their existing
sibling-module families. V47 adds only narrow authority plumbing at those
call sites, so a simultaneous structural move is not warranted in this
custody correction.

The package-wide `cargo clippy --workspace --all-targets --all-features -- -D
warnings` gate is not green on the shared dirty worktree: independent QA
observed broad warning debt in other active package files. That remains a
required `WGHL-CLIPPY-001` package-closure gate and must not be represented as
passed by this focused approval. `cargo deny` was not selected for this V47
increment because V47 changes no manifest, dependency, license, or advisory
surface.

## QA disposition

`APPROVE` for V47 implementation and continued R122 qualification. The exact
same-ID and authenticated-successor alternatives are readable, fail closed,
and supported by positive, poison, rollback, retained-regression, and source-
authority evidence. This focused approval does not close the wider WGHL
package or waive its remaining workspace Clippy, full-regression,
qualification, dual-verification, and final-disposition gates.
