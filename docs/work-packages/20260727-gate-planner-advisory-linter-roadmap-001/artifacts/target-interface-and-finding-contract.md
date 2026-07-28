# Target Interface And Finding Contract

Status: `FROZEN FOR IMPLEMENTATION`

## Command

```text
tools/validation/workplan-lint \
  --package docs/work-packages/<id>/package.md \
  --mode <pre-edit|working-tree|terminal> \
  [--format human|json]
```

There is no default package, base, mode, or repository identity. Ambiguous
identity is reported, not inferred. Human output is default; JSON is written to
standard output only.

## Observation Modes

- `pre-edit` reads the package declaration and named base before implementation.
- `working-tree` includes index, tracked worktree, and untracked paths.
- `terminal` includes the exact declared base-to-HEAD diff plus all dirty paths.

Every result reports mode, repository root, package identity, base, HEAD,
included index/worktree/untracked scopes, policy identity, and completeness.

## Read-Only Allowlist

Direct filesystem reads are limited to normalized paths beneath the resolved
repository root, opened no-follow as bounded-size regular files. Symlinks,
special files, path escape, and size-limit overflow make that analysis
`unavailable`. The only allowed subprocess argv shapes are:

```text
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null rev-parse --show-toplevel
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null rev-parse --verify --end-of-options <validated-revision>^{commit}
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null merge-base --is-ancestor <40-hex-base> <40-hex-head>
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null status --porcelain=v2 -z --untracked-files=all
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null diff --no-ext-diff --no-textconv --name-status -z -- <paths...>
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null diff --cached --no-ext-diff --no-textconv --name-status -z -- <paths...>
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null diff --no-ext-diff --no-textconv --name-status -z <40-hex-base> <40-hex-head> -- <paths...>
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null ls-files -z --cached --others --exclude-standard -- <paths...>
git --no-pager -c core.fsmonitor=false -c maintenance.auto=false -c core.hooksPath=/dev/null cat-file -e <40-hex-object>
```

