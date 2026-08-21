# Event and replay matrix

Status: `PARTIAL EVENT CANDIDATE IMPLEMENTATION / REPLAY BLOCKED`.

`Static:` `select_actual_terminal_candidate` derives candidate ticks from the
parent bounds, actual proposed offset, and actual bracket bounds, filters by
the admitted minimum support, and reruns the actual Stage-3 support evaluator.
It accepts only a trial whose actual quantized event tick equals the candidate
tick. The accepted event ordinal advances only in the cloned candidate.

`Ran:` the focused terminal-event tests passed. The implementation has not
yet connected Child 2C event-boundary coalescing to a covered V11 segment, has
no zero-duration parcel-consumption transaction, and has no restart/replay
matrix. Required restore points and no-replay behavior are `BLOCKED`.
