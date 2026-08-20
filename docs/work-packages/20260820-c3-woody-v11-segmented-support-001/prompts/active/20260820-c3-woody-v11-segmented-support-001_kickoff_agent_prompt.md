# Execute C3 Woody V11 segmented support end to end

Package execution/diff base: `d59ba76f7f514a98ba0f67f764b289206f9f94b9`.
Package scaffold identity: `3bc8562bff700722e928e631280cf13a8b171ee9`.
Actual launch HEAD is recorded dynamically during intake. Before edits:

```bash
cd /workdir/openWEPP
test "$(git branch --show-current)" = "main"
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
git merge-base --is-ancestor \
  3bc8562bff700722e928e631280cf13a8b171ee9 HEAD
git status --short --branch
git diff --check
```

Do not pull, reset, rebase, merge, switch branches, amend the base, create a PR,
or push. Local commits are allowed at authority and terminal checkpoints.

## Scope

Local repository contract/kernel implementation; flat-file reads and edits
only; no external connectivity, messaging, deployment, release, or push.

## Execution mode

Package-end-to-end. Execute all `package.md` phases sequentially through
truthful disposition. Do not stop after types, authority, or a mock consumer.

## Phase plan

Execute Phase 0 intake; V11 contract/model/vectors; dual authority review and
verification; exact authority checkpoint; Rust implementation; actual-consumer
compatibility and segmented tests; three implementation reviews; heavy gates;
dual terminal verification; and Child 2C handoff, in that order.

## Required reading

Core: root/crates/work-package/science-contract/test `AGENTS.md`; this package;
Child 2A handoff; `SC-COUPLEDTIME-001`; `SC-VEGETATION-001`;
`SC-VEGETATIONTRANSACTION-001`; science-contract authoring procedure; kernel
package preparation; prompt wording; and testing/gate strategy.

Conditional: persisted-restart authority before restart design; unit governance
for wire/unit exposure; orchestrator and vegetation source maps before edits.
On-demand: snow/LSE authority only to preserve Child 2C; baseline legacy sources
only where V10 provenance requires confirmation.

Required-reading budget: `390554` bytes, `WARN / REQUIRES-JUSTIFICATION` because
the 203590-byte canonical vegetation contract must be read completely for an
immutable successor. Record launch bytes in `artifacts/required-reading-map.md`.

## Files

Only the prospective write set in `package.md`: two vegetation contracts and
registry; additive V11 vegetation; bounded default-off orchestrator consumer;
additive V11 restart; `c3_woody_v11_*` tests; root Cargo files if required;
package artifacts; and truthful lifecycle files. Amend scope before expansion.

## Contract-first release gate

No production Rust before model/schema/vector/reference authority, derived
tests, two authority reviews, full disposition, two verifications, promotion,
and an exact local checkpoint.

## No surrogate physics

V11 executes the imported V10 constitutive stack over admitted support.
Scaling, proxy integration, duration overrides, independently converted ticks,
and reconstructed approximate state are forbidden.

## Conservation/output acceptance

Record water, energy, carbon, nitrogen, material, and time-support operand
lineage. Use alias-separating fixtures and independently reconstruct cumulative
parent debits, receipts, reductions, and owner endings. Self-consistency alone
is not acceptance.

## Subagent requirement

REQUIRED: spawn `comparator_suite_runner` for full workspace, broad Clippy,
cargo-deny, and heavy comparator/property populations. This prompt explicitly
authorizes two authority reviewers, two authority verifiers, three
implementation reviewers, `comparator_suite_runner`, and two terminal
verifiers. Reviewers/verifiers are read-only except named artifacts; the runner
writes only ignored logs and bounded package artifacts.

## Autonomy

Execute end-to-end without requesting direction unless a genuine authority,
wire, dependency-cycle, or owner-atomicity contradiction remains. Preserve V10,
DirectV10 restart V1, coupled-time restart V2, selectors/defaults, and Child 2C.
