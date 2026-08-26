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

Ran: complete Assurance selections passed 32/32 and 109/109 (two configured
skips). Retained guards passed 47/47. The authority-suite anti-evasion script,
`cargo fmt --all --check`, and `git diff --check` passed. Exact-head workspace
qualification and independent reviews remain required before terminal
disposition.
