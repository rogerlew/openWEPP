# Implementation and test evidence

Status: `EXECUTING`

Evidence mode: `Static + Ran`

Focused source-quality, assurance, vegetation, fixture, fixed-point,
publication, authority, conservation, and final workspace evidence is recorded
here and in `gate-results.md`. Commands, source identities, run IDs, counts,
durations, log paths/hashes, and failure reasons must be exact.

## WGHL-FULL-001D exact-floor open-snow safeguard

Static: `SC-SNOWENERGY-001@31` authorized one private exact-floor,
terminal-one-volume, phase-aware unpublished midpoint in coordinated total
water/enthalpy coordinates. The implementation reconstructs from the immutable
beginning and complete support snowfall, vapor, external-liquid, and ordered
energy operands; snowfall, density, settling, cursor, and raw authentic history
remain exact. It does not change the 60-second floor, 96-iteration cap,
tolerances, equations, ledgers, events, rollback, or fresh-authentic-only
publication. No public API or serialized surface changed.

Ran: the terminal contract-derived suite passed 5/5 in run
`6ed6489d-2ecc-4d26-97cf-d449890bdeae`. It covers the captured
`1860..1980 s` mixed/frozen vector, independent mass/energy closure, exact
`H=0` and `H=LfW` sides, unblended snowfall/density/cursor posture, and
vapor/component/nonfinite/structure refusal. The source obligation proves the
midpoint has one unpublished consumer and cannot enter finalization, replay,
acceptance, or publication.

Ran: real DFF-WS2 remains `FAIL/HOLD`. After correcting two implementation
overconstraints exposed by the real snow-reappearance image (snowfall belongs
in `W`, and adaptive children retain the parent cursor), the exact consumer
reaches and refuses `vapor disposition` at `1860..1920 s`. Run
`93192598-4458-4396-9130-8c5627403fa2` failed after 162.533 s; retained log
`/tmp/wghl_001d_v31/dff_ws2_support_detail.log`, SHA-256
`1893d5f22feb4798669de0994e01c2a5554757ca2fc62234a3f18daeb30d1e99`.
Crossing vapor sign/disposition is explicitly forbidden by
`INV-SNOWENERGY-055`; the guard is therefore a lawful authority refusal, not a
tolerance or implementation defect. No guard was weakened, no production
diagnostic persists, and `WGHL-FULL-001D` is not accepted for package closure.

Ran: an authorized one-run ephemeral capture at that refusal established that
the crossing is not a pure `V = 0` active-set boundary. The current image has
`V = D = +2.12159691239571346e-4 kg/m2` (`0x3f2bcee5deed3256`),
`S = +0` (`0x0000000000000000`), and
`Qlatent = +6.49057936925197964e2 J/m2` (`0x40844876a7a277a6`); the authentic
image has `V = -4.61661230425127085e-3 kg/m2` (`0xbf72e8de6dd75412`),
`D = +0` (`0x0000000000000000`),
`S = +4.61661230425127085e-3 kg/m2` (`0x3f72e8de6dd75412`), and
`Qlatent = -1.30816326253264015e4 J/m2` (`0xc0c98cd0f9dddfc2`). Thus both
vapor disposition and latent-energy sign reverse from deposition to
sublimation. Run `da4af6d5-0e36-422e-8cc7-f807cb3fc166` failed at the expected
typed guard after 172.579 s; retained log
`/tmp/wghl_001d_v31/vapor_qlatent_capture.log`, SHA-256
`7857fbaa52abe71bfb2e2a8abdb103f248d9c44c0c5aecaf76fc2b0521ad38ed`.
The ephemeral output was removed, and the clean source then passed the focused
suite 5/5 in run `85fa7462-8759-4f2a-8efc-ee0e3403c411`.

Static: `SC-SNOWENERGY-001@32` supersedes only the v31 controller for pure
opposite-sign vapor images. The private implementation derives the exact
binary64 vapor root, makes `V=D=S=Qlatent=+0`, interpolates only external
liquid and ordered nonlatent energy, recomputes complete energy, and applies
the unchanged canonical `W/H` projection. Zero-to-one-sided entry uses the
unchanged support-scaled `0.25..0.5` weight and authentic positive finite
specific latent heat. Every numerical image remains synthetic and ineligible
for convergence, finalization, replay, acceptance, persistence, or publication.

