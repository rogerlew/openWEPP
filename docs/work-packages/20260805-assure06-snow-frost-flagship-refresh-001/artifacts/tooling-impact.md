# Assurance Manifest-Adoption Tooling Impact

Status: admitted for bounded correction

Evidence mode: Ran + Static

Ran: after the existing v126 contract drift was adopted, the refreshed DRAFT
manifest declared `SC-SNOWENERGY-001` and 21K-21N as new local-content inputs.
The production check rejected the candidate with:

```text
ERROR: generated identity lock omits identified source
'docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md'
```

Static: `adopt-report-source` already documents manifest selection as adoption
of the complete bound DRAFT source. Its implementation collects drift only for
members already present under the report directory, while the successor helper
has an existing `allow_new_sources` path used for initial report admission.

Correction envelope:

- admit a previously absent source only during exact manifest-selected DRAFT
  adoption;
- require the new external path to be an exact declared `local_content`
  dependency outside `assurance/`, or an owned path within the same report;
- preserve confinement, regular-file checks, deterministic check/apply,
  complete identity generation, review reset/invalidation, and unrelated-drift
  rejection; and
- add a focused integration contract proving the positive path and negative
  boundaries.

The stabilized review-entry attempt exposed a second mismatch in the same
typed amendment module: `review_entry` required and assigned a
`scientific_approver_id`, although an `IN_REVIEW` report may truthfully leave
that independent role unassigned. The bounded correction permits the optional
field to remain absent, writes a YAML null, and retains the existing human-role
checks for any later scientific-approval event. The integration fixture now
enters review without inventing an approver and asserts the null assignment.