For every subprocess, `PATH` is an implementation-owned constant resolved at
installation; the binary must be the expected regular file. The child
environment is cleared, then receives only the implementation-owned `PATH`,
`LC_ALL=C`, `LANG=C`, `GIT_CONFIG_NOSYSTEM=1`,
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_COUNT=0`,
`GIT_TERMINAL_PROMPT=0`, `GIT_OPTIONAL_LOCKS=0`, `GIT_PAGER=cat`, `PAGER=cat`,
and no other variables. Standard input is closed. Standard output/error are
captured with fixed byte limits and timeouts.

`<validated-revision>` must match a conservative revision-token grammar, cannot
begin with `-`, and is resolved immediately to a 40-hex commit. Thereafter only
40-hex identities are used. `<paths...>` are normalized repository-relative
paths, cannot begin with `-`, cannot contain NUL, and occur only after `--`.
Package content may supply only these validated operands. It can never select a
program, subcommand, flag, config, environment value, pager, helper, diff
driver, text converter, wrapper, URL, or registry.

No other Git command or flag is allowed. Cargo manifests and the lockfile are
parsed in process through direct filesystem reads; Cargo is not launched. The
implementation may narrow this list but may not expand it. Shells, network,
tests, builds, formatters,
workflows, remote operations, suggested commands, hooks, filters, and
state-writing commands are prohibited.

Before any Git invocation, the implementation reads repository-local config and
applicable `.gitattributes` files in process without resolving includes. If it
finds an include/includeIf, `core.fsmonitor`, `core.hooksPath`, maintenance
command, alias, pager, credential helper, URL rewrite, external diff,
text-conversion driver, or clean/smudge/process filter declaration, the Git
analysis is `unavailable` and no Git process starts. This is deliberately
broader than the known read-command execution surfaces.

Order 3 must include adversarial repositories with every prohibited config and
attribute class above plus monitored index and object metadata. Tests must
prove preflight refusal, no helper process execution, no network attempt, and
no Git/index/object byte or metadata change for every allowlisted invocation.

## Analysis Result

Top-level `analysis_status` is `complete`, `partial`, or `unavailable`.

| Condition | Status | Exit |
| --- | --- | --- |
| All requested analyses completed, with or without findings | `complete` | `0` |
| At least one analysis completed and at least one was unavailable | `partial` | `3` |
| No requested analysis completed because of an internal/tool availability failure | `unavailable` | `3` |
| Invocation or argument misuse | no analysis status | `2` |

Exit codes are availability signals only and have no lifecycle meaning. In
human mode, results go to standard output and errors to standard error. In JSON
mode, every recognized invocation emits exactly one result envelope on standard
output, including `partial` and `unavailable`. If `--format json` is recognized
before misuse, misuse emits the error envelope below; otherwise it emits a
human error on standard error.

## Result Schema

The JSON root is an object with these required fields:

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | string | exactly `1.0.0` |
| `analysis_status` | string or null | enum above; null only for misuse |
| `mode` | string or null | `pre-edit`, `working-tree`, `terminal`; null only before valid parsing |
| `repository_root` | string or null | normalized absolute path |
| `package` | object or null | required `id` and repository-relative `path` strings |
| `base_sha` | string or null | 40 lowercase hex; null only when unavailable |
| `head_sha` | string or null | 40 lowercase hex; null only when unavailable |
| `observed_scope` | object or null | required booleans `index`, `worktree`, `untracked` |
| `policy_inputs` | array | objects with required `path` and lowercase-hex `sha256` |
| `unavailable_analyses` | array | objects with required `analysis_id`, `reason_code`, and `message` strings |
| `findings` | array | finding objects below |
| `error` | object or null | required `code` and `message` strings when non-null |

Unknown fields are rejected for schema version `1.0.0`. `complete` requires an
empty `unavailable_analyses` array and null `error`. `partial` requires at least
one unavailable analysis. `unavailable` requires at least one unavailable
analysis and may carry a top-level error. Misuse sets `analysis_status` to null,
uses empty arrays, and requires `error`.

## Finding Schema

Each finding requires string fields `rule_id`, `category`, `confidence`,
`impact`, `action`, `message`, `governing_source`, `applicability`, and
`reasoning`. `observed_location` is either null or an object containing required
repository-relative `path`, positive integer `line`, and optional positive
integer `column`. `suggested_command` is either null or an object containing
required `argv` (nonempty array of strings), `working_directory`
(repository-relative string), `affected_surface` (string),
`governing_citation` (string), and `cost_class` (`quick`, `focused`, `domain`,
or `broad`). There is one schema version at the result root.

Allowed categories are `declaration-conflict`, `scope-mismatch`,
`missing-mapping`, `excessive-validation`, `relevant-obligation`,
`suggested-command`, and `unknown`. Confidence is `deterministic` or
`heuristic`; impact is `high`, `medium`, or `low`; action is `inspect`,
`amend-declaration`, or `consider-command`.

When present, `suggested_command` contains exact argv, working directory,
affected surface, governing citation, and expected cost class. It is inert
data. Findings are deterministically ordered by rule identifier and observed
location.

## Example

```json
{
  "schema_version": "1.0.0",
  "analysis_status": "complete",
  "mode": "terminal",
  "repository_root": "/workspace/openWEPP",
  "package": {
    "id": "example-001",
    "path": "docs/work-packages/example-001/package.md"
  },
  "base_sha": "1111111111111111111111111111111111111111",
  "head_sha": "2222222222222222222222222222222222222222",
  "observed_scope": {
    "index": true,
    "worktree": true,
    "untracked": true
  },
  "policy_inputs": [{
    "path": "docs/standards/testing-and-gate-strategy.md",
    "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }],
  "unavailable_analyses": [],
  "findings": [{
    "rule_id": "WP-WRITESET-001",
    "category": "scope-mismatch",
    "confidence": "deterministic",
    "impact": "high",
    "action": "amend-declaration",
    "message": "A changed path is outside the declared write set.",
    "observed_location": {
      "path": "crates/example/src/lib.rs",
      "line": 1
    },
    "governing_source": "docs/work-packages/AGENTS.md",
    "applicability": "implementation package terminal diff",
    "reasoning": "The path is changed but matches no declared write-set entry.",
    "suggested_command": null
  }],
  "error": null
}
```

The example is advice, not a block or verdict.
