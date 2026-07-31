# Gate Results

Status: `complete / validation pass / package hold`

Evidence mode: `Ran`

| Requirement | Result |
| --- | --- |
| Python compile | PASS |
| Analysis-only deterministic regeneration | PASS; two consecutive generations were byte-identical across JSON, CSV, adjudication Markdown, seven SVGs, and seven sidecars |
| Factorial-effect retention | PASS; 96/96 lane-response rows retained with explicit unavailable status and partial cell values |
| Result-bearing rerun refusal | PASS; attempt ledger forced exit 1 before subprocess execution |
| Retained execution inventory and hashes | PASS; 12 lanes, 48 cells, 12/10/2/0 completion, 22/2 failure classes |
| Figure/sidecar and SVG parse | PASS; 7/7/7 |
| `snow_surface_eb03_contract` | PASS 9/9; run `566744fc-9ee9-4fa3-bd03-ce53a9bfca8d` |
| `snow_surface_eb03_runtime` | PASS 6/6; run `565b7b01-8fa8-48f5-b2ce-150ba0275fee` |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Quick profile | PASS, exit 0; PTY did not retain the terminal summary text |
| Frost profile | PASS 324/324, 1 slow, 1,876 skipped, 516.449 seconds; run `db28c935-3cb6-4aff-9232-f66032a420e8` |
| Full profile | PASS 2,159/2,159, 32 slow, 29 skipped, 2,218.897 seconds; run `d9ed74dd-30c6-492e-8bfa-4349636c0ea3` |
| Workspace doctests | PASS; 20 crates, zero doctests present |
| Scoped Markdown lint | PASS; 42 files, zero errors or warnings |
| American-English normalization | PASS; no proposed diff |
| `git diff --check` | PASS |

The earlier interrupted quick attempt is not terminal evidence. The terminal
quick invocation returned exit 0 after the executable tree was frozen; because
the PTY retained neither its run header nor final count, this artifact does not
invent those fields. Frost and full retained complete Nextest headers,
summaries, run IDs, and exit-0 receipts.

## Terminal Identity

- Base HEAD: `89da2dcf46cb3b05d8a67611332bc61fd0681f67`.
- Rust trace-builder SHA-256:
  `4505d17e78087e15119d2a649968d0cfbb7ac96e8eeeb043981e02ed518d942b`.
- Focused contract-test SHA-256:
  `2e0943b335cb7fee3bfe61229e1db09870b677529564b7b100ac79ba99e823cf`.
- Package runner SHA-256:
  `e84a1732a847b978cc529ba95bb276b4f47ff37e991d06798d158523f2bace17`.
- Executable two-file stable patch ID:
  `156024c90a2ba28f843956e7648c62be5803ae29`.
- Retrospectively recorded execution-attempt attestation SHA-256:
  `6383a70cb2a999f2394e1bbeb8d513108edbe95bb60ed7c6e808877bd22f7822`.
- Original executed binary SHA-256:
  `9b004d8f4434e227a232e58f5594d1cf0fb61b5a43bf94ff0f667e92e362b4a5`.

All validation passed. The package remains HOLD because validation success
cannot supply the missing retained shortwave and signed latent/mass operands
or reverse the scientific physical-gate failure.
