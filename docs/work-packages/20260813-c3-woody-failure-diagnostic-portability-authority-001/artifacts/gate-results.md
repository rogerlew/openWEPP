# Gate Results

Evidence: `Ran`

Results are append-only.

| Command/evidence | Result |
|---|---|
| instruction discovery for package, contract, and index | PASS / Ran: root plus nearest work-package/science-contract instructions identified |
| initial V6 fixture regeneration | PASS / Ran: committed vectors matched independent regeneration |
| initial V6 verifier | PASS / Ran: definition, section, fixture, generator, copies, and guards verified |
| `check_sc_unit_compliance.sh --path SC-VEGETATION-001.md` | PASS / Ran: no findings |
| `check_authority_suite_antievasion.sh` | PASS / Ran |
| Markdown lint: package, contract, index | PASS / Ran: 13 + 1 + 1 files, zero findings |
| `git diff --check` | PASS / Ran |
| pre-review admission | EXPECTED FAIL / Ran: Version 10 remains `in_review/draft` pending dual review |
| AUTH11 required-suite guard | PASS / Ran: 3/3 |
| existing authority suite during in-review amendment | EXPECTED FAIL / Ran: 17/21 passed; four V9 lifecycle/digest assertions await V6 contract-derived test update outside this agent's bounded test write set; V5 digest-range failure also exposed and led to moving V6 before V5 |
| immutable V5 restoration | PASS / Ran: both canonical copies restored to `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`; no V5 diff remains |
| amended V6 independent regeneration | PASS / Ran: complete record evaluator exercised every positive/boundary/eligibility/firewall case |
| amended V6 authority verifier | PASS / Ran: executable poisons, exact V6 copies, section markers, and every canonical V1--V5 copy verified |
| amended `git diff --check` | PASS / Ran |
| dual independent science review | PASS / Static + Ran: Reviewer A GO; Reviewer B GO; every iterative finding accepted and closed |
| lifecycle promotion | PASS / Static: `SC-VEGETATION-001@10` and registry promoted to `approved/active` only after dual GO |
| post-review fixture regeneration | PASS / Ran: committed vectors match independent generator exactly |
| post-review V6 verifier | PASS / Ran: executable poisons, identity transition, V6 copies, V5 section `22edf681...`, and all V1--V5 definitions verified |
| post-review science-contract admission | PASS / Ran: `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256 `85210adc267bab7ca4f6693bb0684e354620acc9b04505f0cb188840bceb9576` |
| post-review SC unit compliance | PASS / Ran: no findings |
| post-review authority anti-evasion | PASS / Ran |
| post-review AUTH11 | PASS / Ran: 3/3 |
| post-review Markdown lint | PASS / Ran: package 15 files plus contract and index, zero findings |
| post-review `git diff --check` | PASS / Ran |
| first terminal verification A | FAIL / Ran: authority suite retained two Version 9 assertions; catalog entry and terminal reconciliation were incomplete |
| first terminal verification B | FAIL / Ran: confirmed focused-suite/catalog defects and found premature lifecycle promotion plus noncompliant compressed review/disposition records |
| lifecycle correction after failed verification | PASS / Static: contract and registry returned to `in_review/draft`; promotion is withheld until separate dual verification passes |
| ignored Python cache cleanup | PASS / Ran: removed only the generated package-local `artifacts/__pycache__/reference_calculator_v6.cpython-312.pyc` |
| corrected V6 regeneration and verifier with `PYTHONDONTWRITEBYTECODE=1` | PASS / Ran: committed vectors, executable poisons, exact copies, and immutable V1--V5 verified without recreating cache bytes |
| corrected SC unit, anti-evasion, and AUTH11 gates | PASS / Ran: no unit findings; anti-evasion passed; AUTH11 3/3 |
| first corrected authority-suite retry | FAIL / Ran: 22/23 effective progress; lifecycle assertion still required approved/active while canonical review lifecycle was correctly in_review/draft |
| lifecycle-coherent authority-suite retry | PASS / Ran: 23/23; test accepts only the two canonical lifecycle pairs and requires registry/contract agreement while preserving exact Version 10/V6 and V1--V5 assertions |
| corrected Rust formatting and diff hygiene | PASS / Ran: `cargo fmt --all -- --check`; `git diff --check` |
| corrected Markdown lint | PASS / Ran: package 15 files, science-contract tree 63 files, catalog 1 file; zero errors and warnings |
| first strict-Clippy terminal retry | FAIL / Ran: five intentional exact binary64 assertions used direct float equality in the touched authority test |
| exact-bit assertion correction | PASS / Ran: replaced direct float equality with `to_bits()` identity checks; no suppression or tolerance was introduced |
| verifier-identified Rust-consumer correction | PASS / Static: ordinary Rust test now independently executes the exact 21-field eligibility firewall, failure/domain/rollback guards, and rtol-only comparison over every committed numeric, nonfinite, and poison record, including `lower_side_boundary` |
| final focused Rust retry before separate verification | PASS / Ran: strict Clippy with `-D warnings`; authority suite 23/23; formatting and diff hygiene |
| separate verification A | PASS-WITH-NOTES / Ran: all substantive findings closed; terminal sequencing note only |
| separate verification B | PASS / Ran: all ten review findings closed; no rejected or undispositioned finding |
| post-verification lifecycle promotion | PASS / Static: contract/body/registry promoted from `in_review/draft` to `approved/active` only after both separate verifier verdicts |
| exact promoted-byte admission | PASS / Ran: `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256 `7759fe4819ee3741298abcddf86966ad5fa3d68837ac7cf380f614d1f7b76753` |
| exact promoted-byte focused gates | PASS / Ran: V6 regeneration/verifier, strict authority-test Clippy, authority suite 23/23, SC unit compliance, anti-evasion, AUTH11 3/3, formatting, diff hygiene |
| exact promoted-byte Markdown | PASS / Ran: package 17 files, science-contract tree 63 files, catalog 1 file; zero errors and warnings |

Current reviewed authority identities:

- generator: `bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532`;
- vectors: `2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b`;
- V6 definition and canonical copy:
  `a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426`;
- V6 verifier:
  `a71f0d149a753183d2b97d59d0609c184618f993d83e2a8c4abba87bc8671ba1`;
- V6 contract section:
  `fba3486765a3819ab44659e80f9fb1eb304ee5953cd8c41f3046b95442ef0891`.
