# Prospective Intent Plan

Evidence class: `Static`.

Frozen boundary: documentation-only characterization increment.

Authorized changes are limited to this work package and its catalog entry.
There are no Rust, policy, workflow, schema, dependency, science, or public
behavior changes. The package adds focused existing-test execution as an
operator-requested acceptance suite; this is broader than mechanical docs-only
selection but far narrower than a workspace or campaign gate.

| Gate | Reason | Invalidation |
| --- | --- | --- |
| Package path and diff hygiene | Detect out-of-write-set work and seeded hygiene fault. | Rerun only after package-path edits. |
| `cargo nextest run -p openwepp-gate-planner --lib` | Existing dirty, unknown, tamper, receipt, and fail-closed scenarios. | Rust/test bytes are read-only, so run once. |
| Three focused TESTGATE integration contracts | Existing schema, assurance, workflow, queue, and supersession bindings. | Rust/test/workflow bytes are read-only, so run once. |
| Changed-document Markdown lint | Required for documentation changes. | Rerun after documentation evidence changes. |
| Local committed TESTGATE execution | Prove exact plan selection and local receipt verification. | Run once on executor completion commit; later review docs do not make this local receipt current. |
| Live normal TESTGATE | Prove exact final head reaches forest1, hosted verification, and attestation. | Parent runs once through the normal push. |

Explicitly not selected: workspace/full Nextest, Clippy, coverage, CRAP,
cargo-deny, comparator, campaign certification, release qualification, manual
dispatch, or soak/count observation.

Gate non-deferral: all local gates close before executor handoff. Dual review,
terminal verification, and the live exact-head receipt close before final
package disposition. Nothing required is labelled deferred.
