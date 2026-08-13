# Gate Results

Evidence is append-only. Failed attempts and retries remain visible.

## 2026-08-13 Oracle Freeze Attempt

Ran: the first strengthened calculator run rejected nonzero binary64 closure
roundoff in an intermediate onset interval. The oracle was corrected to apply
the inherited `1e-12` closure threshold while retaining explicit operands.

## 2026-08-13 Oracle Freeze

Ran:

```text
.venv/bin/python artifacts/reference_calculator_v7.py --verify
PASS openwepp_c3_woody_v7_vectors.json d99288741f3cac16f017ffe5cd11620bfde2055e32f18b82e538eaf6d48ef411
```

## 2026-08-13 Focused Pre-Review Gates

Ran:

- `verify_v7_authority.py` -> PASS.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
  -> 25 passed.
- `check_sc_unit_compliance.sh` -> PASS.
- `check_authority_suite_antievasion.sh` -> PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  -> 3 passed.
- `markdown-doc lint --path <package>` -> 33 files, 0 errors/warnings.
- `cargo fmt --all -- --check` -> PASS after applying canonical formatting.

Expected lifecycle failure retained:

```text
check_science_contract_admission.sh --base-ref 79bf36a... --worktree
ERROR: changed science contract is not approved/active
```

This is the required pre-review posture, not an authority defect. Admission is
rerun after deliberate terminal promotion.

## 2026-08-13 First Independent Reviews

Ran: science review NO-GO and closure review FAIL. All findings were accepted:
an unbound closure threshold, incomplete/tautological poisons and rollback,
incomplete migration evidence, evadable test assertions, proportional C/N,
unbound retained reference bytes, and no direct terminal branch.

## 2026-08-13 Review Remediation Gates

Ran after correction:

- V7 verifier and byte regeneration -> PASS.
- vegetation authority contract -> 25/25 PASS.
- SC unit compliance -> PASS.
- `cargo fmt --all -- --check` -> PASS.
- `git diff --check` -> PASS.

## 2026-08-13 Final Independent Re-Reviews

Ran:

- independent science review -> GO, no material findings;
- independent contract/closure review -> PASS, no material findings.

Both reviewers independently reran the V7 verifier, 25-test authority suite,
unit lint, anti-evasion, and applicable hygiene gates on corrected bytes.

## 2026-08-13 Heavy Gate First Attempt

Ran by the comparator suite runner:

- workspace Clippy -> FAIL on two pre-existing
  `clippy::assigning_clones` warnings in the adjacent C3 implementation test;
- full workspace nextest -> infrastructure-interrupted at about 240 seconds,
  with 195 passes, 13 still running, and 2,411 not started; there were no
  assertion failures;
- doctests -> PASS;
- cargo-deny -> PASS;
- format and diff hygiene -> PASS;
- focused V7 verifier/authority/anti-evasion/AUTH11/Markdown -> PASS;
- SC unit command without its required `--path` -> invalid whole-registry
  invocation and 148 unrelated findings.

The two lint findings were corrected mechanically with `clone_from` and no
semantic change. The comparator rerun uses deterministic full-profile
partitions to avoid repeating the external execution-limit interruption and
uses the exact path-scoped SC unit command.

The first warnings-denied retry then exposed
`clippy::too_many_lines (161/100)` in the package-created V7 vector test.
That failure remains in
`artifacts/terminal-runs/retry/cargo-clippy-warnings-denied.log`. The test was
factored into bounded migration, poison, and rollback assertion helpers without
weakening or deleting an assertion; focused and workspace Clippy subsequently
passed.

## 2026-08-13 Heavy Gate Final Results

Ran with `TMPDIR` under `/home/workdir`, outside the repository:

- workspace warnings-denied Clippy -> PASS;
- full workspace nextest, 16 deterministic hash partitions -> 2,619/2,619
  PASS in aggregate;
- doctests -> PASS;
- cargo-deny -> PASS;
- format and diff hygiene -> PASS;
- path-scoped SC unit lint -> PASS.

Failed infrastructure attempts remain in `artifacts/terminal-runs/`: the
original external SIGINT, root-filesystem exhaustion, concurrent duplicate
partition collisions, and partition-13 contention timeout. Each affected
partition subsequently passed in an isolated retained run. No deterministic
assertion failure was rerun into a pass.

## 2026-08-13 Post-Promotion Gates

Ran after deliberate `approved/active` promotion:

- V7 verifier -> PASS;
- science-contract admission against `79bf36a7b...` -> `A0_ADMITTED`;
- SC unit compliance -> PASS;
- vegetation authority suite -> 25/25 PASS;
- format and diff hygiene -> PASS.

## 2026-08-13 Terminal Verification

Ran:

- terminal verifier A -> PASS after two lifecycle-history documentation
  corrections; no unresolved material finding;
- terminal verifier B -> PASS; no material finding.

Both independently reran the V7 verifier, authority suite, admission, unit,
anti-evasion, AUTH11, Markdown, format, and diff gates.
