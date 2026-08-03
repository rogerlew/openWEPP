# Gate Results

Status: technical validation, dual review, and dual terminal verification PASS

Evidence mode: **Ran**

| Gate | Result |
|---|---|
| Corrected intake reproduction | `61/231` refusals reproduced; retained log 01 |
| Prior-trigger reproduction | `37/227` refusals reproduced in detached prior worktree; retained log 02 |
| Unbounded prior/current diagnostic partition | PASS; exact storm partition retained in CSV and logs 03–04 |
| Contract-derived red test | Expected compile failure: helper absent; log 05 |
| Original contract-derived green test | Historical `1/1`; initial reviews found insufficient vectors; log 06 |
| Terminal focused EROD16 | `1/1` passed; `4/231` refusals, 227 depositing; log 08 |
| Owning orchestrator crate | `429/429` passed; log 09 |
| Warnings-denied clippy | Initial needless-borrow finding corrected; terminal pass; logs 10–11 |
| Quick profile | `2150/2150` passed, 38 skipped; log 12 |
| Frost profile | `345/345` passed, 1897 skipped; log 13 |
| Critical full workspace | `2237/2237` passed, five skipped; log 14 |
| Erosion profile | `371/371` passed, 1871 skipped; log 15 |
| `cargo fmt --all -- --check` | PASS; log 16 |
| Assurance plan | PASS; three selected v2 reports; log 17 |
| Assurance validate | PASS; `3/3` reports; log 18 |
| Scoped Markdown lint | PASS; 32 files, zero errors/warnings; log 19 |
| Workspace doctests | PASS; all workspace doctest targets, zero failures; log 20 |
| Review-correction focused W2C | `5/5` passed, 428 skipped; log 21 |
| Review-correction EROD16 | `1/1`; `4/231` refusals, 227 depositing; log 22 |
| Review-correction owning crate | `433/433` passed; log 23 |
| Review-correction clippy/format | PASS with warnings denied; log 24 |
| Straddling-seam corrected W2C | `6/6` passed, 428 skipped; log 28 |
| Boundary-ledger corrected EROD16 | `1/1`; `4/231` refusals, 227 depositing; log 29 |
| Straddling-seam clippy | PASS with warnings denied; log 30 |
| Final evidence-corrected W2C | `7/7` passed, 428 skipped; log 31 |
| Final boundary-ledger EROD16 | `1/1`; `4/231` refusals, 227 depositing; log 32 |
| Final evidence-corrected clippy | PASS with warnings denied; log 33 |
| Terminal quick profile | `2156/2156` passed, 38 skipped; log 34 |
| Terminal frost profile | `345/345` passed, 1903 skipped; log 35 |
| Terminal erosion profile | `377/377` passed, 1871 skipped; log 36 |
| Terminal Critical full workspace | `2243/2243` passed, five skipped; log 37 |
| Terminal owning orchestrator crate | `435/435` passed, zero skipped; log 38 |
| Terminal warnings-denied clippy | PASS; log 39 |
| Terminal formatting | PASS; log 40 |
| Terminal workspace doctests | PASS; log 41 |
| Terminal assurance plan | PASS; three selected v2 reports; log 42 |
| Terminal assurance validate | PASS; `3/3` reports; log 43 |
| Terminal scoped Markdown lint | PASS; 32 files, zero findings; log 44 |
| Revision-58/59 focused W2C | `7/7` passed, 428 skipped; log 45 |
| Revision-58/59 EROD16 | `1/1`; `4/231` refusals, 227 depositing; log 46 |
| Revision-60 scoped Markdown lint | final post-review `35` files, zero findings; log 47 |
| Revision-59 Binding Exposure Index | PASS; six rows fully consolidated; log 48; strict mode also passes |
| `git diff --check` | PASS on verifier-ready tree |

The earlier quick, frost, erosion, Critical full, assurance, documentation, and
doctest rows predate accepted review corrections and remain historical only.
Logs 34–43 are the renewed terminal evidence for the accepted source.

## Review-Correction Provenance

