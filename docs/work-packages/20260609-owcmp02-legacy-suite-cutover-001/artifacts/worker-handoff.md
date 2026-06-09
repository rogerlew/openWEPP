# Worker Handoff

Status: complete

## Current State

OWCMP02 cutover is complete. Use `tools/owcmp` for active PL14S comparison work.

## Active Commands

```bash
tools/owcmp/owcmp wat semantic ...
tools/owcmp/owcmp pl14s run ...
tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>
```

## Removed Path

`tools/legacy_comparison_suite` has been deleted. Do not add new active
references to it.

## Remaining Historical References

Historical work-package artifacts and prompts still contain old command paths as
evidence of what ran at the time. Treat those as archival unless a future
archival cleanup package says otherwise.

## Follow-Up Candidates

- Add full `owcmp manifest run` schema/identity/promotability validation in a
  dedicated manifest package.
- Add `owcmp observe normalize` only under a dedicated observability package.
- Add broader parquet/partition/year-offset dynamic tests if future cutover work
  depends on those paths.
