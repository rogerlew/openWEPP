# Execution Incident 006

Status: `PACKAGE-LOCAL AUTHORITY-PATH DEFECT / CORRECTION REQUIRED`

Evidence class: `Ran + Static`.

The fresh 2026-07-28 attempt at
`/home/workdir/cal04b-attempt-20260728-correction` passed:

- `prepare`;
- corrected executor build;
- production runner build;
- all 12 native production-consumer cases; and
- corrected synthetic production plus both independent reconstructions.

The synthetic result recovered only hidden `GSI-5557`; `GSI-0064` retained a
finite positive objective and both boundary candidates retained missing-
crossing failures. The DAG then advanced to `hubbard_producer`, which failed
before writing a population trace:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

The immutable primary failure is
`/home/workdir/cal04b-attempt-20260728-correction/direct-evidence/primary-failure.json`.
Harvard was not accessed.

## Diagnosis

All six declared producer inputs exist. Their prepared authority rows and
digests pass. `read_authenticated_daymet` and `authority_digest` incorrectly
derive the repository root from the fresh external
`publication/**/input-and-authority-manifest.csv`. Repository-relative Daymet
paths are consequently resolved below the publication tree and fail with
`ENOENT`.

This is a package-local custody/path-resolution defect. The authoritative
source/request manifest remains inside
`docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/`
and provides the correct repository anchor. No forcing, geometry, threshold,
kernel, objective, observation, or acceptance value is implicated.

## Correction Boundary

Derive and validate the repository root from the canonical checksum-bound
source manifest, pass that root explicitly into authority-row resolution, and
retain the external publication manifest only as the attempt-specific
authority ledger. Add regression coverage proving the source manifest anchors
repository-relative paths when the authority ledger lives outside the
repository.

Do not rerun or clean the failed attempt. After focused validation and dual
review, use another wholly fresh attempt root and restart from `prepare`.
Synthetic recovery remains a mandatory prerequisite to population execution;
Harvard remains sealed.