- working directory: `/home/workdir/openWEPP`;
- base commit: `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`;
- revision-57 accepted runtime/test/contract diff SHA-256:
  `a41615fc0a673ca23b70de45e09b6b8a8b2cdfa32e2ce1ba0ac5059c5d9fb176`;
- current four-file runtime/test diff SHA-256:
  `ada609e061f5cc9eb91eaa249169ae0317548aeec71f0c57fc388d05bb1b64ee`;
- current five-file runtime/test/revision-60-contract diff SHA-256:
  `2089324becad4b78809ed11c72830522c99ad73c37dacf1098bfc635807e0f80`;
- Nextest config SHA-256:
  `d793e296952c607ec2136a41dc05ecd780437b2a6fcd8cc669386c5c5b4984cb`;
- fixture identities: `p4.run.toml`
  `cec0563c34518af4b14775182919e245f568773863b5276a2551be0605209a13`,
  `p4.sol` `ac93a019bafa288080db529a3f8f23caf336734b4df79133557050772168b946`,
  `p4.man` `853484a8195b94ef6fde276a8770753f3f026be197b181dc4d6df6d4af9f5c30`,
  `p4.slp` `8d007f0ea6a7899de0903834f352f7d41f87071c04810a5c1a1e76e8f4b5f2a3`,
  and `p4.cli`
  `d4d009652f47b9cfa3655e8b0fae1466d9ecd80cb49bdf78a550f92ff81dc823`.

| Exact argv | Exit / duration | Log |
|---|---|---|
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'` | `0`; `5/5` in `0.012 s` | 21 |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture` | `0`; `1/1` in `14.617 s` | 22 |
| `cargo nextest run -p openwepp-hillslope-orchestrator` | `0`; `433/433` in `147.391 s` | 23 |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | `0`; `8.38 s` | 24 |
| `cargo fmt --all -- --check` | `0` | terminal command output |
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'` | `0`; `7/7` in `0.014 s` | 31 |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture` | `0`; `1/1` in `26.033 s` | 32 |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | `0`; `8.65 s` | 33 |
| `cargo nextest run --workspace --profile quick -j 8` | `0`; `2156/2156`, 38 skipped, `2709.255 s` | 34 |
| `cargo nextest run --workspace --profile frost` | `0`; `345/345`, 1903 skipped, `556.083 s` | 35 |
| `cargo nextest run --workspace --profile erosion` | `0`; `377/377`, 1871 skipped, `148.987 s` | 36 |
| `cargo nextest run --workspace` | `0`; `2243/2243`, five skipped, `2727.948 s` | 37 |
| `cargo nextest run -p openwepp-hillslope-orchestrator` | `0`; `435/435`, zero skipped, `147.905 s` | 38 |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | `0` | 39 |
| `cargo fmt --all -- --check` | `0` | 40 |
| `cargo test --doc --workspace` | `0`; all doctest targets pass | 41 |
| `cargo run --quiet -p openwepp-assurance -- plan --all` | `0`; three selected v2 reports | 42 |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | `0`; `3/3` reports | 43 |
| scoped `markdown-doc lint` | `0`; 32 files, zero errors/warnings | 44 |
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'` | `0`; `7/7`, 428 skipped, `0.011 s` | 45 |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture` | `0`; `1/1`, `4/231`, `13.772 s` | 46 |
| exact scoped `markdown-doc lint` argv in `terminal-markdown-scope.md` | `0`; final post-review 35 files, zero errors/warnings | 47 |
| `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SED-001.md` | `0`; six rows fully consolidated | 48 |
| `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SED-001.md` | `0`; six rows fully consolidated | terminal command output |

`cargo deny check` is not applicable: no manifest, lockfile, dependency, or
feature-resolution file changed.

The initial two independent reviews completed with findings. Every revision-57
finding was accepted and corrected, and both fresh reviews passed. Revision-60
Review B passes; Review A's sole stale-summary finding was corrected and its
narrative-only recheck passes. Both revision-60 reviews therefore pass. Dual
terminal re-verification passes after `R60-VB-01` was accepted and corrected.
No technical, review, or verification finding remains.
