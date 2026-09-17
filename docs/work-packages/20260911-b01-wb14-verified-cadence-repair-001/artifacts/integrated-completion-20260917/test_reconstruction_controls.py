"""Deliberate evidence-corruption controls; no physical fixture execution."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

import reconstruct_credits as credits
import reconstruct_donors as donors
import reconstruct_install as installation


def replace_record(raw, tag, mutate, index=0):
    lines = raw.decode().splitlines()
    matches = [i for i, line in enumerate(lines) if tag + ' ' in line]
    position = matches[index]
    prefix, payload = lines[position].split(tag + ' ', 1)
    value = json.loads(payload)
    mutate(value)
    lines[position] = prefix + tag + ' ' + json.dumps(value)
    return ('\n'.join(lines) + '\n').encode()


def check(raw):
    # Positive controls bind all negatives to parsable, conforming raw evidence.
    credits.reconstruct(raw)
    donors.reconstruct(raw)
    installation.reconstruct(raw)
    lineage = 'B01_INTEGRATED_LINEAGE'
    phase = donors.PHASE_TAG
    phase0 = lambda value: value['selected_phase_chain'][0]
    installed = 'B01_INTEGRATED_INSTALLED'
    def alter_parent(value):
        clock = json.loads(bytes(value['parent']['beginning_clock_canonical_json']))
        clock['parent_transaction_id'] = '0'*64
        value['parent']['beginning_clock_canonical_json'] = list(json.dumps(clock).encode())
    extra_state = replace_record(raw, lineage, lambda v: v['physical_trial']['selected_ending_state'].__setitem__('unsealed_extra', 'fabricated'))
    extra_state = replace_record(extra_state, installed, lambda v: v['soil_owner']['state'].__setitem__('unsealed_extra', 'fabricated'))
    cases = [
        ('receiver_missing_operand', credits.reconstruct,
         replace_record(raw, lineage, lambda v: v['accumulated_operands'].pop())),
        ('receiver_wrong_ordinal', credits.reconstruct,
         replace_record(raw, lineage, lambda v: v['accumulated_operands'][2].__setitem__('ordinal', 99))),
        ('receiver_duplicate_layer', credits.reconstruct,
         replace_record(raw, lineage, lambda v: v['ordered_layer_credit_chain'][0].__setitem__(1, copy.deepcopy(v['ordered_layer_credit_chain'][0][0])))),
        ('donor_wrong_source_transaction', donors.reconstruct,
         replace_record(raw, phase, lambda v: phase0(v).__setitem__('child_transaction_id', 999))),
        ('donor_missing_selected_phase', donors.reconstruct,
         replace_record(raw, phase, lambda v: v['selected_phase_chain'].clear())),
        ('donor_wrong_source_primitive', donors.reconstruct,
         replace_record(raw, phase, lambda v: phase0(v)['pre_ingress_soil_thermal'][0]['layers'][0].__setitem__('ending_enthalpy_bits', 0))),
        ('donor_unselected_receipt_substitution', donors.reconstruct,
         replace_record(raw, phase, lambda v: phase0(v)['top_boundary_credits_by_lane']['1'].__setitem__('snow_soil_heat_receipt_sha256', '0'*64))),
        ('donor_wrong_area_basis', donors.reconstruct,
         replace_record(raw, lineage, lambda v: v['ordered_layer_credit_chain'][0][0]['accepted_operands'][0].__setitem__('basis', 'tile'))),
        ('donor_endpoint_success_false', donors.reconstruct,
         replace_record(raw, phase, lambda v: v.__setitem__('endpoint_finalizer_succeeded', False))),
        ('reseal_wrong_canonical_identity', donors.reconstruct,
         replace_record(raw, donors.RESEAL_TAG, lambda v: v['old_identity'].__setitem__('attempt_ordinal', 99))),
        ('reseal_wrong_selected_ending', donors.reconstruct,
         replace_record(raw, donors.RESEAL_TAG, lambda v: v.__setitem__('carrier_ending_joint_sha256', '0'*64))),
        ('reseal_wrong_phase_probe', donors.reconstruct,
         replace_record(raw, phase, lambda v: phase0(v).__setitem__('probe_child_receipt_sha256', '0'*64))),
        ('reseal_wrong_retained_source', donors.reconstruct,
         replace_record(raw, donors.RESEAL_TAG, lambda v: v['trial_source_receipts_by_lane']['1'].__setitem__('canonical_source_sha256', '0'*64))),
        ('donor_wrong_parent_history', donors.reconstruct,
         replace_record(raw, 'B01_INTEGRATED_PARENT', alter_parent)),
        ('install_wrong_original_namespace', installation.reconstruct,
         replace_record(raw, lineage, lambda v: v['original_prepared_owner'].__setitem__('transaction_id', 47))),
        ('install_unsealed_extra_state_fields', installation.reconstruct, extra_state),
        ('install_unsealed_extra_expected_sources', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['expected_sources'].__setitem__('unsealed_extra', 'fabricated'))),
        ('install_wrong_selected_state', installation.reconstruct,
         replace_record(raw, lineage, lambda v: v['physical_trial']['selected_ending_state']['ofes'][0]['ordered_layers'][0].__setitem__('temperature_k', 290.0))),
        ('install_wrong_installed_state', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_owner']['state']['ofes'][0]['ordered_layers'][0].__setitem__('enthalpy_hi_j_m2_ofe_ground', 0.0))),
        ('install_wrong_receipt_hash', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['credit_receipt'].__setitem__('receipt_sha256', '0'*64))),
        ('install_wrong_expected_source_seal', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['expected_sources'].__setitem__('expected_set_sha256', '0'*64))),
        ('install_wrong_restart_seal', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['seals']['restart'].__setitem__('restart_sha256', '0'*64))),
        ('install_wrong_checkpoint_seal', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['seals']['checkpoint'].__setitem__('checkpoint_sha256', '0'*64))),
        ('install_wrong_orchestrator_seal', installation.reconstruct,
         replace_record(raw, installed, lambda v: v['soil_latest_accepted']['seals'].__setitem__('orchestrator_seal_sha256', '0'*64))),
    ]
    for tag in (lineage, 'B01_INTEGRATED_INSTALLED', phase, donors.RESEAL_TAG):
        lines = raw.decode().splitlines()
        matched = [line for line in lines if tag + ' ' in line]
        fn = donors.reconstruct if tag in (phase, donors.RESEAL_TAG) else credits.reconstruct
        cases.extend([
            (tag + '_missing', fn, ('\n'.join(line for line in lines if tag + ' ' not in line)+'\n').encode()),
            (tag + '_duplicate', fn, raw+b'\n'+matched[0].encode()+b'\n'),
            (tag + '_malformed', fn, raw+b'\n'+tag.encode()+b' {bad}\n'),
            (tag + '_duplicate_key', fn, raw+b'\n'+tag.encode()+b' {"x":1,"x":2}\n'),
        ])
    outcomes = []
    for name, function, changed in cases:
        try:
            function(changed)
        except (ValueError, KeyError, TypeError, OverflowError, AssertionError) as error:
            outcomes.append(dict(case=name, rejected=True, error=str(error)))
        else:
            outcomes.append(dict(case=name, rejected=False))
    return outcomes


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stdout', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    raw = args.stdout.read_bytes()
    script_paths = (Path(__file__), Path(credits.__file__), Path(donors.__file__),
                    Path(installation.__file__), Path(__file__).with_name('source_digest_helpers.py'))
    starting_scripts = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in script_paths}
    outcomes = check(raw)
    ending_scripts = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in script_paths}
    result = dict(input=str(args.stdout), input_sha256=hashlib.sha256(raw).hexdigest(),
                  scope='Evidence reconstruction corruption controls only; no physical runs',
                  cases=outcomes, passed=all(row['rejected'] for row in outcomes) and starting_scripts == ending_scripts,
                  scripts=starting_scripts, scripts_unchanged=starting_scripts == ending_scripts)
    args.output.write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(dict(passed=result['passed'], rejected=sum(row['rejected'] for row in outcomes), cases=len(outcomes))))
    raise SystemExit(0 if result['passed'] else 1)


if __name__ == '__main__':
    main()
