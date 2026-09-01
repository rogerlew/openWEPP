# WGHL-FULL-001J native-V2 carrier joint custody

## Diagnosis and correction

Static and real-consumer diagnosis: native V2 beginning joints contain the
resident active owner's canonical custody bytes, while ending joints contain
the selected unpublished trial ending. These are intentionally different
payloads. The gated diagnostic run observed 2,309 resident bytes versus
1,382/1,383 trial-ending bytes. It also proved that two terminal reconstruction
call sites had discarded the selected unpublished candidate before the carrier
join. Their owning worker changed those sites to source the replacement carrier
through `try_with_selected_stage3_by_lane`; this follow-up did not edit terminal
execution. The temporary gated capture was then removed completely.

Static: `carrier_phase.rs` now represents the two lawful typed postures
explicitly. `ResidentBeginning` derives all six non-snow owners from
`DirectV10RealConsumerShadow::canonical_owner_state_bytes`. `CandidateEnding`
replaces only `soil_thermal` with the selected unpublished V2 trial ending.
Selection requires exactly one complete six-owner profile to match the sealed
seven-owner joint, except that byte-identical V1/no-candidate profiles are one
unambiguous identity. The V2 trial remains retained for exact carry and is not
projected, cached, installed, accepted, or receipted.

Static (initial carrier-joint increment): no support, transaction, snow, LSE,
surface-liquid, hydrology, BGC, vegetation, receipt, or terminal-event field
changed. Exact non-snow cardinality is checked, and stale V2 carry and
substituted non-soil owner bytes fail closed. Later authorized continuation and
finalization increments are recorded below.

The next exact real consumer advanced through both typed/joint postures and
then refused the second-child top-boundary credit. Bounded capture proved that
the carrier correctly bound the credit to the retained unpublished ending, but
passed the active resident's parent-start prepared beginning into the lower V2
credit validator. Only the state identity differed: candidate-absent paths were
exact; candidate-present paths carried the authentic predecessor-child ending.

Static: candidate-present composition now uses the typed
`DirectSoilThermalUnpublishedContinuationV2` seam. It derives the original
parent support and the retained trial's exact prior support from the
authoritative resident, requires exact prior-end/child-start contiguity, binds
the child credit to the retained ending owner/configuration/state and exact
support, and recomposes the accumulated trial once from the original parent
beginning. The retained trial is reconstructed from its operands before it is
admitted, including exact carry, predecessor transaction, receipt-chain, and
support identity. Same-support replacement, gaps, overlaps, cross-support
substitution, stale state/carry, and foreign owner profiles fail closed. This
path remains unpublished and performs no accepted receipt or owner install.
The bounded `OPENWEPP_V2_CARRIER_TOP_CAPTURE` instrumentation was removed.

## Tests and validation

Ran the focused V2 custody and receipt-free carrier set:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(native_v2_selected_joint_binds_resident_beginning_and_trial_ending_exactly) | \
      test(native_v2_selected_joint_rejects_stale_carry_and_substituted_owner) | \
      test(v2_carrier_composition_is_trial_only_and_receipt_free) | \
      test(phase_has_no_stage3_evaluation_or_publication_surface)' \
  --no-capture --no-fail-fast
```

- run: `cff1a59b-5b1e-44dd-919d-dec4c37be35a`
- result: `PASS`, 4/4; 1,186 skipped
- log: `/tmp/wghl-001j-carrier-v2-focused-final.log`
- SHA-256: `247a52e2414c7fd68885f614d8b85f142f6161370c9f940ea55d4980f841fc3e`

Reran the complete carrier-phase unit shard on the exact source after the
real-consumer diagnosis, coordinated terminal custody fix, and capture removal:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/covered_carrier_phase_tests/)' --no-capture --no-fail-fast
```

- run: `5fe33987-bcad-4c72-8487-6610b88ebfb5`
- result: `PASS`, 7/7; 1,183 skipped
- log: `/tmp/wghl-001j-carrier-v2-shard-final.log`
- SHA-256: `23b63088a7f7486e1aa04f158615e4bde82f51da17a6147f04ab83bb791d2ebb`

Ran `nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib`:
`PASS`. Log `/tmp/wghl-001j-carrier-v2-check-final.log`, SHA-256
`4097ad141982dc1197a2cbb19e92fc3d731ea5b273f43d2a6070c32e32d93ea6`.

Ran individual Rust formatting and owned-path `git diff --check`: `PASS`.
`carrier_phase.rs` is 1,949 lines, below the 2,000-line warning threshold.
No public API or production diagnostic was added. A source scan found no
`OPENWEPP_CARRIER_JOINT_CAPTURE`, `CARRIER_JOINT_CAPTURE`, `eprintln!`, or
`dbg!` residue in `carrier_phase.rs`.

