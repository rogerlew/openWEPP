# science-obligations

Read only sections triggered by the assigned mechanism or package type. Their explicit mandatory references remain binding. Relocated obligations, no waiver.

## Science Implementation And Calibration Readiness

- ADR-0042 distinguishes science authority, data authority, and calibration
  readiness. When authoritative process science exists and is in scope,
  implement it even if observations are insufficient for unique calibration.
- Data limitations constrain empirical calibration, identifiability,
  validation, uncertainty, and transferability claims. They do not authorize
  proxy physics, silent defaults, invented physiological bounds, or an
  implementation `HOLD`.
- Parameterized-science packages must declare intent before edits:
  `implementation`, `calibration-readiness`, `empirical-calibration`,
  `independent-validation`, or an explicit combination.
- When empirical calibration cannot close for lack of suitable data, continue
  through applicable readiness work: typed/enumerable parameters, a
  unit/scale-defined observation operator, deterministic candidate execution
  and objective reconstruction, sensitivity/identifiability analysis,
  boundary/failure/equifinality reporting, and synthetic recovery where
  structurally meaningful.
- Synthetic recovery proves only that suitable information can pass through
  the implementation and calibration machinery. It is not empirical
  calibration, real-world identification, external validation, or
  transferability evidence.
- Values or bounds introduced only to execute a demonstration must be labeled
  `ASSUMED_FOR_EXECUTION`; never label them observations, probability priors,
  physiological bounds, or calibrated results.
- Report three orthogonal fields in package.md or a necessary machine-readable
  calibration-readiness matrix:
  `science_implementation_status` (`IMPLEMENTED`, `NOT_IMPLEMENTED`,
  `AUTHORITY_MISSING`), `calibration_evidence_status`
  (`EMPIRICALLY_CALIBRATED`, `CALIBRATION_READY_DATA_LIMITED`,
  `NOT_CALIBRATION_READY`, `NOT_APPLICABLE`), and
  `identifiability_status` (`IDENTIFIED`, `PARTIALLY_IDENTIFIABLE`,
  `NONIDENTIFIABLE`, `NOT_ASSESSED`, `NOT_APPLICABLE`).
- The readiness matrix must disposition every obligation named by
  `docs/specifications/science-contract-spec.md` as `PASS`, `BLOCKED`, or
  `NOT_APPLICABLE`, with evidence path and rationale. A required current-scope
  `BLOCKED` row forces `NOT_CALIBRATION_READY` and package `HOLD`.
- Assign measured observations prospectively to `CALIBRATION`,
  `INDEPENDENT_VALIDATION`, or `DIAGNOSTIC_ONLY`. Do not reuse calibration data
  as independent validation; a reviewed exception forfeits independence.
- Missing, sparse, correlated, interval-censored, scale-mismatched, or
  non-identifying data alone is not a legitimate hold boundary when
  authoritative implementation and in-scope readiness work remain possible.
- ADR-0024 and ADR-0028 authority-admission routes remain available. Hold for
  missing science authority only when no applicable route has succeeded.


## Consumer-Path Closure Rule
- A package that claims `endpoint`, `direct`, `cutover`, `publication`,
  `ready`, `activation`, or equivalent production-readiness language must prove
  the real downstream consumer reads the new path.
- Producer-only, skeleton-only, counter-only, shadow-only, and
  direct-runtime-internal evidence is iteration evidence. It cannot close a
  consumer-facing gate unless the package is explicitly characterization-only
  and makes no readiness or cutover claim.
- Required consumer-path evidence includes a current package artifact naming:
  producer source, in-memory state/frame object, runner handoff, downstream
  consumer call site, output or API surface, and the negative proof that the old
  compatibility path is not used for that claim.
- Historical direct skeleton/shadow transition modes such as
  `DirectSkeletonNoop`, `DirectSkeletonShadowOnly`, and
  `DirectPublicationFrameCutover` are deleted runtime selections. Do not revive
  them for new evidence; use production direct execution or an explicitly named
  diagnostic harness instead.
- Before closure, run a "what still reads the old path?" check over the named
  downstream consumers. If any current-scope consumer still reads compatibility
  state, runtime symbols, writeback payloads, stale logical state, or a wrapper
  around those structures, the package must close in `HOLD` or continue until
  the consumer is moved.
- If that check exposes a blocker outside the package envelope, record the
  concrete blocker, owner, consumer-path proof and first actionable correction
  in package.md. Link an existing follow-on if present; a new package is needed
  when that follow-on is authorized, not just to disposition the current finding.


## Conservation / Publication Acceptance Rule
- For packages that create, correct, or aggregate conservation-sensitive output
  surfaces (water, sediment, energy, mass, routed runoff, or closure ledgers),
  author an operand-lineage table before production edits. Record field name,
  units, normalization/denominator, area or volume basis, source authority, and
  whether each operand is authoritative or diagnostic.
- Regression fixtures must separate every plausible alias that could mask a
  wrong formula. The expected value must differ from rejected candidates such as
  adjacent diagnostic columns, publication areas, internal state areas, per-OFE
  sums, and legacy/interchange aliases when those candidates are in scope.
- Exact self-consistency checks and one-sided bounds are sanity evidence only.
  They cannot close a conservation/output acceptance gate by themselves.
- Acceptance must include independent reconstruction from produced outputs and
  a real closure or magnitude audit on the target fixture/cohort. Include
  two-sided magnitude/ratio checks when a physical range is known, and anchor
  checks for protected output surfaces.
- Reviews and verification must explicitly check anti-tautology: the gate must
  not restate the producer formula with the same operands, and metadata/schema
  descriptions must match the accepted operand lineage.
