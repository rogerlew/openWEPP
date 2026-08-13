# Terminal-Diff Reconciliation

Status: `RECONCILED / dual terminal verification PASS`

Evidence mode: `Static + Ran`

Base HEAD: `4f5bb1c599a683b63be56ecd9e7296f8faf01ed0`

## Exact Scope Reconciliation

`git diff --name-status` and `git ls-files --others --exclude-standard` were
reconciled path by path. The only roots are the V7 vegetation contract and
index, the byte-identical V3 definition copy in the existing model-stack
authority package, this complete authority package, the package catalog, and
the vegetation authority integration test. Every path is inside the declared
write set in `package.md`.

There is no production Rust delta. No crate source, model registry, Cargo file,
runtime selector, default, diagnostic consumer, hydrology/BGC owner, output,
deployment, publication, or activation path changed. The existing V2 public
vegetation transaction therefore remains fail-closed.

## Identity and Protected-Byte Proof

- V3 definition copies are byte-identical at SHA-256
  `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
- Independent fixture SHA-256 is
  `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`.
- Independent generator SHA-256 is
  `7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a`.
- Frozen V1 remains
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Frozen V2 remains
  `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.

The contract is 1,388 lines, the contract-derived test is 1,559 lines, the
independent oracle is 1,487 lines, and the package plan is 260 lines. These are
truthfully recorded in `line-count-governance.md`; no production-module
line-count threshold is implicated.

## Gate and Evidence Reconciliation

Both final science reviews return GO with no unresolved material finding.
Admission, unit governance, anti-evasion, AUTH11, focused authority tests,
warnings-denied workspace Clippy, workspace doctests, dependency policy,
formatting, and diff hygiene pass.

The exact uninterrupted full-profile run passed 2,481/2,481 tests with 51 slow
and 33 canonically skipped in 3,318.773 seconds. Its wrapper duration was 3,329
seconds and its unique `/home` scratch directory was removed. Earlier manual
interrupts and the root-filesystem ENOSPC attempt remain preserved as additive
evidence; they are not represented as scientific test failures.

The package measured 844 KiB before verifier artifacts and remains under 1 MiB.
It contains no nested Cargo `target` tree and no file larger than 10 MiB. Empty
launcher-wrapper attempts were moved intact outside the repository; meaningful
interrupted-run logs remain.

Both independent terminal verifiers passed with no unresolved material
finding. The verifier-owned artifacts and lifecycle summaries are present, and
the kickoff prompt moved byte-for-byte from `prompts/active/` to
`prompts/archived/` at SHA-256
`9252d2ce3fb553d598099762319cb502ae2a06ad2520b0da9ce31c09485f1c5c`.

## V5 Reconciliation In Progress

Base HEAD: `2685a1ea9fcfd51fef426eeb4c3685b419b2f768`

The V5 continuation changes only `SC-VEGETATION-001`, its registry row, one
byte-identical V5 definition copy in the existing model-stack authority
package, and this existing authority package's plan, prompt, generator,
fixtures, verifier, review, and evidence artifacts. There is no production Rust,
Cargo, runtime-selector, diagnostic-consumer, owner, output, deployment, or
activation delta.

Frozen V5 identities are definition `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`,
vectors `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`,
and generator `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`.
The two definition copies are byte-identical and exact V4 digest
`8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`
is imported unchanged. V1--V4 protected bytes are not edited.

The contract is 1,894 lines, the V5 generator is 1,282 lines, the independent
verifier is 405 lines, and the package plan is 399 lines. The package occupies
approximately 1.4 MiB and contains no file larger than 10 MiB. This is
authority/test-data complexity, not a production-module line-count exception.

Focused admission, unit, anti-evasion, AUTH11, authority, Markdown, formatting,
and diff gates pass. Dual independent science reviews return GO and confirm GO
after lifecycle-only cleanup.

The heavy comparator's workspace Clippy, doctests, dependency policy,
formatting, diff, admission, oracle/verifier, unit, anti-evasion, AUTH11, and
authority gates pass. The full nextest profile ran 2,582 tests: 2,581 passed,
33 were canonically skipped, and one source-contract test failed only because
the timestamped absolute TMPDIR made its Unix socket path exceed `SUN_LEN`.
The exact failed test then passed under short TMPDIR `/home/roger/v5t` in
24.027 seconds, with no source or science-byte delta. This is demonstrable
infrastructure exclusion, not borrowed correctness evidence. Package and index
Markdown commands also passed after correcting a wrapper argument-order typo.
All original failures and non-executing attempts remain preserved in
`v5-heavy-rerun-20260813T120528Z/` and
`v5-short-remed-20260813T130609Z/`.

Both terminal verifier artifacts are present and return PASS with no unresolved
material finding. The V5 prompt moved byte-for-byte from `prompts/active/` to
`prompts/archived/` at SHA-256
`959670289be1dc0c89f5f4acddcebbde03eeda938f6b4e9ab9022bc83326bcf5`.
