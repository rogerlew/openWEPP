# Heavy Gate Runner

Ran: complete terminal heavy closure from a renewed, source-quiescent freeze.

Status: `PASS`

Frozen base: `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`

## Freeze Identity

The runner independently enumerated changed and untracked paths relative to the
frozen base, excluded this package's `artifacts/**` subtree, sorted the paths,
emitted one `sha256sum` row per file, and hashed that ordered manifest.

- before: `58` files,
  `4dc7341d4c932ff531e1bc914bba1790fc9dc01f1eb405a7b6ccc31dd0efcb73`
- after: `58` files,
  `4dc7341d4c932ff531e1bc914bba1790fc9dc01f1eb405a7b6ccc31dd0efcb73`

The before and after path lists and complete file-hash manifests are
byte-identical. `HEAD` remained equal to the frozen base. No implementation,
public, governance, test, or exception-registry file was modified by the
runner.

## Command Results

Commands ran sequentially from `/home/workdir/openWEPP`.

| Order | Exact command | Exit | Result |
| ---: | --- | ---: | --- |
| 1 | `cargo fmt --all -- --check` | `0` | **PASS**, 2.261 seconds |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `0` | **PASS**, 4.102 seconds |
| 3 | `cargo nextest run --workspace --profile full` | `0` | **PASS**, 606.867 seconds wall time |
| 4 | `cargo deny check` | `0` | **PASS**, advisories, bans, licenses, and sources `ok`; 3.149 seconds |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 00d985b1c0de77f1ea664df23a6f4999c4dad0cc --output-dir docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/adjudicated-crap` | `0` | **PASS**, 2,352.241 seconds |

The full-profile Nextest JUnit record reports `1,988` executed tests, zero
failures, and zero errors. Its run UUID is
`52e1c25f-848f-4f25-8282-af6c6a383818`; recorded test time is 598.578 seconds.
The local JUnit file is `target/nextest/full/junit.xml`, SHA-256
`b569ab9399ee67ac3054869508f9e5e0d9f2313a75e9cf2951143592af849d1a`.

## Adjudicated CRAP Closure

The fresh, current-source, closure-eligible report ran from
`2026-07-15T01:53:38Z` through `2026-07-15T02:32:50Z` and assessed `8,768`
production entries.

- status and debt assessment: `PASS`
- raw rows strictly over 30: `2`
- exact current adjudications: `2`
- actionable rows: `0`
- touched production files: `14`
- touched actionable rows: `0`
- untouched actionable rows: `0`
- highest raw CRAP in the touched `openwepp-assurance` crate: `30`

Both raw rows are the unchanged established adjudications for
`MeteorologyError::fmt` and `SymbolAliasRegistryError::fmt`. There is no new
waiver or exception. The canonical exception registry remains SHA-256
`10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The source manifests before acquisition, after acquisition, and at final
reporting each contain 230 production sources and are byte-identical at
SHA-256
`e5906851a8a962f4f5e89648fc592fee1602602b4950ac4c1160821abf3bfbfc`.
`sha256sum -c` passed for all 16 entries in the generated evidence checksum
manifest.

The coverage subprocess uses nonordinary `--ignore-run-fail` acquisition and
logged failures for `-p openwepp --test laned_shadow_h2637` and
`-p openwepp-hillslope-orchestrator --lib`. That subprocess is not the binding
test authority. The separate full-workspace Nextest lane above passed all
1,988 executed tests before coverage acquisition.

## Durable Evidence

- JSON report:
  `artifacts/adjudicated-crap/adjudicated-crap-report.json`
  (`da699d2e4dd78ac32a7e043383c27efdaf9267a652755493d2bd2c584b95711c`)
- human report:
  `artifacts/adjudicated-crap/adjudicated-crap-report.md`
  (`70cf7bd8300526cfaa02c819ab544a540abfc97255ead3cd3fc86d9d1872e3f2`)
- raw CRAP JSON:
  `artifacts/adjudicated-crap/workspace-crap.json`
  (`ebfc0cbe5f8bbb711e0b80220667e6f7593205ba8ccffc1081932412d2501146`)
- LCOV:
  `artifacts/adjudicated-crap/workspace.lcov`
  (`d687b96473afe5f4e759f087acad1a09ea9b4a3b29154614a2a73cb402955026`)
- run status:
  `artifacts/adjudicated-crap/run-status.json`
  (`425ce061a3b79b613146c9818632b5c3753588dae216000be674d44bf6349a37`)
- checksum manifest:
  `artifacts/adjudicated-crap/sha256sums.txt`
  (`44c3827a0734c4818693bf44b46cdd210ec5a1ccda86b6fae209d5ce7b04a01d`)

The complete terminal heavy closure sequence passes on the unchanged renewed
candidate.
