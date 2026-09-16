# Invalid in-flight check — 2026-09-16

Status: INVALID; stopped before the archive member decode completed.

The command started at approximately 15:54 UTC before the required frozen
source identity and resource-bounded runner were established:

```
nix develop /workdir/openWEPP --command env \
  CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/event-aware-restoration-20260916 \
  RUST_MIN_STACK=67108864 \
  OPENWEPP_B01_MEMBER_EXPORT=/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1 \
  OPENWEPP_B01_PINNED_SEED=/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-fixture-20260916-1/snow_accuracy_continuous_1789528435904586137/case.run.snow_stage3_v11_owner_seed.json \
  cargo test --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/Cargo.toml \
  -p openwepp-hillslope-orchestrator --locked \
  --features persisted-restart-v1,restart-authority-evidence \
  snow_stage3_v11_current_context_capture::member_backed_restore_tests::archive_records_authenticate_sequentially_from_pinned_seed -- --exact
```

The writer altered the endpoint predicate while the command was compiling, so
the result cannot be source-bound as either the required red or a corrected
result. The cargo parent ended but left test binary PID 1812987 running. It
began archive-member decoding with roughly 1 GiB RSS and unlimited
address-space, output-file-size, and CPU limits. The writer sent TERM to PID
1812987; its subsequent disappearance was observed, but no exit status was
collected. This is a second independent invalidity. No
acceptance claim follows. A fresh source-bound red/green sequence will use a
bounded frozen runner.
