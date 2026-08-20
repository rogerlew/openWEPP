# Gate Results

Status: `COMPLETE / terminal A+B PASS`.

Exact reviewed implementation commit:
`3ea08d81d966ccbf163ee64377aa741308e2665a`.

- orchestrator: 722/722 PASS;
- persisted restart: 30/30 PASS;
- released restart reference: 28/28 PASS;
- focused V10 owner suite: 9/9 PASS;
- focused authority/integration: 31/31 PASS;
- anti-evasion and AUTH11: PASS, 3/3;
- affected warnings-denied Clippy: PASS;
- exact-head workspace: 3,087/3,087 PASS, 33 canonically skipped, after the
  three environment-only failures passed with required LLVM/Nix GCC runtime
  dependencies;
- doctests: PASS;
- cargo-deny: PASS with one non-blocking unmatched-license allowance warning;
- formatting and diff hygiene: PASS;
- exact-current Child-4 comparator: 20/20 PASS (seven benchmark surfaces and
  thirteen Child-4 selectors), recorded at
  `../../20260814-vegetation-land-surface-real-consumer-shadow-001/artifacts/comparator-heavy/20260820T003717Z-child4-root-owner-final/`;
- three independent reviews and both terminal verifiers: PASS through evidence
  commit `b327f9eabdc3bc061aab6d5496aaf7496762eade`.

No production activation or output claim is made.
