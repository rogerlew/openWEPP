# PERFDEEP07 Layout and Allocation Evidence

Status: blocked by HOLD.
Evidence mode: Static.

No new direct-frame structures were implemented because the ordered
default-disabled P0 timing gate failed.

## Required Evidence

Record:

- `size_of` evidence for new frame/view structures;
- preallocated heap buffers and ownership;
- normal-path allocation/static proof;
- absence of hot-path `format!`, owned symbol construction, map operations, and
  payload construction in migrated direct-frame success path;
- RSS evidence from default-disabled and opt-in H2637 runs.
