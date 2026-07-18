# Terminal Gate Results

Ran: the authorized closure runner executed the conservative sequence once on
the frozen remediated tree. Pre/post `git status --short` was byte-identical;
the runner made no source, documentation, configuration, or commit changes.

- `cargo fmt --check`: PASS, exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS, 0.58 seconds.
- `cargo nextest run --workspace --profile full`: PASS, 2,141/2,141 tests
  across 196 binaries; 30 slow and 5 skipped; 684.036 seconds.
- `cargo deny check`: PASS; advisories, bans, licenses, and sources all OK.
- `bash tools/release/run_adjudicated_crap_gate.sh --base-ref
  594f9a184b66ba228e0e81d379172639db3e55b7`: PASS; fresh global workspace
  acquisition, 245 sources, 463 measurement inputs, 10,610 production entries,
  raw/adjudicated/actionable counts 2/2/0, five touched files, closure eligible.
  Runtime: 2026-07-18 18:51:05Z through 19:38:53Z. Production manifest:
  `af20e45f8750795c4160d808cd96d073d8120bb22369098095bdc918d03f8499`.

Global CRAP artifacts are retained under
`/workdir/openWEPP/target/adjudicated-crap`:

- Report JSON:
  `e8de3c68769aee4d94964ff8065fbee58ebec999dce04f7382554ad8cf8b24ea`.
- Report Markdown:
  `c800e7349a32f4993008b4164f568073ab0bb75fdd17993056f4ac55e3134f27`.
- Workspace CRAP JSON:
  `8a6f1a94b34e48ef62782a108a56fa0f1c7f807c77c20eac343ea9bca0adf9a7`.
- Workspace LCOV:
  `486e08c934b3124fdf5198d4afdcf6c893b29f1d79be74e7a8f880d6c7695d49`.
- Adjudication registry:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Run status:
  `996b422e4ed205cb147afddc732cd358a938d282baf88b6dd6efd9f79b00dde8`.
- Checksum manifest:
  `5aaa77678d846c478c3f52586fab82d1122741bbc55c4c926697dd4f02b3f644`.
