# Wave 4 HOLD Ratification Checklist

Status: Draft (ARCH13)
Evidence: Static
Ran evidence: none

## Decision Record Schema

Each decision record is ratified only when all fields are complete:
- `decision_id`
- `question`
- `linked_gap_ids`
- `allowed_options`
- `selected_option`
- `decider`
- `decision_date_utc`
- `required_evidence`
- `downstream_actions`
- `status`

Allowed `status` values:
- `pending`
- `ratified`
- `deferred-with-risk-acceptance`

## Canonical Symbol Continuity Note

Decision text preserves canonical `wepp-forest` symbols where relevant,
including `cbase`, `dtchr`, `ichout`, and `lcwbflg`, rather than replacing
them with renamed boundary aliases.

## Decision Records

| decision_id | question | linked_gap_ids | allowed_options | selected_option | decider | decision_date_utc | required_evidence | downstream_actions | status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `W4DR-001` | Source-authority policy for sidecars lacking dedicated `usersum2024` format tables: ratify legacy/static code provenance as normative? | `TC-GAP-001`, `TCR-GAP-001`, `GWCOEFF-GAP-001`, `PHOS-GAP-001`, `LCWB-GAP-001` | `A) legacy/static provenance ratified as normative until superseded`, `B) provisional-only (no normative authority)`, `C) mixed by file with explicit per-file policy` | `pending` | `pending` | `pending` | contract excerpts + legacy source paths + ratification note | update affected `SC-INFILE-*` HOLD statements and source-authority sections | `pending` |
| `W4DR-002` | Strict vs compatibility open-error policy for sentinel/optional sidecars. | `TC-GAP-002`, `TCR-GAP-002`, `GWCOEFF-GAP-004` | `A) strict hard-fail; compat collapse-with-warning`, `B) strict and compat both hard-fail`, `C) strict and compat both collapse-with-warning` | `pending` | `pending` | `pending` | fixture matrix showing ENOENT vs non-ENOENT behavior per file | standardize error/warning IDs and policy text across contracts | `pending` |
| `W4DR-003` | Program ownership boundary: true parser inputs vs derived compatibility/output-control flags. | `CHANINP-GAP-004`, `LCWB-GAP-004`, `TC-GAP-003`, `LCWB-GAP-003` | `A) parser owns only file input + derived provenance flags`, `B) parser also owns output-control semantics`, `C) split by subsystem with explicit boundary contract` | `pending` | `pending` | `pending` | ownership matrix across parser/orchestrator/output contracts | add/adjust ownership clauses in affected contracts | `pending` |
| `W4DR-004` | `chan.inp` `ichout` domain policy: legacy/usersum `0..3` vs `wepppy` subset `{1,3}`. | `CHANINP-GAP-003` | `A) canonical domain 0..3`, `B) constrained domain {1,3}`, `C) strict 0..3 with compatibility normalization to {1,3}` | `pending` | `pending` | `pending` | usersum table + legacy branch evidence + producer interoperability evidence | finalize domain guards and compat behavior in SC/fixtures/tests | `pending` |
| `W4DR-005` | `chan.inp` `dtchr` normalization/error-path semantics for legacy ambiguous initialization paths. | `CHANINP-GAP-002` | `A) strict reject ambiguous paths; compat normalize`, `B) strict normalize with warning`, `C) preserve raw value and defer normalization downstream` | `pending` | `pending` | `pending` | fixture-backed branch matrix for legacy open/parse/init paths | codify deterministic normalization and warning/error IDs | `pending` |
| `W4DR-006` | `chan.inp` `cbase` semantics: downstream meaning/units and mandatory guards. | `CHANINP-GAP-001` | `A) preserve current unit model with additional range guards`, `B) redefine/rename units with migration aliasing`, `C) provisional semantics with explicit runtime guard-only policy` | `pending` | `pending` | `pending` | downstream consumer trace + units rationale + guard thresholds | finalize field semantics and guard requirements in contract + tests | `pending` |
| `W4DR-007` | `gwcoeff.txt` missing/default policy and present-file parse-failure policy. | `GWCOEFF-GAP-003`, `GWCOEFF-GAP-004` | `A) explicit defaults on absence; strict fail on malformed present file`, `B) absence is typed optional branch with no implicit defaults`, `C) compat defaults + strict explicit values required` | `pending` | `pending` | `pending` | absence/present-malformed branch evidence with fixture coverage | ratify default publication table and failure semantics | `pending` |
| `W4DR-008` | Namespace separation for similarly named coefficients across `gwcoeff.txt` and `chan.inp`. | `GWCOEFF-GAP-002` | `A) hard namespace separation with explicit alias map`, `B) shared namespace with contextual disambiguation`, `C) deprecate one naming surface` | `pending` | `pending` | `pending` | symbol table comparison + alias mapping proposal | add namespace guard tests and alias policy text | `pending` |
| `W4DR-009` | `phosphorus.txt` concentration-range policy and applicability scope (hillslope-only vs watershed-coupled). | `PHOS-GAP-002`, `PHOS-GAP-003` | `A) bounded ranges + hillslope-only scope`, `B) bounded ranges + watershed-coupled scope`, `C) non-negative-only with deferred upper bounds` | `pending` | `pending` | `pending` | literature/usersum/legacy evidence + routing applicability evidence | ratify range checks and scope semantics in parser + downstream contracts | `pending` |
| `W4DR-010` | `tcr.txt` bounds/default governance and producer interoperability policy (including blank/newline sidecar). | `TCR-GAP-004`, `TCR-GAP-005`, `TCR-GAP-003` | `A) strict canonical bounds; compat producer-edge acceptance`, `B) strict + compat both enforce bounds`, `C) retain legacy permissiveness with warnings` | `pending` | `pending` | `pending` | cross-repo producer fixtures + bound/default matrix | finalize bound/default rules and edge-case handling IDs | `pending` |
| `W4DR-011` | `lcwb.txt` active consumer authority for `lcwbflg` semantics. | `LCWB-GAP-002` | `A) current-source consumer authority`, `B) historical snapshot authority`, `C) dual authority with explicit precedence` | `pending` | `pending` | `pending` | consumer-source traceability + snapshot reconciliation note | ratify authoritative consumer and close provisional policy marker | `pending` |
| `W4DR-012` | `tc.txt` / `tc_out.txt` boundary: parser-contract vs output-contract authority for `tc_out.txt` row grammar. | `TC-GAP-003` | `A) parser contract owns tc_out row grammar`, `B) output contract owns row grammar; parser owns sentinel only`, `C) shared contract with explicit ownership split` | `pending` | `pending` | `pending` | contract-boundary matrix + current producer/consumer evidence | update parser/output contract ownership and remove ambiguity | `pending` |

## Ratification Logging Rules

1. Every ratified decision must include a concrete `selected_option` and UTC decision date.
2. Every ratified decision must cite direct evidence paths used to choose the option.
3. Every ratified decision must create at least one downstream implementation action.
4. Decisions without evidence citation remain `pending` and block kickoff if marked required.
