# Reproducer

Ran at exact HEAD `761f990b1d7db93983d2854bf55c2a3756a6d63f`:

- direct LLVM test: 1 passed;
- LCOV SHA-256:
  `983b196bcd79c940cbd7e4fad8b366a11a67fd61997f5a7b4d11433b3ac0091e`;
- CRAP JSON SHA-256:
  `391bf76625170df5546199f1796758a1df2a59cbabea9bc496f609b2df377914`;
- no LCOV source record for `verifier_coverage_tests.rs` and no owned symbols;
- all four owned CRAP rows had null coverage.

Durable OPEN record digest:
`e2606bec117ce4d10dbb2925519ec34869ee0b4d99a31628401621903f155f98`.
