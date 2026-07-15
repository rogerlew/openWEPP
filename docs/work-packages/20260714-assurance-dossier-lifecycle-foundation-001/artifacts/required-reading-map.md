# Required Reading Map

Status: `complete` for execution intake on 2026-07-14.

`FROZEN_BASE`: `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`.

## Instruction Discovery

Ran: `tools/agents/find-agents --for` was rerun on the complete intended write
set before implementation. The scaffolded instruction chain remained current.

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

Ran: core byte counts were recomputed. Core total remained `222716`; the two
read-only wepppy conditional files add `17073` bytes, for `239789` loaded
bytes, still `OK`.

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

## Resolved Conditional Reading

Static: the wepppy specification and vendor registry were read before freezing
the handoff. The contract requires stable `doc_id`, `source`, `rel_path`,
`title`, `min_role`, `category`, `audience_tags`, `status`, and `nav_key`;
wepppy owns the vendor record, synchronization, final merge, roles, navigation,
rendering, and search. No openWEPP vendor currently exists there.

Static: pilot status assignment used the following retained sources:

| Path | Bytes | Role |
| --- | ---: | --- |
| `tests/fixtures/snotel_observed/README.md` | `5833` | Five-site fixture scope and forcing limitations |
| `tests/fixtures/snotel_observed/observations/manifest.json` | `11417` | Observation identities and known absolute-path/null-hash gaps |
| `tests/fixtures/snotel_observed/observations/ssd_characterization.json` | `24633` | Retained station-depth characterization |
| `tests/integration/snowdensity10_3_18_cross_snotel_mechanism_rubric.rs` | `1870` | Diagnostic-only policy guard |
| `tests/integration/snowdensity10_3_19_harder_pomeroy_default_activation.rs` | `6698` | Default activation and retained report guard |
| `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.md` | `6922` | Human diagnostic summary |
| `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.json` | `5121983` | Claim-bearing retained diagnostic output; content-identified, not loaded in bulk |
| `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/disposition.md` | `1478` | No-promotion disposition |
| `docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/artifacts/harder-pomeroy-default-activation.json` | `1756449` | Later activation evidence |

No nextest profile was added, so the local-CI conditional guide was not
triggered. No kernel or science-contract semantic edit was proposed.

## Intake Checklist

- [x] Record `FROZEN_BASE` after the scaffold is committed.
- [x] Re-run `tools/agents/find-agents --for` on the final write set.
- [x] Recompute core bytes and threshold.
- [x] Resolve all core paths.
- [x] Record conditional SNOTEL sources before status assignment.
- [x] Confirm no external-repository write is present.

Ran: `/home/workdir/wepppy` was read only. Its preexisting tracked changes were
recorded at intake in `owned-file-manifest.md`; no write there is authorized.
