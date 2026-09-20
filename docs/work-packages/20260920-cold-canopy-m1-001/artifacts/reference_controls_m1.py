"""Independent component controls. No production model or runtime results used.
These operands exercise equations separately; they are not a coupled solution.
"""
from decimal import Decimal as D, localcontext
from pathlib import Path
import hashlib
import json
from reference_components import controls


def generate():
    out = controls()
    with localcontext() as ctx:
        ctx.prec = 60
        tf, cw, ci, lf = map(D, ('273.15', '4218', '2106', '333700'))
        m, h, dm = D('.018'), D('-6385.68'), D('1e-10')
        def ice_temperature(mass):
            return tf + (h / mass + lf) / ci
        derivative = -h / (m*m*ci)
        finite_difference = (ice_temperature(m+dm)-ice_temperature(m-dm))/(2*dm)
        assert abs((finite_difference-derivative)/derivative) < D('1e-15')
        out['ice_inverse_jacobian'] = dict(m=str(m), h=str(h), step_m=str(dm),
            expected_t=str(ice_temperature(m)), expected_dt_dm=str(derivative),
            centered_dt_dm=str(finite_difference), expected_dt_dh=str(1/(m*ci)),
            rejected_old_dt_dm=str(-(h+lf*m)/(m*m*ci)), relative_tolerance='1e-10')
        # At Tf the cold liquid thermodynamic limit and old warm polynomial
        # have distinct derivatives; do not silently equate them.
        p, e0, rv = D('90000'), D('611.213476'), D('461.52')
        limits = {}
        for phase, latent in [('liquid',D('2501000')),('ice',D('2834700'))]:
            de = e0*latent/(rv*tf*tf)
            limits[phase] = dict(de_pa_k=str(de), dq_kg_kg_k=str(D('.622')*p*de/(p-D('.378')*e0)**2))
        out['cold_saturation_join_limits'] = limits
        # Nonempty ice residual columns include the wet-area, saturation and
        # vapor-enthalpy dependencies. Hold supplied nonvapor Q fixed here.
        cap, gb, area, tcan, qcan = map(D, ('.297','.0186','1.35','266.85','.00135'))
        rho = p/(D('287.05')*tcan)
        def residual(mass, heat):
            temp = tf+(heat/mass+lf)/ci
            a=ci-D('1849')
            latent0=D('2834700')
            log_ratio=(latent0+a*tf)/rv*(1/tf-1/temp)-a/rv*(temp/tf).ln()
            ep=e0*log_ratio.exp()
            qs=D('.622')*ep/(p-D('.378')*ep)
            wet=((mass/cap).ln()*D(2)/3).exp()
            ev=rho*gb*area*wet*(qs-qcan)
            hv=D('2501000')+D('1849')*(temp-tf)
            return mass-m+D(60)*ev, heat-h+D(60)*ev*hv, ev, temp
        temp=ice_temperature(m)
        a=ci-D('1849')
        ep=e0*((D('2834700')+a*tf)/rv*(1/tf-1/temp)-a/rv*(temp/tf).ln()).exp()
        qs=D('.622')*ep/(p-D('.378')*ep)
        dqs=D('.622')*p*ep*(D('2834700')-a*(temp-tf))/(rv*temp*temp*(p-D('.378')*ep)**2)
        wet=((m/cap).ln()*D(2)/3).exp()
        ev=residual(m,h)[2]
        hv=D('2501000')+D('1849')*(temp-tf)
        columns=[]
        for coordinate, step, dt_dx in [('M',D('1e-10'),derivative),('H',D('1e-6'),1/(m*ci))]:
            de_dx=rho*gb*area*wet*(dqs*dt_dx+(D(2)/(3*m)*(qs-qcan) if coordinate=='M' else 0))
            analytic=[(D(1) if coordinate=='M' else D(0))+60*de_dx,
                      (D(1) if coordinate=='H' else D(0))+60*(de_dx*hv+ev*D('1849')*dt_dx)]
            plus=residual(m+step,h) if coordinate=='M' else residual(m,h+step)
            minus=residual(m-step,h) if coordinate=='M' else residual(m,h-step)
            fd=[(plus[j]-minus[j])/(2*step) for j in range(2)]
            assert all(abs((x-y)/x)<D('1e-10') for x,y in zip(analytic,fd))
            columns.append(dict(coordinate=coordinate,step=str(step),analytic=list(map(str,analytic)),finite_difference=list(map(str,fd))))
        out['assembled_reservoir_column_control']=dict(m=str(m),h=str(h),dt='60',q_nonvapor='0',i='0',d='0',
            capacity=str(cap),gb=str(gb),plant_area=str(area),tcan=str(tcan),qcan=str(qcan),pressure=str(p),
            columns=columns,relative_tolerance='1e-6',scope='Fixed nonvapor heat component; coupled radiation columns tested separately')
        # Capacity-active melting control: the heat melts exactly the drainage.
        dt, drain, cap = D(60), D('1e-8'), D('.297')
        m0, m1 = D('.3970006'), D('.397')
        h0, h1 = -lf*(m0-cap), -lf*(m1-cap)
        q = (h1-h0)/dt
        assert m1-m0+dt*drain == 0
        out['capacity_controls'] = [dict(name='capacity_active_melting', dt=str(dt),
            m0=str(m0), m1=str(m1), h0=str(h0), h1=str(h1), capacity=str(cap),
            liquid_mass=str(cap), d=str(drain), e='0', i='0', hl='0', q=str(q),
            expected_rm='0', expected_rh='0', expected_rcap='0'),
            dict(name='capacity_tie', dt='60', liquid_mass='.297', capacity='.297',
                d='0', expected_rcap='0', selected_derivative='capacity'),
            dict(name='overcapacity_trial', dt='60', liquid_mass='.29700001',
                capacity='.297', d='0', expected_rcap='-1e-8',
                expected_trial='residual_evaluable', expected_acceptance='reject'),
            dict(name='negative_drainage', dt='60', liquid_mass='.297',
                capacity='.297', d='-1e-8', expected_acceptance='reject')]
        out['empty_surface_controls'] = [dict(name='cold_'+surface, m='0', h='0', i='0',
            temperature_k='263.15', pressure_pa='90000', qcan='.0019',
            positive_area_surface=surface, area_m2_m2='1', expected_error='VEG-E-142')
            for surface in ['sun','shade','stem']]
        out['empty_surface_controls'] += [
            dict(name='cold_unsaturated',m='0',h='0',i='0',temperature_k='263.15',
                pressure_pa='90000',qcan='.0017',positive_area_surface='shade',
                area_m2_m2='1',expected_e='0',expected_q='0',expected_d='0'),
            dict(name='cold_zero_area_excluded',m='0',h='0',i='0',temperature_k='263.15',
                pressure_pa='90000',qcan='.0019',tested_surface='sun',area_m2_m2='0',
                other_positive_surface_temperature_k='273.15',expected_empty_guard='pass'),
            dict(name='combined_missing_condition_and_supersaturation',m='0',h='0',i='0',
                temperature_k='263.15',pressure_pa='90000',qcan='.0019',
                liquid_conducting_tissue=False,expected_error='VEG-E-140')]
        out['wet_shortwave_control'] = dict(fwet='.25',sw_sun='8',sw_shade='4',sw_stem='2',
            reciprocal_wet_longwave='-2',hwet='1',expected_swwet='3.5',expected_q='.5',
            dt='60',expected_energy_difference_if_sw_omitted='210',
            scope='Independent heat-operand arithmetic, not a solved coupled state')
        # Existing geometric routing applied separately to each source enthalpy.
        def tanh(x):
            v=(2*x).exp()
            return (v-1)/(v+1)
        def route(mass, enthalpy, area, capture, stem):
            captured=mass*capture*tanh(area)
            free=mass-captured
            return [('capture',captured,enthalpy),('stemflow',free*stem,enthalpy),
                    ('throughfall',free*(1-stem),enthalpy)]
        top_mass, top_h = D('.04'), cw*5
        upper = route(top_mass,top_h,D('1.35'),D('.5'),D('.1'))
        lower_inputs = [('upper_throughfall',upper[2][1],top_h),('upper_drainage',D('.006'),D(0))]
        lower = [(origin,kind,mass,specific) for origin,mass_in,specific in lower_inputs
                 for kind,mass,specific in route(mass_in,specific,D('.9'),D('.5'),D('.2'))]
        terminal = [('upper_stemflow',upper[1][1],top_h)] + [
            (origin+'_'+kind,mass,specific) for origin,kind,mass,specific in lower if kind!='capture']
        terminal += [('lower_drainage',D('.002'),cw*2)]
        captured_mass=upper[0][1]+sum(row[2] for row in lower if row[1]=='capture')
        captured_energy=upper[0][1]*top_h+sum(row[2]*row[3] for row in lower if row[1]=='capture')
        ground_mass=sum(row[1] for row in terminal)
        ground_energy=sum(row[1]*row[2] for row in terminal)
        assert abs(captured_mass+ground_mass-D('.048')) < D('1e-55')
        assert abs(captured_energy+ground_energy-D('860.472')) < D('1e-55')
        out['routing_control'] = dict(scope='Geometric routing only; prescribed drainage operands are not a canopy solution',
            top_input_mass='.04',top_input_h=str(top_h),upper_plant_area='1.35',lower_plant_area='.9',
            interception_fraction='.5',upper_stemflow_fraction='.1',lower_stemflow_fraction='.2',
            upper_drainage_mass='.006',upper_drainage_h='0',lower_drainage_mass='.002',lower_drainage_h=str(cw*2),
            upper=[dict(kind=k,mass=str(v),h=str(e)) for k,v,e in upper],
            lower=[dict(origin=o,kind=k,mass=str(v),h=str(e)) for o,k,v,e in lower],
            terminal=[dict(origin=o,mass=str(v),h=str(e),energy=str(v*e),
                ofe_mass=str(v*D('.38')),ofe_energy=str(v*e*D('.38'))) for o,v,e in terminal],
            tile_fraction='.38',ground_mass=str(ground_mass),ground_energy=str(ground_energy),
            captured_mass=str(captured_mass),captured_energy=str(captured_energy),
            mass_absolute_tolerance='1e-12',energy_absolute_tolerance='1e-8',
            poisons=['swap upper/lower order','apply tile fraction twice','route upper stemflow to lower',
                     'assign upper drainage the rain enthalpy','send all upper release into lower reservoir'])
    return out


if __name__ == '__main__':
    output=generate()
    output['generator_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    output['inherited_generator_sha256']=hashlib.sha256(Path(__file__).with_name('reference_components.py').read_bytes()).hexdigest()
    path=Path(__file__).with_name('reference-controls-m1.json')
    path.write_text(json.dumps(output,indent=2)+'\n')
    print(path)
