# Native context restoration correctness review

Reviewer: `/root/restoration_correctness` (independent correctness replacement)

Status: **FINAL — HOLD / NATIVE RESTORATION INCOMPLETE**

Scope: derived reader cut
`/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916`,
the exact retained source and the member-backed export
`corrected-recorder-export-20260916-1`. The stable archive execution is bound to
current-context source SHA-256
`745262fb768f13602569654719888cdba408b71172c7a53c5ba1aab443dadc14`
and restart source SHA-256
`7ba3df2f597a4bd186a023f9486b2c713518a2e8dde8611a2422c29281628735`.
The final narrowed audited component source has tree SHA-256
`711a4300e5317a78444e4ea1bd91962e6766992d47d58dffde6a7dffaa864c40`,
patch SHA-256
`d38956e1b14f941642ea05e2f14b50fbc0d655894a6687e38ecc6454438a3c66`,
current-context source SHA-256
`6ad37681a909e10f75f63f196d88fa6f36e1190ebf7d0ea76adfd963020235f2`,
and frozen binary SHA-256
`9741b3972937ac2f04642e57d68308cc891ba39fdf420a5b07403088aa420363`.

Evidence class: **Static and Ran.** I inspected the exact derived source,
canonical validators, package authority, frozen command/result/resource records,
and compact acquisition evidence. I independently read the four source-bound
archive publication blobs sequentially and compared their last support and
terminal-event owner operands. I did not execute Rust, model physics, a physical
successor, or the unavailable full native consumer restoration. Supplied frozen
executions are identified below as inspected evidence rather than reviewer-run
commands.

## Findings

### A-001 — High — no member-backed full native-consumer/resident restoration exists or ran

The member-backed component entry points in
`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_current_context_capture.rs`
restore clock/parent, the smaller provisional context and the captured prepared
provider day. The complete provider/native function at lines 3690–3723 still
expects one monolithic observation bundle and materializes all physical rows.
No entry point supplies the exported 457,913,228 byte native-consumer member to
`restore_provider_bound_context_from_rows`. No retained result invokes
`DirectV10RealConsumerShadow::diagnostic_restore_native_consumer_v1`.

Consequently the acceptance-critical native hydrology frame, current/committed
soil residents, frozen-litter residents, native lane maps, provider cursor, GSI
state and complete consumer-owner projection remain unrestored. The private
inverse needed to bridge the member-backed native operand is authorized but
unimplemented. This is an incomplete restoration, not a native authority
refusal.

The dormant rows-based path also omits required current-phase checks. It does not
restore or compare `phase_diagnostics.current_stage3`,
`current_snow_enthalpy_material_owner`, `deferred_native_v2_soil_custody`, or
`prefix_validation_surface_configuration_canonical_json`, and it does not call
`validate_snow_free_prefix_membership_v1`. The exact corpus has deferred soil
custody and a pending terminal parcel, so these postures cannot be inferred from
the top-level beginning state.

In
`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`,
`diagnostic_restore_native_consumer_v1` constructs the consumer with accepted
interval count zero, then installs the captured scalar afterward. There is no
independent join proving that accepted interval count, provider/GSI cursor and
current Stage3 position all identify B01 day 4, interval 22, position 214. Thus
even a future call to the present path would not close native chronology.

### A-002 — High — all four archives reach a source-backed canonical refusal

`frozen-b01-archives-009` ran in a fresh process with B01 installed and unchanged
source/input/binary custody. It failed 0/1 after 216.91 seconds, with all four
days returning `V11 actual-consumer identity mismatch: archived publication day
chronology`; peak RSS was 1,480,140 KiB under the 16 GiB limit.

The refusal is the third condition at
`v9_real_consumer_shadow_publication_retention.rs:1806-1808`:
`last.ending_complete_owner_set_sha256 != wire.ending_owner_set_sha256`.
This is not reader byte loss. Each source-produced archive contains accepted
event handoffs at the exact last-support end which carry that support owner to
the declared day-end owner:

- day 0: `35826c…124e` -> vegetation -> `a802d4…31a9`;
- day 1: `9a43a2…147b` -> snow -> `f1444f…5708` -> vegetation -> `45dbcf…5c7e`;
- day 2: `56f2cf…c076` -> vegetation -> `845455…0994`; and
- day 3: `2c3ff5…75b0` -> soil thermal -> `4cf5b1…6f74` -> vegetation ->
  `1acb82…8ee6`.

Every event beginning owner equals its predecessor, and the last event ending
owner equals the wire ending owner. `AcceptedPublicationHistoryV1::replace`
already receives both supports and events before the failing check. The current
canonical restorer therefore rejects the producer's valid terminal-event
chronology by additionally requiring the last *support* owner to equal the
post-event day-end owner. Canonical-validator changes are outside this package;
the exact refusal must be preserved and archive/native restoration remains
incomplete.

### A-003 — High — archive beginning-owner and root-chain acceptance remains unproved

Even if the canonical publication refusal were absent,
`authenticate_archived_day_with_configuration_v1` at
`snow_stage3_v11_current_context_capture.rs:3001-3017` checks the restored parent
receipt only at its ending owner/cursor. It never joins the parent receipt's
source-defined beginning owner to
`Stage3CommittedDayArchiveEntryV1::beginning_owner_set_sha256`.

