"""Adviser's reduced thermal test-design calculation; not the Rust evaluator."""
import math

T = 273.15 + 2**-26
ref, sigma = 273.15, 5.670374419e-8
h, z0, wind, kappa, lai = 12.5, 1.25, 3.7, .4, 2.708333333333333
re = .02 / 1.5e-5
drag = 1.328 * 2 / math.sqrt(re) + .45 * ((1-.12)/math.pi)**1.6
d = 1.1*h*math.log(1 + (drag*lai)**.25)
ustar = kappa*wind/math.log((h-d)/z0)
eddy = kappa*ustar*(h-d)
rg = h/(2*eddy) * (math.exp(2*(1-.007/h)) - math.exp(2*(1-(d+z0)/h)))
rh, capacity, dt = 20.99229315129214, 3235.68, 1800.
enthalpy = capacity*(T-ref)
lw = sigma*(T*T)*(T*T)-1.
g0 = 2/(.04/.103 + .08/1.1)
g1 = 2/(.08/1.1 + .18/1.35)

def tolerance(scale):
    return 1e-6 + 1e-10*max(1., scale)

def residual(v):
    tc, tg, t1, t2 = v
    rho = 101325/(287.05*tc)
    ground_h = rho*1004.64*(tg-tc)/rg
    reference_h = rho*1004.64*(tc-T)/rh
    flux0, flux1 = .5*g0*(tg-t1), .5*g1*(t1-t2)
    storage = (capacity*(tg-ref)-enthalpy)/dt
    operands = [0., lw-sigma*(tg*tg)*(tg*tg), -ground_h, 0., -flux0, -storage]
    s1, s2 = 120000*(t1-T)/dt, 180000*(t2-T)/dt
    return [(ground_h-reference_h)/tolerance(abs(ground_h)+abs(reference_h)),
            sum(operands)/tolerance(sum(map(abs, operands))),
            (flux0-flux1-s1)/tolerance(abs(flux0)+abs(flux1)+abs(s1)),
            (flux1-s2)/tolerance(abs(flux1)+abs(s2))]

x = [T]*4
base = residual(x)
matrix = [[0.]*4 for _ in x]
for j in range(4):
    perturbation = 2**-26*abs(x[j])
    minus, plus = x.copy(), x.copy()
    minus[j] -= perturbation
    plus[j] += perturbation
    lower = base if j == 1 else residual(minus)
    upper = residual(plus)
    denominator = perturbation if j == 1 else 2*perturbation
    for i in range(4):
        matrix[i][j] = (upper[i]-lower[i])/denominator
rhs = [-v for v in base]
for j in range(4):
    pivot = max(range(j, 4), key=lambda r: abs(matrix[r][j]))
    matrix[j], matrix[pivot] = matrix[pivot], matrix[j]
    rhs[j], rhs[pivot] = rhs[pivot], rhs[j]
    for i in range(j+1, 4):
        factor = matrix[i][j]/matrix[j][j]
        for column in range(j+1, 4):
            matrix[i][column] -= factor*matrix[j][column]
        rhs[i] -= factor*rhs[j]
delta = [0.]*4
for i in range(3, -1, -1):
    delta[i] = (rhs[i] - sum(matrix[i][j]*delta[j] for j in range(i+1, 4)))/matrix[i][i]
print(T, rg, base, delta)
for exponent in [20, 21, 39, 40]:
    trial = [v + 2**-exponent*change for v, change in zip(x, delta)]
    print(exponent, trial[1]-ref, trial[1] >= ref, max(map(abs, residual(trial))))
