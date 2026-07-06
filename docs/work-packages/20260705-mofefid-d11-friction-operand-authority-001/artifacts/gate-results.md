# Gate Results

Status: executed
Evidence mode: Static + Ran

| Gate | Runner | Result | Evidence / log |
|---|---|---|---|
| `git diff --check` | Codex | Ran: PASS | No whitespace errors. |
| Markdown lint | Codex | Ran: PASS | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --no-ignore`; `markdown-doc lint --path docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001 --no-ignore`; `markdown-doc lint --path docs/work-packages/README.md --no-ignore`; `markdown-doc lint --path docs/planning/mofe-fidelity-campaign-strategy.md --no-ignore`. All returned 0 errors / 0 warnings. |
| Contract/profile/BEI checks | Codex | Ran: PASS | `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` returned `PASS-DEFERRED`: 5 binding exposure rows, 4 science-review-follow-on rows. This is an accepted checker pass for active-but-unpromoted Lane D surfaces and does not claim production closure. |
| Unit-governance checks | Codex | Ran: PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`; `bash tools/release/check_unit_registry.sh`. |
| Focused friction tests | Codex | Ran: PASS | `cargo test -p openwepp-hillslope-orchestrator friction -- --nocapture`: 8 existing friction tests passed. |
| Builder/fail-closed tests | Codex | Static: BLOCKED | No builder tests exist or were authored. They are blocked because no source-authorized builder can be implemented until `GAP-OFEROUTE-007` source/default authority is ratified. |
| H2637 or targeted Lane D fixture evidence | Codex | Static: NOT RUN | D11 made no executable Lane D builder/runtime change and the current H2637 shadow remains diagnostic-only with hardcoded friction operands; rerunning it would not prove `GAP-OFEROUTE-007` closure. |
| Focused D-val Case-4 / resolution gates | Codex | Static: NOT RUN | Out of D11 scope and blocked by D10 `GAP-OFEROUTE-005` hold. D11 did not alter shock numerics, `k_o` mapping, or Case-4 acceptance surfaces. |
| `cargo fmt --check` | Codex | Ran: PASS | Formatting unchanged. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Codex | Static: NOT RUN | No Rust source or executable surfaces were edited; D11 closed as source-authority HOLD before implementation. |
| `cargo nextest run --workspace --profile full` | Codex | Static: NOT RUN | No Rust source or executable surfaces were edited; D11 closed as source-authority HOLD before implementation. |
| `cargo deny check` | Codex | Ran: PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Anti-evasion guards, if triggered | Codex | Static: NOT RUN | Not triggered: D11 did not touch external-authority suite posture, cohort fixtures, or required-case bindings. |
