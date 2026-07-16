# ASSURE-06 Human Review-Entry Gate Evidence

Evidence class: Ran unless labeled Static.

## Lifecycle And Identity

- Named and all-source V2 validation: PASS. The catalog contains one
  groundwater `DRAFT` and one snow/frost `IN_REVIEW` source; public report count
  is zero.
- Named planning: PASS with all dependency identities current.
- Exact review-root calculation: PASS in two unrelated checked staging roots;
  the declared subject and finding-ledger roots match both calculations.
- Approval and release roots: correctly absent.
- Normalization lock: PASS. The lifecycle command rejected normalization of the
  `IN_REVIEW` source. Direct `uk2us` comparisons found no American-English
  drift in the changed manuscript, supplement, package, attestation, or roadmap.

## Scientific And Consumer Stability

- Exact deterministic result reproduction: PASS, 188/188 values equal.
- Two unrelated narrative-seeded builds and checks: PASS; complete staging
  trees were byte-identical.
- Authored-content changes are limited to lifecycle and accountability
  disclosures in the manuscript front matter, release-transfer limitation, and
  About This Report section; the supplement version, human-boundary section,
  and revision log; and corresponding agent/descriptor records. Claims,
  methods, results, values, tables, figures, datasets, and research procedures
  are unchanged.

## Focused Contracts

- First `assurance-editorial` run: 64/65 PASS. One source-contract assertion
  still required both real production reports to be `DRAFT`.
- Accepted remediation: update that assertion to require the real mixed
  lifecycle and rename the schema test descriptively. No production code
  changed.
- Final `cargo nextest run --workspace --profile assurance-editorial`: 65/65
  PASS, run `2f39e382-5b12-4991-a133-85c27ee770d7`.
- First full publication-contract run: FAIL. Its synthetic publication fixture
  copied the complete real catalog, so the newly `IN_REVIEW` snow/frost source
  correctly blocked the fixture's unrelated synthetic publish-all operation.
- Accepted remediation: retain only the intended groundwater source before the
  fixture clones its synthetic second report, matching the existing assembly
  fixture boundary.
- Final `cargo nextest run --test assurance_v2_publication_contract`: 25/25
  PASS, 16 slow, zero skipped, run
  `32c6b980-4c2b-4d91-b038-8eb9d8cc0266` in 374.877 seconds.
- Final `cargo nextest run --test assurance_dossier_build_contract`: 13/13
  PASS, run `0ee8150c-2a91-4ce8-ac50-64c52fb943f6`.
- First formatting check: FAIL on mechanical wrapping of the new fixture helper.
  `cargo fmt --all` applied only that formatting.
- Final `cargo fmt --all --check`: PASS.
- Final `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- Post-format `cargo nextest run --test assurance_v2_source_contract`: 12/12
  PASS, run `0cf8965a-40e9-45cd-9293-14891725eb20`.

No production Rust changed. The adjudicated CRAP gate is exempt. The touched
source-contract test contains 762 lines. The touched publication-contract test
contains 2,123 lines, triggering the required 2,000-line `WARN`; it is below the
3,000-line mandatory-refactor threshold. The package adds one bounded fixture
helper and does not grow production code.

## Documentation And Protected Boundaries

- Pre-terminal `markdown-doc lint`: PASS, 17 scoped files, zero errors and
  warnings. A final scoped closure lint is recorded below.
- Final package schema validation: PASS, nine files, zero errors.
- `git diff --check`: PASS.
- ASSURE-05 report and package paths: unchanged from frozen Git base.
- `usersum/assurance` inventory: only `README.md`.
- Protected hashes remain:
  - public README/template:
    `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70`;
  - V1 catalog:
    `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f`;
  - generated usersum manifest:
    `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb`.

No public report, approval lock, release transfer, export, vendoring, or
WEPPcloud action was created.

## Independent Review And Terminal Verification

- Independent coding-agent review A: PASS after accepted lifecycle and
  accountability wording corrections; no remaining findings.
- Independent coding-agent review B: PASS after accepted supplement,
  write-set, and evidence-scope corrections; no remaining findings.
- Finding disposition: all six findings accepted and closed; none deferred or
  waived.
- Independent coding-agent terminal verification A: PASS with exact 188-value
  reconstruction, byte-identical dual staging trees, exact roots, protected
  boundaries, proportional tests, and documentation gates.
- Independent coding-agent terminal verification B: PASS with coherent roles
  and lifecycle, exact reproduction and roots, focused source/publication
  contracts, protected boundaries, and truthful line-count governance.
- Final scoped closure `markdown-doc lint` and package schema validation: PASS,
  zero errors and warnings.

These coding-agent reviews and verifications do not replace the required
independent human scientific, reproduction/publication, or assurance-steward
approvals. The correct package disposition is
`HOLD-INDEPENDENT-HUMAN-REVIEW`.
