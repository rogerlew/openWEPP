# Research Basis

Evidence class: `Static`

The authority uses primary project documentation and original research rather
than secondary CI advice.

## Large-project execution models

- [Rust compiler CI](https://rustc-dev-guide.rust-lang.org/tests/ci.html): a
  small subset runs on pull-request updates; the full suite runs in the merge
  queue; low-risk changes may be batched in rollups.
- [Rust compiler test execution](https://rustc-dev-guide.rust-lang.org/tests/running.html):
  local full-suite execution is described as almost never appropriate; focused
  suites are the normal development loop.
- [Chromium commit queue](https://chromium.googlesource.com/chromium/src/+/master/docs/infra/cq.md):
  affected suites run pre-submit, while tests that are too slow or expensive may
  run post-submit; additional trybots provide explicit escalation.
- [Firefox CI and Taskgraph](https://firefox-source-docs.mozilla.org/taskcluster/index.html):
  a large task graph supports selected try pushes and optimization that removes
  work already performed.
- [Kubernetes Prow jobs](https://docs.prow.k8s.io/docs/jobs/): presubmit,
  post-submit, and periodic jobs are distinct; changed-path conditions and
  explicit triggers select work.

## Selection and dependency mechanics

- [Cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html):
  supplies machine-readable workspace membership and resolved dependencies.
- [Nextest filtersets](https://nexte.st/docs/filtersets/): package and reverse-
  dependency predicates can execute affected Rust tests.
- [Nextest partitioning](https://nexte.st/docs/ci-features/partitioning/): full
  runs can be sharded when a single executor is too slow.
- [Nextest coverage integration](https://nexte.st/docs/integrations/test-coverage/):
  `cargo-llvm-cov` can execute Nextest while collecting coverage, supporting a
  follow-up prototype that removes duplicate workspace test passes.
- [Snakemake rerun triggers](https://snakemake.readthedocs.io/en/stable/executing/cli.html):
  scientific workflows determine currency from code, inputs, parameters,
  environment, and modification state rather than a blanket rerun rule.

## Predictive selection boundary

- [Predictive Test Selection](https://arxiv.org/abs/1810.05286) reports a
  production reduction in test cost while retaining a high faulty-change
  detection rate.
- [Assessing Transition-based Test Selection Algorithms at Google](https://research.google/pubs/assessing-transition-based-test-selection-algorithms-at-google/)
  shows that apparently intuitive history-based selection does not reliably
  solve the problem and emphasizes careful empirical evaluation.

openWEPP therefore adopts deterministic dependency and risk rules first.
Learned selection remains a possible future optimization only after the
deterministic planner produces a trustworthy outcome history and a separate
decision defines acceptable miss risk.

## Deterministic state and evidence trust

- [RFC 7493](https://www.rfc-editor.org/rfc/rfc7493) supplies the interoperable
  I-JSON constraints used before canonicalization, and
  [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785) supplies deterministic JSON
  canonicalization.
- [Git status](https://git-scm.com/docs/git-status),
  [Git diff](https://git-scm.com/docs/git-diff), and the
  [Git index format](https://git-scm.com/docs/index-format) define the tracked,
  index, worktree, raw-diff, rename-control, and index identities used by the
  canonical change-set contract.
- The Cargo references for
  [metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html),
  [resolution](https://doc.rust-lang.org/cargo/reference/resolver.html),
  [features](https://doc.rust-lang.org/cargo/reference/features.html), and
  [configuration](https://doc.rust-lang.org/cargo/reference/config.html) support
  explicit resolver, target, feature, dependency-kind, lock/offline, and
  configuration binding.
- [SLSA v1.2 provenance](https://slsa.dev/spec/v1.2/provenance) distinguishes
  verifiable production provenance from content hashes alone. GitHub's
  [artifact-attestation guidance](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations)
  provides the initial protected-CI identity and offline-verification mechanism
  selected by the authority.
- [Git atomic push](https://git-scm.com/docs/git-push#Documentation/git-push.txt---atomic)
  and [reference transactions](https://git-scm.com/docs/git-update-ref) support
  all-or-none remote publication and exact-old-value compare-and-swap.
  GitHub's documentation for
  [creating branch/tag rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)
  and [restricting updates/deletions](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)
  supports the selected evidence branch/tag namespaces and dedicated GitHub App
  bypass authority.

## Repository-specific basis

- `docs/specifications/correctness-authority-model.md` keeps scientific
  authority distinct from execution frequency.
- ADR-0021 preserves coverage, obligation, CRAP, and exception quality.
- `docs/standards/local-ci-gate-selection.md` supplies the existing proportional
  intent and timing instrumentation.
- `docs/governance/scientific-assurance-v2-architecture.md` already assigns
  non-Cargo dependency planning to the assurance builder rather than Nextest.
- `docs/governance/scientific-assurance-dossier-lifecycle.md` already separates
  assessed report identity from release transfer, providing the basis for
  campaign-head impact status.
