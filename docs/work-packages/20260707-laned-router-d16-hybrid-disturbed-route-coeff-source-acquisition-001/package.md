# LANED Router D16 Hybrid Disturbed Route-Coefficient Source Acquisition

Status: EXECUTED-HOLD-D16-SUITE

## Objective

Implement the Disturbed module as the canonical WEPPcloud producer for native
`ow-lanuse-1` management files carrying Lane-D `routing_coefficients`, then
use those generated native inputs to lift the openWEPP D16 route-coefficient
authority hold.

The package must assign sensible, defensible values for the five static route
coefficients by disturbed class and soil texture in the Disturbed extended
lookup table. It must not infer those values mechanically from legacy WEPP
cropland fields such as row width, ridge spacing, random roughness, residue, or
cover without separate authority.

## Rationale

The preceding package
`20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001` held because
the selected D16 roots have no native `ow-lanuse-1` managements,
`routing_coefficients`, or active openWEPP runfiles, and current openWEPP
authority rejects legacy-field inference for the five static Lane-D route
coefficients.

WEPPpy Disturbed is the correct producer boundary: it already owns disturbed
class, burn severity, soil texture, soil parameterization, and management-file
selection for the affected WEPPcloud interfaces. Its extended lookup table
already merges the per-row key shape with management-file fields:
`sev_enum`, `landuse`, `disturbed_class`, `stext`, `ini.data.*`, and
`plant.data.*`.

This package converts the prior missing-source hold into an explicit
source-acquisition and producer implementation package.

## Required Reading

openWEPP:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/artifacts/worker-handoff.md`

WEPPpy:

- `/home/workdir/wepppy/AGENTS.md`
- `/home/workdir/wepppy/wepppy/nodb/AGENTS.md`
- `/home/workdir/wepppy/wepppy/wepp/management/AGENTS.md`
- `/home/workdir/wepppy/docs/standards/parameterization-adr-standard.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/README.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/ENDUSER.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/data/disturbed_land_soil_lookup.csv`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/data/extended_land_soil_lookup.csv`
- `/home/workdir/wepppy/wepppy/wepp/management/data/`
- `/home/workdir/wepppy/tests/disturbed/PLAN.md`
- `/home/workdir/wepppy/tests/disturbed/test_disturbed_matrix.py`
- `/home/workdir/wepppy/tests/test_2_validate_managements.py`
- `/home/workdir/wepppy/tests/test_managements_module.py`

Coefficient authority:

- Primary papers and reference materials cited by `SC-OFEROUTE-001` for Lane-D
  friction and roughness terms.
- Any operator/source records used to assign route coefficients.
- Internal Disturbed calibration notes only when they are explicitly recorded
  as parameterization authority, not as hidden inference from legacy fields.

## Scope

Included:

- Extend the WEPPpy Disturbed extended lookup schema with additive route
  coefficient columns:
  - `route_skin_friction_coefficient_ko`
  - `route_form_drag_coefficient`
  - `route_roughness_element_height_m`
  - `route_roughness_concentration`
  - `route_vegetation_drag_coefficient`
- Add provenance columns sufficient for review:
  - `route_coeff_source_ref`
  - `route_coeff_authority_class`
  - `route_coeff_confidence`
  - `route_coeff_notes`
- Assign coefficient values for every active Disturbed row needed by selected
  D16 cohort members.
- Add a Disturbed native-management producer that emits `ow-lanuse-1`
  management files with explicit `routing_coefficients` when the target runtime
  is openWEPP native Lane D.
- Preserve legacy WEPP/Fortran management output unless the native producer is
  explicitly selected.
- Update WEPPpy docs and a parameterization ADR for the new route-coefficient
  defaults.
- Amend openWEPP management authority docs if needed to name Disturbed as a
  source-authorized native producer, without turning this into a legacy-field
  bridge.
- Prove openWEPP parses and consumes the generated native files on the active
  Lane-D path.
- Re-run the D16 active input preflight and construct or unblock the executable
  active plain-vs-hybrid cohort evidence.

Excluded:

- D16 default selector promotion unless the current package fully closes input
  authority and all D16 promotion gates are explicitly added and met.
- Any all-lane placeholder such as the H2637 `500.0 0.0 0.0 0.0 0.0` timing
  recipe as production-cohort policy.
- Any mechanical inference from legacy row/ridge/random roughness/residue
  fields.
