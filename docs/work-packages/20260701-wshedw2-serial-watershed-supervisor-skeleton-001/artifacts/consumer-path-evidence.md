# Consumer-Path Evidence

Status: `QUEUED`

W2 execution must fill this artifact before closure.

Required evidence:

- producer source: `WatershedRunPlan`, `HillslopeJob`, and `PassInventory`;
- in-memory state/frame object names and module paths;
- runner handoff: public `openwepp-cli-watershed` call path that selects the
  new serial supervisor for `--jobs 1`;
- downstream consumer: routing/publication call site consuming validated pass
  inventory;
- negative proof: the W2 claim does not use package-local shell loops, shared
  output directories, or ad hoc path rewrites;
- output/API proof: watershed outputs or explicit fail-closed errors are
  produced through the new path.