Ran: v32 focused root, larger-direct-support, branch-entry, canonical `W/H`,
same-sign-v31, synthetic-publication, mixed-disposition, nonfinite, component,
latent, and identity vectors passed 6/6 on clean post-capture source in run
`724b13ba-e00d-4aca-9ec7-ca278c1f966c`. The retained v31 suite passed 5/5 in
clean run `aace7e5e-4aa0-4d05-a07f-8fd2bd6c6286`; the source-only unpublished seam
passed in run `59c7f727-de2f-4147-97be-07528f34adf4`; and the exact canonical
contract test passed 6/6 in run `886f8202-838e-465c-ad0f-8791b42265e5`.

Ran: the real DFF consumer advances beyond the former vapor-disposition guard
but remains `FAIL/HOLD`. Run `9ad9d95e-cad4-4a89-8dcb-bc4f6803da7b` failed
after 194.100 s at the unchanged 96-iteration Picard cap on `1860..1920 s`.
Audit run `4690104d-b224-4dad-9838-7823e743361e` proved all four coupled
predicates false and a material Stage-3 mass-SWE gap (`0.31450299027359325`
versus `0.30584550379469504 kg/m2`). Retained log
`/tmp/wghl_001d_v32/dff_ws2_fixed_point_audit.log`, SHA-256
`19c20bfeb4ea641a996cf2b632daaf978e9a50819cbe917566d1764fc685747c`.

Ran: one removed ephemeral transition capture proved the exact controller
state machine repeatedly resets rather than progressing to a fresh-authentic
fixed point. On `1860..1920 s`, odd iterations root the branch value
`V=-2.30830615212563543e-3 kg/m2` against a fresh authentic deposition tending
to `+1.16875492798e-4 kg/m2`; even iterations enter from exact zero toward the
unchanged authentic `V=-4.61661230425127085e-3 kg/m2`, recreating that same
half-authentic branch. The same reset occurs on the direct 120-second support.
Run `ab0a2430-e5bd-479c-87eb-68fcde33b1a0` failed after 211.340 s; retained log
`/tmp/wghl_001d_v32/dff_ws2_transition_capture.log`, SHA-256
`1e76362f229ebc8dbe41f481c91c69f69689858b771338ffe54b23fc8bfd9590`.
This is an authentic-map discontinuity outside v32's one-root/one-entry
authority, not a tolerance, cap, linked-latent, root, or `W/H` implementation
defect. No accepted/rejected count, width distribution, or complete-day real
ledger result is claimed because the first canonical run aborts before parent
completion. No production diagnostic remains, and v32 is not accepted for
real-consumer closure.

### Version-33 corrective coupled solve and native V2 adoption

Static: v33 removes the v31/v32 synthetic controller from production control
flow and retains those paths only as diagnostic/refusal oracles. The private
solver uses the contract `W/H/E/T` coordinates, canonical phase projection,
one shared 96 physical-evaluation budget, deterministic safeguarded dense
solve, and `CoupledAuthentic` admission that bypasses only Picard prior-iterate
equality. Finalization remains a fresh physical replay/reseal with the existing
owner, receipt, ledger, event, and rollback checks. The 60-second floor,
tolerances, equations, and publication policy are unchanged.

Static: the covered evaluator now consumes `DirectSoilThermalResident::V1` or
`V2` natively. V2 trials use the prepared support plus authenticated physical
energy operands and exactly one private top-boundary CN operand; exact high and
carry state stays in `DirectSoilThermalCandidate::V2`. No V2-to-V1 projection,
cache, sealing, installation, or trial receipt occurs. The existing final
segment finalizer remains the only authoritative accepted V2 seal/install.

Ran: after native V2 integration, focused v33 vectors pass 7/7; the canonical
terminal numerics contract passes 10/10; resident V2 custody vectors pass 4/4;
and exact-carry/source/order/rollback vectors pass 6/6. Affected terminal
mechanical parity also passes: joint terminal batch 1/1, restart-equivalent
terminal partition 1/1, and qualification physical/successor partition 1/1.
`cargo check -p openwepp-hillslope-orchestrator --lib` passes. No production
diagnostic output is present.

