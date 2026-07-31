# Conservation Evidence

Status: `complete`

Evidence mode: `Ran`

Ran: the four kernel cells independently reconstruct hourly longwave:

`subcanopy - outgoing = net longwave`.

Ran: the same cells independently reconstruct hourly surface energy:

`net shortwave + net longwave + latent = potential surface energy`.

Ran: the same test independently reconstructs the full Stage 3 cold-content
ledger from published diagnostics:

`surface energy + conduction + refreeze + exported cold content
- (cold content before - cold content after) = 0`.

Ran: selected sublimation hours satisfy:

`latent flux * 3600 s = vapor mass exchange * latent heat`,

with maximum admitted residual `<= 1e-6 J m^-2`. Snow-column mass loss equals
the diagnosed sublimation loss; the vapor amount is absent from routed melt and
liquid. Cold-content closure includes the proportional cold content exported
with removed ice and passes at `<= 1e-6 J m^-2`.

Ran: anti-tautology tests reject direct-cover sky view, air-temperature snow
emission, water-surface saturation for frozen snow, wrong latent sign,
double latent debit, omitted cold-content export, and retained exhausted
surface layers.
