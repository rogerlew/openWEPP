# Inventory And Diff

Evidence class: `Static` and `Ran`

The Rust diff is 13 insertions and 1 deletion. The deletion only renames the
oversized helper; insertions add two helper declarations, two repeated
read-only workflow loads, boundary braces, and a three-call wrapper. No
existing assertion line is changed.

Pre/post lexical inventories:

| Construct | Before | After |
| --- | ---: | ---: |
| `assert!` plus `assert_eq!` | 126 | 126 |
| `.contains(` | 105 | 105 |
| `.matches(` | 5 | 5 |
| `.find(` | 16 | 16 |
| `.split_once(` | 7 | 7 |

No `#[allow]`, test attribute, production path, workflow, policy, schema,
fixture, assertion operand, or asserted source string changes. The original
call site still invokes `assert_workflow_and_rollback_contract`; that wrapper
now calls the surface, job-order, and rollback/release helpers in original
source order.
