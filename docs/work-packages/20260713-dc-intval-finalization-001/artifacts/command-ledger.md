# Command Ledger

Status: `RELEASE-CANDIDATE-PASS`

Evidence class: **Ran** unless marked otherwise.

| Command | Result | Evidence |
| --- | --- | --- |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` before restoration | FAIL, exit 100; generic guard stopped at missing AUTH05 | `logs/01-auth11-red.log`, `logs/01-auth11-red.time` |
| focused nextest over AUTH11 plus five restored targets | PASS, 11/11 | terminal capture; exact command recorded in execution transcript |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | terminal capture |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3 | terminal capture |
| `git diff --check` | PASS | terminal capture |
| exact pinned-input release candidate 1 | FAIL, exit 101 in workspace Clippy on two `float_cmp` findings; authority/stability not reached | `logs/02-release-candidate.log`, `logs/02-release-candidate.time` |
| post-candidate `cargo clippy --workspace --all-targets -- -D warnings` | FAIL on AUTH11 extension comparison and predecessor AUTH06 `too_many_lines` | terminal capture |
| corrected `cargo clippy --workspace --all-targets -- -D warnings` | PASS | terminal capture |
| focused nextest over AUTH05, AUTH06, AUTH11, and HPHYS0224..0227 | PASS, 16/16 | terminal capture |
| exact pinned-input release candidate 2 | FAIL, exit 1 after all 1,955 full tests, deny, fixtures, authority suites, binaries, sidecars, and lint passed; stability 1,185 pass / 255 fail | `logs/03-release-candidate.log`, `logs/03-release-candidate.time`, `logs/03-release-candidate-hillstab-results.json` |
| exact focused WB05C-CO-H0034 CLI rerun | FAIL, exit 1; lane 3 day 404 `erosion.wave1.segment_toe` | candidate-2 scratch tree; reproduced during read-only diagnosis |
| focused EROD16 near-terminal contract vector before runtime correction | FAIL, exit 100 at terminal-station normalization assertion | `logs/04-erod16-terminal-normalization-red.log`, `.exit` |
| `cargo fmt --all`; focused EROD16 vector; focused orchestrator all-target Clippy | PASS | `logs/05-erod16-terminal-normalization-green.log`, `.exit`; terminal capture |
| exact release build of `openwepp-cli-hill` | PASS; binary size 10,731,432 bytes, mtime 2026-07-13 10:39:42 -0700 | terminal capture |
| rebuilt release CLI on exact WB05C-CO-H0034 p34 scratch input | PASS, exit 0; real runner consumed corrected Wave-1 geometry | `logs/06-wb05c-co-h0034-green.log`, `.exit` |
| first candidate-3 launch | INVALIDATED after fixed-source package evidence changed; interrupted exit 100 and excluded from acceptance | `logs/07-release-candidate.invalidated.*` |
| restarted fixed-source exact release candidate 3 | FAIL, exit 1 after 40:42.63; all non-stability gates pass; stability 952/1185 pass, 233 fail, 0 timeout | `logs/07-release-candidate.log`, `.time`, `-hillstab-results.json` |
| focused candidate-3 three-family vectors before runtime correction | FAIL, exit 100; 0/3 pass with each named mechanism | `logs/08-three-family-red.log`, `.exit` |
| focused candidate-3 three-family vectors after runtime correction | PASS, 3/3; broader growth/R4MO selection 13/13 | `logs/09-three-family-green.log`, `.exit`; terminal capture |
| exact focused H0259, H0327, MO-H0001, and OR-H0080 release CLI | PASS | `logs/10-three-family-focused-cli.log` plus terminal capture |
| material thaw-complete layer-basis vector before runner correction | FAIL with exact 1.303248764 mm no-final-frost debit signature | `logs/11-frost-thaw-clear-red.log`, `.exit` |
| material thaw-complete plus retained nonmaterial stale-clear vectors after correction | PASS, 3/3; runner all-target Clippy PASS | `logs/12-frost-thaw-clear-green.log`, `.exit`; terminal capture |
| rebuilt release CLI on p13/p14/p25/p27/p40/p43/p45/p49 | PASS, 8/8 real watchlist replays | `logs/13-frost-thaw-watchlist-green.log`, `.exit` |
| exact fixed-source release candidate 4 | FAIL, exit 100 after 10:38.97; fmt/Clippy pass; full nextest 1,927 pass / 32 fail, all stale `contract_version: 115`; later gates not reached | `logs/14-release-candidate.log`, `.time` |
| focused 32 corrected contract-marker targets | PASS, 120/120 | `logs/15-contract-version-bind-green.log`, `.exit` |
| exact fixed-source release candidate 5 | FAIL, exit 1 after 49:24.25; all workspace/deny/fixture/authority/build/sidecar/lint gates pass; stability 1,183/1,185 with exactly OR-H0081 and OR-H0204 failing the same negative Wave-1 publication class-fraction guard | `logs/16-release-candidate.log`, `.time`, `-hillstab-results.json` (SHA-256 `3ce5306d46f985f8564985625457f19918d3a51bf4e1c3a79155abce1e4bccf2`) |
| direct enrichment trace-load vector before producer correction | FAIL, exit 100 with `[3143.153396..., -785.538349..., ...]` | `logs/17-enrichment-floor-red.log`, `.exit` |
| direct enrichment trace-load vector after producer correction | PASS, 1/1; all HB04 internal characterizations 10/10; focused all-target Clippy PASS | `logs/18-enrichment-floor-green.log`, `.exit`; `logs/19-enrichment-hb04-green.log`, `.exit`; terminal capture |
| rebuilt release CLI on exact OR-H0081 and OR-H0204 | PASS, 2/2 real failed-case replays | `logs/20-enrichment-real-cli-green.log`, `.exit` |
| exact fixed-source release candidate 6 | PASS, exit 0 after 49:12.97; fmt/Clippy, full nextest 1,960/1,960, deny, fixture provenance, 9 required suites/7 targets/12 tests, release binaries/sidecars/lint, main stability 1,166/1,166, and watchlist 19/19 all pass | `logs/21-release-candidate.log`, `.exit`, `-authority-results.md`, `-hillstab-results.json` (SHA-256 `54f7a9eec0e01f73d113c227458e2274638167f2fa1ed191276e57decd0e64b0`) |
| first frozen-candidate integrated restart | INVALIDATED after all domain/consumer/package lanes passed: independent reconstruction found missing H2637 terminal groundwater state; the restarted release had passed 1,960/1,960 and all pre-stability gates and was stopped at stability because its evidence could not carry across the required correction | integrated campaign `restart-*` logs and `restart-conservation-and-consumer-evidence.md`; release restart exit 130 is an intentional invalidation, not a test failure |
| real H2637 terminal-storage manifest vector before producer correction | FAIL, exit 100 after 456.82 s at missing `initial_groundwater_storage_m3` | `logs/22-groundwater-terminal-storage-red.log`, `.time`, `.exit` |
| focused groundwater summary, runner/orchestrator Clippy, and real H2637 recurrence after producer correction | PASS; summary 1/1, all-target Clippy, H2637 1/1 after 459.08 s; timing and post-export residuals both about `-4.25e-11 m3` | `logs/23-groundwater-terminal-storage-green.log`, `.time`, `.exit`; manifest SHA-256 `ad939c97cfaaf4f2313827b48f99a346ed582f25f2ede03d12f1fe8c07e22eee` |
| exact release candidate 7 | FAIL, exit 101 at workspace Clippy before tests; H2637 real-consumer test is 107 lines after recurrence assertions | `logs/24-release-candidate.log`, `.time`, `.exit` |
| exact fixed-source release candidate 8 | PASS, exit 0 after 51:15.13; fmt/Clippy, full nextest 1,960/1,960, deny, fixture provenance, all required authority, release binaries/sidecars/lint, main stability 1,166/1,166, and watchlist 19/19 all pass | `logs/25-release-candidate.log`, `.time`, `.exit`, `-authority-results.md`, `-hillstab-results.json` (SHA-256 `7538b1ef17958b84dd5b0f4e998d9cdefa9c5aa026e25132aa5e1bdc89750f96`) |

## External release inputs

- `/workdir/wepp-forest` HEAD: `375ccc296ed1ea491f599ff1b1a25b415d494a2a`.
- cohort seed SHA-256:
  `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958`.
- watchlist SHA-256:
  `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab`.

These match the package-pinned identities before the first exact release run.