- Changes to openWEPP friction equations, solver numerics, or hybrid selector
  policy.
- Silent fallback to legacy `.man` files when native route coefficients are
  required.

## Correction Authority Envelope

This package may close the D16 route-coefficient authority hold only by
creating explicit source-authorized input data:

- WEPPpy Disturbed may become the native `ow-lanuse-1` producer because it owns
  the disturbed class / severity / texture parameterization harness.
- The route coefficients are explicit Disturbed native input parameters.
- The package is not a legacy-field bridge unless it amends canonical authority
  with a named mapping and review; default execution must avoid that path.

If due diligence cannot assign defensible coefficients for all required rows,
the package must stop at `EXECUTED-HOLD-ROUTE-COEFF-DUE-DILIGENCE`, not fill
gaps with guessed values.

## Due-Diligence Requirements For Coefficient Values

Before implementation writes producer outputs, author
`artifacts/coefficient-due-diligence.md` with:

- Complete row coverage for every `(disturbed_class, stext)` key in the active
  extended table or a justified narrower D16 cohort subset plus explicit hold
  for uncovered rows.
- A source/provenance entry for every value set.
- An explanation of whether each coefficient varies by soil texture. If no
  source supports texture-specific variation, repeat values across textures and
  mark the row `texture_invariant`; do not invent texture gradients.
- Physical-domain checks:
  - `route_skin_friction_coefficient_ko` finite and `> 0`
  - `route_form_drag_coefficient` finite and `>= 0`
  - `route_roughness_element_height_m` finite and `>= 0`
  - `route_roughness_concentration` finite and in `[0, 1]`
  - `route_vegetation_drag_coefficient` finite and `>= 0`
- Presence coupling checks:
  - roughness-element absence must use both
    `route_roughness_element_height_m = 0` and
    `route_roughness_concentration = 0`;
  - if one roughness-element term is positive, both must be positive;
  - vegetation drag may be zero only when the row records physically absent
    vegetation for the routing surface.
- Directional sanity checks against Disturbed semantics:
  - high-severity burned rows must not have stronger vegetation drag or
    roughness-element protection than corresponding unburned rows unless a
    source/provenance note explains why;
  - mulch/treatment rows may increase roughness only with explicit treatment
    rationale;
  - forest, shrub, grass, bare, and mulch classes must be reviewed separately.
- Sensitivity evidence on at least one H2637-class and one contrasting
  Disturbed cohort member showing the coefficient set is numerically stable and
  does not trip active closure guards.

Authority classes:

- `measured`: row value comes from measured/project/source data.
- `literature_range`: value is chosen within a cited primary-source range for
  the represented surface class.
- `operator_calibration`: value is an explicit WEPPcloud/Disturbed calibration
  choice with source notes and bounded physical checks.
- `unsupported`: not acceptable for authoritative lift; package must hold or
  narrow scope before production output.

## Intended Write Set

openWEPP:

- This package directory.
- `docs/work-packages/README.md`.
- `docs/contracts/openwepp-management-lanuse-authority-contract.md` only if the
  final authority model needs to name Disturbed native production explicitly.
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
  or `SC-OFEROUTE-001.md` only if existing contract text is insufficient for
  the generated native inputs.
- Tests/fixtures only for openWEPP native parse/projection and active consumer
  proof.

WEPPpy:

- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/data/extended_land_soil_lookup.csv`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/README.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/ENDUSER.md`
- `/home/workdir/wepppy/docs/adr/` or the repository-standard ADR location.
- `/home/workdir/wepppy/tests/disturbed/`
- `/home/workdir/wepppy/tests/wepp/management/` or adjacent management tests.
- Type stubs only if public APIs are added.

## Phase Plan

1. **S0 Scaffold and authority confirmation.** Confirm repo state in openWEPP
   and WEPPpy, required reading, and write-set boundaries.
2. **S1 Disturbed schema design.** Define the additive extended-table columns,
   compatibility behavior, and provenance fields. Record downstream-impact and
   backward-compatibility plan before edits.
3. **S2 Coefficient due diligence.** Build the value matrix, sources,
   confidence classes, domain checks, directional sanity checks, and sensitivity
   evidence. Hold if values cannot be defended.
4. **S3 WEPPpy implementation.** Extend the extended lookup and native
   management producer. Preserve legacy output unless openWEPP native mode is
   selected. Add docs and ADR.
