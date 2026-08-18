# Gate Results

Exact-current authority hashes:

- contract: `1dc78f05bf6841254da2016f7eeec69d4a84963af763603d53a5ae5e6dc0e638`;
- calculator: `67154eb54f09f394c317fa82f6faff93ee3195bb944ba3ba7dc0839b3e839f59`;
- vectors: `e69c39187aac47993a63c3aa9e0ede922ccd3249b7fe4fa72116a8cc2d5c645c`;
- schema: `c2c08620052ca4e463a81031c42e05cdca904b90803877b78cde2a7ebad79a3c`.
- receipt provider definition: `4658de9f7590897633ffbfe0facedd52b5c9b9754f7d829f25869ef2c592f153`;
- model definition: `bb31aa1a48f9f7d19c117a995203362f8c3d57d06a011a5015558a62270de615`.

Ran on 2026-08-18:

- independent calculator twice versus frozen fixture: `PASS`, byte-identical;
- Draft 2020-12 complete-receipt validation plus separate full carry-field vector checks: `PASS`;
- `cargo test --test snow_free_half_hour_forcing_authority_contract --no-fail-fast`: `4/4 PASS`;
- focused warnings-denied Clippy for that target: `PASS`;
- `check_science_contract_admission.sh --base-ref 6abeac4... --worktree`: `PASS`, `A0_ADMITTED`;
- authority-suite anti-evasion: `PASS`;
- AUTH11 required-suite obligation guards: `3/3 PASS`;
- independent climate/radiation review: `GO / PASS`;
- independent vegetation/LSE review: `GO / PASS`;
- `cargo fmt --all -- --check`: `PASS`;
- `git diff --check`: `PASS`.

The lifecycle-only promotion occurred after the scientific reviews. Later
terminal-review remediation added only explicit noncircular provider identity
and exact schema/descriptor bindings, then regenerated and re-reviewed those
authority bytes. Terminal verification is pending. No implementation package
is released by this artifact alone.
