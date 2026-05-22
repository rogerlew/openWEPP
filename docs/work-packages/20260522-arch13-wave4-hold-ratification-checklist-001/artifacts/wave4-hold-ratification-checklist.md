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
| `W4DR-001` | Source-authority policy for sidecars lacking dedicated `usersum2024` format tables: ratify legacy/static code provenance as normative? | `TC-GAP-001`, `TCR-GAP-001`, `GWCOEFF-GAP-001`, `PHOS-GAP-001`, `LCWB-GAP-001` | `A) legacy/static provenance ratified as normative until superseded`, `B) provisional-only (no normative authority)`, `C) mixed by file with explicit per-file policy` | `A) legacy/static provenance ratified as normative until superseded` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` evidence anchors `E-WF-*` + `E-SPEC-*` in linked contracts | Update source-authority language and close linked `*-GAP-001` entries with W4 linkage. | `ratified` |
| `W4DR-002` | Strict vs compatibility open-error policy for sentinel/optional sidecars. | `TC-GAP-002`, `TCR-GAP-002`, `GWCOEFF-GAP-004` | `A) strict hard-fail; compat collapse-with-warning`, `B) strict and compat both hard-fail`, `C) strict and compat both collapse-with-warning` | `A) strict hard-fail; compat collapse-with-warning` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` strict/compat branch matrices in `SC-INFILE-TC-001`, `SC-INFILE-TCR-001`, `SC-INFILE-GWCOEFF-001`, `SC-INFILE-LCWB-001` | Standardize non-ENOENT strict hard-fail and compatibility collapse-with-warning semantics and IDs. | `ratified` |
| `W4DR-003` | Program ownership boundary: true parser inputs vs derived compatibility/output-control flags. | `CHANINP-GAP-004`, `LCWB-GAP-004`, `TC-GAP-003`, `LCWB-GAP-003` | `A) parser owns only file input + derived provenance flags`, `B) parser also owns output-control semantics`, `C) split by subsystem with explicit boundary contract` | `C) split by subsystem with explicit boundary contract` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` ownership tables in linked contracts + parser survey | Parser owns sentinel/file parse + provenance; output contracts own row/output grammar and runtime output-control semantics. | `ratified` |
| `W4DR-004` | `chan.inp` `ichout` domain policy: legacy/usersum `0..3` vs `wepppy` subset `{1,3}`. | `CHANINP-GAP-003` | `A) canonical domain 0..3`, `B) constrained domain {1,3}`, `C) strict 0..3 with compatibility normalization to {1,3}` | `C) strict 0..3 with compatibility normalization to {1,3}` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` `SC-INFILE-CHANINP-001` + `wepppy` producer constraints | Preserve canonical/legacy strict domain while retaining explicit compatibility interoperability branch. | `ratified` |
| `W4DR-005` | `chan.inp` `dtchr` normalization/error-path semantics for legacy ambiguous initialization paths. | `CHANINP-GAP-002` | `A) strict reject ambiguous paths; compat normalize`, `B) strict normalize with warning`, `C) preserve raw value and defer normalization downstream` | `A) strict reject ambiguous paths; compat normalize` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` legacy normalization behavior in `E-WF-CHN-01` + contract guard map | Enforce deterministic normalization with explicit strict failure and compatibility warning path; add fixture obligations. | `ratified` |
| `W4DR-006` | `chan.inp` `cbase` semantics: downstream meaning/units and mandatory guards. | `CHANINP-GAP-001` | `A) preserve current unit model with additional range guards`, `B) redefine/rename units with migration aliasing`, `C) provisional semantics with explicit runtime guard-only policy` | `A) preserve current unit model with additional range guards` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` canonical field table + downstream routing references in `SC-INFILE-CHANINP-001` | Retain canonical `cbase` semantics/units, require explicit finite/non-negative guard and consumer-closure fixtures. | `ratified` |
| `W4DR-007` | `gwcoeff.txt` missing/default policy and present-file parse-failure policy. | `GWCOEFF-GAP-003`, `GWCOEFF-GAP-004` | `A) explicit defaults on absence; strict fail on malformed present file`, `B) absence is typed optional branch with no implicit defaults`, `C) compat defaults + strict explicit values required` | `B) absence is typed optional branch with no implicit defaults` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` optional-absence branch semantics in `SC-INFILE-GWCOEFF-001` + legacy branch evidence | Ratify explicit optional absence (`lr_bf=0`) without implicit coefficient defaults; strict malformed present-file remains typed failure. | `ratified` |
| `W4DR-008` | Namespace separation for similarly named coefficients across `gwcoeff.txt` and `chan.inp`. | `GWCOEFF-GAP-002` | `A) hard namespace separation with explicit alias map`, `B) shared namespace with contextual disambiguation`, `C) deprecate one naming surface` | `A) hard namespace separation with explicit alias map` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` `GW-E-005`/`G-GW-008` and `cbase` mapping evidence in linked contracts | Lock namespace-separation guard tests and alias-map documentation across both contracts. | `ratified` |
| `W4DR-009` | `phosphorus.txt` concentration-range policy and applicability scope (hillslope-only vs watershed-coupled). | `PHOS-GAP-002`, `PHOS-GAP-003` | `A) bounded ranges + hillslope-only scope`, `B) bounded ranges + watershed-coupled scope`, `C) non-negative-only with deferred upper bounds` | `B) bounded ranges + watershed-coupled scope` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` watershed routing evidence (`E-WF-PHOS-01`) + parser contract fanout surfaces | Ratify watershed-coupled semantics and require explicit bounded-range governance beyond non-negative checks via follow-on guard table. | `ratified` |
| `W4DR-010` | `tcr.txt` bounds/default governance and producer interoperability policy (including blank/newline sidecar). | `TCR-GAP-004`, `TCR-GAP-005`, `TCR-GAP-003` | `A) strict canonical bounds; compat producer-edge acceptance`, `B) strict + compat both enforce bounds`, `C) retain legacy permissiveness with warnings` | `A) strict canonical bounds; compat producer-edge acceptance` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` `SC-INFILE-TCR-001` gap/guard evidence + producer-path references in `E-WP-TCR-01` | Preserve strict correctness guardrails while keeping explicit compatibility branch for producer newline/blank edge cases. | `ratified` |
| `W4DR-011` | `lcwb.txt` active consumer authority for `lcwbflg` semantics. | `LCWB-GAP-002` | `A) current-source consumer authority`, `B) historical snapshot authority`, `C) dual authority with explicit precedence` | `A) current-source consumer authority` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` current-source contracts + runtime/output pipeline references (`E-WP-LCWB-01`) | Ratify current-source authority with historical snapshot retained as provenance evidence only. | `ratified` |
| `W4DR-012` | `tc.txt` / `tc_out.txt` boundary: parser-contract vs output-contract authority for `tc_out.txt` row grammar. | `TC-GAP-003` | `A) parser contract owns tc_out row grammar`, `B) output contract owns row grammar; parser owns sentinel only`, `C) shared contract with explicit ownership split` | `B) output contract owns row grammar; parser owns sentinel only` | `Roger Lew + Codex` | `2026-05-22` | `[DIRECT]` sentinel scope in `SC-INFILE-TC-001` + output lifecycle references (`E-WP-TC-01`) | Keep parser contract scoped to sentinel mode/provenance; require output contract to govern `tc_out` row grammar. | `ratified` |

## Ratification Logging Rules

1. Every ratified decision must include a concrete `selected_option` and UTC decision date.
2. Every ratified decision must cite direct evidence paths used to choose the option.
3. Every ratified decision must create at least one downstream implementation action.
4. Decisions without evidence citation remain `pending` and block kickoff if marked required.
