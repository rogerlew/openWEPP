# ASSURE-04A Source Contract Map

Status: implemented and consumer-proven

Evidence class: Static

## Chosen Serialization

The smallest source demonstrated by the accepted groundwater prototype is:

| Source | Serialization | Role |
| --- | --- | --- |
| `assurance/v2/catalog.yaml` | strict YAML | Versioned source/schema registry and report-manifest identity |
| `reports/<id>/report.yaml` | strict YAML | Typed identities and relations; never canonical prose |
| `manuscript.md` | Markdown | Canonical scientific argument and reader-first findings |
| `supplement.md` | Markdown | Canonical evidence map, methods detail, and reproduction notes |
| `results/*.json` | strict JSON | Claim-bearing operands, values, units, and precision statements |
| `schemas/*.schema.json` | JSON Schema 2020-12 | Tracked companion contracts bound by catalog SHA-256 |

Splitting every record family into a separate file would add locator and hash
ceremony without improving the one-report prototype. The report manifest keeps
small identity records together while result values and scientific prose retain
their own content identities. ASSURE-04B may plan these declared edges but may
not reinterpret the source format.

## Executable Record Map

| Record | Required identity/meaning | Mechanical consumer |
| --- | --- | --- |
| Catalog | schema/contract version, internal state, schema/report entries | `V2Repository::open` admits exact versions and unique paths/IDs |
| Report | ID, semantic version, title, owner, lifecycle, fixture flag | Catalog binding and deterministic report summary |
| Authorship | draft authors, human report lead, scientific approver, accountability state, external-review claim | Blocks review while accountable human roles are unassigned and prevents false peer-review claims |
| Agent assistance | procedure/model identity, objective, input and exact-output dependencies, disposition, nondeterminism, limits, review state | Dependency-family closure and explicit incomplete-provenance review blocker |
| Manuscript/supplement | owner/title, confined path, digest, provenance, procedure, logical references | Regular-file/hash check and reference closure |
| Dependency | local, immutable external, or restricted identity plus access/license | Kind-specific confinement, digest, immutable-ID, and nonleakage rules |
| Unit | ID, symbol, quantity, definition | Result-value lookup plus claim/method/result references and unused check |
| Claim | statement, type, scope limit, method/result/evidence/unit/reference links | Reference closure and manuscript/supplement reachability |
| Method | description, procedure, dependencies, units | Result ownership and reference closure |
| Result | strict JSON source, method, dependencies, units, semantics, precision, realization, provenance | Content check, typed value parsing, finite values, unit closure |
| Figure | result-bearing/conceptual kind, results, procedure, alternative text, caption | Kind consistency and source-result closure; no rendering in 04A |
| Reference | citation, immutable identity, access/license, dependency | Dependency closure |
| Research object | access/license, safe content or restriction, linked results/methods/dependencies, instructions | Safe-file identity or restricted nonleakage and reproduction closure |
| Review | draft decision/root/approver/independence posture | Rejects any premature review claim |
| Publication | draft public/snapshot/release/export/vendor/supersession/withdrawal posture | Rejects every premature publication route |

## Identity Roots

Every local source is hashed from bytes after path confinement and regular-file
checks. A report source root is a domain-separated SHA-256 over the sorted
repository-relative path/digest set consumed by that report. The repository
root additionally binds the v2 catalog and all three schema companions. File
modification times, absolute workspace paths, hostnames, clocks, network
resources, shell commands, and agent output do not enter either identity.

The positive fixture resolves to one internal `DRAFT`, `fixture_only` report
and zero public reports. `validate --report` and `validate --all` call the same
loader and summary logic. With the current single-report catalog, both produce
the same source root. The source root changes whenever any validated source
byte, path, or declared relation changes and its digest chain is reconciled.

The fixture is disclosed as Codex-authored architecture evidence. Its human
report lead and scientific approver are unassigned, its exact historical agent
configuration is unavailable, and its typed accountability state therefore
blocks review entry. Coding-agent architecture review is not represented as
external scientific peer review.

## Publication Boundary

`Assurance` remains the separate ASSURE-03 zero-public builder.
`V2Repository` admits internal sources only. The CLI composes them for
validation, but `plan --all`, `build --all`, `check --all`, and zero-report
snapshot behavior remain unchanged. Report-specific plan/build/check fail with
their future owning package. No v2 source is a `usersum` consumer or export
input in ASSURE-04A.

## Explicit Deferrals

- dependency impact, freshness, and incremental selection: ASSURE-04B;
- table/figure generation, value substitution, Markdown assembly, and staging:
  ASSURE-04C;
- review-root locks, approval, promotion, public catalog/export integration,
  release snapshots, supersession, withdrawal, and vendoring: ASSURE-04D; and
- scientific revision and approval of the groundwater report: ASSURE-05.
