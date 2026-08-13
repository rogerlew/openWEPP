# Review Agent A

Status: `NO-GO / material findings unresolved`

Evidence mode: `Static + Ran`

Review snapshot: worktree based on commit
`60ad5c5a9a499f94f3a9fe63cf10521893242c44`, including V3 definition SHA-256
`6bee2a26fac1d6825a4ae7d1f3df4357cb1cf62d88a73263245f94c15379bae9`
and fixture SHA-256
`bf0edfa96bef293fb2551895a40b0f17501dbf89ff525e7355d81e85877e0447`.

## Scope and evidence

Static review covered `SC-VEGETATION-001@7`, both V3 definition copies, the
independent calculator and committed vectors, package authority/provenance/unit/
schema ledgers, the imported V2 numerical contract, and the V3 authority tests.
Primary CLM5 technical-note pages were checked for the `L+S` coordinate and
area-weighted optics, canopy incident wind identity, common-root/height/gravity
hydraulics, maximum-transpiration coupling, and Rd response constants.

Ran:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick -E
  'test(v3_potential_pass_authority_is_digest_bound_and_prior_models_are_immutable)
  or test(v3_vectors_close_radiation_hydraulics_respiration_and_failure_payloads)
  or test(v2_tile_liquid_authority_is_digest_bound_and_v1_is_historical)'`:
  PASS, 3 tests.
- Python syntax compilation of the V3 calculator: PASS.
- SHA-256 and byte comparison: V1 and V2 retain their protected digests; the
  two V3 definition copies are byte-identical.

Passing focused tests prove current byte/digest consistency. They do not cure
the constitutive and evidence defects below.

## Material findings

### A-CRITICAL-001 — The accepted Stage-A beta residual is not uniquely specified and the oracle selects an unadmitted equation

Locations:

- `SC-VEGETATION-001.md`, V3 clauses 8--9 (approximately lines 750--773).
- `artifacts/reference_calculator.py`, `hydraulic_fluxes()` (lines 513--542).
- `artifacts/openwepp_c3_woody_v3_vectors.json`,
  `hydraulic_potential_pass.accepted_uncapped_stage_a`.

The contract requires a common accepted `beta_hyd`, separate gas/energy-to-q1
equalities, and imports the prior `E_class=E_class,max*v(psi_class)` hydraulic
demand authority. It does not state the additional exact equation that selects
one scalar beta while preserving both class loss identities. The calculator
invents

```text
beta = (Emax_sun*v_sun + Emax_shade*v_shade) / (Emax_sun + Emax_shade)
```

as its fifth residual. That equation is absent from V3 and CLM provenance, and
the accepted fixture does not satisfy the imported class loss equations. For
example, accepted `q1_sun` is about `9.58e-5`, whereas
`Emax_sun*v_sun` is about `4.69e-5` in the same fixture. Consequently the V3
residual is not implementation-reproducible and the oracle does not represent
the stated accepted model.

Required correction: canonically select and state the complete independent
residual set, including the exact relationship among class hydraulic loss,
class stressed conductance, common or class-specific beta, and re-solved
gas/energy flux. Demonstrate equation count/unknown count and require every
selected class and total equality in an independently regenerated fixture.
Do not retain the weighted-beta equation unless it is explicitly admitted with
provenance and the conflicting class-loss text is dispositioned.

### A-CRITICAL-002 — The claimed E11--E15 fixture is a reduced demonstration solver, not the digest-imported coupled system

Locations:

- `artifacts/reference_calculator.py`, `class_gas_energy_response()` and
  `coupled_solve()` (lines 396--488 and 553--627).
- `artifacts/openwepp_c3_woody_v3_vectors.json`,
  `hydraulic_potential_pass`.
- `tests/integration/vegetation_boundary_authority_contract.rs`,
  `assert_v3_hydraulics_and_migration()` (approximately lines 890--945).
- Imported
  `20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/
  numerical-solver-and-convergence-contract.md`.

