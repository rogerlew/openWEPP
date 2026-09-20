"""Independent Decimal controls, not a canopy solver or runtime surrogate.

Evaluates the prospectively named draft03 component inputs from Ambaum's
constant-cp thermodynamics and the retained liquid polynomial. No production
Rust function or produced result is used to construct these expectations.
"""
from decimal import Decimal as D, localcontext
from pathlib import Path
import hashlib
import json


def controls():
    with localcontext() as ctx:
        ctx.prec = 60
        tf, e0, rv = D('273.15'), D('611.213476'), D('461.52')
        cw, ci, cpv, lf = D('4218'), D('2106'), D('1849'), D('333700')
        p = D('90000')
        polynomial = list(map(D, ['6.11213476', '.444007856', '.0143064234',
            '.000264461437', '.00000305903558', '1.96237241e-8',
            '8.92344772e-11', '-3.73208410e-13', '2.09339997e-16']))

        def sat(t, phase):
            if t >= tf:
                x = t - tf
                e = sum(c*x**k for k, c in enumerate(polynomial) if k) + polynomial[0]
                de = polynomial[1] + sum(D(k)*c*x**(k-1)
                    for k, c in enumerate(polynomial) if k > 1)
                e, de = e*100, de*100
            else:
                cp, latent0 = (cw, D('2501000')) if phase == 'liquid' else (ci, D('2834700'))
                a = cp - cpv
                # Integrated Clausius-Clapeyron in log-pressure coordinates.
                b = latent0 + a*tf
                log_ratio = b/rv*(1/tf-1/t) - a/rv*(t/tf).ln()
                e = e0 * log_ratio.exp()
                de = e*(b-a*t)/(rv*t*t)
            q = D('.622')*e/(p-D('.378')*e)
            dq = D('.622')*p*de/(p-D('.378')*e)**2
            return e, de, q, dq

        saturation = []
        for ts in ['263.15', '273.149999', '273.15', '273.150001', '278.15']:
            t = D(ts)
            for phase in ['liquid', 'ice']:
                # Above Tf an ice evaluation is only a join/control value,
                # never a physical warm-ice admission.
                e, de, q, dq = sat(t, phase)
                row = dict(temperature_k=ts, phase_control=phase,
                           pressure_pa=str(p), e_pa=str(e), de_pa_k=str(de),
                           q_kg_kg=str(q), dq_kg_kg_k=str(dq))
                if t != tf:
                    h = min(D('0.00001'), abs(t-tf)/4)
                    fd = (sat(t+h, phase)[2]-sat(t-h, phase)[2])/(2*h)
                    relative = abs((fd-dq)/dq)
                    row['centered_step_k'] = str(h)
                    row['derivative_relative_difference'] = str(relative)
                    assert relative <= D('1e-6')
                saturation.append(row)
        assert sat(D('263.15'), 'liquid')[2] > sat(D('263.15'), 'ice')[2]

        inverse = []
        for hs in ['0', '-6006.6', '-6006.600001', '1']:
            m, h = D('.018'), D(hs)
            if h >= 0:
                ml, mi, t, phase = m, D(0), tf+h/(m*cw), 'liquid'
            elif h >= -lf*m:
                mi = -h/lf
                ml, t, phase = m-mi, tf, 'mixed'
            else:
                ml, mi, t, phase = D(0), m, tf+(h+lf*m)/(m*ci), 'ice'
            reconstruction = ml*cw*(t-tf)+mi*(ci*(t-tf)-lf)
            assert abs(reconstruction-h) < D('1e-50')
            inverse.append(dict(mass_kg_m2=str(m), enthalpy_j_m2=hs,
                phase=phase, liquid_mass=str(ml), ice_mass=str(mi), temperature_k=str(t)))

        ledger = []
        t, dt, m0 = D('263.15'), D(60), D('.018')
        hi, hv = ci*(t-tf)-lf, D('2501000')+cpv*(t-tf)
        for es in ['1e-8', '-1e-8']:
            e = D(es)
            m1 = m0-dt*e
            q_nonvapor = e*(hv-hi)
            h0, h1 = m0*hi, m1*hi
            residual = h1-h0-dt*(q_nonvapor-e*hv)
            assert residual == 0
            ledger.append(dict(e_kg_m2_s=es, dt_s=str(dt), temperature_k=str(t),
                m0=str(m0), m1=str(m1), h0=str(h0), h1=str(h1),
                q_nonvapor_w_m2=str(q_nonvapor), hv_j_kg=str(hv),
                expected_mass_residual='0', expected_energy_residual='0',
                extra_fusion_debit_poison_j_m2=str(dt*e*lf)))
        return dict(evidence_class='Independent analytical component reference only; no Rust/model execution',
            decimal_precision=60, saturation=saturation, inverse=inverse,
            constant_temperature_ice_vapor_controls=ledger)


if __name__ == '__main__':
    output = controls()
    output['generator_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    path = Path(__file__).with_name('reference-components-draft03.json')
    path.write_text(json.dumps(output, indent=2)+'\n')
    print(f'Wrote {path}: 10 saturation, 4 inverse, 2 analytic ledger controls')
