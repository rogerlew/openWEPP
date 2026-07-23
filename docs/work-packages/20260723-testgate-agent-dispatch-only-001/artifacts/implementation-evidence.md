# Implementation Evidence

Evidence classes: Static + Ran.

Static: `.github/workflows/testgate-shadow.yml` now declares only
`workflow_dispatch`. Both `base_ref` and `intent_package` are required inputs.
Both the forest1 executor and independent hosted verifier reject `base_ref`
unless it is one lowercase 40-character commit ID before invoking the generic
resolver.
Forest1 labels, hosted verification and aggregate jobs, current-main checks,
concurrency identity, gate execution, and attestation steps are unchanged.

Static: `trusted_workflow_binds_one_explicit_intent_package` parses the YAML
event map, requires `workflow_dispatch` to be its sole entry, independently
requires `base_ref` and `intent_package` without defaults, and binds both exact
base guards.

Ran:

- Ruby YAML parsing passed for `.github/workflows/testgate-shadow.yml`.
- `cargo fmt --all -- --check` passed.
- The focused workflow source-contract case passed 1/1.
- The focused policy/authority contract cases passed 2/2.
- Markdown lint passed nine scoped files with zero errors or warnings.
- The policy JSON parsed, its generation is 15, and its strategy digest is
  `64ef643ba7fef9945b54c516216d313f33d8aa3f73604315fcfd4b1bf1c3b1ce`.
- `git diff --check` passed.

Static: no TESTGATE run was dispatched and no expensive gate was executed.
