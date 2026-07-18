# Terminal Gate Results

## Attempt 1

Ran: HOLD. The delegated sequence stopped at the first failure, as required.

| Command | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | FAIL: `clippy::too_many_lines`, TESTGATE-ASSURE integration test function 116/100 lines |
| Full workspace Nextest | NOT RUN after Clippy failure |
| `cargo deny check` | NOT RUN after Clippy failure |
| Adjudicated CRAP gate | NOT RUN after Clippy failure |

The runner made no source, documentation, or configuration changes; pre/post
Git status was identical. The test body was mechanically extracted into three
named assertion helpers without changing cases or assertions. Focused target
Clippy then passed. A fresh terminal sequence is pending narrow review of that
remediation.

## Attempt 2

Ran: HOLD. The fresh reviewed sequence advanced through full regression and
stopped at dependency-license policy.

| Command | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS, 0.67 seconds |
| `cargo nextest run --workspace --profile full` | PASS, 2,154/2,154; 32 slow; 5 declared skips; 197 binaries; 699.676 seconds |
| `cargo deny check` | FAIL: OSI-approved `MIT-0` from base-present `borrow-or-share` was absent from the permissive license allowlist |
| Adjudicated CRAP gate | NOT RUN after cargo-deny failure |

The runner again made no edits and pre/post Git status was identical. Frozen
base inspection confirmed the `borrow-or-share -> fluent-uri -> referencing ->
jsonschema` stack was already present; this package exposed a pre-existing
allowlist omission rather than adding that crate. The package write set was
amended before adding `MIT-0` to the permissive license allowlist with exact
dependency rationale. Focused `cargo deny check licenses` now passes. Narrow
review and resumption at the failed cargo-deny gate are pending. The successful
full Nextest result will not be repeated: the remediation changes only license
policy and documentation and cannot affect compiled behavior.

## Final Resumption And Combined Verdict

Ran: PASS. Both independent reviewers approved retaining the successful
format, Clippy, and full Nextest evidence because the post-Nextest remediation
changed only `deny.toml` and package documentation. Closure resumed at the
failed gate without repeating regression or coverage commands.

| Gate | Final evidence |
| --- | --- |
| Format | PASS |
| Workspace Clippy | PASS, warnings denied |
| Full workspace Nextest | PASS, 2,154/2,154; 32 slow; 5 declared skips; 197 binaries; 699.676 seconds |
| Cargo-deny | PASS; advisories, bans, licenses, and sources all OK |
| Adjudicated CRAP | PASS; raw 2, adjudicated 2, actionable 0, touched files 6, `closure_eligible=true` |

Fresh global CRAP coverage ran from `2026-07-18T21:32:13Z` through
`22:20:02Z` (47m49s). It measured 246 production sources, 465 measurement
inputs, and 10,672 production entries. Production source-manifest SHA-256:
`c55307d1451356d1922a05ed32d1358dc4b45a4bb21d91f00e59fe332b6ce731`.

Artifacts under `target/adjudicated-crap/`:

- report JSON: `630ceeaeb05947d9ab09812bc34a8ab845d010ae9c2e47ad9c902d0dff7ed590`;
- report Markdown: `c01303dd56c786fd965c90234a0bdf728c754845eba7e72f55250a23ec14e950`;
- workspace CRAP JSON: `0dd575d8adc2b28b0bc11942ade937be0ced3b5e50451f7265f05d56585ca126`;
- LCOV: `44bb26c60a08b0533d4295afdcb9ca44464319be8bdb0a1adf0ab51a8004007a`;
- registry: `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`;
- run status: `13f6aae872c51fea71967478bc3def16c570376484df846f625afe85cbbe9b3a`;
- checksum manifest: `3964883d411c4006ac10148255fab2b39adb99e237ce9a6f71cc3d70f20e0871`.

The closure runner's pre/post Git status was identical for each attempt and
resumption. It made no source, documentation, configuration, or commit edits.
