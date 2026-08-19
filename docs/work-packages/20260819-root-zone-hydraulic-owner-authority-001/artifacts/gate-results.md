# Gate Results

Evidence class: `Ran + Static`; base `2465849c9e0eed83c4e3aec11caa8b27cf7597ad`.

| Gate | Result | Evidence |
|---|---|---|
| Exact intake | PASS | HEAD/origin required base; clean; diff hygiene PASS. |
| Primary sources | PASS (Static) | CTSM release-clm5.0 sections/hashes and Clapp--Hornberger DOI; explicit non-use list. |
| Root path/current K authority | PASS (Static) | Required non-defaulted stratum path; live `Ksat*S^(2B+3)`; no aliases/defaults. |
| Artifact regeneration | PASS (Ran) | Independent Python generator; contract test byte-compares eight artifacts. |
| Root-zone authority test | PASS (Ran) | Nix shell Nextest quick, 7/7; every accepted vector exact-bit checked with `libm`; every rejected vector executes typed guards; configuration/receipt joins, atomic poisons, schemas, manifest, and independent calculator execute. |
| Anti-evasion | PASS (Ran) | `check_authority_suite_antievasion.sh`. |
| AUTH11 | PASS (Ran) | Nextest quick, 3/3. |
| Format/diff hygiene | PASS (Ran) | `cargo fmt --all`; `git diff --check`. |
| Warnings-denied focused Clippy | PASS (Ran) | `cargo clippy --test root_zone_hydraulic_authority_contract -- -D warnings`. |
| Science-contract admission | PENDING | Candidate is intentionally not yet approved; rerun after review/promotion. |
| Reviews/verifiers | PENDING | Dispatch against frozen candidate commit. |
| Production implementation | NOT RUN | Contract-first sequencing. |

Rust commands used `nix-shell -p cargo rustc cargo-nextest`; ambient Cargo absence
is infrastructure, not a science HOLD.
