"""Independent Decimal cold/warm dark-leaf and FullSupply hydraulic controls.

Prescribed trial temperatures and wet fraction; this is not a canopy simulator.
The hydraulic coordinates follow exact algebraic elimination of the retained
four continuity equations and are checked by separately reconstructing fluxes.
"""
from decimal import Decimal as D, localcontext
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def generate():
    original_path = HERE / 'original-input-reference.json'
    saturation_path = HERE / 'reference-controls-m1.json'
    original = json.loads(original_path.read_text(), parse_float=D)['inputs']
    saturation = json.loads(saturation_path.read_text())['saturation']
    rows = []
    with localcontext() as context:
        context.prec = 60
        gas_r, pressure, canopy_t, canopy_q = map(D, ('8.31446261815324', '90000', '268', '.0015'))
        tf, reference, wet_fraction = map(D, ('273.15', '298.15', '.25'))
        density = pressure / (D('287.05') * canopy_t)
        for temperature_text in ('263.15', '278.15'):
            temperature = D(temperature_text)
            qsat = D(next(row['q_kg_kg'] for row in saturation
                         if row['temperature_k'] == temperature_text and row['phase_control'] == 'liquid'))
            for occupancy in original['occupancies']:
                def vulnerability(potential, p50):
                    return (-D(2).ln() * (potential / p50) ** occupancy['vulnerability_exponent']).exp()

                leaves = []
                for name in ('sun', 'shade'):
                    leaf = occupancy[name]
                    gs = occupancy['g0_umol_m2_s'] * D('1e-6') * gas_r * temperature / pressure
                    rs, rb = 1 / gs, 1 / occupancy['gb_leaf_m_s']
                    rd_factor = (D(46390) / gas_r * (1 / reference - 1 / temperature)).exp()
                    rd_factor *= (1 + ((reference * 490 - 150650) / (gas_r * reference)).exp())
                    rd_factor /= (1 + ((temperature * 490 - 150650) / (gas_r * temperature)).exp())
                    rd = leaf['rd25'] * rd_factor
                    ci = original['ca_pa'] + (D('1.4') * rb + D('1.6') * rs) * gas_r * temperature * rd * D('1e-6')
                    flux = density * (qsat - canopy_q) / (rb + rs) * leaf['leaf_area_m2_m2_tile'] * (1 - wet_fraction)
                    leaves.append(dict(name=name, gas_branch='Inactive' if leaf['leaf_area_m2_m2_tile'] == 0 else 'ExactZeroPar',
                                       beta=D(1), rd=rd, an=-rd, ag=D(0), gs_m_s=gs,
                                       ci_pa=ci, vapor_kg_m2_tile_s=flux))
                demand = sum(leaf['vapor_kg_m2_tile_s'] for leaf in leaves)
                root_coefficients = []
                for layer in occupancy['root_layers']:
                    if layer['accessible'] and not layer['frozen'] and layer['root_fraction'] > 0:
                        kr = occupancy['k3_max_m_s'] / layer['z3_m'] * vulnerability(layer['soil_potential_mm'], occupancy['p50_root_mm'])
                        ks = layer['ksoil_m2_s'] / layer['dxroot_m']
                        coefficient = kr * ks / (kr + ks) * (occupancy['lai'] + occupancy['sai']) * layer['root_fraction'] * occupancy['root_to_leaf_area']
                        root_coefficients.append((layer, coefficient))
                psi_root = (sum(coefficient * (layer['soil_potential_mm'] + layer['gravity_head_mm'])
                                for layer, coefficient in root_coefficients) - demand) / sum(coefficient for _, coefficient in root_coefficients)
                stem_coefficient = occupancy['k2_max'] / occupancy['height_m'] * vulnerability(psi_root, occupancy['p50_xylem_mm']) * occupancy['sai']
                psi_stem = psi_root - 1000 * occupancy['height_m'] - demand / stem_coefficient
                leaf_potentials = []
                branch_fluxes = []
                for leaf in leaves:
                    name = leaf['name']
                    coefficient = occupancy['k1_' + name + '_max_s1'] * occupancy[name]['leaf_area_m2_m2_tile'] * vulnerability(psi_stem, occupancy['p50_xylem_mm'])
                    psi_leaf = psi_stem if coefficient == 0 else psi_stem - leaf['vapor_kg_m2_tile_s'] / coefficient
                    leaf_potentials.append(psi_leaf)
                    branch_fluxes.append(coefficient * (psi_stem - psi_leaf))
                root_fluxes = [coefficient * (layer['soil_potential_mm'] - psi_root + layer['gravity_head_mm']) for layer, coefficient in root_coefficients]
                stem_flux = stem_coefficient * (psi_root - psi_stem - 1000 * occupancy['height_m'])
                residuals = [branch_fluxes[i] - leaves[i]['vapor_kg_m2_tile_s'] for i in range(2)]
                residuals += [sum(branch_fluxes) - stem_flux, stem_flux - sum(root_fluxes)]
                assert max(map(abs, residuals)) < D('1e-45')
                assert demand > 0 and leaves[1]['rd'] > 0 and leaves[1]['ci_pa'] > original['ca_pa']
                rows.append(dict(occupancy_id=occupancy['occupancy_id'], leaf_temperature_k=temperature,
                                 internal_liquid_qsat=qsat, leaves=leaves, total_leaf_demand=demand,
                                 potentials_mm=[*leaf_potentials, psi_stem, psi_root],
                                 root_layer_ids=[layer['layer_id'] for layer, _ in root_coefficients],
                                 root_fluxes=root_fluxes, stem_flux=stem_flux, continuity_residuals=residuals))
        return dict(evidence_class='Static independent Decimal prescribed-trial constitutive/continuity controls; no coupled simulation',
                    inputs=dict(pressure_pa=pressure, canopy_temperature_k=canopy_t, canopy_q=canopy_q, wet_fraction=wet_fraction),
                    source_input_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in (original_path, saturation_path)},
                    comparisons=dict(rule='abs(actual-reference) <= max(absolute, relative*abs(reference)); every actual value finite',
                                     respiration_assimilation=dict(absolute='1e-12', relative='1e-8'),
                                     conductance_m_s=dict(absolute='1e-15', relative='1e-8'),
                                     ci_pa=dict(absolute='1e-7', relative='1e-8'),
                                     specific_humidity=dict(absolute='1e-12', relative='1e-8'),
                                     water_flux_kg_m2_tile_s=dict(absolute='1e-16', relative='1e-8'),
                                     hydraulic_potential_mm=dict(absolute='1e-7', relative='0'),
                                     continuity_residual_kg_m2_tile_s=dict(absolute='1e-12', relative='0'),
                                     discrete='Exact gas branch, occupancy/layer ordering, inactive sun identity and beta=1. No scalar tolerance substitutes for a discrete predicate.'),
                    authority='Retained CLM peaked Rd and zero-PAR diffusion; M1 liquid internal saturation; FullSupply four-potential continuity. Exact algebraic elimination is a reference construction, not a new production solver claim.',
                    rows=rows, required_comparison='Existing gas/hydraulic consumer must match values and reconstruct every flux/guard; no scalar-only or positive-foliage bypass acceptance.')


if __name__ == '__main__':
    result = generate()
    result['generator_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    path = HERE / 'dark-gas-reference-m1.json'
    path.write_text(json.dumps(result, default=str, indent=2) + '\n')
    print(path)
