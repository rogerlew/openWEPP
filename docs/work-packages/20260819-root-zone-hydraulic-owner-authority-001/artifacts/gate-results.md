# Gate Results

Evidence class: `Ran + Static`; base `2465849c9e0eed83c4e3aec11caa8b27cf7597ad`.

| Gate | Result | Evidence |
|---|---|---|
| Exact intake | PASS | HEAD/origin required base; clean; diff hygiene PASS. |
| Primary sources | PASS (Static) | CTSM release-clm5.0 sections/hashes and Clapp--Hornberger DOI; explicit non-use list. |
| Root path/current K authority | PASS (Static) | Required non-defaulted stratum path; live `Ksat*S^(2B+3)`; no aliases/defaults. |
| Artifact regeneration | PASS (Ran) | Independent Python generator; contract test byte-compares sixteen generated artifacts and independently verifies the fifteen-entry payload manifest. |
| Root-zone authority test | PASS (Ran) | Nix shell Nextest quick, 7/7; every accepted vector exact-bit checked with `libm`; every rejected vector executes typed guards; configuration/receipt joins, atomic poisons, schemas, manifest, and independent calculator execute. |
| Anti-evasion | PASS (Ran) | `check_authority_suite_antievasion.sh`. |
| AUTH11 | PASS (Ran) | Nextest quick, 3/3. |
| Format/diff hygiene | PASS (Ran) | `cargo fmt --all`; `git diff --check`. |
| Warnings-denied focused Clippy | PASS (Ran) | `cargo clippy --test root_zone_hydraulic_authority_contract -- -D warnings`. |
| Science-contract admission | PASS (Ran) | `A0_ADMITTED`; 48 contracts, zero production science surfaces in this authority-only package. |
| Independent authority reviews | PASS (Static + Ran) | Soil hydraulics, geometry/ownership, and Rust/numeric/schema reviews all bind PASS to exact authority commit `b30f42de67136bca37f888fa62e8f1145537a230`; no finding waived or deferred. |
| Terminal verifier A | PASS (Static + Ran) | Exact promoted candidate `5cb09bb9029ef0c2991de3b8477c1c15504a3117`; focused 7/7, validator, A0, anti-evasion and diff hygiene PASS; no production/restart/activation mutation. |
| Terminal verifier B | PASS (Static + Ran) | Exact promoted candidate `5cb09bb9029ef0c2991de3b8477c1c15504a3117`; generator, validator, focused 7/7, Clippy, A0, anti-evasion, AUTH11 3/3 and cumulative hygiene PASS. |
| Production implementation | NOT RUN | Contract-first sequencing. |

Rust commands used `nix-shell -p cargo rustc cargo-nextest`; ambient Cargo absence
is infrastructure, not a science HOLD.