The calculator fixes canopy-air temperature and humidity instead of solving
their residual nodes, omits wet-surface and dry-stem energy nodes, uses one
fixed boundary conductance, does not derive surface VPD or `cs`, replaces the
admitted Medlyn equation with a linear `medlyn_gain*An`, and solves only two
leaf energy balances nested inside a five-variable hydraulic/beta Newton solve.
It also uses bisection rather than Brent--Dekker for `ci`, forward rather than
centered finite differences, different step sizes, different pivot threshold,
and different iteration/halving limits from the imported numerical contract.
The authority test checks the reduced hydraulic closures but not the missing
E11--E13 nodes or canonical numerical behavior.

Required correction: regenerate the potential family with the complete
authority-selected FvCB--Medlyn--`cs`/VPD--leaf/wet/stem/canopy-air energy--
four-node hydraulic residual and the exact imported algorithms, brackets,
tolerances, steps, pivots, and limits. If a reduced analytic fixture remains,
label it only as component evidence; it cannot be the package's independent
E11--E15 release vector or satisfy the exit condition.

### A-HIGH-003 — Most required poisons are declarations, not independently evaluated alternatives

Locations:

- `artifacts/reference_calculator.py`, `poison_manifest()` and
  `quantitative_poisons()` (lines 820--886).
- `tests/integration/vegetation_boundary_authority_contract.rs`,
  `assert_v3_respiration_failures_and_poisons()` (approximately lines
  1003--1013).

Only seven alternatives have quantitative operands/results. The remaining
manifest entries all carry the same string
`reject_or_numerically_distinct_from_accepted_vector`; the Rust test generally
checks only that seven names exist. Required radiation alternatives such as
leaf-only optics over all plant area, area-only ownership, doubled clumping,
VIS/NIR swap, direct/diffuse swap, zero lower boundary, and direct-summed
reflection are not computed and distinguished. The same gap affects most wind,
hydraulic coupling, and migration poisons.

Required correction: encode every required poison with executable independent
operands and either a typed expected rejection or a numerical result proven
distinct from the accepted vector. Make the authority test reconstruct and
assert each alternative rather than accepting a manifest string.

### A-HIGH-004 — Numerical failure fixtures are hand-authored payload examples and do not prove the frozen failure algorithms or precedence

Locations:

- `artifacts/reference_calculator.py`, `numerical_failure_payload_examples()`
  (lines 644--660).
- `tests/integration/vegetation_boundary_authority_contract.rs`, failure checks
  (approximately lines 970--1002).

The sun-ci, shade-ci, canopy-energy, outer-coupling, and capped payloads are
literal examples rather than outputs of failing oracle cases. Tests assert
field presence and solve-name presence, not that the named bracket, residual,
iteration, pivot, bound, or precedence arose from the frozen algorithm. Even
the reduced solver's iteration-limit diagnostics place raw residuals in a field
named `residual_norms`. This cannot establish deterministic failure behavior.

Required correction: generate independent failing cases through each exact
solver, bind normalized residual construction and optional-field semantics,
and add competing-failure vectors that prove identity/schema, domain, bracket,
singular-pivot, and iteration-limit precedence. Assert finite payload values
and exact solve/pass/cap identities.

### A-HIGH-005 — The Atkin provenance is not immutably acquired or directly resolvable from the cited URL

Locations:

- `SC-VEGETATION-001.md`, `REF-VEGETATION-034` (approximately line 143) and
  V3 clauses 11--12.
- `artifacts/reference-acquisition-ledger.md`.

`REF-VEGETATION-034` links a mutable CTSM master Photosynthesis page. That page
supports the Rd activation/deactivation constants, but the review record does
not identify immutable acquired bytes or a directly resolvable chapter/equation
for the selected Atkin intercept, `0.2061`, and `-0.0402` relation. The ledger's
“reviewed Chapters 17/19” assertion has no repository path, revision, or digest.
An implementer therefore cannot independently recover the precise source used
for this new constitutive identity.

