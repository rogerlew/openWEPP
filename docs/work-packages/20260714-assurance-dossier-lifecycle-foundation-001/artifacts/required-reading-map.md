# Required Reading Map

Status: `scaffolded`; execution intake must revalidate paths, bytes, and
applicable instruction chains before edits.

## Instruction Discovery

Static: `tools/agents/find-agents --for` was run on the complete intended write
set during scaffolding.

| Write area | Applicable instructions |
| --- | --- |
| Root Cargo, nextest, release tooling, governance, roadmap, `assurance/**`, and `usersum/**` | `AGENTS.md` |
| `docs/standards/**` | `AGENTS.md`; `docs/standards/AGENTS.md` |
| `docs/work-packages/**` | `AGENTS.md`; `docs/work-packages/AGENTS.md` |
| `crates/openwepp-assurance/**` | `AGENTS.md`; `crates/AGENTS.md` |
| `tests/integration/**` | `AGENTS.md`; `tests/AGENTS.md` |

No nested `AGENTS.md` currently applies under `assurance/`, `usersum/`,
`docs/governance/`, or `tools/release/`. Re-run discovery if any nested guide
is added before or during execution.

## Core Pre-Edit Reading

| Path | Purpose | Bytes |
| --- | --- | ---: |
| `AGENTS.md` | Repository invariants and routing | `10822` |
| `docs/work-packages/AGENTS.md` | Package execution and closure | `21107` |
| `docs/standards/AGENTS.md` | Standards ownership | `3328` |
| `docs/standards/prompt-wording-guidance.md` | Kickoff and delegation wording | `9780` |
| `crates/AGENTS.md` | Rust crate rules | `5171` |
| `tests/AGENTS.md` | Integration-test rules | `4534` |
| `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/package.md` | Authorized objective and gates | `32992` |
| `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/required-reading-map.md` | Tiering and instruction map | `4389` |
| `docs/governance/openwepp-verification-validation-strategy.md` | Assurance philosophy and roadmap | `24490` |
| `docs/standards/scientific-assurance-dossier.md` | Dossier content/status standard | `16207` |
| `docs/standards/usersum-authoring-style-guide.md` | Public narrative/style contract | `11322` |
| `docs/decisions/0028-observed-data-admission-authority.md` | SNOTEL admission and forcing interpretation | `8042` |
| `usersum/README.md` | Public documentation routing | `2188` |
| `usersum/snow-frost-modeling-and-validation.md` | Pilot why/evidence narrative | `22242` |
| `docs/governance/openwepp-release-procedure-draft.md` | Release evidence boundary | `8502` |
| `tools/release/README.md` | Existing release gates | `7098` |
| `tools/release/check_hillslope_schedule_export.sh` | Deterministic generated-doc precedent | `959` |
| `Cargo.toml` | Workspace/crate integration | `22955` |
| `.config/nextest.toml` | Current test scheduling policy | `6588` |

Core total: `222716` local bytes.

Threshold: `OK`. The execution threshold is `OK` at no more than 400000
bytes, `WARN` above 400000 through 650000 bytes, and
`REQUIRES-JUSTIFICATION` above 650000 bytes.

## Conditional Reading

| Trigger | Paths |
| --- | --- |
| Before freezing the wepppy export/handoff | `/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/specification.md`; `/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/vendors.yaml` |
| Before changing or adding a nextest profile/heavy gate | `docs/standards/local-ci-gate-selection.md` |
| Before assigning the pilot evidence status | Exact ADR-0028/SNOTEL evidence, commands, manifests, review, and retained outputs discovered during inventory |
| If a kernel or science-contract semantic edit is proposed | `docs/specifications/science-contracts/AGENTS.md` plus the exact canonical contract; stop and amend package scope first |

Conditional material is loaded at its trigger, not forced into initial context.
Record paths and byte counts as the inventory resolves them.

## On-Demand Reading

- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/artifacts/disposition.md`
- `docs/work-packages/20260714-vv-asymmetric-assurance-reframe-001/artifacts/disposition.md`
- official nextest documentation for unresolved runner behavior only
- read-only wepppy manifest/navigation/loader examples needed to validate the
  export fragment

## Intake Checklist

- [ ] Record `FROZEN_BASE` after the scaffold is committed.
- [ ] Re-run `tools/agents/find-agents --for` on the final write set.
- [ ] Recompute core bytes and threshold.
- [ ] Resolve all core paths.
- [ ] Record conditional SNOTEL sources before status assignment.
- [ ] Confirm no external-repository write is present.
