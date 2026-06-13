# MOFE per-OFE State Architecture — Design Artifact (increment M-D)

Status: queued (produced by increment M-D; NO production code in M-D)

Architectural analog of FDHP01's `d3-fine-sublayer-port-scope.md`. Fill the
sections below so the M-E implementation lands against a declared shape, not
an evolving one. All findings `Static:`, file:line-cited against the **current
tree** (read the lines; do not infer from symbol tables — recorded FDHP01-Dh
lesson). Authority for the legacy routing model is `mofe-routing-port-scope.md`;
authority for the as-is collapse is `m-c2-per-ofe-daily-state-scope-evidence.md`.

## 1. Target per-OFE state shape

(OFE-keyed daily WB state/flux collection replacing the single aggregate maps
at `HillslopeWritebackSurface` and `KernelWritebackPayload`; per-OFE record
contents; lifecycle.)

## 2. Sequential execution model

(How OFE i's completed daily state feeds OFE i+1 run-on; topology-N-nodes vs
per-OFE lane iteration over the phase graph; where legacy `irs`/`rochek`
continuation maps.)

## 3. Contract surface

(Per-element identity, transfer identity Σsent≡Σreceived, per-OFE daily-state
semantics as measurable invariants; which contracts amend.)

## 4. Change map (kernel-contract / scheduler / writeback / publication)

(Every touched seam; aggregate→per-OFE migration path; single-OFE-anchor
preservation argument.)

## 5. Red tests + M-E sub-increment breakdown and sizing

(Per-identity / per-arm red tests; the implementation sub-increments, each
behind a conservation hard stop, per-element + transfer identities first
proven; sizing.)
