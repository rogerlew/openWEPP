# PERFDEEP06 Layout and Allocation Ledger

Status: queued.
Evidence mode: not-run.

## Required Content

Record layout/type-size and allocation risks from ADR-0025 Amendment 1:

- proposed field and array counts;
- fixed array versus `Vec`/boxed-slice rationale;
- unit-wrapper `#[repr(transparent)]` needs;
- validity/dirty bitset design;
- `Option<T>` and enum-storage risks;
- bounds-check strategy;
- normal-success-path allocation prohibitions;
- measurement plan for `size_of`, type-size, allocation, endpoint, and RSS
  evidence in the follow-on implementation package.

## Gate

This artifact is complete only when the follow-on implementation has concrete
layout and allocation evidence to collect, not a generic performance reminder.
