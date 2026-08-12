# Verification Agent A

Status: `PASS / no unresolved material finding`

Evidence mode: `Static + Ran`

Role: independent Stage-A terminal verifier against the current exact worktree
bytes at base HEAD `02631ae92af6b073ed7957592fef4bad68dcf77f`.

## Terminal Checks

- Reconciled the complete tracked and untracked diff with the declared write
  set. Canonical changes are limited to the two vegetation contracts and
  registry, the two byte-identical V2 definitions, authority/package evidence,
  and contract-derived tests. The only production Rust change is the explicitly
  authorized fail-closed containment move: E04 topology authority is validated
  immediately after execution validation and before any E04 evaluation. No
  runtime selector, production consumer, output, or activation path changed.
- Found and reported an accidentally retained 996 MiB Cargo `target/` tree
  beneath the comparator logs. The parent moved it intact outside the
  repository. Reinspection found 100 concise evidence files totaling 3.5 MiB;
  no generated build tree or root scratch directory remains in the diff.
- Historical V1 bytes match both HEAD and the recorded immutable identity,
  SHA-256
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Both recursively sorted canonical V2 definition copies are byte-identical,
  SHA-256
  `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.
  The promoted shared transaction contract is SHA-256
  `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8`.
  The contract headers, bodies, registry, definition bindings, and tests agree
  on `approved/active` V2 authority.
- Reassessed every original and repeat-review finding through
  `review-finding-disposition.md`. All are accepted and corrected. The final
  independent topology/energy review and resource/transaction review both
  return PASS with no unresolved material finding; their historical HOLD
  artifacts remain unchanged.
- The admitted authority specifies one persistent lane per valid occupancy,
  conditional plant area, exact top-to-bottom potential and fixed-cap final
  columns, same-tile first/second drainage, stemflow bypass, immutable
  arbitration, and descendant consumption of final upstream release.
- `SC-VEGETATIONTRANSACTION-001` supplies the cross-owner identity missing from
  the initial reviews. Water retains transaction, stratum, tile, layer,
  resource, and stand-ground amount basis through request, proportional
  authorization, finalized use, debit, and receipt. Energy independently
  reconstructs occupancy and weighted-stand component operands. NH4/NO3 and
  layer identity remain distinct, and all four candidates commit atomically.
- The exhaustive occupancy state schema binds units, cardinality, recursive
  lexical serialization, state digest, prior transaction identity, and
  fail-closed migration. V1 warm starts are neither copied nor synthesized;
  nonunique multi-tile liquid migration is rejected.
- Regenerated the independent Python fixture and compared it byte-for-byte with
  the committed fixture: PASS, SHA-256
  `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
  The fixture and Rust authority test cover routing, area conversion,
  reductions, permutation, schema/state identity, owner rollback, shared C/N,
  mineral-N identity, and the accepted poison alternatives.
- The controlled vapor operands claim only E04 routing causality. No invented
  cap-to-vapor or substitute constitutive equation remains. Exact coupled
  physiology is explicitly retained as the named Stage-B
  `STAGE_B_E11_E15_EXACT_ORACLE` acceptance gate.
- Science admission rerun: PASS,
  `A0_ADMITTED contracts=45 science_surfaces=0`, receipt
  `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628`.
- Vegetation authority suite rerun: PASS `14/14`. Oracle regeneration comparison
  and `git diff --check` also pass.
- Heavy evidence is complete: workspace Clippy PASS; doc tests PASS; `cargo
  deny check` PASS; formatting and diff hygiene PASS; and the terminal exact
  full-profile run reports `2422/2422` passed with 33 canonically skipped in
  `3270.563 s`. Earlier interruption, disk-space, and launcher failures remain
  preserved rather than overwritten.
- Rust line-count governance: the touched production transaction file is 2,088
  lines, a `WARN` above 2,000 but below the 3,000-line mandatory-refactor
  threshold. This package adds only the 15-line early containment helper/move;
  the implementation package retains ownership of its later decomposition.
- No runtime activation, real-consumer cutover, calibration, canopy-snow, soil
  transformation, validation, or transferability claim appears in the diff.
  The implementation package remains gated until Stage-A final disposition;
  its handoff freezes the exact V2 digest and the complete Stage-B coupled gate.

## Findings

No unresolved material finding remains. The still-active kickoff prompt and
in-progress disposition are correct pre-terminal state: they may be archived
and marked complete only after both independent terminal verifiers pass these
same bytes.

## Verdict

`PASS`

Stage A may complete as
`COMPLETE / OPENWEPP_C3_WOODY_V2 implementation authority released`. This
verdict releases implementation authority only; it does not authorize runtime
activation or production cutover.
