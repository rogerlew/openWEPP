# Gate Results

Status: `exact-head implementation gates PASS`

Evidence commit: `5143a829d3eb25f1f241256eb267f90a69ab8661`.

Ran:

- released reference tests: 28/28 PASS;
- released reference warnings-denied Clippy: PASS;
- production restart all-feature suite: 30/30 PASS;
- three focused restart integration targets: 10/10 PASS (terminal verifier A);
- direct authority: 7/7; forcing: 7/7; V10 nighttime: 3/3; AUTH11: 3/3;
- authority anti-evasion: PASS;
- workspace warnings-denied Clippy: PASS;
- full workspace Nextest: 3,076/3,076 PASS, 33 profile-declared skips;
- workspace doctests: PASS;
- cargo-deny: advisories, bans, licenses, and sources PASS;
- rustfmt and cumulative `git diff --check`: PASS.

An earlier full run passed 3,075/3,076 and exposed only the Nix shell's absent
rustup component. Binding the existing LLVM 21 tools closed the failed 4/4
contract. The final exact-head run used those bindings from the start.
