# ASSURE-06 Gate Evidence

Evidence class: Ran unless labeled Static.

## Scientific Result And Source

- Package reconstruction: PASS. The standard-library procedure regenerated all
  188 strict result values from the content-identified inputs, including four
  independently calculated selected-row residuals, and parsed JSON was exactly equal to
  `results/snow-frost-synthesis.json`.
- Named V2 validation: PASS; one selected report, two total V2 reports, zero
  public reports, lifecycle `DRAFT`.
- Named V2 plan: PASS; the report target and every prerequisite were current.
- American-English normalization check: PASS with `changed: false`.

## Disposable Consumer

Two unrelated terminal staging roots, `/tmp/assure06-terminal-a` and
`/tmp/assure06-terminal-b`, were independently seeded with the snow/frost model
narrative. Named `build` and `check` passed in both roots. `diff -qr` reported
no difference between complete staged trees.

The reader audit used the rendered `index.md`, supplement, two SVG figures,
seven tables, and 16 staged research objects. No tracked `usersum/assurance` report was
created.

## Rust And Contract Tests

- `cargo fmt --all --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile assurance-editorial`: PASS, 65/65,
  final run ID `e3057481-8fe1-4444-b35b-e8662e0d74ad`.
- `cargo nextest run --test assurance_dossier_build_contract`: PASS, 13/13,
  run ID `48c5134f-a5c0-417d-94b5-0aac21576813`.

The first focused run exposed six one-report test assumptions. The accepted
remediation preserved a one-report synthetic fixture for mutation tests while
strengthening real-repository assertions for named isolation, two-report
planning/building, deterministic subset equivalence, and the zero-public
boundary. The post-remediation run above is the closure result.

No production Rust changed. The adjudicated touched-file CRAP gate is therefore
exempt under this package. The four touched integration-test files contain 798,
686, 546, and 762 lines respectively; each is below the 2,000-line warning
threshold and the 3,000-line required-refactor threshold.

## Documentation And Hygiene

- `markdown-doc lint` on the package, report source, V2 README, and roadmap:
  PASS with 20 Markdown files, zero errors, and zero warnings.
- `git diff --check`: PASS.
- Required-reading Core budget: 141,405 bytes, `OK`.

## Protected Public State

Static plus Ran identity comparison:

| Path | Expected and observed SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

`usersum/assurance` still contains only `README.md`. Public report count remains
zero. No export, snapshot, vendor, WEPPcloud, review-lock, approval, or release
record was created.

## Review Closure

The package-authorized domain-science and reproduction/publication reviews both
returned findings on their first passes. Every finding was accepted and fixed;
both narrow re-reviews returned PASS. The final conservation fix removed
duplicate headline residual fields and makes all row and summary residuals
derive from the fail-closed reconstructed-row map. See `review-disposition.md`.

These are internal coding-agent reviews. Formal human review remains
`not_started`; no scientific or publication approval is implied.

## Terminal Verification

Two package-authorized independent terminal verifiers returned PASS with no
actionable findings:

- Verifier A exactly reproduced all 188 retained values, confirmed 281/281 plan
  nodes current, checked the catalog and evidence digests, reran focused
  fail-closed lifecycle/public-boundary tests (3/3 PASS), and confirmed the
  protected inventory and hashes.
- Verifier B independently located the phase, snow, frost, and conservation
  evidence in both the source and rendered scientific report; reran exact
  reproduction, named/all validation and planning, both disposable-root checks,
  the 65-test editorial profile, and the 13-test build-contract target; and
  confirmed review closure, ASSURE-05 noninterference, and the human-authority
  boundary.

These are coding-agent terminal verifications, not human scientific review or
approval. The package disposition is `HOLD-HUMAN-APPROVAL`; the report remains
`DRAFT`, formal review remains `not_started`, and public report count remains
zero.