The frozen archive test at lines 4270–4361 authenticates each selected day
independently and sorts only the final results. It does not append entries to a
manifest or exercise the previous/resulting ordered-day and archive-content-root
chain. The fuller dormant rows path contains manifest and publication-prefix
checks, but no member-backed execution reaches them because every day refuses
first. A-003 is therefore open independently of the canonical refusal.

### A-004 — Medium — zero-physics measurement does not cover the archive refusal or missing native path

The inspected component call graphs contain restoration, validation, chronology
replay and provisional coupled-time acceptance on a clone. They do not call
`execute_adaptive_snow_free_successor_v1`, `execute_real_v11_parent`, or a model
publication path. `frozen-provisional-006` also proves that the original restored
clock bytes are unchanged while only the clone advances.

The final audited prefix and provisional probes do report zero solver maps,
solves, sweeps, probes, evaluations and leaf calls; zero direct compute,
mutation, commit and publication counters; and zero native litter, surface and
WB14 physics calls. Those counters are asserted on successful completion.

The stable archive refusal `frozen-b01-archives-009` predates this RAII audit and
has no equivalent runtime counters. The archive decoder's inspected path performs
canonical restoration and publication chronology validation, not physical
execution, but the package cannot promote the successful components' counters to
the separately executed archive attempt. The missing native/provider path did
not run and has no runtime evidence.

### A-005 — Medium — negative evidence covers only the clock/provisional subset

The four isolated controls pass their declared harness predicates. Slab omission,
slab reorder and wrong parent support reach typed canonical `ERR-CT-015 invalid
restart state`; the wrong provisional owner reaches an assertion/equality guard
and is not a typed canonical refusal. There are no executed semantic poisons for
prefix event/proof substitution, provider position, current/deferred custody,
terminal parcel posture, archive beginning owner, archive ordering/root chain or
native owner projection. The absent paths cannot claim fail-closed coverage.

## Verified fixes and component evidence

The derived source corrected the target row kind to the actual snow-free record
and uses `v11-real-consumer` for its provisional constraint source. It also added
exact clock-bound joins for all eight slabs and the terminal event. It recomputes
the native inactive-prefix proof and compares the result with the recorded value;
the recorded proof is never trusted. These source fixes
remove the retained wrong-phase defects.

`scoped-prefix-016` is a valid fresh-process component PASS (1/1, exit 0,
0.84 seconds). Installing the captured B01 policy resolved
the earlier `frozen-b01-prefix-008` reader-context refusal. Independent operand
reconstruction had already ruled out field, ordering, integer and binary64 loss.
The final probe now authenticates the clock-bound eight-receipt/one-event prefix,
recomputes its native proof, and reports/asserts all relevant physical counters
at zero.

`scoped-provisional-017` is a valid fresh-process component PASS (1/1,
exit 0, 0.95 seconds). It restores clock/parent from the
exact member export, reconstructs the snow-free provisional candidate on a clone,
matches the captured reduction, ledger and receipt byte for byte, and reports/
asserts the same physical counters at zero. It does not validate provider-derived
forcing, full native owners, phase custody or archives.

`scoped-provider-018` is a valid fresh-process component PASS (1/1, exit 0,
0.85 seconds). It restores all 48 supports, binds the prepared day at day 4,
selects interval 22, joins its parent support to the target row and recomputes the
captured forcing digest, with the same physical counters at zero. Its static
provider configuration is reconstructed from the captured provider member, not
an independent pinned provider authority. This result qualifies that bounded
component only; it does not invoke the native consumer inverse or close A-001.

The supplied custody records state that all 49 export files totaling
1,796,372,827 bytes, the acquisition source and frozen inputs were unchanged.
The frozen result records also report `source_unchanged`,
`frozen_binary_unchanged` and `inputs_unchanged`. This is byte custody, not
semantic admission.

Final scope reconciliation reports exactly two files changed from the acquisition
source: `snow_stage3_v11_current_context_capture.rs` and
`snow_stage3_v11_restart.rs`. I independently hashed the complete current and
retained source directories and found only those two differences plus the
derived-cut marker. An earlier terminal cut had also changed 61 unrelated Rust
files. Reapplying the same Rust 1.95 formatter to the acquisition originals
reproduced all 61 earlier hashes exactly (61/61), establishing that those changes
were formatter output rather than semantic edits; all 61 now match the retained
acquisition bytes. The final narrowed binary was rebuilt and all three component
PASSes above were rerun against that narrowed source.

## Residual risk and missing tests

The package lacks a frozen fresh-process full member-backed native-consumer and
resident attempt, complete archive admission, the parent beginning-owner/root-
chain joins, archive-refusal runtime counters and the relevant semantic poisons.
Runtime QA is also unavailable and cannot be replaced by this correctness
review.

The archive canonical refusal is precisely localized and is a legitimate bounded
diagnostic outcome. It does not validate the unexecuted native consumer or turn
the missing private inverse into an authority refusal. No physical successor,
model publication, production restart, scientific qualification, original
regression repair, cadence release or overall checkpoint completion follows.

## Verdict

**HOLD.** I accept the frozen provisional component and the archive refusal
attribution as truthful partial evidence. I do not approve complete native
context restoration or its acceptance claims. The package may report native
restoration **INCOMPLETE**, archive authentication **CANONICALLY REFUSED at the
terminal-event chronology check**, prepared provider-day component **PASS with
captured configuration**, full native consumer/residents **NOT RUN**, and
production/scientific/cadence status **HOLD**.
