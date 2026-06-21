# Implementation Test Evidence

Status: blocked.
Evidence mode: Static.

## Required During Execution

Record focused tests for:

- ledger-promotion authority and contract/schema tests;
- HBP typed direct projection cutover;
- WAT typed direct projection cutover;
- PASS typed direct projection cutover;
- loss JSON typed direct projection cutover;
- run manifest typed direct projection cutover;
- anti-alias fixtures;
- independent operand reconstruction;
- no-compatibility direct-publication counters and source scans.

## Current Disposition

NOT RUN. No implementation tests were added or run because R6 stopped after
canonical ledger promotion and before production Rust/output edits. A valid
implementation test cannot be written against the current output cutover target
without first creating a run-bound direct publication frame.
