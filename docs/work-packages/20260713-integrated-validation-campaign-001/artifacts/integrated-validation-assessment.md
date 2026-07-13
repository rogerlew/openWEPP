# Integrated Validation Assessment

Status: `HOLD-INTEGRATED-VALIDATION`

Evidence class: **Ran + Static**

Frozen source: `f80a115148e75a08269eb14a8c1b0e7791ca891a`.

Passed evidence includes anti-evasion and AUTH11; H2637 active production and
three authority failures; p61/p102 erosion and 367-test erosion profile;
320-test frost profile; W7R serial/parallel p102 publication; MT3 hourly,
totalwatsed3, and watershed hourly suites; and full runner/watershed package
suites. These provide partial pre-fix test bindings for the independent
reconstruction and real-consumer maps. They do not supply complete H2637
groundwater or snow numeric operands/output hashes and cannot be reused as
terminal evidence after the required restart.

The default release lane fails before completing required authority, stability,
and final closure. It exposes `INTVAL-REL-001`: stale threaded libtest release
orchestration violates an explicit nextest-only H2637 isolation contract. The
failure is infrastructure, not a comparator or production-physics verdict.

Recommendation: `HOLD-INTEGRATED-VALIDATION`. Close `INTVAL-REL-001`, freeze
the correction commit, and restart every lane; do not reuse these pre-fix
results as terminal evidence. Two independent reviews and two independent
verifications passed this corrected HOLD boundary and restart rule.
