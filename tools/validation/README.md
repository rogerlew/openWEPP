# Work-plan Advisory Linter

`workplan-lint` is a read-only convenience tool. It inspects a declared work
package and repository state, then reports cited findings and inert command
suggestions. It never grants permission, executes a suggestion, changes
lifecycle state, or replaces direct repository requirements.

## Usage

```text
tools/validation/workplan-lint \
  --package docs/work-packages/<id>/package.md \
  --mode <pre-edit|working-tree|terminal> \
  [--format human|json]
```

Modes:

- `pre-edit` reads the declaration and its named base;
- `working-tree` also observes index, tracked worktree, and untracked paths;
- `terminal` observes the exact base-to-HEAD name/status diff plus dirty paths.

Completed analysis exits zero even when findings exist. Exit 3 means an
analysis was partial or unavailable. Exit 2 means invocation misuse. These
codes report tool availability only; they have no package or campaign meaning.

## Read-only boundary

The implementation reads bounded no-follow regular files beneath the resolved
repository. It uses only literal read-only Git argument vectors with an empty,
implementation-owned environment, closed standard input, fixed output limits,
and timeouts. Repository config or attributes that could invoke helpers,
filters, pagers, hooks, maintenance, credentials, URL rewriting, external
diffs, or text conversion cause refusal before Git starts.

The tool does not run tests, builds, formatters, workflows, suggested commands,
shells, network operations, hooks, filters, or remotes. It has no CI, daemon,
database, receipt, ledger, attestation, recovery, publication, CAL, custody, or
protected-data role.

## Manual route

If the tool is absent, wrong, partial, or unavailable:

1. run `tools/agents/find-agents --for <intended-write-paths>`;
2. read the package intent, declared write set, and canonical testing strategy;
3. inspect the exact base, index, worktree, and untracked paths;
4. select and run applicable commands directly;
5. record exact commands and evidence in the owning package; and
6. continue the originating work without opening a linter-repair prerequisite.

A real unmet governing requirement can prevent truthful closure. Linter
availability cannot create, waive, upgrade, or downgrade that requirement.

## Conservative refusal

The frozen safety contract rejects repository-local Git filters even when they
are ordinary Git LFS declarations. In such a repository, package and policy
analysis remains available but Git-backed diff analysis is reported `partial`
and exits 3. This is intentional fail-closed behavior for the thin slice, not a
work-package hold. Use the manual route; do not remove or bypass repository
filter configuration merely to satisfy the linter.