Ran: the canonical default-stack DFF fixture remains `FAIL/HOLD` before any
accepted/rejected/width/ledger metric can be produced. After the committed v9
interval-envelope split, the terminal subslab frame fell from 452,664 B to
159,744 B and its former 399,016 B union closure was replaced by disjoint
helpers no larger than 163,840 B. The exact endpoint constructor fell from
387,880 B out of the leading frame set; its largest clone-heavy selection
helper is 196,608 B and physical-child chain helper is 151,552 B. The provider
retention obligation and real two-lane terminal batch parity each pass 1/1.

Ran: the next exact default-stack run was retained at
`/tmp/wghl_001d_v33_dff_ws2_native_v2_default_stack_post_terminal_split.log`,
SHA-256 `43f2248ed7ea4ab5b8c66aa1054e149e21e958347d1a7815858e63d3b7614f09`.
It exposed a 192,512 B `GenericShunt<DirectDayFrame>` constructor. The assigned
adapter correction replaced that generic collection with ordered boxed seeding;
adapter parity passes 9/9 and public contracts pass 3/3, while its largest
collection-path frame fell to 17,656 B.

Ran: exact-source rerun
`/tmp/wghl_001d_v33_dff_ws2_native_v2_default_stack_post_day_start_split.log`,
SHA-256 `2b9cb05a75839fbbcf4da837ce19ef1f86021238bad7696736ed36bb9f89d21f`,
still aborts on cumulative default-stack depth. Conditional GDB catches
`construct_covered_interval_envelope_with_duration` entering with only about
76 KiB above the 2 MiB thread-stack guard; that inner frame is already reduced
to about 16 KiB. Its caller resolves to
`DirectV11SnowCoveredRealConsumerStack::stage3_lower_boundaries_by_destination`
from `open_snow.rs:2121` inside `execute_imported_v10_stack`. The remaining
leading outer frames include the out-of-slice 001H parent at 393,216 B and
single-subslab closure at 368,640 B. Per ownership, those are returned to the
001H owner rather than edited here. No larger-stack run is used as acceptance
evidence.

Ran: the exact-source 001H handoff reduced the adaptive parent frame from
394,728 B to 19,520 B and its active-loop frame to 121,400 B; focused parity
passed 4/4 and the two long real consumers passed 2/2 in run
`f6d526d0-4476-45c7-a2e2-c1dfd772be29`. On that source the canonical DFF
executes on the ordinary default stack without stack overflow. The first
failure is now the typed native-owner refusal at the initial 0..1800 s
support: `V1 soil accessor on V2 resident`. Retained run
`/tmp/wghl_001d_v33_dff_ws2_default_stack_post_001h.log`, SHA-256
`ed201faa68c12644d461e67e6c20bbcfb26386f6169849433c8c7a8d0173c13b`;
test execution reached the typed refusal in 0.42 s (18.45 s including the
incremental build). A second exact run after generalizing the owned
snow-soil-credit beginning lookup to the native V1/V2 read view retained the
same refusal; log
`/tmp/wghl_001d_v33_dff_ws2_default_stack_v2_read_view.log`, SHA-256
`4b65ed5958b78d04e56e3dc7e32ed652a14c2e41530fa42f3e77a3bd4fd107fc`.

Static: the remaining refusal is outside the authorized V33 source set.
`v9_real_consumer_shadow.rs::construct_canopy_soil_interval_envelope_with_duration`
unconditionally calls the V1-only `soil_thermal.v1()` before the provisional
covered physical evaluation; both `project_live_vegetation_forcing` and
`execute_v8_lse_runtime_shadow_v11_physical_with_carriers` then require a V1
`SoilThermalSnapshot`. The V33 controller therefore cannot receive the native
V2 provisional physical image even though its downstream candidate, operand,
carry, and replay carriers are V2-aware. No V1 projection, cache, downgrade,
or guard weakening was added. This is a prospective v9/LSE read-only evaluator
authority dependency, so real accepted/rejected counts, width histogram,
solver evaluations, and complete-day ledger closure remain `HOLD`.

