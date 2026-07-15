# Gate Results

Status: `complete`; focused and terminal heavy gates pass on the renewed
freeze, and both independent accepted-fix verifications are `PASS`.

- Date: 2026-07-14
- `FROZEN_BASE`: `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`
- Candidate scientific root:
  `bb4b8b5f6188613e22ca9a7bec301bd7d6a94f8ef5e3e2ed83f98ad532d45e8c`
- Candidate publication root:
  `9d3432db6eee33201c03d50ac9666bc050d46d4a0519170d05f05132ed5c32e8`
- Candidate public dossier SHA-256:
  `6d2dea9f676d996b7b1ddf8b6737cc61d80fbbf06ba473250fd8800842fdfbfd`
- Re-freeze implementation-manifest SHA-256:
  `4dc7341d4c932ff531e1bc914bba1790fc9dc01f1eb405a7b6ccc31dd0efcb73`

The re-freeze manifest covers the ordered `SHA-256  path` records for all 58
changed or new files outside this package's mutable `artifacts/` directory,
relative to `FROZEN_BASE`. Gate reports and reviewer verification artifacts may
now change; implementation, source, public, release, and test files may not.

These identities must be replaced if review remediation changes a bound
source. A `PASS` below describes the current candidate only until the terminal
source freeze is recorded.

After all exit criteria and both verifications passed, three administrative
records changed only to mark completion: `package.md`, `docs/ROADMAP.md`, and
`docs/work-packages/README.md`. The resulting final 58-file manifest is
`3c66ea10e590154ffc1e1bf15a8e734d6af9b80248ac95ae5971194820fc98d6`.
No binding source, public, release, governance-contract, test, or exception
file changed after the heavy freeze.

## Executed Candidate Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | `PASS` | Ran after accepted-finding remediation; exit 0. |
| Focused clippy | `PASS` | Ran `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`; exit 0. |
| Crate nextest | `PASS` | Ran the focused crate target; 10 passed, 0 skipped. |
| Integration nextest | `PASS` | Ran `assurance_dossier_build_contract`; 18 passed, 0 failed, 0 skipped in 100.154 seconds. |
| Validate | `PASS` | Ran `cargo run -p openwepp-assurance -- validate --all`. |
| Plan all | `PASS` | Emitted ordered transitive inputs, outputs, source/review roots, and per-node fingerprints. |
| Plan dossier | `PASS` | The sole selected dossier reported its complete shared and dossier output set. |
| Build and committed drift check | `PASS` | Ran `build --all` followed by `check --all`; all five output hashes matched. |
| Release hook | `PASS` | Ran `bash tools/release/check_assurance_dossier_exports.sh`; validation and drift check passed. |
| Deterministic rebuild | `PASS` | Two clean temporary builds compared by `diff -qr`; no byte differences. |
| Snapshot create/confirm | `PASS` | Temporary `package-proof` create and confirm produced manifest SHA-256 `68059305c87af056c6c7d81dd21de104670270ccdce9afd21d7f4ccf2aab44a8`. |
| CRAP-remediation bound | `PASS` | The five previously actionable functions were decomposed into helpers whose zero-coverage static CRAP is at most 20; both reviewers traced unchanged behavior, and the fresh measured gate reports zero actionable rows. |
| Review invalidation and snapshot negatives | `PASS` | Prefix-history edit/remove/reorder, stale roots, self-review, pending participants, snapshot conflicts, no-follow roots, and collision preservation are asserted. |
| Security negatives | `PASS` | Unknown fields, portable-path violations, traversal, symlinks, orphan output, unsafe ID, fragment/Markdown injection, absolute paths, secret families, and forbidden execution surfaces are covered. |
| Release shell syntax | `PASS` | Ran `bash -n` for both changed release scripts. |
| Markdown lint | `PASS` | `markdown-doc lint` validated all 44 changed/new Markdown files with 0 errors and 0 warnings. |
| Spelling preview | `PASS` | `uk2us` preview found only preexisting catalog/roadmap wording, deliberate `non-agricultural` terminology, the published title “Hydrological modelling,” and review text documenting those exceptions; no rewrite applied. |
| Whitespace | `PASS` | Ran `git diff --check`; exit 0. |
| Source banners | `PASS` | Four generated Markdown files exist and all four carry an `openwepp-assurance` do-not-edit source banner. |
| Private-path/secret scan | `PASS` | Generated public/export content contains no `/home/workdir`, `/workdir`, credential, token, or secret marker. |
| Rust line count | `PASS` | Every touched `.rs` file is below 2000 lines; detailed counts follow. |

## Terminal Heavy Gates

Ran: the delegated heavy runner independently reproduced the 58-file manifest
before and after the complete sequence. The path lists and per-file hashes were
byte-identical. Durable details and artifact checksums are in
`heavy-gate-runner.md`.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --all -- --check` | `PASS` | Exit 0 in 2.261 seconds. |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` | Exit 0 in 4.102 seconds. |
| `cargo nextest run --workspace --profile full` | `PASS` | 1,988 executed, 0 failed, 3 configured skips; run `52e1c25f-848f-4f25-8282-af6c6a383818`. |
| `cargo deny check` | `PASS` | Advisories, bans, licenses, and sources all passed. |
| Fresh adjudicated CRAP | `PASS` | 8,768 production entries; raw 2, adjudicated 2, actionable 0, touched files 14; no waiver or registry change. |

