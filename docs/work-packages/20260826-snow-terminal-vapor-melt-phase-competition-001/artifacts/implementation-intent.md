# Implementation intent

Status: `PRE-IMPLEMENTATION / RESEARCH ONLY`.

Static: initial changes will be behavior-preserving source decomposition and
cfg(test)-only candidate/evidence work. No production equation, tolerance,
terminal behavior, API, output, selector, owner, receiver, restart, runner, or
cutover change is intended. Production implementation requires a passing matrix
and dual GO followed by contract-first authority.

Ran: the source split and cfg(test) Candidate A/B allocators are implemented.
Candidate A is frozen for review only. Production intent remains unchanged;
no contract or production phase equation is edited before dual GO.