5. **S4 WEPPpy verification.** Add tests proving schema migration, lookup
   coverage, native `ow-lanuse-1` output, legacy-output isolation, and generated
   run-artifact propagation.
6. **S5 openWEPP authority lift.** Parse/project generated native files,
   verify active Lane-D consumes the route coefficients, and update openWEPP
   authority docs only if needed.
7. **S6 D16 active preflight.** Re-run selected D16 cohort active preflight
   using Disturbed-produced native inputs. If it passes, hand off to executable
   active plain-vs-hybrid suite/tolerance work.
8. **S7 Review, verification, and closure.** Complete dual review, disposition,
   dual verification, gate table, final disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to implementation, science-authority review, WEPPpy QA,
openWEPP verification, comparator/timing, and documentation subagents for
bounded work on the intended write set. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`,
`artifacts/coefficient-due-diligence.md`, and compact command/comparator
evidence. Write access is read-only unless a worker is explicitly assigned a
bounded implementation fix in either openWEPP or WEPPpy.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/downstream-compatibility-plan.md`
- `artifacts/disturbed-schema-design.md`
- `artifacts/coefficient-due-diligence.md`
- `artifacts/wepppy-implementation.md`
- `artifacts/openwepp-authority-lift.md`
- `artifacts/active-consumer-proof.md`
- `artifacts/d16-active-preflight.md`
- `artifacts/command-evidence.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Required Gates

WEPPpy:

- `git status --short --branch` in `/home/workdir/wepppy`.
- Parameterization ADR present for route-coefficient defaults.
- CSV schema validation for the extended lookup.
- Coverage check: every active Disturbed row in scope has all five route
  coefficients and provenance fields.
- Domain and coupling checks from the due-diligence section.
- Legacy-output isolation proof: default/legacy Disturbed management outputs
  remain unchanged when native openWEPP mode is not selected.
- Native-output proof: generated `.man` files use `ow-lanuse-1`, native
  cropland/forest sentinels as appropriate, and exactly five
  `routing_coefficients` values per scheduled plant/lane.
- Run-artifact propagation proof under a real or fixture Disturbed project.
- `cd /home/workdir/wepppy && wctl run-pytest tests/disturbed/test_disturbed_matrix.py`
- `cd /home/workdir/wepppy && wctl run-pytest tests/test_2_validate_managements.py tests/test_managements_module.py`
- Additional focused tests added by the package.
- `cd /home/workdir/wepppy && wctl doc-lint --path wepppy/nodb/mods/disturbed/README.md`
- `cd /home/workdir/wepppy && wctl doc-lint --path wepppy/nodb/mods/disturbed/ENDUSER.md`
- `cd /home/workdir/wepppy && wctl doc-lint --path <ADR path>`

openWEPP:

- `git status --short --branch` in `/home/workdir/openWEPP`.
- `git diff --check`.
- Markdown/doc lint for touched openWEPP docs.
- Native management parse/projection tests for Disturbed-produced files.
- Active missing-coefficients guard remains live for legacy/non-native inputs.
- Active consumer proof showing the Lane-D active path reads the
  Disturbed-produced route coefficients, not a fallback or H2637 recipe.
- D16 selected-cohort active preflight with Disturbed-produced native inputs.
- `cargo fmt --check`.
- Focused Lane-D / `ofe_routing` tests if openWEPP code/contracts/fixtures are
  touched.
- Contract/profile/BEI checks if `SC-*` contracts are touched.
- Anti-evasion guards if any required-case binding, cohort fixture, or external
  authority suite posture changes:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Closure Outcomes

- `EXECUTED-COMPLETE-SOURCE-AUTHORITY`: Disturbed emits defensible native
  route-coefficient inputs, openWEPP consumes them, and the route-coefficient
  authority hold is lifted.
- `EXECUTED-HOLD-ROUTE-COEFF-DUE-DILIGENCE`: coefficient values cannot be
  defended for required rows.
- `EXECUTED-HOLD-WEPPPY-PRODUCER`: values are defensible, but Disturbed native
  output cannot be implemented safely in the package envelope.
- `EXECUTED-HOLD-OPENWEPP-CONSUMER`: Disturbed native outputs exist, but
  openWEPP parse/projection/active consumer proof fails or requires a separate
  contract/code package.
- `EXECUTED-HOLD-D16-SUITE`: source authority is lifted, but executable D16
  active plain-vs-hybrid cohort work remains for a follow-on package.
