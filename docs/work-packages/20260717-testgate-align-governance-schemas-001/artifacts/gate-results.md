# Gate Results

Evidence class: `Ran`

Frozen base: `371988a787281416226658b5e6ef6ebf56f98e0a`

The initial pre-review closure run passed but was invalidated by accepted review
remediation. The first post-review closure run also passed but was invalidated
by first-round terminal-verification findings. Its historical command logs are
retained locally under
`target/testgate-align-remediation-closure-20260717/`; sealed CRAP evidence is
replaced by the final second-remediation run before closure.

## Focused Gates

- `jq empty` passed for every JSON schema, fixture, and the production impact
  map.
- `cargo nextest run --test testgate_align_authority_contract` passed `7/7`
  after remediation, run ID `16aa3f89-a355-45e1-af8a-9159f0900306`.
- `cargo clippy --test testgate_align_authority_contract -- -D warnings`
  passed.
- `cargo fmt --check` passed after formatting the new integration test.
- `git diff --check` passed.
- Scoped `markdown-doc lint --path ...` passed for every changed and added
  Markdown file with zero errors and zero warnings.
- The production-source placeholder scan passed with no matches.

## Critical Closure Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exit `0`; `0.74 s` |
| `cargo nextest run --workspace --profile full` | PASS | Exit `0`; `2,092/2,092` passed, `24` slow, `5` skipped; run ID `8c35a3e0-4858-49da-8784-05989ebe0c60`; `557.44 s` |
| `cargo deny check` | PASS | Exit `0`; advisories, bans, licenses, and sources all `ok`; `1.18 s` |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 371988a787281416226658b5e6ef6ebf56f98e0a` | PASS | Exit `0`; fresh and closure eligible; `2,448.49 s` |

The adjudicated CRAP report assessed `9,746` production entries. It found two
raw rows, both validly adjudicated, with zero actionable rows, zero invalid or
stale adjudications, and zero touched production files. The before, after, and
final 235-source manifests all matched SHA-256
`d1ea5bc386eabc02bb2bbbfc9c1c2bf1c657ac9687e7443ba607dbfd07e3645f`.

The frozen Git diff SHA-256 remained
`31c9a5b7205d13216b18a9e952aaae078fb1b8dea88f7bd8cca82696e85df1c1`
and the frozen status SHA-256 remained
`d5812c9e1094a4ba53f7d02de62f1e235eac845e39121cb3cf26d17185e0407f`
from runner intake through completion.

## Evidence Integrity

The final CRAP artifacts bind:

- workspace CRAP JSON SHA-256:
  `18458443c8c5b01aaa32f52161e030734db69bf2424b04d87b1fe4bd8c28fe52`;
- LCOV SHA-256:
  `cb605227fbb1c0291ce7d7c9c8bfa3008594aba3f7571c7207fb7068bfa0526a`;
  and
- adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The first-remediation required gate set had no `FAIL`, `BLOCKED`, or `NOT RUN`;
the later bounded final-run disposition is recorded separately below.

## User-Directed Bounded Final Run

After second terminal-remediation, the focused contract passed `9/9` (run ID
`1e22b97a-67c8-4c72-9a63-62c08dfb2caf`). The final content-bound runner then
recorded:

- workspace/all-target Clippy: PASS in 1 second;
- full Nextest: PASS, `2,094/2,094`, 24 slow, 5 skipped, run ID
  `4061422a-acbb-4042-933f-de464b99bb5d`, 566 seconds wall time;
- cargo-deny: PASS in 1 second; and
- a 64-path modified/untracked input manifest with path-list SHA-256
  `99e8dccbe01eb647ca4a301e7572e2b973636a1e5175b8952a7a9de073d422ae`
  and content SHA-256
  `8d8bb3c437ab2a485129ec18e45ee1f473ac014e7dfa54aae647981af1edcb55`.

The user then directed the agent to stop unnecessary repeated testing and wrap
up. The third global CRAP rerun was terminated during coverage collection. A
post-stop reconstruction proved the same 64 paths and bytes remained unchanged.
This interrupted attempt is not reported as a PASS. Two earlier fresh global
CRAP runs passed with two raw, two adjudicated, and zero actionable rows; the
second-remediation changes were limited to schemas, tests, and documentation,
with no production simulation source change.