Required correction: bind an immutable reference revision or acquired local
artifact with path and SHA-256, identify the exact chapter/equation/table for
the Atkin relation and source units, and distinguish the directly sourced
terms from the openWEPP unit/conversion and nonpositive-branch selections.

## Non-blocking positive observations

- The V3 radiation clauses unambiguously define transport over conditional
  `L+S`, area-weighted leaf/stem optics, one application of clumping,
  leaf-only sunlit area, physical absorption ownership, exact zero-direct and
  zero-area branches, whole-column lower-boundary coupling, and no stem PAR.
- The radiation oracle uses one complete two-rank boundary solve, independently
  closes VIS/NIR direct/diffuse energy, exercises the exact resonance integral,
  and returns leaf/stem owner sums consistent with plant absorption.
- The neutral `u_star` identity, semantic surface winds, height/gravity path,
  common-root schema, and bitwise-only V2 migration are stated clearly.
- V1 and V2 model-definition bytes remain unchanged at their protected
  digests, and the public Rust implementation is outside this package.

## Exit-criteria assessment

| Criterion | Verdict |
| --- | --- |
| V7 mixed radiation/wind/schema selections transcribed | PASS (Static) |
| V1/V2 immutability | PASS (Ran) |
| V3 definition and fixture bytes internally digest-bound | PASS (Ran) |
| Accepted potential residual unambiguous/reproducible | FAIL (`A-CRITICAL-001`) |
| Independent exact E11--E15 vectors | FAIL (`A-CRITICAL-002`) |
| All named poison alternatives pass | FAIL (`A-HIGH-003`) |
| Numerical failure behavior independently evidenced | FAIL (`A-HIGH-004`) |
| Constitutive provenance immutably recoverable | FAIL (`A-HIGH-005`) |
| Package release GO | **NO-GO** |

The package must not promote `SC-VEGETATION-001@7` to `approved/active`, archive
the kickoff prompt, or release V3 implementation authority until every finding
above is accepted/rejected with evidence and all accepted corrections pass a
fresh independent review against stable bytes.

## Final Remediation Rereview — 2026-08-12

Status: `NO-GO / material findings remain`

Evidence mode: `Static + Ran`

This is an additive rereview. The original findings and NO-GO above remain
immutable review history. The rereview assessed the remediated exact worktree
with V3 definition SHA-256
`563d6f0758e5a16c19acba68ef29fe5771fe9d2ba1f80ebf8471a2c2a763d7a3`,
oracle SHA-256
`50a6366ec72383f94ff7c806cf4d08ad5f5564ac345a825fa2bdf2550ac0645e`,
and fixture SHA-256
`cccc02b0ba835ae4e9788acfb674fa055d28fbf4f7106ee46243bb0c113931b4`.

Ran:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 17 tests.
- `.venv/bin/python .../artifacts/reference_calculator.py`: PASS; deterministic
  40,887-byte fixture with all eight self-checks true.
- SHA-256 reconstruction for oracle/fixture/definition: PASS.
- Fresh acquisition of pinned CTSM
  `PhotosynthesisMod.F90`: PASS; SHA-256
  `e4c9ad718209af44fcfdfc1d591bd2729d345f9e422cf5d9c8a889525d6a1cdf`
  and cited Atkin lines verified.
- `git diff --check`: PASS.
- Protected V1/V2 definition digests remain respectively
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`
  and `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.

Passing commands establish deterministic current bytes, not scientific
completeness where the tests omit a canonical operand or equation.

### Original-finding disposition

#### A-CRITICAL-001 — RESOLVED

V7 now selects distinct `beta_sun` and `beta_shade`, six named unknowns, and
six independent residuals. The accepted fixture satisfies both
`E_gas=Emax*v(psi)` class equalities, both `E_gas=q1` equalities, and both
downstream continuity equations. Persisted `beta_hyd` is explicitly an
Emax-weighted diagnostic/warm start after convergence, with an exact zero-demand
branch; it is no longer an oracle-invented convergence equation.

