# contract implementation evidence

Status: M-F-REDO2 executed; active per-OFE handoff, anti-clone evidence, and
public `QOFE`/`Q` geometry normalization implemented

Evidence mode: Ran + Static

## M-F-REDO2

M-F-REDO2 amends the M-F/M-F-REDO/M-F-REDO-CLONE authority with explicit public
runoff-normalization requirements:

- `SC-WATBAL-001` version 159 pins public `QOFE = runoff * efflen / slplen`,
  public `Q = runoff * efflen / totlen`, and the downstream `QOFE == Q`
  alias rejection.
- `SC-SYSTEM-001` version 82 pins the corresponding manifest/consumer gate.
- Per-OFE WB13 publication receives raw routed runoff and explicit
  OFE-local/cumulative publication geometry.
- Internal WB13 conservation identities use raw transfer/runoff operands
  rather than the public `Q` normalization.

Validation:

- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- H1/H6/H9/H11 smoke row cardinality, active handoff, anti-clone, and
  `QOFE/Q` geometry audits: PASS.
- H1/H6/H9/H11 local semantic comparisons: command PASS with complete row-key
  coverage; value-family semantic pass remains false and is classified outside
  the M-F-REDO2 publication-normalization gate.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28: PASS, 28/28 byte-identical to
  M-F-REDO-CLONE single outputs.

## M-F-REDO

M-F-REDO amends the M-F/M-E4-REDO authority with explicit anti-clone and active
handoff requirements:

- `SC-WATBAL-001` version 157 adds `INV-WATBAL-098`.
- `SC-SYSTEM-001` version 80 adds `INV-SYSTEM-031`.
- Multi-OFE lane runtime surfaces are rebuilt from OFE-local static inputs
  instead of cloned aggregate surfaces.
- WB14 current surface carry, WB19 lateral carry, and WB12 same-pass runon
  storage reconciliation now make real inter-OFE handoff active.
- Runtime manifests and public WAT rows preserve M-F row cardinality and
  storage-lineage metadata.

The contract implementation remains incomplete because public `QOFE` still
aliases public `Q` on real H1/H6/H9/H11 smoke runs. The pinned baseline writes
public `Q` with `efflen/totlen` and public `QOFE` with `efflen/slplen`; current
per-lane execution has not yet carried that geometry authority into
publication.

Validation:

- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- H1/H6/H9/H11 smoke row cardinality, active handoff, and anti-clone audit:
  PASS.
- H1/H6/H9/H11 `QOFE != Q` geometry audit: FAIL.

## M-F

M-F consumes the M-E0/M-E4-REDO authority without amending canonical contracts.
The implementation publishes public WAT/WB13 rows from internal per-OFE records
instead of splitting aggregate WB13 rows:

- multi-OFE runner paths append rows from `DailyInternalPerOfeWb13Collection`;
- aggregate single-OFE publication remains unchanged;
- publication provenance validates `day_count * contributor_ofe_count` row
  cardinality, grouped OFE keys, and per-OFE storage lineage;
- per-OFE `QOFE` routes through the current transfer output source, but
  M-F-REDO later proved the value still aliases public `Q` under one-OFE lane
  `efflen`/`slplen` seeding;
- watershed contributor validation checks the new per-OFE metadata shape.

The contract implementation remains incomplete because the authoritative
surface export producer still publishes zero current surface carry on real
multi-OFE runs. M-F-REDO must close that producer before the M-F publication
transition can be accepted.

Validation:

- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- H1/H6/H9/H11 smoke row cardinality/provenance: PASS.
- H1/H6/H9/H11 surface `UpStrmQ` acceptance: FAIL.

## M-E4-REDO

M-E4-REDO amends `SC-WATBAL-001` to version 156 and implements the
non-tautological internal WB13 identity acceptance rules:

- `TOL-WATBAL-007 <= 1e-11 mm` is the internal per-OFE WB13 residual tolerance,
- per-element residuals use pre-day OFE dynamic storage snapshots and post-day
  WB13 storage, including frozen water,
- row/input matching remains structural evidence only,
- adjacent transfer residuals compare upstream sent records against downstream
  received records,
- manifests expose internal record counts, expected counts, identity statuses,
  and residual maxima.

M-E4-REDO deliberately preserves public aggregate WB13/WAT publication. M-F owns
the public publication policy flip.

Validation:

- `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture`: PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- Required H smoke H1/H6/H9/H11: PASS runtime execution and identity manifest
  audit.

## M-E3

M-E3 consumes the M-E0/M-D contract authority without amending canonical
contracts. The increment persists dynamic OFE-local writeback state across
days behind the sequential executor:

- multi-OFE runner paths allocate one persistent writeback surface per OFE,
- daily climate and scheduler seed surfaces are applied to each lane before
  execution,
- ordered lane execution uses the M-E2 transfer input/output seam,
- persistent lane state is replaced only after a full same-day sequence
  succeeds,
- aggregate WB13/WAT publication remains the current public policy with
  `per_ofe_record_count = 0`.

