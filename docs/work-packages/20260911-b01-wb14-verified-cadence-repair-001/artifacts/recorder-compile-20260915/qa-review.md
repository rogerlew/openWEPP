Static/Ran evidence — independent QA review of the authorized constructor-projection correction, retained source custody, compile/listing/test records, and prior accepted phase/oracle review. I ran no Rust, runtime, reader, collector, raw-corpus, or probe commands.

## Findings

- **BLOCKER** — [producer-test.json](producer-test.json): the actual covered/first-snow-free producer control ran one selected test and failed, exit 100. Its threaded fixture panicked at `v9_real_consumer_shadow_wb14_tests.rs:2107` because `ModelTimeNs(360000000000) != ModelTimeNs(540000000000)`, before the recorder assertions could establish both-path capture. This is the first downstream fixture failure and the authorization requires stop; recorder preparation is not accepted.

- **BLOCKER** — [producer-test.json](producer-test.json): the failure prevents the required enabled/disabled/capture-only-failure isolation, wrong-phase/required-omission refusal, and independent operand assertions from becoming executed acceptance evidence. Matched owning and runner lint are unperformed at this stop.

## Non-blocking debt/follow-ups

- The narrow compile correction is source-bound and passes ordinary library plus test-target compilation: [library-and-tests-check.json](library-and-tests-check.json), exit 0. The exact focused listing selected two nonzero tests; the selector control passed, exit 0.
- [terminal-rustfmt-check.json](terminal-rustfmt-check.json) passed exit 0 at 03:36:25 UTC on the same source identity as the later producer test.
- Static review confirms the correction is limited to the diagnostic constructor projection and test-only helper boundary. It does not add production `Serialize` or dependency changes. Prior accepted phase-specific producer snapshots remain unaffected.
- The previously disclosed dynamic-hydrology independent-output-oracle limitation remains open.

## QA verdict

**HOLD / NOT ACCEPTED.** Preserve the failed fixture and frozen source. The accepted selective extraction is unchanged; full context acquisition, native reader, authentic isolation, conservation/restart, and cadence remain incomplete.