Ran: terminal mechanical parity after the split is green: adaptive
preterminal closure/disposition 5/5, provider retention 1/1, and real joint
batch decision 1/1. Retained log SHA-256 values are respectively
`5bf0b32793c6ae087951e6e6af1ade6b5d42dfe2a94c929fa530c48879141f39`,
`26c6478e68e88465f97fdd0ce250008babbe6f769a948da12dc86a813be54555`,
and `a35798b1ae62738ed44450520ea1a149d94cadcc1347f222df753df349e716be`.

## WGHL-FULL-001E committed publication source and custody

Static: requested WAT5 generation now receives its opt-in and accepted source
before generation from sealed Stage-3 supports: exact WB14 parameters, raw
accepted precipitation timing, and hourly accepted snow/runon supply. The
helper rejects payload overwrite and cannot reconstruct from public daily
rows, LSE runon parcels, or synthetic uniform timing. All mutations remain in
the cloned pending publication frame, preserving atomic rollback.

Static: accepted upstream runon is reconstructed from sealed accepted ingress
receipts/dispositions on the destination OFE basis. Source parcel identity and
source-area sent volume must independently close destination-area received
volume. Public `UpStrmQ` and `SubRIn` now consume
`runon_carry_downstream_operands`; normalization remains the beginning-transfer
identity and is neither overwritten nor double-counted. No public API or
serialized surface changed, and no production diagnostic persists.

Ran:

- pre-edit WAT5 red `461dad20-1056-4145-9164-ee8b9e3cf53b` failed before the
  intended close path because no runtime WAT5 ledger was installed;
- pre-edit real CLI red `f5f53e64-24ad-48e2-8d6f-848e55ae5ad8` exposed zero
  downstream `UpStrmQ`;
- focused accepted-source, public-row, WAT5 shape, and real routed-receipt
  consumers passed 4/4 in terminal run
  `6ad7b421-ab55-4e9e-b74a-1652f1127301`;
- the destination-basis anti-alias test in that run closed 0.4 m3 sent/received while
  distinguishing 0.002 m runon, 0.003 m local liquid, and 0.005 m total;
- the corrected canonical CLI consumer passed 1/1 in run
  `0ee69cbf-70b8-4e01-8981-362f72b22858` after 61.958 s, publishing nonzero
  destination-basis `UpStrmQ` of 144.71027400837232 mm and
  483.03341075609865 mm and rejecting public `Q` as the expected-value source;
- the normal WAT5 transaction target remains blocked upstream in run
  `75b21f23-db97-476a-bfc2-6f750bee8e22` after 105.944 s by
  `SURFACELIQUID-E-003` soil-thermal ending-enthalpy closure at 1800..2700 s,
  before the day-two WAT5 source guard. Therefore transaction endpoint closure
  remains `HOLD` pending the separately owned source fix and exact rerun.

Static: the mechanical split leaves `stage3_committed_publication.rs` at 2,463
lines, the focused WAT5 helper at 350, the extracted primary tests at 503, and
the retained tail at 689; every file is below 3,000 lines. The public surface
is unchanged.

Ran: affected orchestrator/runner all-target/all-feature `cargo check` passed
after the terminal helper correction. Authority-suite anti-evasion passed, and
the required-suite obligation guard passed 3/3 in run
`ea91484f-06ed-49f5-950c-ebb0ebacb9f8`. Impact-map generation 39 parses and
contains exactly ten new `wghl-001e-*` exact-path bindings. Rustfmt, owned-path
diff hygiene, and the production no-diagnostic scan pass.

Ran: science admission is still blocked outside 001E because
`snow_stage3_v11_adaptive_execution.rs` lacks a current SC binding. The
warnings-denied affected-crate Clippy campaign is also not terminally clean:
it reports approximately 1,001 shared/pre-existing diagnostics across the
orchestrator test surface. The sole new 001E production diagnostic, a direct
`u128`-to-`f64` support-offset cast, was corrected to checked `u64` plus
`Duration::as_secs_f64`; remaining reported diagnostics are inherited or in
concurrently owned files. These failures are retained for parent terminal
reconciliation and are not represented as 001E gate passes.

