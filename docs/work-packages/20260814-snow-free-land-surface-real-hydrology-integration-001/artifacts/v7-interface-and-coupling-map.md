# V7 Interface And Coupling Map

Status: `Static impact decision complete`

The existing public V7 candidate, occupancy energy proposal, typed water
request/authorization/final-use path, independent owner candidates and atomic
diagnostic replacement are inputs to this campaign.

V7 treats `longwave_up_w_m2` as prescribed caller forcing. Its canopy-air heat
and vapor residuals include atmosphere, leaf, wet-canopy and stem exchange but
omit ground sensible and vapor exchange. It also has no reciprocal
canopy-to-ground longwave receipt.

A consumer that only reads unchanged V7 receipts would not require a new model
identity. The campaign objective, however, requires physically coupled
ground-to-canopy longwave and ground sensible/vapor feedback. That changes the
accepted residual equations and coupled unknown system. Child 1 must therefore
issue a successor immutable vegetation model identity and preserve V1--V7
bytes. Mutating V7 under its existing digest is prohibited.
