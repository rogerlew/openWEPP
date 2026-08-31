# WGHL-FULL-001I runner V2 soil bootstrap

Status: assigned runner cutover complete; focused runner checks green.

Evidence mode: `Static + Ran`

## Implementation

Static: the production runner continues to admit only the immutable
`OPENWEPP_SNOW_STAGE3_V11_PRODUCTION_SEED_V1` artifact and its existing V1
checkpoint. No V1 schema, version, field, fixture, tag, digest formula, or wire
byte changed. The checked V1 soil restart is a migration source only.

Static: bootstrap now requires the support transaction to be the exact
successor of the V1 owner's last accepted transaction. It binds the existing
V1 restart-payload digest as the V2 receipt-chain predecessor, performs one
native `migrate_soil_thermal_v1_to_v2`, prepares the half-open first parent
support, creates native receipt-free restart/checkpoint seals, and constructs
`DirectV10RealConsumerShadow::try_new_v2`. Layer-map construction reads the
prepared V2 beginning owner. Before migration, the V1 owner/configuration is
joined to the external LSE authority and its OFE/layer topology is joined by
exact cardinality and identity to both the LSE configuration and live lane
count. The local V1 value is never installed into or retained by the successor
host.

Static: no post-credit V2 sidecar was introduced. Therefore there is no new
runner resume tag to confuse with V1 and no runner downgrade path. Post-WAT5
accepted restart remains the separately tagged persisted-restart V2 authority;
every native V2-to-V1 conversion continues to refuse. The runner bootstrap is
pre-first-support, so a before/after accepted WAT5 split is not reachable in
this seed-only slice. WAT5 exact-credit/split coverage remains in the released
orchestrator/persistence evidence; this runner evidence does not duplicate or
weaken it.

Static: the V2 bootstrap test recursively scans persisted owner keys and
refuses diagnostic, microstep, iteration, solver, and rejection keys. No
production diagnostic field was added.

## Focused evidence

Ran:

```text
nix develop -c cargo nextest run -p openwepp-runner \
  snow_stage3_v11_production_seed
```

- terminal runner-seed run: `462c3d68-305f-4c7d-85cb-28fed80a4441`
- result: `PASS`, 15/15; 270 skipped
- covers the unchanged V1 golden bytes and pinned file SHA-256
  `e1d9d6164d4fe47a31e29266de12ca3908e3ecd8972efb0b45d1bbf56b890a4b`,
  strict V1 tag/digest admission, zero-carry high-word/temperature bit
  preservation, exact parent/transaction/support/receipt-chain joins, sole V2
  resident custody, native receipt-free seals, replay refusal without install,
  unconditional downgrade refusal, mixed owner/configuration and truncated
  topology refusal, and absence of persisted diagnostics

Ran:

```text
nix develop -c cargo nextest run -p openwepp-runner \
  explicit_stage3_runner_fixture_bootstraps_before_day_execution
```

- run: `6e8911bb-e398-491b-849a-0316ebc42a3a`
- result: `PASS`, 1/1; 283 skipped
- proves the parsed one-day production frame installs the Stage-3 attachment
  through the real runner bootstrap using the explicit test-only V1 owner

Ran: terminal `nix develop -c cargo check -p openwepp-runner` passed on the
same shared-source snapshot as the terminal 15/15 runner-seed run.

Ran: individual `rustfmt` on every owned Rust path and `git diff --check` on
the complete runner write set passed. The main seed remains below the
2,000-line source warning threshold after extracting the 79-line V2 bootstrap
module.

## Write set and handoff

- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs`
- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed_v2_bootstrap.rs`
- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed_v2_bootstrap_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs`
- this evidence file

No fixture/sidecar file changed. No commit was created by this slice.
