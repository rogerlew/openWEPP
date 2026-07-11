# Coverage and CRAP after

Status: PASS
Evidence mode: Ran

Post-decomposition targeted same-test coverage (27/27 passing):

- lines: `636/653` (`97.397%`), science-tier PASS;
- regions: `659/672` (`98.065%`), science-tier PASS;
- functions: `25/25` (`100%`);
- per-function region floor: PASS; minimum is `parse_preamble` at `35/41`
  (`85.366%`), then `validate_datver_policy` at `34/36` (`94.444%`) and
  `parse_events` at `95/100` (`95%`); all others are 100%;
- eligible CRAP rows above 30: zero. Maximum is `parse_fixeddate_str=17`;
  extracted `parse_events=16.4025` and `parse_preamble=10.3134`.

The targeted denominator includes the entire production module and is the
appropriate after-equivalent because all public consumers are in this single
focused integration suite. It improves on and does not regress the full-
workspace before threshold.

Artifacts:

- LCOV: 216,801 bytes, SHA-256
  `70e15db29d76b856df9bb93446b72bf0ef97b9fc72c7a9dfb85fff769ceb21cb`;
- JSON: 1,106,349 bytes, SHA-256
  `611246f53e4a7cdec34fde183c9489d768ef408be89ac301c6e76bddaaa2543c`;
- CRAP JSON: 2,777,237 bytes, SHA-256
  `28bf37af478c2d9b784215291d306bd1256d6c03cfbd515ab34f9f5ef45d6a88`.

Exact current-source commands and results on production SHA-256
`70aa60e562f7e5d972ec53330e856122ac38ee4b8c9a0a4623834599a04a4b45`
and final focused-test SHA-256
`473d2ba682562122cf16bbc2ea6f83a43cd2a8352d47c7f72ff1a30167b3d87e`:

| Command | Exit | Elapsed |
| --- | ---: | ---: |
| `cargo llvm-cov clean --workspace` | 0 | included before timed runs |
| `cargo llvm-cov --workspace --test infile_irrigation_fixeddate_parser_contract --lcov --output-path /tmp/fdir-fq01-after.lcov` | 0 | 21.22s |
| `cargo crap --workspace --lcov /tmp/fdir-fq01-after.lcov --min 0 --format json --output /tmp/fdir-fq01-after-crap.json` | 0 | 0.93s |
| `cargo llvm-cov --workspace --test infile_irrigation_fixeddate_parser_contract --json --output-path /tmp/fdir-fq01-after.json` | 0 | 21.93s |

Eligible surface: the entire production module, with no denominator exclusion.
CRAP rows are deduplicated by logical function and source line. Defensive paths
that remain uncovered are retained and counted: line-4 absence after a
`while idx < lines.len()` admission; a missing furrow row after the later
successor record was already retrieved; the fallback `jtemp` arm after `{1,2}`
validation; strict legacy-no-datver state after strict preamble rejection; and
error propagation from furrow successor `parse_line3_record`. These are
structurally/type-impossible or defensive propagation arms, not omitted
contract cases, and no `COVERAGE-EXCLUDE` annotation or denominator reduction
is claimed.

The worktree included package evidence plus a concurrent unrelated root
`README.md` edit. The latter is excluded from this package commit and does not
affect the recorded source/test hashes.
