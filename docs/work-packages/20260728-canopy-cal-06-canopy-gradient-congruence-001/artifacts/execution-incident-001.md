# Execution Incident 001

Evidence class: `Ran`

The first matrix launch used the WAT `year` field as a calendar year. WAT
publishes simulation year while the protected climate file carries calendar
identity, so calendar reconstruction failed before any result artifact was
written.

The executor was corrected to:

1. parse the protected CLIGEN daily calendar;
2. require a complete consecutive 16,437-day calendar;
3. require WAT simulation-day and Julian-day agreement with that calendar; and
4. derive calendar date and water year only from the protected calendar.

The failed attempt and its remaining subprocess group were terminated. Its
temporary execution root was deleted and no first-attempt output was promoted.
An isolated corrected forest/member probe passed trace/WAT date identity and
exact-date observation matching before the complete matrix was relaunched.