The CRAP acquisition's before, after, and final 230-source manifests are
byte-identical at
`e5906851a8a962f4f5e89648fc592fee1602602b4950ac4c1160821abf3bfbfc`.
All 16 generated evidence checksums verify.

## Rust Line Counts

Ran: `wc -l` on every touched/new Rust file:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/authoring.rs` | 223 | `PASS` |
| `crates/openwepp-assurance/src/cli.rs` | 202 | `PASS` |
| `crates/openwepp-assurance/src/engine.rs` | 1942 | `PASS` |
| `crates/openwepp-assurance/src/error.rs` | 74 | `PASS` |
| `crates/openwepp-assurance/src/graph.rs` | 379 | `PASS` |
| `crates/openwepp-assurance/src/hash.rs` | 102 | `PASS` |
| `crates/openwepp-assurance/src/lib.rs` | 21 | `PASS` |
| `crates/openwepp-assurance/src/main.rs` | 9 | `PASS` |
| `crates/openwepp-assurance/src/model.rs` | 538 | `PASS` |
| `crates/openwepp-assurance/src/path.rs` | 203 | `PASS` |
| `crates/openwepp-assurance/src/publication.rs` | 415 | `PASS` |
| `crates/openwepp-assurance/src/render.rs` | 415 | `PASS` |
| `crates/openwepp-assurance/src/review.rs` | 415 | `PASS` |
| `crates/openwepp-assurance/src/snapshot.rs` | 305 | `PASS` |
| `tests/integration/assurance_dossier_build_contract.rs` | 1451 | `PASS` |

No file reaches the 2000-line warning threshold or the 3000-line closure
threshold.

## Exit-Criterion Classification

| ID | State | Candidate evidence or open condition |
| --- | --- | --- |
| `ASSURE-LIFE-001` | `PASS` | Canonical ownership table and public why/how/what/so-what route. |
| `ASSURE-LIFE-002` | `PASS` | Five-state lifecycle, transitions, immutable history, and trigger matrix. |
| `ASSURE-LIFE-003` | `PASS` | Contract and public pages keep all seven assessment concepts distinct. |
| `ASSURE-LIFE-004` | `PASS` | Source/generated roots, IDs, versions, supersession, and snapshots are declared and checked. |
| `ASSURE-BUILD-001` | `PASS` | Four frozen CLI operations pass for the all/targeted forms that apply. |
| `ASSURE-BUILD-002` | `PASS` | Typed fixed graph includes authoring-only dependencies; exact streaming fingerprints, frozen input/path identities, strict schemas, and forbidden-surface tests pass. |
| `ASSURE-BUILD-003` | `PASS` | Separate clean builds are byte-identical; plans, exact output inventory, and drift failure are tested. |
| `ASSURE-BUILD-004` | `PASS` | Published source/root mismatch produces `REVIEW_REQUIRED`; semantic history edit/remove/reorder invalidates the prefix-bound approval. |
| `ASSURE-BUILD-005` | `PASS` | Explicit content-bound snapshot create/confirm/conflict, no-follow creation/layout, and exclusive collision-safe staging tests pass. |
| `ASSURE-PILOT-001` | `PASS` | Inventory and evidence manifest classify retained and missing evidence without promotion. |
| `ASSURE-PILOT-002` | `PASS` | Pilot remains `CANDIDATE / INSUFFICIENT_EVIDENCE`; narrow verification stays separate. |
| `ASSURE-PILOT-003` | `PASS` | Committed catalog, method, dossier, narrative, and worksheet form a navigable route. |
| `ASSURE-PILOT-004` | `PASS` | Templates/banners, link tests, public scan, and narrative de-duplication pass. |
| `ASSURE-XREPO-001` | `PASS` | Deterministic export and precise handoff exist; downstream deployment is not claimed. |
| `ASSURE-REL-001` | `PASS` | Real release runner consumes drift hook and records explicit snapshot manifest digest. |
| `ASSURE-TEST-001` | `PASS` | Nextest executes tests only; focused, negative, deterministic, consumer, and snapshot tests pass. |
| `ASSURE-SEC-001` | `PASS` | Negative filesystem, parser, public-output, and forbidden-surface checks pass. |
| `ASSURE-GOV-001` | `PASS` | Governing documents and navigation use the reconciled asymmetric vocabulary. |
| `ASSURE-CLOSE-001` | `PASS` | Terminal formatting, workspace clippy, full-profile nextest, and deny pass on the unchanged freeze. |
| `ASSURE-CLOSE-002` | `PASS` | Fresh closure-eligible CRAP report has zero actionable rows; every touched assurance function is at most 30. |
| `ASSURE-CLOSE-003` | `PASS` | Candidate line counts are recorded above; repeat if review changes Rust. |
| `ASSURE-CLOSE-004` | `PASS` | Both independent verifiers closed their assigned findings and confirmed the unchanged terminal freeze and heavy evidence. |

## Cross-Repository And Scope State

Static: no command in this execution wrote to `/home/workdir/wepppy`.
Concurrent work advanced that repository after intake; observed re-freeze HEAD
and status/diff hashes are recorded without attributing them to this package in
`owned-file-manifest.md`.

Ran: the intended-write-set comparison over every changed/new openWEPP path
returned `violations=0`. Final closure checks repeat this classification after
the administrative package-status delta.