Ran the typed core continuation matrix on the exact integrated source:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  'unpublished_continuation_' --lib --no-fail-fast --no-tests=fail
```

- run: `fd27f3b6-b8e0-4d95-8df2-bd96d4215776`
- result: `PASS`, 2/2; 1,205 skipped
- log: `/tmp/wghl-001j-unpublished-continuation-core.log`
- SHA-256: `ce058f1f5756e8a56f1a1217b55052bcbc21c9fcbf154b33cb995e1b0ab0e2b6`

The matrix proves two and three contiguous unpublished child supports,
accumulated one-pass recomposition, exact nonzero carry, receipt-free resident
immutability, and refusal of substituted trial, stale ending identity,
same-support replay, gap, cross-support beginning, foreign owner, receipt-chain,
and exact-carry poisons with byte-identical rollback.

Reran the complete carrier-phase shard after continuation integration and
diagnostic removal:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  'covered_carrier_phase_tests::' --lib --no-fail-fast --no-tests=fail
```

- run: `4fc576a3-c486-417f-baf7-d028eff6f035`
- result: `PASS`, 8/8; 1,199 skipped
- log: `/tmp/wghl-001j-carrier-continuation-focused.log`
- SHA-256: `fba69eca5acbfa578d7f9af6939ed5ab6352a56c4ec169282c0d1c7351a7dc6c`

Reran `nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib`:
`PASS` with unrelated dead-code/unused warnings in concurrent V3 and V33 work.
Log `/tmp/wghl-001j-carrier-continuation-check.log`, SHA-256
`956249bfc8f4c88e37483b8fd1763a12927bb87a2a55c3acb85d622e15eb4f69`.

## Sequential exact-carry continuation and one outer acceptance

Real-consumer evidence subsequently proved that a state-dependent sequence of
native-V2 children cannot be compressed by reapplying all child operands with
one beginning heat-capacity projection. The lawful unpublished representation
therefore retains both the selected sequential physical trial and an opaque,
ordered child-major credit/operand sidecar. Each child advances from the prior
unpublished ending, with exact support adjacency, predecessor trial seal,
ordered OFE/layer topology, temperature bits, enthalpy-high bits, exact dyadic
carry, and debit identity. No child issues an accepted receipt or installs an
owner.

Finalization independently composes one outer accepted receipt from the
authenticated original beginning and the full ordered child-major chain. It
validates every child's complete layer set, exact high/carry reconstruction,
per-child temperature projection, cross-child ending-to-beginning equality,
canonical accumulated operands, unique debit identities, and final selected
physical ending. The accepted outer transaction joins vegetation, LSE, BGC,
surface, hydrology, and snow exactly once. The specialized install admits only
the authenticated original beginning or an already-installed byte-exact outer
accepted resident; the latter is a validated no-op for terminal physical reuse.

The terminal reuse seed now retains the exact selected soil candidate and
opaque continuation sidecar and checks them against the retained endpoint
before reuse. This closed the second-child posture in which the retained owner
covered `0..120 s`, while the current accepted beginning covered `0..60 s` and
a child-only recomputation covered `60..120 s`. The V11 ending owner uses
`v11_soil_thermal_owner_envelope`, so native V2 publication carries canonical
active-owner bytes rather than the enum wrapper. V1 serialization remains on
its existing exact bytes.

The final integrated focused selector was:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/covered_carrier_phase_tests/) | \
      test(/terminal_custody_lane_set_tests/) | \
      test(/unpublished_continuation/)' \
  --no-capture --no-fail-fast
```

- exact-head run: `5056cb11-6982-46bb-807c-ce5c8e20452f`
- result: `PASS`, 21/21; 1,186 skipped
- log: `/tmp/wghl-001j-integrated-final-focused.log`

`nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib`,
`nix develop -c cargo fmt --all -- --check`, and owned-path
`git diff --check` all passed. The only check warnings are unrelated unused or
dead-code warnings in concurrent V3/V33 work. `carrier_phase.rs` is 1,984 lines.
A production-source scan found no retained V2 capture environment variables,
`eprintln!`, or `dbg!`; the two textual `eprintln!` matches are source tests
that prohibit diagnostic residue.

The exact 64 MiB-stack canonical consumer then ran:

```text
RUST_MIN_STACK=67108864 nix develop -c cargo test \
  --test dff_ws2_ksatadj_direct_runtime \
  dff_ws2_forest_high_severity_loam_runs_with_live_direct_ksatadj_effect \
  -- --nocapture
