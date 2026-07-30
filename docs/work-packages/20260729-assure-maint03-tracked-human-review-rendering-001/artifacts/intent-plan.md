# Intent And Gate Plan

Status: frozen before implementation

Evidence class: Static + Ran

Implementation base:
`e8b46b1d67b956adc09cb74230d61815976b2de9`.

Current assurance generation:
`30db4a7e6a691601426428b7772e28143ff9fa1bf10dd9d1ae80062d7f0002a2`,
with 22 anchored transitions, three DRAFT sources, and zero public reports.

Protected baseline:

| Path | SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

The disposable full-catalog build contains 91 files with path-inventory SHA-256
`9c2dc4858f0628e0067ceb49a626786fe5a7ed09c7583bb93819a1ae7527990a`.
The canopy report and supplement contain 13 known generated count-noun
duplications (`members members`, `runs runs`, or `members-member`).

## Direct Gate Selection

Classification escalated to Critical when the typed adoption dry run proved
that manifest selection could not adopt the report-owned manuscript/supplement
source set required for readable review output. Selected:

- script self-test and focused tracked-review contract test;
- typed multi-internal-source CAL-09 adoption, negative unrelated-drift,
  check/apply/no-op, rollback, and receipt tests;
- assurance validate/plan/build/check and anchored generation verification;
- fresh unrelated-build inventory and byte equality;
- resolved-directive, duplicate-phrase, Markdown-consumer, link, and SVG
  accessibility checks;
- Python syntax/lint where repository tooling exists;
- Rust formatting, strict workspace Clippy, assurance crate/focused integration
  tests, and full-workspace all-feature Nextest;
- documentation lint and `git diff --check`;
- strict public/protected-path comparison;
- two independent implementation reviews and two terminal verifications.

No kernel, comparator, conservation, empirical, or external-authority gate is
selected because no such surface changes. Full-workspace correctness is
required immediately because the package changes a trust-root transaction.