## WGHL-FULL-001F covered no-update witness

Static: canonical `SC-LANDSURFACEENERGY-001@13`, the revised expected-red
consumer gate, predicate refusal vectors, and production implementation follow
contract-first ordering for the expanded domain-invalid-or-full-step-excess
authority. The implementation accepts the current iterate only and retains
strict decrease for every installed update. After independent-review findings,
the first `b>=1` domain-valid witness inspection now precedes the unchanged
`b=0..20` strict-decrease update search, and the complete residual-vector gate
checks every member for finiteness and threshold passage.

Ran:

- revised unchanged-production expected red: `0/2`, run
  `dd87636c-6728-4b2a-b601-5e36f42eddb0`, both exact `FinalFixedCap`
  iteration-4 `LSEB-E-034` failures with exact owner rollback;
- terminal focused no-update/refusal vectors: `3/3`, run
  `8cf71b71-1a6f-443a-abca-3144bb14ff4f`;
- terminal complete `openwepp-land-surface-energy`: `84/84`, run
  `9a5aaf67-de5a-4c85-b149-225c52196c66`;
- warnings-denied all-target/all-feature LSE Clippy: `PASS`;
- exact frozen-oracle/genuine-update, natural failure/no-publication, and
  numerical-failure rollback-lineage selection: `3/3`, run
  `c90fa3ca-049c-4fcb-af9c-f9281dfdea0e`;
- unchanged interior-terminal consumers: prior LSE failure absent in both;
  current later failure is `qualification terminal snow-free successor
  chronology`, run `ec067bbd-443d-45ce-ba76-5c4fdd2e252b`.

Static: `solver_covered_solve.rs` is 1,172 lines (`PASS`, below 2,000). The new
helper is private; no `pub` item, function signature, enum, struct, serialized
surface, or model-definition identity changes. `git diff --check` passes on
the F-owned contract/source/evidence paths, and no persistent diagnostic print
remains.

Static review correction: production now uses the same private ordered
controller exercised by two additional vectors. They prove domain-invalid
halvings are skipped only until the first complete candidate; an incomplete or
step-failing first domain-valid candidate ends witness consideration without
skipping later; and no full-trial refusal means no halved probe. The controller
returns only the examined exponent and step metadata, so it cannot carry or
install a prospective trial. Reviewer A and B closed their solver-order/test
findings statically.

Ran: after the successor compile-red cleared, the exact-source five focused
vectors pass 5/5, run `baaf9f04-769f-4de0-82bd-f98695c081db`, and the complete
LSE crate passes 87/87, run `dcd3e84b-d3ce-4bae-8960-df2c2a2c1767`.
Warnings-denied crate Clippy currently reports separately owned v14
litter-phase diagnostics; the one 001F-local `needless_continue` diagnostic
was corrected. Owned-source rustfmt, diff hygiene, and no-print checks pass.

Ran: independent verifier A passes 5/5 focused controller/predicate, run
`a1f7fec7-d6d5-4b4f-aa44-3426498e8a45`; 3/3 protected
oracle/failure/rollback, run `a34651f3-fcac-4aa6-862c-2d5058db83f6`; and 97/97
current full LSE, run `cf69b0fc-c62a-4ac8-b56f-4d0152a93e17`. Independent
verifier B passes 8/8 combined focused protections, run
`23f86a44-ab40-4a68-8f8c-a70326762a74`, and 97/97 current full LSE, run
`31de0cbe-4b3a-4174-91e4-d133763c3d0c`. Both report no 001F code finding.

Ran: final hardening independently poisons each full-trial governed coordinate
with NaN, positive infinity, and negative infinity and proves none is
classified as a complete exact threshold excess. Finite excess remains
classified for each coordinate. Focused 6/6 passes, run
`6efcec2e-2666-4a58-b911-80a2267bf0dd`; current full LSE 103/103 passes, run
`86f824a0-4486-4b9d-80ff-fe8fe0e8fbfd`. Terminal source SHA-256 is
`9784b674044a61f14dcb21523fbb53f6717735c1d4e9beb22339d8ee69e91122`;
line count is 1,207; rustfmt, diff hygiene, and no-print checks pass.
