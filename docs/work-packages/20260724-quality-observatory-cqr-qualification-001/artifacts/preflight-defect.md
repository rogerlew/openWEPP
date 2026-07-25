# Preflight Defect And Correction

Evidence class: Ran and Static.

Preflight found that QA trusted only current-main identity; it did not require
the supplied source to have a successful exact-head TESTGATE qualification.
That violated the Order-7 dispatch protocol even though current-main and
workflow-revision equality remained fail closed.

The correction adds required `qualification_run_id` input and admits QA only
when the GitHub run is:

- from this exact repository;
- manual `workflow_dispatch`;
- completed successfully;
- exact `main` head equal to `source_sha`; and
- exact workflow path `.github/workflows/testgate-shadow.yml`.

The first characterization attempted to couple QA to named TESTGATE jobs. The
existing workflow-separation contract rejected that coupling before commit.
The corrected implementation authenticates the TESTGATE aggregate run without
embedding TESTGATE job names or execution logic in QA.

No live workflow was dispatched during diagnosis or correction.
