# Worker Handoff

Status: `endpoint tools ready for result-blind review`.

The next action is result-blind review of the execution-custody tool and the
independent consumer. After both reviews pass on one clean tool commit, dispatch
the required comparator execution at that exact SHA. The comparator must run
the four endpoint cells first and independently reconstruct both forcing-matched
source gates. It may traverse checkpoints only for forcing lanes selected by
the frozen WY-or-median predicate.

Final handoff must name the exact next package only when evidence requires work
outside this package's declared boundary.
