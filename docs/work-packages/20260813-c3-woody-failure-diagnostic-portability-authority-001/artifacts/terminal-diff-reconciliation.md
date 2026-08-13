# Terminal Diff Reconciliation

Evidence: `Static`

Status: `PASS / exact promoted and archived bytes reconciled`

The final write set is limited to the package tree, the Version 10 amendment
and registry row, the canonical V6 definition copy, the work-package catalog,
and the contract-derived authority test. No production crate, Cargo manifest,
runtime selector, deployment, publication, or consumer path changed.

Protected V1--V5 model definitions retain their recorded SHA-256 identities.
Both V6 definition copies are
`a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426`;
the generator is `bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532`;
the vectors are `2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b`;
and the verifier is
`a71f0d149a753183d2b97d59d0609c184618f993d83e2a8c4abba87bc8671ba1`.

Separate verifier A returned `PASS-WITH-NOTES` and verifier B returned `PASS`
before promotion. Both post-promotion addenda returned `PASS`. Exact promoted
admission produced authority SHA-256
`7759fe4819ee3741298abcddf86966ad5fa3d68837ac7cf380f614d1f7b76753`.
The active prompt was moved byte-for-byte to `prompts/archived/` and retains
SHA-256 `2228a6426779e742bd93121353a978fe9dd3161d366adda0cc12c2b0cce79efe`;
`prompts/active/` contains no kickoff file.