#### A-CRITICAL-002 — NOT RESOLVED

The replacement oracle now solves the missing wet-surface, dry-stem,
canopy-air heat/vapor, class leaf-energy, `ci`, and class-beta nodes, and its
Brent/centered-difference/pivot/halving mechanics are materially closer to the
frozen numerical contract. It still does not execute the exact imported
E07--E15 system:

1. `reference_calculator.py:513--517` replaces the admitted peaked Vcmax and
   Jmax responses with one-factor Arrhenius expressions using hard-coded
   `60000` and `30000`. Lines 525--534 likewise hard-code `Gamma`, `Kc`, `Ko`,
   and TPU instead of consuming the digest-bound E07--E10 capacities and
   temperature-response operands. The imported parameter manifest requires
   Vcmax/Jmax activation, deactivation, and entropy operands plus Kc/Ko/Gamma
   responses. A coupled potential vector built on different biochemical
   kernels is not an exact E11--E15 acceptance vector.
2. `reference_calculator.py:756--763` replaces canonical E14 soil-to-root
   construction with caller-supplied `soil_conductance` and
   `root_conductance*root_area`. It omits
   `kr=(k3max/z3)*v(psi_soil;p50_root)`, `RAI=(LAI+SAI)*r_i*f_root_leaf`,
   `ks=ksoil/dxroot`, and their explicit path-length/vulnerability operands.
   This directly conflicts with the imported equation authority ledger E14 and
   V7's statement that the existing explicit soil-to-root path authority is
   unchanged.
3. The fixture supplies `rah_s_m=42` and `raw_s_m=48` independently of its
   reference wind and roughness operands. It therefore does not independently
   demonstrate the admitted neutral aerodynamic derivation used by the
   canopy-air residual, even though it derives the three local surface
   conductances from `u_star`.

The focused Rust test checks the resulting six hydraulic residual identities
but never reconstructs the omitted biochemical responses, root-path
conductances, RAI, or aerodynamic resistances. The release definition therefore
overstates the fixture as exact complete E11--E15 evidence.

Required correction: use the complete digest-imported E07--E15 operand set and
equations in the independent calculator; expose every intermediate temperature
response, `kr`, `ks`, `RAI`, `k3`, path, and aerodynamic-resistance operand;
and make Rust independently reconstruct them. Regenerate and rebind all fixture,
generator, definition, and section digests afterward.

#### A-HIGH-003 — NOT RESOLVED

The new poison object improves on name-only declarations: all 34 entries carry
`executed=true`, 25 contain a distinct numeric pair, and nine contain a typed
expected error. However, `typed()` merely constructs an expected-error object;
it does not call a validator. Several numerical entries compare one derived
operand with hand arithmetic rather than execute the rejected alternative
through the owning calculation. For example, the direct/diffuse swap compares
two already-accepted top-reflection values, and the leaf/stem optics alternatives
compare only effective rho rather than alternate column solutions. In addition,
the package test-vector ledger and V7 obligations name the zero-lower-boundary
and direct-summed-reflection radiation poisons, but neither exists in the 34-entry
manifest. The Rust test asserts only seven names plus the common `executed`
flag.

Required correction: execute every rejected alternative through its owning
oracle/validator, include the two missing whole-column poisons, and have the
Rust authority test enumerate and independently validate the complete required
inventory and expected disposition.

#### A-HIGH-004 — NOT RESOLVED

The remediation now obtains ci domain/bracket/limit, canopy-energy domain/limit,
outer iteration-limit, and hydraulic singular failures from real function
calls, with no candidate or last iterate. That is meaningful progress. It does
not prove the canonical precedence rule. There are no competing-condition
vectors that demonstrate identity/schema before domain, then domain before
bracket, bracket before singular pivot, and singular pivot before iteration
limit. Moreover, every executed ci failure records `iterations=0` and an empty
`residual_norms` list, including iteration-limit and endpoint-bracket cases
where evaluated residual and completed-evaluation evidence is available. The
Rust test checks field presence but not residual applicability, finiteness,
iteration accuracy, or precedence.

