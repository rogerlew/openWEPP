# Independent verification B

Static: inspected corrected specification, integration diff, frozen acceptance,
handoff, primary gate evidence, both reviews and accepted-finding dispositions.
Ran: independent Python assertions, legacy lint, Markdown/scope checks, focused
Rust governance test and Git identity/history comparisons described below.

Role/session: `/root/format_verify_b`, non-forked independent command/diff verifier.
Requested effort: medium; effective runtime settings: UNOBSERVED.
Reviewed identity: `f68411f8f1193e5eb7c4c43fd34152b4399157e3` against
`d8249849d6e015070818be7caf6f8caa75485098`; corrected substantive source
`e5971d16da993b8e11a7b3b53d28e8e99a415cac`.
Write scope: this artifact only. Verifier A's artifact was not read.

## Independent checks

All commands ran in `/workdir/openWEPP`.

- Executed both `sh` fences from `artifacts/gate-results.md` using
  `.venv/bin/python` to extract them and `subprocess.run(['bash','-c',command])`.
  Exit 0 for both: three existing test functions PASS; 20 owned paths and 33
  local Markdown links/anchors PASS; `git diff d8249849d HEAD --check` PASS.
  The scanner checks this changed Markdown subset, not a universal renderer.
- `.venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py`:
  exit 1, `No module named pytest`, independently reproduced. This is not a
  pytest PASS. Inspected the complete test module: its three functions require
  only a fresh `tmp_path`, have no pytest hooks, parameterization or other
  fixtures, and invoke the unchanged checker using `sys.executable`. The direct
  execution supplies a separate temporary directory for each and executes the
  same assertions, including rejection of absent invariant and qualified OBL
  IDs. It satisfies the existing-regression acceptance without pretending the
  named runner ran. No assertion or checker source changed.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0, PASS, 16 binding exposure rows. This establishes legacy lint only.
- `nix develop --offline --command cargo nextest run --offline --test adr0017_comparator_distrust_ratification_contract governance_docs_encode_comparator_flag_adjudication_gates`:
  exit 0, one passed, three filtered; run ID
  `0b16fae1-af53-44c6-8e72-1605db7dd1b5`. Inspected that test's actual procedure
  dependency. Existing `ending_snow_hint` dead-code warning remains; no clean
  warnings claim. No full-workspace or scientific regression executed.
- `git rev-parse HEAD` confirmed the frozen identity. Worktree-to-cut and staged
  diffs were empty. `git diff --name-only e5971d16d f68411f8f` contains only six
  package assurance records, preserving the reviewed substantive correction.
  Protected `crates`, `tests`, `tools`, and canonical SC contract paths have no
  base-to-cut changes. Existing untracked `$pkg/` and `tmp/` were preserved;
  verifier A's new artifact appeared during verification and was not inspected.
- Independently compared the two `git ls-tree -r` maps for all 26,889 preexisting
  `docs/work-packages` paths other than the two authorized catalog/locator files:
  every blob identity and file mode matches. Compared scaffold `216fb0614` and
  frozen-cut `package.md` using `git show`: acceptance bytes match. Exit 0.
  An initially slower per-file Git comparison was stopped and replaced by this
  complete tree-map comparison; the stopped run is not counted as a pass.

## Findings and requirement legitimacy

Findings: none. B-01's corrected exact anchor/table grammar includes the accepted
form, same-ID condition, mandatory cells, fenced-example exclusion and rejected
forms. B-02 now names the 18 artifact and 14 kernel-profile coverage keys; checked
their source lists and example item mappings. Subordinate requirements remain
binding. Both fixes have actual focused re-review at the corrected source cut;
the freeze did not alter those fixes.

The exact diff supports substantive prospective specification governance, with
focused existing tooling/consumer validation. It changes neither executable
physics nor current authority-suite posture or protected fixtures. No Rust edits
means line-count governance is not applicable. Full-workspace science testing
and authority-suite anti-evasion runs are not triggered by this bounded diff.

Non-deferral checked explicitly: acceptance was frozen before implementation;
checker implementation, LSE migration, fresh-agent usability measurements and
reading savings were excluded prospectively and remain separately authorized
adoption work. They are not unrun current gates relabeled after failure. Candidate
claims correctly distinguish executed legacy checks from specified future
conformance cases, retain full current/frozen reading obligations and make no
parser, migration, science qualification, activation or savings claim.

Disposition: PASS for verifier B's assigned corrected-cut scope. No finding is
deferred. The candidate correctly remains NOT COMPLETE pending both independent
verifications and terminal publication reconciliation; this artifact does not
substitute for verifier A or a check of later status/catalog edits.

Uncertainty: directory conformance behavior cannot be exercised because this is
a specification and the parser is not implemented. No scientific preservation
experiment or measured context reduction is certified. Evidence reuse after this
cut is limited to unchanged inputs; later substantive changes reopen assurance.

Verdict: PASS, bounded to the frozen specification and independently reproduced
existing checks above.