M-E3 makes dynamic state real but deliberately does not make per-element or
transfer identities measurable yet. M-E4 must produce internal per-OFE WB13
records from this state; M-E5 owns public WAT publication.

Validation:

- `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`:
  PASS.
- Required H smoke H1/H6/H9/H11: PASS runtime execution.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28: PASS byte-identical to M-E2
  outputs.

## M-E2

M-E2 consumes the M-E0/M-D contract authority without amending canonical
contracts. The increment adds executor wiring for the already-declared
`TransferInput`/`TransferOutput` semantics:

- same-day upstream transfer arrays are overlaid before each OFE lane run,
- current-lane transfer arrays are extracted after the lane run,
- source/recipient identity and malformed transfer values fail closed,
- downstream area-ratio scaling is explicit in the transfer input,
- stale current-lane output arrays are cleared before each lane run so missing
  fresh output fails closed,
- non-finite or overflowed transfer totals fail closed,
- dynamic state persistence and per-OFE daily WB record production remain
  later M-E gates.

The M-E2 focused tests prove the required two-OFE synthetic handoff and
malformed-array rejection without changing WAT publication. Final H1-H36
runtime replay and no-publication-flip audit confirm the executor increment
does not perturb the current public aggregate WB13/WAT path.

## M-E1

No science-contract text was changed in M-E1. The implementation lands against
the M-E0 authority already installed in:

- `SC-RUNOFFPART-001#INV-RUNOFFPART-029`,
- `SC-WATBAL-001#INV-WATBAL-097`,
- `SC-SYSTEM-001#INV-SYSTEM-030`.

M-E1 implements the data-model/shadow-state subset only. It preserves the
M-E0 contract prohibition on deriving multi-OFE records from aggregate WB13/WAT
state by constraining the legacy aggregate adapter to the N=1 case and by
keeping runner manifests at `per_ofe_record_count = 0` until real dynamic
records exist.

Validation:

- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- `cargo test --workspace`: PASS.

## M-E0

M-E0 reopened the contract gate declared by M-D and amended the three canonical
contracts before any production implementation:

- `SC-RUNOFFPART-001`: version 43, `last_reviewed: 2026-06-13`.
  - Added `INV-RUNOFFPART-029`.
  - Added `MOFE01 M-E0 Per-OFE Runoff Lane-State Addendum`.
  - Bound ordered OFE lane execution, typed `TransferInput`/`TransferOutput`,
    no aggregate handoff synthesis, no `TopologyGraph` OFE-node encoding, and
    single-OFE bit-identical anchors before publication reshaping.
- `SC-WATBAL-001`: version 155, `last_reviewed: 2026-06-13`.
  - Added `INV-WATBAL-097`.
  - Added `MOFE01 M-E0 Per-OFE Dynamic Water-Balance State Addendum`.
  - Bound OFE-keyed daily records, dynamic state-family lineage, transfer
    bindings, aggregate derivation limits, and publication-policy transition
    constraints.
- `SC-SYSTEM-001`: version 79, `last_reviewed: 2026-06-13`.
  - Added `INV-SYSTEM-030`.
  - Added `MOFE01 M-E0 Per-OFE Dynamic-State Publication Policy Addendum`.
  - Bound the future policy value
    `per-ofe-dynamic-water-balance-state`, row cardinality, identity-status
    manifest gates, and `storage_lineage_policy = "per-ofe-dynamic-wb-state"`.
  - Corrected the stale header version 77 while preserving the existing version
    78 revision row.
- `docs/specifications/science-contracts/index.md`:
  - Updated `Last updated: 2026-06-13`.
  - Updated the registry review dates for the three touched contracts.

Validation:

- `cargo test --test mofe01_per_ofe_state_contract mofe01_me0_contract_authority_is_present -- --nocapture`: PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --path docs/specifications/science-contracts/index.md --format plain`: PASS; final post-evidence run validated 35 files with 0 errors and 0 warnings.

## M-D

No contract edits were made in M-D. The increment intentionally stopped at a
design artifact and named the M-E0 contract amendments required before
production implementation:

- `SC-RUNOFFPART-001`: per-OFE lane execution, runon continuation, transfer
  arrays, and no aggregate substitution.
- `SC-WATBAL-001`: per-OFE dynamic WB state rows, row cardinality, `QOFE`,
  `UpStrmQ`, `SubRIn`, and storage aggregation semantics.
- `SC-SYSTEM-001`: per-OFE dynamic-state publication policy, manifest
  evidence, and downstream fail-closed intake behavior.

## M-C2

No contract edits were made in M-C2. The increment reached the same authority
boundary one layer earlier than publication: current contracts do not define an
OFE-keyed daily WB output state surface, and current code has no such surface
to implement against.

The M-C2 scoping evidence proves that the existing MOFE hourly arrays are
hour-indexed transfer/copy-forward state. They are insufficient authority for
per-OFE daily water-balance rows or per-element/transfer identity measurement.
Changing `SC-WATBAL-001` without adding real state would only make the missing
state look contracted.

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
