# Execute ASSURE-04B

Scope: local openWEPP repository assurance engineering; read-only ordinary
planning plus local source edits, builds, and tests; no external connectivity
or external-system writes.

Execution mode: package end-to-end. Execute every phase in `package.md` through
final disposition unless a proven hard blocker prevents a current-scope gate.

Required reading: root/package/crate/test/prompt governance; ADR-0038; the v2
architecture, lifecycle, source/build contract, report standard and roadmap;
the completed 04A handoff/source map; this package and its required-reading map;
and current assurance source/tests for touched consumers.

Files: only the declared write set. Public assurance and all `usersum` files,
v2 source fixtures/schemas, science contracts, kernels, export, snapshot, and
vendoring surfaces are read-only.

Task: implement deterministic content-identity dependency planning for one
named report and all reports. Prove current, stale, blocked, and transitively
selected states; stable graph order; human/JSON equivalence; selection
isolation; graph-defect failures; and the real CLI consumer.

Constraints: typed fail-closed errors; no timestamps, cache, automatic hash
rewrite, network, shell, agent, hostname, environment interpolation, absolute
path leakage, source/publication writes, generated interpretation, rendering,
approval, publication, release transfer, or vendoring. Report-specific
build/check remain ASSURE-04C work.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to one heavy-gate runner plus two independent read-only
reviewer/verifier agents for package Phases 4 through 6. The heavy runner may
write only compact package evidence and must run full workspace, deny, and
fresh CRAP closure. Reviewers/verifiers return compact independent findings to
the parent and have no write authority.

Autonomy: execute end-to-end without requesting next steps unless hard-blocked.
A phase is complete only with direct current evidence; no 04B gate may be
deferred to 04C/04D and called complete.

Outputs: planner implementation/API/CLI, focused contract tests, protected and
consumer proof, gate/CRAP/line-count records, dual reviews, complete finding
disposition, heavy evidence, dual terminal verification, final disposition,
and ASSURE-04C handoff.
