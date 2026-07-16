# Scientific Assurance V2 Implementation Roadmap

Status: prospective decomposition — ASSURE-04D is next eligible and requires a
separately authorized work package

## Sequencing Principle

Implement only machinery demonstrated by the accepted hand-authored prototype.
Each package must prove a real downstream consumer and leave the repository in a
coherent state. No package may use renderer output or a passing schema as a
substitute for scientific review.

## ASSURE-03 — Retire V1 And Establish Zero Reports

Foundation status: completed by
`20260714-assure03-v1-retirement-zero-report-001`. The exact recovery record,
zero-report state, and release/validation split are binding starting conditions
for the prospective packages below.

Outcome: execute the
[migration plan](scientific-assurance-v2-migration-plan.md).

Primary consumer: public `usersum` catalog, release assurance check, and dormant
export handoff.

Required gates: exact v1 recovery manifest; zero public candidate routes;
neutral zero-report catalog; deterministic empty build/check; release/export
negative proof; link/search repair; snow/frost science-preservation review; dual
review and verification.

First technical gate: close `ASSURE03-REL-001`. Ordinary PR/push validation
must not create or upload a release-candidate snapshot, and explicit release
mode must fail closed while the v1 candidate or transition marker exists. Only
then may the package install and prove the positive zero-report release path.

Rollback: Git recovery of historical bytes for audit only. Public restoration
is prohibited without a new decision.

## ASSURE-04A — Completed Source And Identity Foundation

Foundation status: completed by
`20260715-assure04a-v2-source-identity-foundation-001`.

Outcome: added the smallest v2 source layout and schemas for an authored
manuscript, supplement, dependency declarations, claims, result objects,
figures, references, review, and publication record. The accepted groundwater
prototype is the only positive fixture; the public catalog remains empty.

Primary consumer: `openwepp-assurance validate` and the subsequent dependency
planner. Prove the real CLI reads every admitted field; schema-only fixtures do
not close the package.

Required gates: valid prototype admission; missing/duplicate/unknown/unused
identity failures; unit and content-identity guards; path confinement;
restricted-evidence safety; version migration tests; touched-code coverage and
CRAP closure; workspace gates; dual review and verification.

Rollback: remove v2 source admission while retaining the zero-report v1-retired
state.

## ASSURE-04B — Completed Dependency Planner And One/All Plans

Foundation status: completed by
`20260715-assure04b-v2-dependency-planner-001`.

Outcome: implemented deterministic transitive planning for one report and all
reports. The planner explains current, stale, blocked, and selected targets from
content identities rather than modification time.

Primary consumer: the real `openwepp-assurance plan` command in human and JSON
forms, plus a report whose changed method, result, figure, review, narrative,
contract, or software dependency causes the expected plan. The typed planner
API is the required input to the report-specific build/check assembly that
ASSURE-04C implements; 04B does not render or check generated manuscripts.

Required gates: cycle detection; missing and unused edge failures; transitive
impact; one/all equivalence; no unrelated output changes; stable ordering;
machine-readable and human-readable plans; real CLI consumer; code-quality and
workspace closure.

Rollback: validation continues without incremental planning; no public
publication is enabled.

## ASSURE-04C — Completed Deterministic Manuscript Assembly

Foundation status: completed by
`20260715-assure04c-deterministic-manuscript-assembly-001`.

Outcome: built staging-only report and supplement pages from canonical authored
sources while mechanically resolving claim-bearing values, tables, figures,
citations, and cross-references. Interpretation remains authored prose.

Primary consumer: staged groundwater prototype rendered through the actual
`usersum`-compatible Markdown path, including accessible table/figure
alternatives.

Required gates: one/all deterministic bytes; semantic preservation; stale,
missing, mismatched-unit, changed-precision, orphaned-result, and inaccessible-
figure failures; no network/shell/agent/wall-clock/absolute-path dependency;
ordinary-build sandbox test; real rendered-link consumer; code-quality and
workspace closure.

Rollback: remove generated staging output and return to the hand-authored
prototype; zero public reports remain.

## ASSURE-04D — Review Locks, Publication, And Snapshots

Outcome: implement scientific and reproduction/publication review locks,
staging-to-public promotion for approved roots only, catalog integration, drift
checks, and immutable release snapshots.

Primary consumers: the production promotion code path, a confined temporary
`usersum`-shaped fixture root, and the openWEPP release gate. A synthetic
approved fixture may exercise mechanics only inside the temporary root. The
tracked public `usersum` catalog must remain in its zero-report state until
ASSURE-05 supplies a genuinely reviewed report.

Required gates: draft and in-review negative publication proofs; changed-root
lock invalidation; complete finding disposition; named human approval; catalog
lists approved only; snapshot completeness and immutability; exact release
transfer; removed-report cleanup; release/export consumer; accessibility;
an explicit negative assertion that tracked `usersum` is byte-unchanged; a
test-only snapshot marker that release assembly rejects; anti-evasion,
code-quality, workspace, dual review, and verification gates.

Rollback: delete the confined fixture root and retain only clearly marked
test-only snapshot evidence. No synthetic fixture is promoted, withdrawn from,
or ever written into the tracked public tree or a release snapshot.

## ASSURE-05 — First Production V2 Report

Outcome: convert the accepted groundwater prototype into the first complete v2
scientific report and supplement.

Scientific work:

- freeze an exact release candidate and declared groundwater paths;
- rerun analytical, domain, consumer, and production recurrence evidence;
- retain input, output, log, and result objects with stable identities;
- independently reproduce or reconstruct every material value;
- revise the manuscript from review rather than mechanically promoting the
  ASSURE-02 prototype; and
- state the formulation's prior Priest River evidence without misattributing
  its performance statistics to openWEPP.

Primary consumers: hydrologist/soil-scientist reader, related model narrative,
public assurance catalog, and release snapshot.

Required gates: report standard's minimum useful publication test; domain
scientific review; independent reproduction; publication/accessibility review;
claim/evidence resolution; contrary and limitation coverage; exact release
transfer; deterministic public build; catalog/narrative cross-links; named human
approval; dual implementation review and verification.

Rollback: withdraw the report and return public navigation to zero reports
without deleting the reviewed snapshot or evidence.

## ASSURE-06 And Later Portfolio

After the pilot proves the complete reader and build lifecycle, author the
snow/frost flagship synthesis from the existing scientific narrative,
observational campaigns, process contracts, conservation evidence, negative
mechanism trials, and production outputs. Split reports when precipitation
phase, SWE, snow depth/density, frost, or runoff timing require materially
different data, methods, scales, or conclusions.

Subsequent selection follows user importance and evidence readiness across
hydrology, erosion, sediment, plant, channel, and watershed processes. A public
portfolio always shows approved reports and explicit ordinary-language gaps;
it never fills the catalog with status-only placeholders.

## WEPPcloud Transfer

Vendoring is not part of ASSURE-03 through ASSURE-07. ASSURE-08 refreshes the
cross-repository contract and proves the real WEPPcloud manifest, navigation,
roles, rendering, link rewriting, accessibility, search, snapshot, and vendor
consumer immediately before the openWEPP beta release in WEPPcloud.
