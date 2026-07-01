# WSHED-FIXTURE01 Committed Watershed Fixture Adoption

Status: `QUEUED-HANDOFF-AUTHORED`

Date opened: `2026-07-01`

Package type: fixture adoption / documentation / focused regression gate.

## Objective

Adopt an auditable watershed development fixture for the new watershed runtime
program. The target fixture is a committed openWEPP fixture derived from
`/wc1/runs/ca/carnivorous-adobo/wepp`, exercising a 32-hillslope watershed and
usable by future W2/W3 runtime packages without relying on `/wc1` or scratch
paths.

## Rationale

WSHEDPERF01 proved arboreal-dendrite is useful as a tiny smoke/baseline fixture
but too small to drive watershed runtime development. ADR-0032 ratifies
`strict-committed-fixture` as the canonical benchmark/ratification mode.
Therefore WSHED-FIXTURE01 must make the next development fixture local,
versioned, and auditable before runtime implementation packages use it as a
persistent gate.

The user reported a related wepppy package:
`/home/workdir/wepppy/docs/work-packages/20260701_wshed_fixture01/package.md`,
with focused fixture tests in `wepppy/tests/topo/test_wshed_fixture01.py`.
That evidence is useful orientation, but it does not by itself close openWEPP
WSHED-FIXTURE01 because openWEPP requires adopted fixtures to be committed in
this repository.

## Included Scope

- Inspect the wepppy WSHED-FIXTURE01 package and test for transferable fixture
  contract ideas.
- Capture or construct a committed openWEPP fixture under
  `tests/fixtures/watershed/`.
- Record fixture provenance, expected hillslope count, topology summary, and
  intended gate scope in a fixture README.
- Add a focused test or fixture-contract check proving the openWEPP gate reads
  the committed fixture path, not `/wc1` or wepppy directly.
- Update package artifacts, `docs/ROADMAP.md`, and `docs/work-packages/README.md`
  with truthful status.

## Excluded Scope

- No production Rust runtime implementation.
- No W2/W3 worker-pool implementation.
- No benchmark speedup claim.
- No large 1,000+ hillslope fixture adoption.
- No use of `/wc1`, `/tmp`, or wepppy-only files as the sole persistent gate
  evidence.
- No silent substitution of a non-32-hillslope or non-carnivorous-adobo-derived
  fixture unless the package records an `EXECUTED-HOLD` or the architecture
  authority is amended explicitly.

## Intended Write Set

- `tests/fixtures/watershed/**`
- Focused fixture-contract test files, preferably in the existing test location
  that matches adjacent openWEPP fixture tests.
- `docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `explorer`, `rust_code_reviewer`, and `rust_qa_reviewer`
subagents for read-only fixture discovery, review, and verification; expected
outputs are compact findings with file/path references and any command evidence;
write access is read-only for subagents, with parent disposition recorded in
`artifacts/review-disposition.md` and `artifacts/verification.md`.

## Phase Plan

1. Inventory candidate evidence:
   - read this package, the active kickoff prompt, ADR-0032, and the watershed
     runtime architecture spec fixture sections;
   - inspect the wepppy package/test for fixture contract shape;
   - inspect `/wc1/runs/ca/carnivorous-adobo/wepp` only as source substrate.
2. Define the openWEPP fixture boundary:
   - identify the minimal input/runfile files required by the intended
     openWEPP gate;
   - confirm the fixture has 32 hillslopes and record the topology summary;
   - if the source substrate is unavailable, too large, or mismatched, close
     `EXECUTED-HOLD` with the exact blocker.
3. Commit fixture and contract:
   - place fixture files under `tests/fixtures/watershed/<fixture-id>/`;
   - write the fixture README and provenance artifact;
   - add a focused fixture-contract test that reads the committed path.
4. Validate:
   - run the focused fixture test;
   - run scoped docs lint for this package and touched docs;
   - run `git diff --check`;
   - run broader tests only if the touched test surface requires them.
5. Review and close:
   - complete dual review and verification;
   - update gate results, disposition, roadmap, and work-package README.

## Exit Criteria

- A committed fixture exists under `tests/fixtures/watershed/`.
- The fixture README records source substrate path/date, expected 32-hillslope
  count, topology summary, required input/runfile inventory, adopting package,
  and intended scope.
- A focused test or contract check proves the gate reads the committed
  openWEPP fixture path and does not read `/wc1` or wepppy as the persistent
  fixture.
- Package evidence records whether and how the wepppy WSHED-FIXTURE01 package
  informed this adoption.
- `docs/ROADMAP.md` and `docs/work-packages/README.md` are updated truthfully.
- Dual review/verification findings are dispositioned.
- Validation commands are recorded with `Ran:` evidence; skipped full-suite
  gates are labeled plainly.

## Security and Safety

Fixture adoption is local repository work. Do not commit secrets, credentials,
absolute machine-local scratch outputs, generated bulky caches, or transient
operator paths. Do not run network-dependent commands. Large fixtures must be
reduced, held, or explicitly justified before committing.
