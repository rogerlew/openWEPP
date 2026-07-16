# Execute ASSURE-04C

Scope: local openWEPP repository assurance engineering; flat-file source edits,
staging builds, and tests only; no network or external-system writes.

Execution mode: package-end-to-end. Execute every phase in `package.md` through
final disposition unless a proven hard blocker prevents a current-scope gate.

Required reading — Core: root/package governance; ADR-0038; the v2 architecture,
lifecycle, source/build, report, and `usersum` authoring contracts; the
prospective ASSURE-04 roadmap; the completed 04B handoff; this package and
`artifacts/required-reading-map.md`. Conditional: crate/test/prompt governance
before matching edits. On-demand: current assurance sources/tests for touched
mechanisms. Required-reading budget: 179,725 local bytes including this package
and all triggered Conditional files, `OK`; map:
`artifacts/required-reading-map.md`.

Files: only the declared write set. Tracked `usersum`, protected zero-report
catalog/template/export, science contracts, kernels, snapshots, releases, and
vendor paths are read-only.

Task: implement deterministic staging-only manuscript/supplement assembly for
one named v2 report and all v2 reports. Consume the 04B typed plan; resolve
retained values, tables, accessible figures, citations, and portable links;
prove the actual `usersum`-compatible Markdown consumer and exact staging check.

Constraints: human-authored interpretation remains canonical; typed fail-closed
errors; descriptor-confined reads and confined staging writes; no arbitrary
templating, timestamps, modification times, automatic source-hash rewrite,
network, shell, agent, hostname, environment interpolation, absolute path
leakage, review approval, tracked-public writes, publication, snapshot, release
transfer, export, or vendoring.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to one heavy-gate runner plus two independent read-only
reviewer/verifier agents for Phases 4 through 6. The heavy runner may write only
compact package evidence and must run the full workspace, deny, and fresh CRAP
closure. Reviewers/verifiers return compact independent findings to the parent
and have no write authority.

Autonomy: execute all phases and update artifacts without requesting next steps
unless hard-blocked. Every current gate needs direct evidence; no ASSURE-04C
gate may be deferred to ASSURE-04D and called complete.

Outputs: assembly contract, schema/source vocabulary, assembler API/CLI,
focused tests, retained staged consumer, protected/sandbox proof, gate/CRAP/
line-count records, dual reviews, complete disposition, heavy evidence, dual
terminal verification, final disposition, and ASSURE-04D handoff.
