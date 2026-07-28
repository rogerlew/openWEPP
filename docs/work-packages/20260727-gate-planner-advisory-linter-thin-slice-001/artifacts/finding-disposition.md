# Review Finding Disposition

Evidence class: Ran + Static.

| Finding | Disposition |
| --- | --- |
| `core.pager` not refused | Fixed. It is now a prohibited qualified key and is covered before every narrowed argv. |
| Config grammar variants bypassed refusal | Fixed. In-process section parsing accepts Git's trailing `#`/`;` comment form and general whitespace before subsections, and fails closed on ambiguous headers, keys, and continuations. |
| `core.attributesFile` not refused | Fixed. External attribute selection now refuses before launch. |
| `.git/info/attributes` not read | Fixed. It is included as a bounded no-follow optional attribute source. |
| Nested attributes were pruned | Fixed. No worktree directory class is excluded; formerly pruned dependency, cache, build, and target directories have direct pre-launch regressions. |
| Dirty rename old path omitted | Fixed. Both new and original paths are retained. |
| Unmerged path misparsed | Fixed. Porcelain-v2 `u` records use their exact field count. |
| Missing/ambiguous/mismatched identity lacked advice | Fixed. Stable cited declaration-conflict findings are emitted without inferred identity. |
| Detached HEAD lacked advice | Fixed. Bounded `.git/HEAD` inspection emits an explicit advisory finding. |
| Output bound was post-hoc | Fixed. Nonblocking selector reads stop and kill at the per-stream byte limit or timeout. |
| JSON misuse lost recognized mode | Fixed. A valid explicitly supplied mode is retained. |
| Markdown suggestion mismatched surface | Fixed. The inert command and affected surface both name repository Markdown. |
| Mutation proof was too narrow | Fixed. Tests snapshot every worktree and `.git` path with mode, size, mtime, ctime, inode, link count, and regular-file hash. |
| Network proof was only mocked | Fixed. Every mode runs under `strace`; tests reject any IPv4/IPv6 socket or address-bearing send/receive call. Structural argv tests still prove no remote-capable Git command exists. |
| Line count was incorrect | Fixed. Evidence now records 1,011 product lines, below 3,000. |

The corrected suite contains 25 tests and passes. Both original reviewers must
re-review the corrected implementation before closure.
