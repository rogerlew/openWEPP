# Occupancy Characterization

Evidence class: Ran + Static.

QA run `30205750420` observed `CLEAR` occupancy before admission and after
acquiring `/testgate-history/forest1-heavy.lock`. No TESTGATE run was queued or
active. The same lease remained held through ordered full and science-manual
execution, post-processing, independent upload verification, and publication.
All observed supervisor, collector, profile, and verifier processes exited,
the lease was released, and terminal occupancy was `CLEAR`.

No concurrent heavy run was manufactured. The retained deterministic workflow
fixture and focused contract gates prove that live TESTGATE priority produces
a typed successful deferral before acquisition; the 22 focused package
contracts passed, including the occupancy behavior.
