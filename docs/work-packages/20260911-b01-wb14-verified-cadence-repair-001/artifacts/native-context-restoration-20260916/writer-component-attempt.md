# Native context restoration writer attempt

Ran: the derived reader cut restored the exact member-backed snow-free row's coupled clock and parent without decoding the 457,913,228-byte native-consumer operand.

The read-only candidate module was SHA-256 `4ea2d12f5e6d65b68a54640fcbdcd5e7dc287ed2cb4a7b8dcf3c32ac2897a887`. The derived reader module at the first successful executable component was SHA-256 `6727057fbe46b93abf42953cdb9708eb570cc1bbfc6bf094d08641b6422d14c8`.

The frozen test binary was `/tmp/openwepp-b01-wb14-native-context-target/debug/deps/openwepp_hillslope_orchestrator-6acf9a74daf335ab`, SHA-256 `72e3eb156c71bae77a4c2750cfc8ec124a722a6e151e4d2fea0c621055cd9370`.

Command:

```text
env OPENWEPP_B01_MEMBER_EXPORT=/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1 /tmp/openwepp-b01-wb14-native-context-target/debug/deps/openwepp_hillslope_orchestrator-6acf9a74daf335ab snow_stage3_v11_current_context_capture::member_backed_restore_tests::snow_free_member_export_restores_clock_and_parent_without_native_decode --exact --nocapture
```

Result: passed, 1 passed; 0 failed; 1474 filtered out; 0.56s. This initial component is limited to source-backed member decoding and clock/parent restoration; it did not independently admit the full pre-child context.

First relevant failure: a fresh prefix-test compilation attempted `serde_json::from_slice::<DirectSurfaceLiquidConfiguration>` and failed because the configuration deliberately does not implement `Deserialize` (E0277). The correction uses its existing `from_canonical_bytes` validator; no configuration API or serialization contract changed.