```

- log: `/tmp/wghl_001d_v33_native_v2_preterminal_validator_64m_r29.log`
- SHA-256: `0b867913913b31df6293b20a4eb49fcaf13776d67756e9fd367faca3305fd86c`
- result: the carrier/finalization/native-V2 receipt, exact-carry, atomic-install,
  canonical-owner publication, and terminal-reuse joins all passed. Execution
  advanced to the next snow-owned guard and lawfully refused
  `adaptive preterminal V2 soil high/carry/temperature/chain join` after
  42.78 seconds. This is the open downstream disposition, not a carrier or
  finalization closure claim.

Subsequent exact-source progression cleared that snow physical verifier and
classified a deeper V9 physical-beginning defect. A receipt-free resident
reserved transaction 41 over the parent union, while its authentic first
unpublished trial used the same reserved transaction over `0..60 s`; the old
accepted-successor preparation API rejected that lawful unpublished posture.
The typed base-continuation API now authenticates the receipt-free resident,
reserved transaction, prepared union, retained trial support/state/carry, and
seal without weakening accepted-successor preparation. Focused carrier and
continuation tests passed 10/10 in run
`f5a61e7e-1d46-43f1-a54b-16a4b8bacc1e`.

The capture-free canonical rerun then exposed the remaining structurally
invalid V9 physical path: both physical-envelope constructors ignored their
supplied native-V2 candidate read view and prepared from the resident with the
accepted-support API. Core, V8 projection, and V9 now provide a typed
`SoilThermalUnpublishedPhysicalBeginningV2` path that authenticates either the
base trial or retained continuation without synthesizing or installing an
owner. Its native owner suite passes 9/9 and the shared library compiles.

The subsequent narrow carrier-engine authority extension migrated both V9
physical-envelope constructors to the typed
`*_with_duration_and_soil_beginning` APIs. The frozen legacy carrier builders
remain exact `None` wrappers, while the adaptive carrier envelope path threads
the existing `CoveredCarrierEphemeralCandidatesV1` candidate and opaque
continuation references directly. It does not reconstruct, project, cache, or
install a soil owner.

Exact-source validation after that caller migration:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/covered_carrier_phase_tests/) | \
      test(/unpublished_continuation/) | test(/carrier_engine_tests/)' \
  --no-capture --no-fail-fast
```

- run: `e8c4a990-846a-4602-a141-8ce0e870f2d3`
- result: `PASS`, 10/10; 1,197 skipped

r34 showed that the shared provisional fixed-point evidence helper still used
the frozen `None` wrappers internally. Its two real callers both possess the
exact current `iteration_soil_state`; neither possesses or requires a
cross-child sidecar because these are same-support fixed-point and equation-
residual re-evaluations, and physical-reuse paths exit before this loop. The
helper now accepts the typed candidate/continuation pair and threads it through
both its force-full and physical-only branches. Each caller passes the current
candidate only for native V2 and `None` for V1, retaining byte-identical V1
behavior; continuation remains `None` for this posture.

The combined helper/caller source reran the same selector in nextest run
`1d54ac5c-2aa0-4799-9bd1-78f84a7d82ab`: `PASS`, 10/10; 1,197 skipped.

r35 then proved that four independent accepted fixed-point reseals still used
the frozen wrapper: final candidate, sealed source, boundary
self-reconstruction, and installed-boundary replay. Each uses the same
converged native-V2 `soil_candidate` over the same support, while its
`final_candidate` host remains the immutable resident clone. The snow owner
migrated all four to the typed builder with that V2-filtered candidate and no
cross-child continuation; V1 remains `None`. Check, formatting, and diff-check
passed. r36 still stopped at the same lower `prepare V2 soil support` guard
after 62.11 seconds, but static source inspection confirms no covered-carrier
wrapper caller remains. The surviving `None` path is the general V9 snow-free
parent-child evaluator used by the open-snow/non-covered portion of the same
composed V11 support, outside this carrier helper's ownership. It has been
escalated to the V9 owner for a typed snow-free-caller disposition rather than
being inferred or weakened here.

The bounded r37 posture capture
(`/tmp/wghl_001d_v33_snow_free_v2_custody_capture_64m_r37.log`, SHA-256
`e7563cc843e21ebc5c97d704181540ecdb68cb4f9da2f369d3c3b3ecdc3346b2`)
classified the snow-free path exactly. Receipt-free first-support rows
lawfully retained reserved transaction 41 over supports beginning at zero.
The failing successor was an installed accepted resident at transaction 41,
last accepted 41, support `0..900 s`, with child support `900..1800 s`; the
stale parent input still requested transaction 41. This was not a dropped
unpublished candidate/continuation posture. The capture was removed.

Both native-V2 no-candidate V9 branches now derive support custody through
`prepare_next_v2_support`: receipt-free residents retain their reserved
transaction, while accepted residents receive the checked numeric successor
and exact predecessor/receipt-chain/carry validation. V1 is unchanged. The
current-source focused selector passed 12/12 (1,195 skipped), nextest run
`919d6300-ca82-4ca8-a4d7-a24f7c2a5061`, covering accepted successor
transaction/carry/one-install custody, receipt-free continuation poisons and
rollback, the two-child snow-free parent, and the full carrier shard. r38 was
released for the next exact downstream transaction-join disposition.

`nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib`
and `nix develop -c cargo fmt --all -- --check` also passed. The latest
capture-free pre-migration canonical log remains
`/tmp/wghl-001j-carrier-base-fixed-r32.log`; r35 was released immediately for
the exact downstream disposition against the fully threaded source.
