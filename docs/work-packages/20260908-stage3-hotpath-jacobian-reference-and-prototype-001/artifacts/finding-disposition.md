Static: independent observation-only review, not terminal derivative approval.

Checkpoint update: worker froze cut2 with all three corrections implemented.
The assignments below describe cut1 findings; they remain OPEN pending cut2
tests and independent re-review. Cut1 passing tests are not cut2 evidence.

CAP-01 medium: Session::finish accepts outstanding Observation handles; stale
index-only handles can corrupt a new session or panic. Correction assigned:
live-scope refusal, generation binding and stale-handle tests. OPEN.

CAP-02 medium: Corpus::validate ignores participation rows, allowing malformed
or inconsistent wire counts. Correction assigned: closed buckets, uniqueness,
checked lifecycle reconciliation and negative tests. OPEN.

CAP-03 evidence completeness: preserve exact scaled generic coordinates and
unit scales before claiming generic FD replay, and independently reconcile
column cardinality. Correction assigned. OPEN.

No solver arithmetic/control/error-precedence change found in static five-file
review. Observation-on/off behavior still needs executable evidence.
