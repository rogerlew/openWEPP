# Typed feed-forward terminal-carrier contract cycle

Evidence mode: `Static + Ran + Expected-red`

Base commit under review: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`.
The authorized uncommitted contract-first increment amends
`SC-SNOWENERGY-001` to contract revision 61, adds
`INV-SNOWENERGY-088` and `OBL-SNOWENERGY-C-056`, and creates no numerical
process-solver V61, physics, tolerance, adaptive, event, wire, receipt, or
owner change.

## Source-real authority

The exact release attribution directly observed 400 common physical-map calls.
Static call-graph reconstruction and separate focused real-capture evidence
support the inference that the generic coupling loop executes two successful,
bit-identical calls with exact-zero deltas per captured invocation. They do not
directly prove 200 groups in that exact release record; revision 61 therefore
reserves exact 200-invocation multiset evidence for the postimplementation run.
The real carrier consumes neither the preceding ending-snow hint nor the
coupling-iteration ordinal.

Revision 61 makes that absence structural: a typed feed-forward request has no
hint/iteration fields and executes once per evaluator invocation. The guard is
invocation-local, never a global result cache. It retains every distinct
Full/Retry, Half1, Half2, Root, discovery, shortened exact-endpoint,
terminal-batch, and canonical final-map path. Feedback-capable evaluation is a
separate typed interface with no runtime switch or fallback.

## Reproducible validation

Ran contract assertion:

```text
nix develop --command cargo test --test snow_terminal_enthalpy_event_numerics_contract revision_61_binds_one_typed_feed_forward_call_per_logical_terminal_group -- --exact --nocapture
```

Result: `PASS`, 1/1.

Expected-red structural production-seam assertion:

```text
nix develop --command cargo test --test snow_terminal_enthalpy_event_numerics_contract revision_61_structural_production_seam_is_expected_red -- --exact --nocapture
```

Result: `EXPECTED_RED`; it fails first on absent
`FeedForwardTerminalCarrierRequestV1`. This assertion classifies structural
absence only and is not behavior acceptance authority. Postimplementation
acceptance requires independently executed package-owned in-crate call-count,
forced-reference, path/role, competing-poison, rollback, and compile-time
negative-capability tests.

The complete integration target must pass every other test and fail only the
named structural expected red before production editing. All current-contract
revision assertions are pinned to revision 61; historical process-version text
is unchanged.

The prospective performance baseline is the exact record
`artifacts/terminal-heavy-gates/carrier_static_attribution_one_ofe_release.log`,
Rust manifest `c300ea39b355105bff0349265933accab1b451b77c8e3f023ea1f9cf5b84ce70`,
binary `1ba2024784888019495cb7ca76a604f6813cbcf7f918fff75d9c07d1ee0e165d`,
with `provider_carrier=2,049,833 us`, `run_wall_us=4,984,488 us`, and RSS
`62,560 KiB`. Three unchanged-binary postimplementation CPU-0 runs must have
medians no greater than `1,299,833 us` and `4,234,488 us`; every run must retain
exact science/count/multiset identity and RSS no greater than `65,536 KiB`.
Failure requires full revision-61 production reversion.

Ran the complete integration target after reconciling all current-revision
pins:

```text
nix develop --command cargo test --test snow_terminal_enthalpy_event_numerics_contract -- --nocapture
```

Result: `EXPECTED_RED`, with exactly
`revision_61_structural_production_seam_is_expected_red` failing; 39 tests
passed and 22 historical superseded-path tests were ignored. No stale revision
pin or unrelated contract assertion failed.

Ran formatting:

```text
nix develop --command cargo fmt --all -- --check
```

Result: `PASS`.

Ran strict binding-exposure and unit-compliance gates:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
```

Result: `PASS`; 51 binding-exposure rows are fully consolidated and unit
compliance reports no findings. The strict gate exposed and the amendment
closed a pre-existing missing core-summary registration for `INV-087/C-055`;
revision 61 is registered as `INV-088/C-056` in the same canonical tables.

## Ordered manifest

The manifest covers, in this exact order:

1. `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
2. `docs/specifications/science-contracts/index.md`
3. `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`
4. `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md`

Recipe:

```text
sha256sum <the four paths above in the listed order> | sha256sum
```

First-pass review manifest SHA-256:
`8bc5f02fe1f3f777fc01f7f488c7f14ad068139314acff3fe5e83ac389967be3`.

Corrected review manifest SHA-256 after the final A-004 taxonomy amendment:
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`.

Production implementation gate: `PASS`. Both independent reviewers and both
independent verifiers reproduced corrected manifest `a8a667...804`, confirmed
all seven finding dispositions closed, and issued implementation `GO`.
