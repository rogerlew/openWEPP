# Independent Terminal Review B

Evidence mode: `Static + Ran` (read-only analysis; no model execution).

Reviewer: independent terminal QA reviewer B.

Scope: package plan, frozen authority and version-6 contract, retained-only
consumer and report, EB-04R attempt/protocol/provenance, validation evidence,
roadmap/catalog claims, and exact write set. Reviewer A's terminal artifact was
not read before this review was formed.

Decision: `GO_WITH_AMENDMENTS`.

The authority reconciliation is scientifically and dimensionally correct. The
retained values also support the reported nonpromotion outcome: all 48 cells
complete, the maximum vapor-to-sublimation residual is
`8.109983287707401e-8 kg m^-2`, B is `177/16`, and LS is `180/16`. However,
the retained adjudicator does not yet enforce every identity dependency needed
to make that outcome self-contained and reproducible. The findings below must
be corrected and the analysis-only outputs regenerated before closure.

## Findings

### High — scoring inputs and decision dependencies are not enforced before observation access

`tools/adjudicate_retained.py:79-127` validates the authority seal, retained
attempt inventory, and per-cell output provenance, then loads observations and
scores the cells. It does not compare the current EB-04R tool/protocol,
`crates`/`tests` trees, fixture trees, observation files, or rubric/decision
dependencies with their hashes in `execution-attempt.json`. The before/after
tree checks at `tools/adjudicate_retained.py:81-82,139-163` prove only that the
EB-04R package and retained-output tree did not change *during* this analysis;
they do not establish that the scoring consumer and observation inputs are the
frozen bytes. This is especially important because EB-04R criterion 8 requires
no fixture, observation, rubric, or post-result operator change
(`EB-04R/artifacts/prospective-decision-protocol.md:69-88`), while the inherited
criterion reconstruction derives `no_forcing_or_input_mutation` only from
trace/provenance flags.

Independent review rehashed the current state against the attempt and found no
actual drift: the current EB-04R tool and protocol, 12 fixture trees, 12
observation identities, eight decision dependencies, and both source trees all
match their frozen hashes. Thus this is an admissibility-gate implementation
gap, not evidence that the reported scores changed.

Required amendment: before `score_cell` can run, validate and record all of
those identities against `execution-attempt.json`; include the conjunction in
the population gate and in the independently reconstructed criterion 8. A
mismatch must produce `HOLD_PHYSICAL_OR_PROVENANCE_GATE`. Regenerate the
retained report and summary after adding the check; no simulation rerun is
needed.

### Moderate — the retained physical gate omits EB-04R's stricter vapor-aggregation predicate

The amended contract correctly preserves canonical hourly/daily vapor
aggregation at `1e-9 kg m^-2`, independently from the corrected
vapor-to-sublimation transfer tolerance. EB-04R's own frozen experiment,
however, prospectively imposed a stricter daily vapor-aggregation bound of
`1e-12 kg m^-2` (`EB-04R/artifacts/prospective-decision-protocol.md:29-43`).
The retained consumer imports the EB-04E audit and reduces
`execution_status` without adding an explicit EB-04R `1e-12 kg m^-2` check
(`tools/adjudicate_retained.py:102-108`). The package therefore has not directly
implemented every unchanged EB-04R physical predicate.

Independent reconstruction found a population maximum of
`7.993605777301127e-15 kg m^-2`, so every cell passes the stricter frozen bound
and this omission does not alter the outcome.

Required amendment: retain the canonical `1e-9 kg m^-2` contract statement,
but explicitly apply and publish EB-04R's prospective `1e-12 kg m^-2`
experiment bound in this successor adjudication. Include its maximum and
population conjunction in the report/gate evidence.

### Moderate — terminal self-check evidence is not reproducible as stated

`artifacts/gate-results.md:21` says both package-tool self-checks pass. At the
reviewed terminal state, the retained-only self-check passes, but
`authority_reconciliation.py --self-check` fails because
`tools/authority_reconciliation.py:59-79` searches the now-superseded
version-5 sentence in the amended version-6 contract. The Phase-A freeze hash
does correctly match the committed version-5 contract, and the failure is a
lifecycle-verification defect rather than a challenge to the frozen decision.

Required amendment: either provide a terminal verification mode that validates
the frozen receipt against its sealed pre-amendment input identities, or state
truthfully that the authority self-check ran only before amendment and replace
the terminal check with a receipt/seal verification. Update the gate row and
logs to match what can be rerun at the final state.

### Low — Phase-A reading map conflicts with the enforced four-file whitelist

`artifacts/required-reading-map.md:13-16` labels four entries after the contract
row as Phase-A whitelist material and includes the EB-04E `package.md`, while
`authority-input-manifest.md`, `authority-freeze.json`, and
`authority_reconciliation.py` consistently enforce exactly four total files
and do not include that package. Correct the reading-map row's tier (for
example, pre-scaffold/core context) or remove it from Phase A so the package
does not claim both a four- and five-file result-blind boundary.

## Checks That Pass

- Cross-unit authority: `1e-9 m SWE * 1000 kg m^-3 = 1e-6 kg m^-2` is correct,
  and `SC-SNOWENERGY-001` version 6 keeps transfer closure, vapor aggregation,
  and represented-layer lifecycle as distinct predicates.
- Authority integrity: the frozen receipt hash is
  `20c227029ccc876209cd81cdc830c9c68811307ee055d300836a769aa388798f`;
  the version-6 contract hash and authority seal agree. Dual authority findings
  are incorporated narrowly without changing runtime physics.
- Retained outputs: 48/48 cell records and 288 per-run file identities pass the
  attempt/provenance consumer; reported before/after EB-04R and retained-tree
  hashes are equal. No evidence of a model rerun or prior-package rewrite was
  found.
- Physical values: all reported mass, energy, thermal, layer, selector, sign,
  chronology, and finiteness gates pass. Both the corrected transfer tolerance
  and EB-04R's stricter unaffected aggregation predicate pass.
- Decision reconstruction: the independent reducer agrees on aggregates,
  protected groups, new failures, compensation, all eight booleans, and
  `CLOSE_NONPROMOTION_EMPIRICAL_RULE`. The failed `16 < 16` predicate invokes
  the frozen stop-loss.
- Claim limits: mechanisms remain default-off; warm-maritime conifer transfer
  remains withheld; the documents characterize empirical nonpromotion rather
  than invalid process physics. EB-04R remains an unchanged historical HOLD.
- Write set: no `crates/`, `tests/`, fixture, observation, or prior-package
  edit was found. The EB-04S changes are confined to its package, the canonical
  snow-energy contract, and the declared roadmap/catalog documents. The
  uncommitted EB-04R tree is the separately authorized predecessor increment.
- Validation selection: no Rust regression run is required for this
  documentation-authority and retained-analysis-only increment. Scoped unit
  compliance and Markdown evidence are appropriate, subject to rerunning
  affected documentation checks after the amendments above.

## Disposition

`GO_WITH_AMENDMENTS`.

The scientific conclusion is likely stable because this review independently
confirmed the currently installed frozen identities and the stricter omitted
aggregation bound. Closure still requires the consumer and evidence to enforce
those facts themselves, followed by regenerated analysis-only artifacts,
updated gate evidence, finding disposition, dual terminal verification, and a
truthful final package/roadmap status. No simulation rerun, coefficient change,
observation change, or empirical-rule change is authorized.
