# Assurance Mechanical Work Queue

Evidence class: Static architecture audit

ASSURE-MAINT-01 moves identity propagation, reader-block rendering, bounded
attribution and role changes, lifecycle recording, normalization, candidate
validation, receipt selection, and focused gate evidence out of agent-owned
work. The following deterministic work remains deliberately outside this
package.

| Priority | Mechanical owner | Proposed interface | Inputs | Outputs | Human boundary | Dependency |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Report scaffolder | `openwepp-assurance scaffold report --request <yaml>` | Report ID, title, owner, scientific question, process, quantity, initial source paths | Valid DRAFT directory, catalog entry, generated locks, receipt | A human selects the study and owns its scientific question; the command does not draft claims or conclusions | Stable v2 report template after two production reports complete review |
| 2 | Research-object ingester | `openwepp-assurance ingest object --report <id> --request <yaml>` | Existing local object, declared role, license, provenance, public-safety disposition | Confined copy, typed descriptor entry, identity transition, receipt | A human determines relevance, provenance sufficiency, license, restriction, and publication safety | Typed object classes and license vocabulary |
| 3 | Reproduction receipt recorder | `openwepp-assurance lifecycle --event reproduction_record --request <yaml>` | Exact procedure, inputs, environment identity, outputs, comparison, named responsible principal | Immutable reproduction event and generated index | The responsible person runs or supervises reproduction and interprets discrepancies; software records rather than invents the disposition | Stable reproduction-event schema and realization projection |
| 4 | Evidence and report catalog builder | `openwepp-assurance catalog build --all` | Current approved report locks and public-safe research objects | Deterministic internal/public catalog candidates | Publication and inclusion remain named human and release decisions | At least one approved production report |
| 5 | Dependency currency scanner | `openwepp-assurance currency --all --format json` | Declared immutable references, repository paths, software identities | Current/stale/blocked inventory and affected-report closure | A human decides scientific materiality and required rereview | Versioned external-reference resolvers; offline mode remains authoritative |
| 6 | Application-context worksheet assembler | `openwepp-assurance application-assessment scaffold` | Selected report, application quantities, site/regime descriptors, decision owner | A consistent assessment worksheet with report evidence linked | The hydrologist or decision owner judges application fitness; the command cannot issue a verdict | Public report and application-assessment schema |
| 7 | Release handoff packager | `openwepp-assurance handoff --release <identity>` | Approved realization, release-transfer event, public research objects | Self-contained release candidate and verification instructions | Release owner and assurance steward authorize transfer; WEPPcloud adoption remains separate | ASSURE-08 beta-release campaign |

## Work That Must Remain Human Or Agent-Assisted

Scientific question selection, literature synthesis, method selection,
interpretation, limitation writing, finding adjudication, competence and
independence assessment, scientific approval, application fitness, and release
authority are not candidates for mechanical ownership. Agents may assist only
through disclosed, retained, reproducible procedures whose outputs receive the
required human review.

## Ordering Rationale

Scaffolding and ingestion eliminate the next largest sources of repetitive
schema and identity bookkeeping. Reproduction recording follows because its
event model must be learned from real review. Catalog, currency, assessment,
and handoff commands should be built only after approved reports establish
their real consumer contracts; implementing them earlier would optimize an
unproven publication shape.
