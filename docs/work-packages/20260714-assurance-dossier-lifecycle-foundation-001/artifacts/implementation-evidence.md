# Implementation Evidence

Status: `complete`; implementation, initial independent review,
accepted-finding remediation, focused proof, terminal workspace gates, and
dual accepted-fix verification pass.

## Phase 0: Authority And Intake

Static: `FROZEN_BASE` is
`00d985b1c0de77f1ea664df23a6f4999c4dad0cc`. The required-reading map records
the applicable instruction chains and the governance, usersum, release,
SNOTEL, nextest, and downstream export authorities. No branch was created or
switched.

Static: the pilot inventory traces the retained five-site observation and snow
diagnostic artifacts to exact files, SHA-256 identities, and historical
commits. It distinguishes tracked, externally located, unavailable,
verification, comparative, empirical, forcing-robust, and forcing-limited
evidence before assigning a characterization.

## Phase 1: Lifecycle And Ownership Contract

Static: `docs/governance/scientific-assurance-dossier-lifecycle.md` is the
canonical contract. It assigns why/how/what/so-what ownership, declares role
separation, defines all five lifecycle states and transitions, distinguishes
mechanical drift from scientific currency, freezes the CLI, defines immutable
snapshot semantics, and supplies the material-change trigger matrix.

Static: the V&V strategy, dossier standard, usersum style guide, release
procedure, governance/standards indexes, and root documentation map now point
to the contract and use the asymmetric vocabulary. The obsolete public
`validated / bounded / open` status ladder is removed. Verification can close
declared obligations; empirical characterization remains graded and revisable;
the named user or institution owns application fitness.

## Phase 2: Sources And Public Information Architecture

Static: `assurance/` contains one catalog, six typed and compiler-bound JSON
schema documents, four strict Markdown templates, one method, and one dossier
split into dossier, evidence, interpretation, limitations, agent-assisted
authoring, and structured review-history records. The catalog declares stable
IDs, typed narrative metadata, and one producer for each generated output.

Static: the pilot is deliberately
`CANDIDATE / INSUFFICIENT_EVIDENCE`. Its aggregate verification state is
`BLOCKED`. Three exact historical selector and phase-partition obligations are
individually `PASS`; current-release lineage, numerical solution verification,
and independent release reproduction are explicitly blocked or unrun. The
profile does not strengthen the empirical record, include
frost/runoff/erosion/watershed claims, or decide fitness.

Static: the public route is complete:

- why: `usersum/snow-frost-modeling-and-validation.md`;
- how: `usersum/assurance/methods/snow-snotel-evaluation-v1.md`;
- what: `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md`; and
- so what: `usersum/assurance/application-context-worksheet.md`.

The catalog links all four layers, and the narrative links back to the method,
dossier, and worksheet. Claim-bearing pilot scores are not duplicated in the
why narrative.

## Phase 3: Bounded Rust Compiler

Static: the new `openwepp-assurance` workspace crate exposes typed library and
CLI operations for `validate`, `plan`, `build`, and `check`. It implements:

- strict typed YAML parsing and compiler-bound schema identity/size checks;
- stable ID, version, lifecycle, evidence, review, date, and path validation;
- digest verification for every tracked evidence asset;
- a typed acyclic dependency graph with explicit authoring input/output nodes
  and version/path/length/raw-byte transitive fingerprints;
- deterministic template rendering and ordered export serialization;
- targeted or all-dossier selection;
- clean-temporary drift checking;
- separate scientific/publication roots, structured review histories,
  prefix-bound ordered-history approval locks, and authoring-output approval;
- typed verification obligations with mechanical aggregate semantics;
- public scalar, local-link, fragment, raw-HTML, absolute/private-path, and
  expanded secret/token rejection on validate, plan, build, and check;
- explicit, path-safe, immutable release snapshots; and
- bounded and streaming source reads, open-to-operation input/path-set identity,
  exact generated-root inventory, output containment, exclusive snapshot
  staging, collision, traversal, and symlink defenses.

Static: the normal compiler contains no command runner, network client,
credential handling, plugin, agent invocation, semantic status adjudication,
or nextest dependency scheduler. Nextest only executes Rust tests.

Ran: focused clippy passes with warnings denied. The 10 crate tests and 18
integration contract tests pass. A clean `build --all`, `check --all`,
two-directory byte comparison, structured review-invalidation suite, and
explicit snapshot create/confirm/security proof pass.

Ran: the delegated terminal sequence passes formatting, workspace all-target
clippy, 1,988 full-profile workspace tests, dependency-policy checks, and a
fresh closure-eligible CRAP gate. The CRAP report assesses 8,768 production
entries with two existing exact adjudications and zero actionable rows; all 14
touched production files meet the closure contract without a new waiver.

Ran: the canonical agent-assisted synthesis packet content-identifies 17
inputs, six outputs, and six accepted decisions. An independent procedural
reviewer reproduced all identities and approved accepted-output root
`01aa0936d0dce5c859440f56a9bd0eca87976462a524696307840103a9fae9ed`
with no findings. That approval is explicitly narrower than domain-science or
publication approval.

## Phase 4: Public And Release Consumers

Ran: `build --all` produced and `check --all` confirmed the catalog, worksheet,
method, dossier, and wepppy export. The public dossier's first screen states
what was tested, the separate verification and empirical results, where the
evidence applies, and what remains unknown.

Static: `assurance/generated/wepppy-usersum.yaml` supplies stable document IDs,
source-relative paths, titles, minimum role, category, audience, lifecycle
metadata, downstream-compatible status, and navigation keys. Ran: its five
records pass the real downstream parser in a read-only transformed fixture. It
is still a handoff contract, not evidence of wepppy deployment.
`artifacts/wepppy-handoff.md` assigns all downstream vendoring, manifest, role,
navigation, rendering, and search work to a future authorized wepppy package.

Static and Ran: `tools/release/check_assurance_dossier_exports.sh` performs
validation and committed-output drift checks. The release-candidate runner
invokes that real consumer, creates an explicitly tagged snapshot, and records
the snapshot manifest digest in the release evidence directory. Both changed
shell scripts pass `bash -n`; the package-specific release hook passes.

## Preserved Limits

- No openWEPP kernel, science-contract, numerical, simulation, or public result
  path changed.
- No new dataset was admitted or empirical campaign executed.
- No external hydrologist review is claimed; the candidate review record stays
  pending.
- Raw acquisition replay and a portable locked environment remain incomplete
  and visible.
- No file in `/home/workdir/wepppy` was edited by this package.
- No database, service, cache, generalized workflow engine, PROV export, or
  automated scientific adjudicator was introduced.
