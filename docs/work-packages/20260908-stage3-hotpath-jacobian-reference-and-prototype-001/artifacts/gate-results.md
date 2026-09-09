Ran: observation-only validation, not derivative admission.

Corrected isolated A invocation: focused nextest filter
test(solver_residual_corpus_capture), 5 passed, 148 skipped. Log:
/tmp/openwepp-hotpath-jacobian-jmvOZh/capture-tests-attempt2.log.
The initial attempt failed cwd/PATH setup; its primary-cwd zero-test output is
not A evidence. Preserved capture-tests.log records exit 127 (nix unavailable
on that invocation's PATH). Corrected invocation uses absolute Nix and
login=false, target /workdir/.cache/openwepp/targets/hotpath-reference-A-20260908.

Actual target is /workdir/.cache/openwepp/targets/A-b67d5833dd54: Nix shell
overrode the requested pre-export. Authentic release capture passed 1/1 with
239 filtered; build 6m42s, test 5.14s (untimed observation, not cost evidence).
Release binary is release/deps/openwepp_runner-682944f41aa5eb67 beneath that
target, not the debug LSE binary mistakenly described in runner handoff.
Independent capture review found two medium capture-integrity defects;
cut2 corrections frozen, executable verification pending. All derivative, full-regression,
terminal review and verification gates remain NOT RUN.

Ran: checkpoint observation-cut2.patch applies cleanly with git apply --check
against retained predecessor J. Staged documentation whitespace check passes
excluding the patch artifact. Whole staged check reports only the patch's
required single-space blank context lines; these are retained as valid unified
diff syntax. No compilation or tests were rerun for this documentation/evidence
checkpoint. No production Rust paths are staged.
