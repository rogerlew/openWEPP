# ASSURE-MAINT-01 Performance Evidence

Evidence class: Ran

Host: `forest`, Linux 6.8.0-111-generic, x86_64

Final release binary SHA-256:
`010cf889644f8c921bcac204cf09330e908709e29b905d83e44240184ebd9c66`

This terminal campaign supersedes the earlier 39-test measurement. The release
binary was prebuilt, and compilation was excluded from every trial. Each trial
copied an isolated fixture, applied the same typed attribution change, selected
the unique archived transition to the resulting generation, and ran the
focused receipt runner with explicit test-fixture authority. The runner
validated, built, and checked the affected report and ran the final pinned
45-test `assurance-amendment` profile.

| Corpus | Trials | Reports | v2 bytes | Apply p50 / p95 / max (s) | Runner p50 / p95 / max (s) | Apply-through-evidence p50 / p95 / max (s) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| current | 10 | 2 | 454,902 | 1.228 / 1.574 / 1.574 | 43.191 / 47.517 / 47.517 | 44.419 / 48.766 / 48.766 |
| scaled | 20 | 100 | 49,860,554 | 8.704 / 9.515 / 9.661 | 43.739 / 48.691 / 51.016 | 52.100 / 57.551 / 60.197 |

Percentiles use the conservative nearest-rank method. Ten observations make
the current-corpus p95 the maximum. The scaled campaign was extended from ten
to twenty unchanged observations after its first end-to-end result was 60.197
seconds. That result is retained as the maximum; the 20-observation
nearest-rank p95 is the nineteenth ordered value, 57.551 seconds.

The current transaction passes the five-second limit, and the scaled
transaction passes the ten-second limit. Both corpora pass the 60-second p95,
120-second maximum, and 300-second hard-regression limits. Every retained gate
record binds the same final release-binary identity above.

Raw current trials (`trial`, apply seconds, runner seconds):

```text
1  1.249  47.517
2  1.028  46.408
3  1.309  45.209
4  1.228  43.191
5  1.038  41.100
6  1.574  43.920
7  1.056  40.136
8  1.043  42.802
9  1.387  41.741
10 1.330  44.654
```

Raw scaled trials:

```text
1   9.181  51.016
2   8.860  48.691
3   9.661  43.764
4   8.447  42.362
5   8.839  36.003
6   8.607  41.621
7   8.704  44.383
8   8.660  46.200
9   8.743  44.733
10  9.515  45.464
11  8.226  41.088
12  8.871  45.659
13  8.454  40.925
14  9.251  44.431
15  8.913  41.089
16  8.696  43.052
17  8.971  40.144
18  8.361  43.739
19  8.661  43.128
20  8.391  46.820
```

The scaled fixture contains one deliberate 32-MiB assurance object. System-call
inspection during development showed that this filesystem rejects reflink
cloning, so the fail-closed transaction uses a complete copy fallback. The
measured cost remains within the explicit scaled transaction and end-to-end
limits.

Ran after the campaign: `cargo nextest run --workspace --profile
assurance-amendment` passed 45/45 selected tests in 40.174 seconds; 53 tests and
182 binaries were skipped by the pinned profile.
