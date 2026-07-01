# 7. Contributing: how a change lands

The mechanics of getting work into openWEPP — gates, work packages, review —
and how the human and agent roles fit together. The normative versions are
[AGENTS.md](../../AGENTS.md) (Codex playbook), [CLAUDE.md](../../CLAUDE.md)
(Claude Code playbook), and `docs/work-packages/AGENTS.md` (package
conventions); this chapter is the orientation.

## 7.1 The authoring model

openWEPP is developed with an unusual division of labor:

| Role | Owner | Typical work |
|---|---|---|
| Code authoring, tests, refactors | **Codex** (coding agent) | implements work packages end to end |
| Documentation, architecture guidance, debugging, review | **Claude Code** | authors specs/ADRs/docs, root-causes defects, reviews Codex output |
| Direction, adjudication, dispatch | **Maintainer (human)** | decides what gets built, resolves contract questions, dispatches packages |

The point of recording this here: **none of the agents is trusted on
say-so** — the same posture that demotes the legacy binary (chapter 2)
applies to agent output. Correctness authority is the contract; every change
passes the same mechanical gates and review regardless of who or what wrote
it. Human contributors slot into the same system: your change also lands
through a package, gates, and review.

## 7.2 The mechanical gates

Every change must pass (see `AGENTS.md` → Validation Gates for the exact
commands):

```bash
cargo fmt --check
cargo clippy --workspace --all-targets   # warnings denied
cargo nextest run --workspace            # full suite
cargo deny check                         # licenses + advisories (no copyleft)
```

plus, depending on the surface touched:

- **Authority guards** — scripts under `tools/` that verify the units
  registry, symbol aliases, SC-contract compliance, and required
  external-authority suites haven't been weakened (anti-evasion: a gate that
  detects gate-removal).
- **Protected-output identity** — behavior-preserving changes to runtime,
  publication, or parsers must show H2637 outputs byte/value-identical
  before/after (chapter 5, "identity gate").
- **Conservation/closure checks** — kernel-affecting changes carry
  contract-derived invariant tests, not just unit tests.

## 7.3 Work packages: the unit of execution

Substantive work happens inside a **work package**:
`docs/work-packages/<yyyymmdd-name-nnn>/` with a `package.md` (scope,
objectives, acceptance gates), `prompts/` (the execution spec), and
`artifacts/` (evidence produced during execution). Conventions that matter:

- **Front-loaded autonomy.** A package is written so an agent can execute it
  without mid-course intervention: required reading, entry gates, stop
  conditions, and acceptance gates are all stated up front.
- **Contract-first sequencing** for kernel work: amend the science contract,
  write contract-derived tests, record the pre-implementation gate, *then*
  touch production code.
- **Dual independent review.** Two agent reviews with every finding
  dispositioned (`accepted` / `rejected` / `deferred`, with rationale);
  accepted findings fixed and re-verified before closure.
- **Honest closure.** Packages that fail record *why* — the log keeps dead
  ends (the PERFMIG/PERFDEEP series is the canonical example) because
  negative results are load-bearing evidence for later decisions.
- **Defect work is diagnose-and-correct** in one package
  ([Defect-Closure ExecPlans](../defect_closure_execplans.md),
  [ADR-0018](../decisions/0018-defect-closure-execplans-conversion-rule.md)),
  not diagnostic-only relays.

Ideas that aren't ready for a package start as **backlog concept notes**
(`docs/backlog/`, tracked in its `TRACKER.md`) and are promoted when
prioritized; the forward queue itself is [ROADMAP.md](../ROADMAP.md).

## 7.4 Evidence discipline

The rule that shapes every artifact (and this guide): **match the verb to the
evidence.** "Ran" means the command executed in that session, output in hand;
"Static" means read-and-reasoned. Reviews label their evidence class up
front; performance claims come from endpoint runs; delegated results are
attributed ("Codex's run reported…"). If you internalize one process rule,
make it this one — the record's trustworthiness is the project's foundation,
and it is cheap to keep and expensive to restore.

## 7.5 Practical starting points

- **Find something to work on:** [ROADMAP.md](../ROADMAP.md) (queue),
  `docs/backlog/TRACKER.md` (concepts), open holds in recent work packages.
- **Understand a subsystem:** its architecture doc in `docs/architecture/`,
  then the owning crate's code, then the work packages that built it
  (search `docs/work-packages/` for the subsystem name).
- **Question a design:** find the ADR; if there is none, that's a real gap —
  raise it.
- **Tooling:** Rust toolchain is pinned (`rust-toolchain.toml`); Python
  tooling (comparison utilities under `tools/owcmp/`) uses a repo-local
  `uv`-managed `.venv` (see README).
