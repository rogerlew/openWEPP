# Coverage closure

Status: PASS
Evidence mode: Ran

The safety net was measured before any production decomposition:

| Metric | Covered / total | Percent | Required | Result |
| --- | ---: | ---: | ---: | --- |
| Lines | 964 / 992 | 97.177% | >=90% | PASS |
| Regions | 1,579 / 1,699 | 92.937% | >=90% | PASS |
| Functions | 61 / 67 | 91.045% | reviewed with named floor | PASS |

The final alias tests lifted both previously deficient named helpers. The
uncovered function symbols are closed-list format/code arms and
compiler-emitted closure shims. Their runtime variants were exercised through
stable public assertions and the enclosing closures; treating each synthetic
symbol as a separate science obligation would add no runtime-path evidence.
All source-named non-format helpers met the pre-decomposition 75% region
floor.

Terminal measurement against source SHA-256
`c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`
retained 1,020/1,048 lines (97.328%) and 1,597/1,717 regions (93.011%).
Every eligible CRAP row is at most 30; the maximum is 23.0. The only
source-named floor exception is `for_batch` at 66.667%, CC 7, CRAP 8.815.
Independent Review A accepted it as a closed-list non-science infrastructure
exclusion: all public/selectable behavior is exercised, while the remaining
arms are dependency-origin reader-build/page-read failures that require a
corrupt-Parquet implementation coupling or a test-only production seam.

Terminal raw hashes are recorded in `coverage-after.md` and `crap-after.md`.

Raw evidence hashes:

- `lcov-safety-net.info`: SHA-256
  `8e98f4136447599d06f12546fc6bae6b2c5778f1d159a82a6c76dc9712c66f65`,
  524,011 bytes.
- `coverage-safety-net.json`: SHA-256
  `8fa631f3ef5b282c5af5afef8a35991541fd9180e07d60ec894018947494adb6`,
  5,621,033 bytes.
