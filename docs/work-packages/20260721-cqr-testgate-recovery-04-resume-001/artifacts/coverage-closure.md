# Coverage Closure

Ran: PASS under ADR-0021 glue-tier closure. Matching production coverage is
92.3823% lines and 85.4962% regions, and all 29 production functions meet the
75% region floor.

Ran: the first post-refactor measurement at `7faa45f9` was retained at
`/tmp/cqr-resume-post-Qu3eaQ` but failed closure: regions were 80.8312%,
`load_accepted_receipt` was unexecuted/CRAP 90, and three functions missed the
floor. It was not rerun or reused. Receipt/envelope/reuse characterization
changed the head before the single corrected measurement at `47eb418d`.
