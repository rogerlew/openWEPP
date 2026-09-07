# Independent verification B

Static: root/common/role-verification instructions, frozen package/handoff,
primary utility/tests, accepted review findings and their corrections, exact
base-to-candidate diff and identity membership. Ran: own checks below.
Role/session: /root/verify_b, independent non-forked verifier B.
Requested effort: medium; effective runtime effort: UNOBSERVED.
Assigned candidate: 74e2911937f72f4123e77cc2f2645378af9f91e5;
reviewed substantive predecessor: 772eade86; base:
033dfe30073bc22aaa30aed877cc301b6745a086. During verification the executor
committed VA-01 correction 4ea8a5518179f7d56aacdf4f990145f16d8a0046;
bounded correction checked below. Only this assigned repository artifact edited.

## Commands and direct results

All repository commands ran from /workdir/openWEPP. No network, full-workspace
test command, real Stage-3 run, production edit, branch switch or push.

- `.venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py' -v`:
  23 PASS, exit 0. Repeated after VA-01: 23 PASS, exit 0, 0.683 seconds;
  retained `/tmp/openwepp-verifier-b-9fWKjk/administrative-tests.log`.
- `nix develop --offline --command cargo nextest run --offline --test
  adr0017_comparator_distrust_ratification_contract --test
  advisory_linter_authority_contract`: exit 0; 11 passed, 0 skipped, two
  complete binaries, 0.036-second run. Nextest ID
  173a94ae-ecc3-4216-9e0b-8a3ce0fb9d9b. Existing `ending_snow_hint` dead-code
  warning observed; this is not a Clippy result. Own execution, not copied
  executor evidence. Exact command/output retained in this session transcript.
- `.venv/bin/python /tmp/openwepp-verifier-b-9fWKjk/verify_b.py`: exit 0.
  Independently authored miniature/CLI harness, successful capture, verify,
  restore and executable run; 19 separately exercised negative cases each
  returned expected CLI exit 2. Every subprocess argv, cwd, exit and output is
  retained in `/tmp/openwepp-verifier-b-9fWKjk/results.json`.
- Independent Python exact-diff/membership audit: exit 0; candidate changes
  60 files, 35 outside this package, zero outside the declared owned set.
  The other 25 paths are the package's enumerated artifacts/prompts/package.
  AST parsing: all three administrative Python files PASS.
- `git diff --check 033dfe30073bc22aaa30aed877cc301b6745a086
  74e291193` and the same command against HEAD after VA-01: exit 0.

## Independent standalone recovery

The new fixture did not use CaptureTests' construction helper. It contains
binary-distinct index/worktree source, a working mode 0640, untracked mode
0600, executable mode 0751 versus staged mode 100644, a staged symlink,
tracked deletion and rename, input, protocol and expected result.
Capture and restore were invoked through the public CLI. The exact disposable
original `/tmp/openwepp-verifier-b-9fWKjk/original` and its Git objects were
deleted before restoration; that generated fixture cannot be recovered as its
original Git history. Its selected bytes remain recoverable from the bundle.

Independent assertions verify worktree bytes `worktree-B\x00\xfe\n` versus
index bytes `index-B\x00\xff\n`, all modes above, symlink target and staged
120000 mode, missing deleted/old names, preserved renamed/untracked bytes and
their expected index membership. The restored executable was directly executed:
`/tmp/openwepp-verifier-b-9fWKjk/restored/executable/bin/run
/tmp/openwepp-verifier-b-9fWKjk/restored/fixture/input/data`.
Exit 0; exact stdout `verifier-B: fixture-B-20260907\n` matches both an
independent literal and the restored result bytes.

Retained bundle: `/tmp/openwepp-verifier-b-9fWKjk/bundle`.
Detached manifest SHA-256:
`1c19d79daf224167902a623403d8b9c36baac9090753398f2d5742059a116687`.
Restoration, harness, selection, logs and negative copies remain beside it.
Retention owner: repository owner, through disposition plus explicit owner
audit release; no cleanup of this external evidence is scheduled.

Negative probes covered existing capture/restore destinations, source overlap,
noncanonical `..` overlap, missing fixture, unsafe working and distinct staged
composed symlinks, destination symlink parent, restoration inside a checkout,
six malformed archive-member paths, corrupt/missing/symlink blobs and manifest
corruption. Path probes recomputed detached manifest hashes, so rejection came
from path validation rather than a stale digest. Failure excerpts include
`parent-traversing symlink`, `noncanonical destination path`,
`unsafe archive path: .git/config`, `blob hash/size mismatch` and
`blob inventory mismatch`. Unsafe restores created no destination; the retained
positive bundle independently verified again after all probes.

## Exact identity, findings and legitimacy

Actual administrative utility/reporter/test and Rust-test bytes matched the
assigned 74e291193 candidate throughout the subsequent correction. The only
Rust diff is the stated test count 22 -> 27 plus formatter-equivalent layout.
Independent JSON comparison confirms every impact-map field except the approved
live policy digest is identical to the package base; no fixture, cohort or
atomic-binding change is hidden in that repair. No production Rust, Cargo
manifest/lockfile or canonical science contract appears in the exact write set.
The selected full 11-test run verifies A-01's corrected behavior without waiver.
Own working/staged composed-link and destination probes verify B1/B2 fixes.
Identity mutation and distinct-revision tests independently ran; their result
is byte-membership detection, not automatic semantic-reuse classification.

VA-01's three-file committed diff was inspected: one README.md write-set line
added to the CQR template, accepted-finding prose and corresponding hashes.
The template already requires catalog updates; the line reconciles scope and
preserves that obligation. No tool/test implementation changed. The complete
23-test suite passes after this correction. Reviewer focused re-review remains
their independent obligation, not substituted by this check.

Regenerated experiment and evidence_claim identities exactly match 4ea8a5518.
Publication mismatch is expected while reviewers/verifiers are editing their
assigned records; final publication collection must be rebound. Existing
untracked `$pkg/` and `tmp/` remain untouched. No mismatch was silently treated
as prior signoff over future publication bytes.

Non-deferral explicitly checked: all selected executable checks in this scope
PASS, accepted behavior fixes have actual re-execution, and no inherited failed
attempt is relabeled PASS. Final package completion still requires the other
independent roles, finding disposition and final publication-diff reconciliation.
No new finding or current-scope gate waiver is proposed by verifier B.

Limitations: synthetic byte custody/execution only; not scientific calibration,
rebuildability, bit-reproducible rebuilding, original history reconstruction,
remote authenticity, adversarial atomic snapshots or full Rust binary custody.
Context completeness is verifier A's assigned assurance; B did not independently
redo its full source-reading graph. Runtime effort and workflow-total remain
UNOBSERVED. Current hashes do not grant semantic evidence reuse.

Verdict: PASS for verifier B's assigned executable, recovery, safety and exact
diff scope through the bounded VA-01 correction. Final publication-diff
confirmation remains available; this record alone is not package closure.
