# Local TESTGATE Command Plan

Evidence class: `Static` before execution.

| Command | Purpose | Expected invalidation scope |
| --- | --- | --- |
| `cargo build -p openwepp-gate-planner --bin openwepp-gate-plan` | Establish exact current planner-binary provenance because the pre-existing binary had no bound source identity. | Planner-binary provenance only; package documentation does not invalidate it. |
| `python tools/local_ci/testgate.py --binary target/debug/openwepp-gate-plan --base HEAD --artifact-root /tmp/openwepp-testgate-adversarial-QKvyqa --intent-package docs/work-packages/20260720-testgate-adversarial-agent-acceptance-rerun-001/package.md --dirty --execute` | Run the one admitted local intent/terminal plan and its selected node against the current dirty documentation-only change set. | Produces a local/untrusted receipt; any edit to bound execution/authority inputs or an additional TESTGATE attempt invalidates this acceptance evidence. |

The external artifact root is fresh and empty. The controller sentinel was
verified unchanged and removed before this command. `--base HEAD` is required
to describe the current dirty package increment, rather than folding the
already-closed predecessor documentation commit into the change set.