Required correction: create competing-failure cases for every precedence edge;
preserve every available labeled normalized residual and actual completed
iteration/evaluation count; and assert optional-field applicability, numeric
finiteness, and precedence in the independent Rust test.

#### A-HIGH-005 — RESOLVED

The Atkin selection is now bound to immutable CTSM commit
`8e1309ab0db671d884b80746cbae9bbaafbe78a7`, exact source path, source SHA-256,
and line ranges. Fresh acquisition reproduced the digest and confirmed leaf N
in `g N m^-2 leaf`, Celsius T10, coefficients `0.2061` and `0.0402`, the
positive-N branch, and that `lmr25top` is already a photosynthesis-rate operand.
V7 correctly removed the erroneous g-C/day conversion and explicitly labels
its stricter nonpositive posture as openWEPP authority.

### Other authority-family assessment

- **Radiation:** PASS. Mixed leaf/stem weighting, `K_eff`, whole-column upward
  coupling, leaf/stem ownership, zero branches, resonance, band/component
  identity, and closure remain reproducible. Poison coverage remains subject
  to `A-HIGH-003`.
- **Wind surface identity:** PASS for `u_star` and distinct semantic
  leaf/wet/stem conductances. Complete canopy-air aerodynamic evidence remains
  part of `A-CRITICAL-002`.
- **Height/gravity/common root/schema/migration:** PASS for the V3 delta.
  Complete layer E14 construction remains part of `A-CRITICAL-002`.
- **Class-resolved potential semantics:** PASS for equation count and accepted
  class/total identities, subject to the incorrect imported component kernels
  in `A-CRITICAL-002`.
- **Respiration and provenance:** PASS.
- **Failure DTO schema:** PASS statically; executed precedence evidence remains
  incomplete under `A-HIGH-004`.
- **Definition/fixture/oracle independence:** PASS for deterministic byte and
  digest binding and absence of Rust-generated expected values. Independence
  does not make substitute equations authoritative.
- **V1/V2 immutability:** PASS.

### Final rereview verdict

| Finding | Final status |
| --- | --- |
| `A-CRITICAL-001` | RESOLVED |
| `A-CRITICAL-002` | **UNRESOLVED** |
| `A-HIGH-003` | **UNRESOLVED** |
| `A-HIGH-004` | **UNRESOLVED** |
| `A-HIGH-005` | RESOLVED |
| New material findings | None separate; newly identified exact omissions are within `A-CRITICAL-002` |
| Package release | **NO-GO** |

The package must remain pre-release. `SC-VEGETATION-001@7` must not be promoted
to `approved/active`, and V3 implementation authority must not be released,
until the exact imported component equations and the remaining poison/failure
evidence pass another independent stable-byte rereview.

## Second Stable-Byte Remediation Rereview — 2026-08-12

Status: `GO / no unresolved material finding`

Evidence mode: `Static + Ran`

This is a second additive rereview. The two earlier NO-GO reviews remain
immutable history. This review assessed the current exact worktree bytes with:

- V3 model-definition SHA-256
  `fa6b7fa7c86a059b9d0a46065a23a7e35c2ce749d494e04e7842c0341bd901f0`;
- independent fixture SHA-256
  `ee98dd49b0054e1488aead34ee4eceb49905f0f7978afb6554c7f61f16b894ed`;
- independent generator SHA-256
  `fb08da650b92b58cf34fb609eab6284d45be9a88d9e82b32ba5f3e4cfcb8b905`;
- protected V1 and V2 definition SHA-256 values respectively
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`
  and
  `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.

Ran:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 17 tests.
- `.venv/bin/python .../artifacts/reference_calculator.py`: PASS; reproduced
  the byte-identical 49,960-byte fixture and all eight self-checks were true.
- SHA-256 reconstruction for the V3 definition, fixture, and generator: PASS.
- `git diff --check`: PASS.

