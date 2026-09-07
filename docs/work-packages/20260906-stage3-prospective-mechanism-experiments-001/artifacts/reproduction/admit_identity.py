#!/usr/bin/env python3
"""Freeze output identity, not scientific admission; gates remain separately mandatory."""
import argparse
import json
from pathlib import Path
from run_series import FRAME_POINTER, digest, exact_json, frame_work_rule, record_identity

RULES = {
    '/invoked_utc':'invoked_utc', '/run_dir':'run_path', '/run_file':'run_path',
    '/input_checksums':'run_path_keys', '/output_checksums':'run_path_keys',
    '/stage3_evidence_archive/output_path':'run_path',
    '/wat5_output/output_path':'run_path',
    **{'/resolved_sidecars/' + key:'run_path' for key in
       ('frost','pmetpara','snow','snow_stage3_v11_owner_seed','wepp_ui')},
}

def identity(record, binary, source):
    binary = Path(binary).resolve()
    sidecar = Path(str(binary) + '.json')
    metadata = json.loads(sidecar.read_text())
    expected = {'binary_path':str(binary), 'binary_sha256':digest(binary),
                'binary_sidecar_path':str(sidecar), 'binary_sidecar_sha256':digest(sidecar),
                'source_commit':source['checkout']}
    if metadata['sha256'] != expected['binary_sha256'] or metadata['source_commit'] != expected['source_commit']:
        raise ValueError('binary sidecar does not bind frozen executable and checkout')
    rules = dict(RULES)
    rules[FRAME_POINTER] = frame_work_rule(record)
    for field,value in expected.items():
        if record['output_manifest'][field] != value:
            raise ValueError('manifest provenance mismatch: ' + field)
        rules['/' + field] = {'expected_arm_identity':value}
    return {'manifest_rules':rules, 'common_identity':record_identity(record,rules),
            'carrier_counts':record['carrier_counts'], 'lse_counts':record['lse_counts'],
            'source_identity':source['source_identity'],
            'evidence_class':'identity only; scientific gates separately required'}

def main():
    p = argparse.ArgumentParser()
    for name in ('log','binary','source-manifest','out'):
        p.add_argument('--' + name, type=Path, required=True)
    p.add_argument('--compare',type=Path)
    a = p.parse_args()
    records = [json.loads(line.split('STAGE3_CONTROLLED_MECHANISM ',1)[1])
               for line in a.log.read_text().splitlines() if 'STAGE3_CONTROLLED_MECHANISM ' in line]
    if len(records) != 1:
        raise ValueError('expected one successful consumer record')
    result = identity(records[0], a.binary, json.loads(a.source_manifest.read_text()))
    if a.compare and exact_json(result['common_identity']) != exact_json(json.loads(a.compare.read_text())['common_identity']):
        raise ValueError('full scientific/input/output/control identity differs')
    result['log_sha256'] = digest(a.log)
    with a.out.open('x') as output:
        json.dump(result,output,indent=2)
    print(json.dumps({'status':'IDENTITY_FROZEN','out':str(a.out),'compared':str(a.compare)}))

if __name__ == '__main__':
    main()
