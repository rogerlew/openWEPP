# Implementation diff reconciliation

Evidence class: `Static` plus focused `Ran` evidence.

The implementation diff is authority/test/documentation only. It contains no
production Rust source, fixture, selector, restart, receiver, runner, CoE,
terminal-behavior, or cutover edit.

The four canonical science-contract Git objects equal their exact objects at
fully qualified physical implementation `43cc9bbe`:

| Contract | Git object |
|---|---|
| `SC-SNOWENERGY-001.md` | `7f307af660d4af81a0f884aa555317052adda88c` |
| `SC-SNOWFREEZE-001.md` | `8889a720a3a7ffcd9195bf518e36341ca0b2b370` |
| `SC-LANDSURFACEENERGY-001.md` | `f221301095eceae898a89afbb31a28662e71893c` |
| `SC-COUPLEDTIME-001.md` | `d4566a3bde676f9bf975744ce9d66edf4a745548` |

The lifecycle index changes only those four rows and is likewise the exact
`43cc9bbe` object. Rejected v21/v139 and companion candidate bytes remain in
Git history and preserved research/work-package artifacts, not in active
canonical authority.

Seven test files change only stale exact bindings:

- six SnowFreeze assertion occurrences across five test files name released
  v136 while retaining their full invariant/obligation token sets;
- the Stage-0 guard adds two exact Stage-3-only paths while retaining its
  global scan and forbidden-token set; and
- the attachment guard expands scanning to the decomposed terminal module,
  retains all rejected-token checks, and requires the current exact parcel
  constructor.

The first exact-clean workspace attempt after that increment exposed six
additional stale assertions across three historical candidate-test binaries.
Those tests required v19/v20/v21/v139/v6 candidate text to remain in active
canonical contracts, which conflicts with the terminal NO-GO disposition.
Their complete existing assertion sets now read immutable preserved Git
objects: held chronology/v20 at `83fb00514e8932561bee5aff26ccdf7c130d470f`
and the final rejected v21 set at owner-provided starting checkpoint
`e3b9e20eebbf5ecd319c372c3d31b1a05a2479d7`. They do not read or alter current
canonical authority. Focused execution passed 7/7, nextest run ID
`a0b3029a-2546-4359-9804-cfeb7a7602bc`.

Ran: complete Assurance selections passed 32/32 and 109/109 (two configured
skips). Retained guards passed 47/47. The authority-suite anti-evasion script,
`cargo fmt --all --check`, and `git diff --check` passed. Exact-head workspace
qualification and independent reviews remain required before terminal
disposition.