### Finding disposition

#### A-CRITICAL-001 — REMAINS RESOLVED

The exact six-unknown/six-residual class-specific beta system remains
canonical and the fixture independently satisfies both class gas/energy-to-q1
equalities, both vulnerability-demand equalities, and both total continuity
equations. The persisted scalar beta remains only the admitted Emax-weighted
warm-start diagnostic with an exact zero-demand branch.

#### A-CRITICAL-002 — RESOLVED

The independent oracle now executes and exposes the complete selected
potential-pass component set that the preceding rereview found missing:

- stable log-domain peaked Vcmax/Jmax temperature responses;
- Arrhenius Kc, Ko, and compensation-point responses;
- parameterized electron transport, Rubisco, electron, TPU, and both
  co-limitation quadratics;
- boundary-layer carbon drawdown, solved surface VPD, Medlyn conductance, and
  bracketed Brent-Dekker `ci` solves from temperature-adjusted compensation
  point to ambient carbon dioxide;
- simultaneous sun-leaf, shade-leaf, wet-surface, dry-stem, canopy-air heat,
  and canopy-air vapor residuals with canonical centered finite differences,
  pivot threshold, backtracking, step tests, and limits;
- neutral `rah` and `raw` derived from the explicit reference-wind and
  momentum/heat/vapor roughness operands rather than independent constants;
- canonical E14 `kr`, `ks`, series `k3`, root-area index, soil vulnerability,
  soil/root paths, gravity, common-root node, and every layer flux; and
- the outer six-variable gas-energy-hydraulic solution with alternate warm
  starts, exact inaccessible/frozen zero branches, typed redistribution
  rejection, and stand-ground request conversion.

The Rust authority test independently reconstructs the aerodynamic
resistances, temperature-response consequences and biochemical identities,
the complete root-path conductance chain, root-area index, request conversion,
and all six coupled residual identities. The released vector is therefore no
longer a reduced demonstration solver.

#### A-HIGH-003 — RESOLVED

The fixture contains 40 executed release poisons: 30 quantitatively distinct
alternatives and ten typed owning-validator rejections. The inventory includes
the previously absent whole-column zero-lower-boundary and direct-summed lower
reflection alternatives, as well as the added root-weighted migration,
legacy-respiration-input, wrong-Rd-response, and sun/shade-respiration cases.
The Rust authority test enumerates all 40 exact names, requires every case to
be executed, and requires every typed case to identify its owning validator.
The oracle itself fails generation if a quantitative alternative does not
discriminate or a typed poison does not exercise the expected rejection.

#### A-HIGH-004 — RESOLVED

The oracle now executes ci domain, bracket, and evaluation-limit failures for
both classes; canopy-energy domain and iteration-limit failures; a hydraulic
singular-pivot failure; and an outer-coupling iteration-limit failure. Available
endpoint residuals are labeled and normalized, completed evaluation/iteration
counts are retained, unavailable nonfinite domain residuals are omitted rather
than serialized as NaN, and every payload publishes neither candidate nor last
iterate. All payload numeric values are finite.

The executed competing-condition family proves the canonical precedence order
`identity_schema -> domain -> bracket -> singular -> iteration` for every
successive competing set. The Rust test checks the exact order and selections,
all required solve identities and fields, finite JSON numerics, bracket and
residual applicability, completed ci evaluation counts, and null candidate
posture.

#### A-HIGH-005 — REMAINS RESOLVED

The immutable CTSM source commit, path, byte digest, line ranges, source units,
and openWEPP-specific conversion/nonpositive selections remain bound in the
authority bytes. No mutable-only Atkin provenance was reintroduced.

### Authority-family conclusion

Mixed leaf/stem whole-column radiation, neutral local wind, complete selected
E07--E15 potential coupling, height/gravity/common-root hydraulics, strict V3
state and V2 migration, class-resolved respiration, deterministic numerical
failure behavior, and the independent fixtures are implementation-ready at
the reviewed digests. V1 and V2 remain immutable historical authorities.

