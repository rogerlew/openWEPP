# Repository Transition Plan (2026-05-11)

Status: `planned`  
Execution state: `not executed in this change`

## Goal

Restore `wepp-in-the-woods/wepp-forest` to a familiar baseline for Anurag, and
continue ongoing modernization work in a separate repository lineage under
Roger's namespace.

## Planned sequence

1. Revert `github.com/wepp-in-the-woods/wepp-forest` to commit `dac3c95`.
2. Create a new repository lineage at
   `github.com/rogerlew/wepp-palimpsest` from the current active work state.
3. Update downstream documentation and references in dependent repos
   (`openWEPP`, `wepppy`, `wepppyo3`) after the migration is complete.

## Constraints

- Migration is deferred while active work continues in the current local
  `wepp-forest` checkout.
- No destructive or namespace-modifying repository operations are executed as
  part of this documentation update.

## Preconditions for later execution

- Confirm target commit `dac3c95` is the intended restoration point.
- Prepare communication notes for collaborators (especially Anurag) describing
  what changes and what remains stable.
- Prepare a cross-repo reference-update checklist so post-migration links and
  contracts remain coherent.

## Out of scope for this document

- Running git push/force-push operations.
- Creating or transferring GitHub repositories.
- Rewriting local git history.
