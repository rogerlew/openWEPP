# Gate Results

Status: complete / dual review and verification pass

Evidence mode: Ran

All commands ran from `/home/workdir/openWEPP`.

| Requirement | Command | Result |
| --- | --- | --- |
| Python syntax | `.venv/bin/python -m py_compile <four package tools>` | PASS |
| Focused unit tests | `.venv/bin/python -m unittest .../tools/test_analyze.py` | PASS, `8/8` |
| Accepted analysis | `.venv/bin/python .../tools/analyze.py` | PASS, `189` annual, `37,303` daily, `895,272` hourly, `273` dry, and `35` paired rows |
| Independent reduction | `.venv/bin/python .../tools/verify_results.py` | PASS; CoE, Stage-3, annual/monthly, forcing/state, dry, site, and pair reductions reproduced |
| Figure rendering | `.venv/bin/python .../tools/render_figures.py` | PASS; five PNGs, five sidecars, five source tables |
| JSON parsing | `find <package> <target> -name '*.json' ... jq empty` | PASS |
| Package Markdown lint | `markdown-doc lint --path <package> --format plain` | PASS, `26` files, zero findings |
| Package Markdown schema | `markdown-doc validate --path <package> --format plain` | PASS, `26` files |
| Catalog/roadmap docs | `markdown-doc lint/validate` on three changed catalog surfaces | PASS |
| US spelling preview | `uk2us` read-only preview on the package and changed catalogs | PASS; `CoE` token/path rewrites and unrelated pre-existing suggestions rejected |
| Authority anti-evasion | `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| AUTH11 obligations | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, `3/3` |
| Diff hygiene | `git diff --check` | PASS |
| Prompt archive | `sha256sum .../prompts/archived/20260804-execute-21l.md` | PASS, `10eaa736304c5e0262752a0df789b588aab0796aaa19d39b9f816cbd35e723fe` |
| Dual terminal verification | Two independent read-only terminal verifiers | PASS / PASS; no unresolved finding |

## Accepted identities

- Freeze: `ea403abd3648f7c1ff4e01f65a2159cd593fb7ed964e4403f08c35b738c05dfd`.
- Analyzer: `67856a153e24cd2ed1f9b816164d56f75d4c650cf933e58738f0b366c191cf34`.
- Result: `8ae39e3cb7e1206f787e5333e619b2e7f529f122566c566589be293e374f13e5`.
- Target execution receipt:
  `273b8438e1e8007c9b96bc8d0ca54d879273bfd05982e0a3f903c5b8ad62d4ea`.
- Independent verifier/tool:
  `ffda0c8117fa297a4b0d60ddf65779446348161c89737cd5295b797c0ebb4c26` /
  `e8f8dcde078b2ee5dcaca19306e8d938a11372f3327cae05ca341909a934f7db`.
- Figure renderer:
  `c9a2a5ca7100bfa63fbd3c3b6e84d2a34575eb0afe32e1c01281c6b5d01b1275`.
- Figure manifest:
  `f2e6af4da4e3ee2d2b2723298fd7ce4c9ef8273118522440c1836173818bfc81`.

Five trace identities match the 21K receipt. Maximum validation residuals are
`9.998e-13 m` snow mass, `3.438e-13 m` accumulation, `2.043e-17 m` CoE
reconstruction, `2.63e-17 m` independently reconstructed Stage-3 mass, and
`2.33e-8 J m^-2` independently reconstructed Stage-3 energy,
all inside the frozen tolerances.

## Failed prepublication attempts

Four prepublication correction classes were retained rather than hidden:

1. a 61-character Paradise digest transcription failed source identity;
2. omission of 21J's finite observed-`Tmax` dry guard failed exact interval
   inventory reconciliation;
3. a reusable target directory leaked stale downstream files into the receipt
   inventory and failed independent receipt verification; and
4. adversarial review rejected v1 for ambiguous dry-entry wording, missing
   retained-table custody, ineligible-day fallback, pooled component ranking,
   incomplete output scope, and producer-only closure evidence.

Each attempt failed closed before acceptance and closure. The rejected v1
namespace remains intact; v2 changed no scientific threshold or adjudication
matrix and inventories the analyzer's 12 owned outputs explicitly.

No Rust, production, contract, fixture, observation, or public-schema change
exists, so Rust formatting, Clippy, domain profiles, and full-workspace
regression are not applicable to this read-only analysis increment.
