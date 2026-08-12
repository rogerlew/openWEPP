# Independent Science Review B: Final Source/Lifecycle Exact-Byte Recheck

Status: `PASS / no material findings`

Evidence mode: `Static + Ran`

## Exact reviewed identity

- Reviewer scope: corrected de Pury/Farquhar source identity and locator;
  approved/active lifecycle; all C/N, phenology, turnover, mineral-N,
  litter/CWD, strict-schema, digest, readiness, and executable Review-B
  surfaces.
- Base commit: `669aafb60df3ac4eeed2661cc4db4ad33f3f2265`.
- Worktree-status digest: `aedf00efd1a46e8d9f2901f6cbde2a070ba7f004f0171beabb0cf0d63f624941`.
- Aggregate SHA-256 of the sorted SHA-256 manifest over the package excluding
  this artifact, plus both canonical contracts, the focused test, bibliography,
  and dated rights record:
  `2e0bc8874c791c0384f9e507018a3247359ab57268f066bc08c70c58efd58bca`.

| Surface | SHA-256 |
|---|---|
| `SC-VEGETATION-001.md` | `2c8cc8322ce8c4404e212f3a12f7f2aea7547ec8f23ab3eafc5e53f7672127e7` |
| `SC-BIOGEOCHEM-001.md` | `6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4` |
| `references/annotated_bibliography.md` | `587b629d995b3c9312f4adcb10933b91104b158f80ceb7c9e173f829f882f959` |
| `reference-acquisition-ledger.md` | `b1deb70b77c8312d70e26b9bae09a8f6001105d61a2b784a500c3bb1db7f1508` |
| `reference-rights-and-checksum-disposition.md` | `9a28c5568fcc53eae6ceb2cdbdf6650fbb1580e7d1802b5053b939a778e32df7` |
| de Pury/Farquhar reviewed PDF | `8a847133cf3d546bccd3e2dc076fa3b1e5e6f71edf2dd2efcc32282f3fc41fc6` |
| `equation-authority-ledger.md` | `5ec5a24893c8d2b0516e764de2164ff027dbd331cafe885c01e27fe48e9bddb2` |
| `parameter-and-configuration-manifest.md` | `5ed13860c21dce30f4f0b09393bb70ea6f18304b3210a957f4925d3ef1c64e99` |
| `state-ownership-and-transaction-ledger.md` | `f7e0c47eb7cdc17d20a1c9e9f22c46c06584390f5b422dce02534881b7fe19e1` |
| `reference_calculator.py` | `ac8caf95e2b8bccadc528e168d0e466504bca88c15e86b7bfba89438f4ec13e8` |
| model-definition JSON | `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157` |
| focused Rust contract test | `7aeb2201bec9fdf078b114ceb569ed6fad7b3d8f9d03c76c1fd21647dcb658b3` |

The local PDF is restricted and gitignored. Its hash records exact reviewed
bytes without implying redistribution permission. No clean-worktree claim is
made.

## Ran evidence

- Recomputed SHA-256 of
  `references/copyrighted/DePuryFarquhar1997_SunShade.pdf`:
  `8a847133cf3d546bccd3e2dc076fa3b1e5e6f71edf2dd2efcc32282f3fc41fc6`.
- `pdfinfo` reported 21 pages, unencrypted; `pdftotext` inspection identified
  the journal title, authors, volume 20, pages 537--557, Table 1, and Appendix 1
  sunlit/shaded/direct/diffuse material.
- `.venv/bin/python .../artifacts/reference_calculator.py`: exit `0`,
  `"all_pass": true`; output SHA-256
  `bd180d63e8d4e3ccae78fbeec308ddd27024db9c466e1d8eef47656d6df0f368`,
  unchanged from the preceding science PASS.
- `cargo nextest run --test vegetation_boundary_authority_contract`: nextest
  run `7e2d7c97-d8ca-4bd7-8ee2-92c9bef9d28a`; 12 passed, 0 skipped.
- Targeted `rg`, `sed`, `jq`, `pdfinfo`, `pdftotext`, `sha256sum`, `git status`,
  and `git rev-parse` inspection of source, rights, contract, digest, schema,
  oracle, and test surfaces.

## Source identity and locator determination

- The canonical DOI is consistently
  `10.1111/j.1365-3040.1997.00094.x` in the bibliography, current contract, and
  rights metadata. No stale `...1997.0094...` occurrence was found in the
  reviewed repository text.
- Bibliography record `R-149` identifies de Pury and Farquhar (1997), “Simple
  scaling of photosynthesis from leaves to canopies without the errors of
  big-leaf models,” *Plant, Cell & Environment* 20:537--557, the CSU SiB mirror
  acquisition route, access date, 21-page local PDF, exact hash, quality,
  restricted rights, and local path.
- Supporting locators are adequate and match the reviewed bytes: journal pages
  539--543; Table 1 equations (1)--(14); Table 2 units; and Appendix 1 Tables
  A1--A2 for direct/diffuse/scattered PAR and sunlit/shaded integration.
- `REF-VEGETATION-025` binds the corrected DOI, exact reviewed-byte hash, pages
  538--543, `PRIMARY_PROCESS_AUTHORITY`, and the selected sunlit/shaded scaling
  role. `E02` records the corresponding exact class-LAI equations and page
  locators.
- Current package rights disposition correctly retains the PDF under
  gitignored `references/copyrighted/` as restricted Wiley material. The dated
  May first-pass record is preliminary history; the current acquisition and
  rights ledgers govern the acquired local-byte disposition.

## Lifecycle, digest, and Review-B non-regression

- Both science contracts remain consistently `approved` / `active` in
  frontmatter and body status.
- The BGC whole-file hash remains correctly embedded in the model-definition
  JSON and checked by Rust. The JSON digest remains correctly bound in
  `SC-VEGETATION-001` and the focused test. The DOI correction changed no
  admitted equation, numerical policy, field schema, or oracle-science byte.
- Potential/final N demand, internal-N priority, layer/species requests,
  proportional competitor arbitration, finalized-use debit, tissue C:N credit,
  NSC carry, and exact receiver ownership remain deterministic and unchanged.
- Persistent C/N and phenology state, six-tissue display/storage/transfer
  fields, mineral NH4/NO3, litter/CWD C/N/dry-material receivers, exact turnover
  equations, poison vectors, independent closure, and atomic rollback remain
  present and passing.
- Site parameters and complete compatible initial state remain caller supplied.
  No hidden default, immutable-N shortcut, temporary nutrient source,
  diagnostic-only photosynthesis endpoint, or calibration/validation claim was
  introduced.
- Readiness remains separated from admission: production implementation,
  calibration readiness, and identifiability are not claimed.

## Prior-finding closure

`REVIEW-B2-001` through `REVIEW-B2-006`, `REVIEW-B3-001` through
`REVIEW-B3-003`, and `REVIEW-B4-001` through `REVIEW-B4-002` remain closed on
these exact source/lifecycle bytes. No finding was reopened.

## Final recommendation

**PASS the final Review-B science/source/lifecycle gate and retain
implementation-authority release for the assigned C/N/biogeochemistry scope on
the exact bytes identified above.** No material finding remains. This is
contract-first implementation authority only; it is not production
implementation, calibration readiness, empirical validation, activation, or
deployment evidence.