| Finding | Second rereview status |
| --- | --- |
| `A-CRITICAL-001` | RESOLVED |
| `A-CRITICAL-002` | RESOLVED |
| `A-HIGH-003` | RESOLVED |
| `A-HIGH-004` | RESOLVED |
| `A-HIGH-005` | RESOLVED |
| New material findings | None |
| Science review release | **GO** |

This science review authorizes progression to the remaining independent review
and terminal package gates at these exact bytes. It does not itself activate a
runtime selector or claim completion of the separate Rust implementation
package.

## Final Immutable-Constant Delta Rereview — 2026-08-12

Status: `GO / prior GO remains valid`

Evidence mode: `Static + Ran`

This additive delta review follows Review B's correction of two immutable
imported energy constants. The exact reviewed bindings are:

- V3 model-definition SHA-256
  `8a718b2f51fe8006221c2b4276c1f20776d86f3949fc4c8a91c5937abd0a4e61`;
- independent fixture SHA-256
  `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`;
- independent generator SHA-256
  `11dcdc67a2a3b82ee269f9d908b37046f928cf85910522c4328974245662b3df`.

Static inspection confirms that the executable oracle and committed fixture
use the immutable imported values `cp_air=1004.64 J kg^-1 K^-1` and
`lambda_vap=2,501,000 J kg^-1`. The Rust authority test explicitly asserts both
operands before reconstructing the coupled vector. No stale `1005.0` or
`2450000` literal remains in the live contract, oracle, fixture, definition, or
authority test. Their only package occurrences are preserved historical Review
B and gate-result text documenting the rejected predecessor bytes.

Ran:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 17 tests.
- `.venv/bin/python .../artifacts/reference_calculator.py`: PASS; deterministic
  regeneration retained fixture SHA-256
  `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`.
- SHA-256 reconstruction for definition, fixture, and generator: PASS.
- `git diff --check`: PASS.

The correction changes the accepted numerical vector consistently without
altering the selected equations, state schema, poison coverage, failure
semantics, or prior finding dispositions. All `A-CRITICAL` and `A-HIGH`
findings remain resolved, no new material finding was identified, and Review
Agent A's science-release verdict remains **GO** at the exact digests above.

## Terminal-Hygiene Byte-Only Delta Rereview — 2026-08-12

Status: `GO / prior scientific and heavy evidence reusable`

Evidence mode: `Static + Ran`

The only scientific-artifact delta since the preceding GO is removal of one
whitespace-only line from the independent generator and the consequent update
of the generator digest bound by both V3 definition copies. Exact current
identities are:

- both byte-identical V3 definition copies:
  `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`;
- independent fixture, unchanged:
  `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`;
- whitespace-normalized generator:
  `7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a`.

Static inspection confirms that canonical contract-section digests, equations,
operands, computed values, fixture bytes, poison vectors, and numerical failure
evidence are unchanged. The two definition copies are byte-identical and bind
the current generator and unchanged fixture digests. The authority test binds
the same exact current identities.

Ran against the final bytes:

- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: PASS, 17 tests.
- `.venv/bin/python .../artifacts/reference_calculator.py`: PASS; regeneration
  retained the byte-identical fixture SHA-256
  `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`.
- Definition-copy byte comparison and SHA-256 reconstruction: PASS.
- `git diff --check`: PASS.

Prior heavy evidence is reusable. The hygiene edit changes no Rust production
or test behavior, contract section, authority equation, fixture value, or
generated expected byte. Its only identity consequence is the generator digest
and the definitions that cryptographically bind it; the focused exact-byte
authority rerun directly covers those invalidated identity surfaces. Repeating
the unchanged full-workspace, Clippy, documentation, and dependency executions
would add no new correctness evidence for this byte-only delta.

No previous finding is reopened, no new material finding exists, and Review
Agent A's final science-release verdict remains **GO** at the exact identities
above.
