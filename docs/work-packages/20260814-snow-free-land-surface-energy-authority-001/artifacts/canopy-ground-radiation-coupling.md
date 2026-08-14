# Canopy--Ground Radiation Coupling

Shortwave remains the exact V7 two-stream solution. The ground surface-class
VIS/NIR albedos are its lower boundary. Direct/diffuse VIS/NIR terminal fluxes
reach that tile ground once; reflected flux re-enters and traverses every
overlying occupancy. Ground absorption is the remainder only after the full
column solution. Band, direction, tile and amount basis remain typed.

The successor woody model uses the FSM2/ORCHIDEE no-longwave-reflection
multilayer recurrence. For each top-to-bottom occupancy `i`, V8 selects
`P_i=LAI_i+SAI_i` on tile-ground basis and
`tau_i=exp(-0.8*Omega_i*P_i)`, where `0.8 m2 plant area m^-2 ground` is the
frozen longwave extinction coefficient and clumping `Omega_i` is applied
exactly once. Downward
longwave is recursively

```text
Ldn[i+1] = tau_i*Ldn[i] + (1-tau_i)*E_i
```

and upward longwave is recursively, bottom-to-top,

```text
Lup[n] = epsilon_g*sigma*Tg^4
Lup[i] = tau_i*Lup[i+1] + (1-tau_i)*E_i.
```

For component `j` in layer `i`, let `w_i,j=a_i,j/sum_j(a_i,j)` and
`E_i=sum_j(w_i,j*sigma*T_i,j^4)`. Incoming longwave absorption is distributed
by `w_i,j`, but each component's emission remains source-resolved:

```text
LWnet[i,j] = (1-tau_i)*w_i,j
             *(Ldn[i] + Lup[i+1] - 2*sigma*T_i,j^4).
```

The layer sum is `(1-tau_i)*(Ldn[i]+Lup[i+1]-2*E_i)`. It is never computed
first and repartitioned by area, because that alias loses unequal component
temperature ownership. Ground net longwave is
`epsilon_g*(Ldn[n]-sigma*Tg^4)`. All emissivities must be exactly one in this
initial recurrence; a non-unit value is typed unsupported rather than silently
dropping reflection.

This is evaluated from current nonlinear trial temperatures. A previous-step
or prescribed upward-ground flux is prohibited.
