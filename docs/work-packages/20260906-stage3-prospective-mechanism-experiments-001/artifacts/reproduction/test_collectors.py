#!/usr/bin/env python3
"""Offline negative controls for narrow manifest exceptions and stencil enumeration."""
import copy
import json
from pathlib import Path
import unittest
from run_series import FRAME_POINTER, exact_json, frame_work_comparison, frame_work_rule, record_identity, validate_memory
from audit_stencils import bits, enumerate_sweep, proof_closed_component
from analyze_series import summarize

ROOT=Path(__file__).resolve().parents[1]

class CollectorTests(unittest.TestCase):
    def setUp(self):
        records=[json.loads(line.split('STAGE3_CONTROLLED_MECHANISM ',1)[1])
                 for line in (ROOT/'raw/resume-20260908-A05-admission.log').read_text().splitlines()
                 if 'STAGE3_CONTROLLED_MECHANISM ' in line]
        self.record=records[0]
        self.frozen=json.loads((ROOT/'raw/resume-20260908-A05-identity.json').read_text())

    def test_every_retained_scientific_leaf_is_exact(self):
        original=exact_json(self.frozen['common_identity'])
        for field in ('laned_source_m3','accepted_publication_support_count'):
            bad=copy.deepcopy(self.record)
            bad[field]+=1
            self.assertNotEqual(original,exact_json(record_identity(bad,self.frozen['manifest_rules'])))
        bad=copy.deepcopy(self.record)
        bad['output_manifest']['coupling_vectors']['winter']['runtime_swe']=0.0
        self.assertNotEqual(original,exact_json(record_identity(bad,self.frozen['manifest_rules'])))

    def test_identity_and_path_poisons_fail(self):
        for field in ('binary_sha256','source_commit','run_file'):
            bad=copy.deepcopy(self.record)
            bad['output_manifest'][field]='foreign'
            with self.assertRaises(ValueError):
                record_identity(bad,self.frozen['manifest_rules'])

    def test_current_a_runs_have_same_identity(self):
        for name in ('resume-20260908-A05-admission.log',):
            for line in (ROOT/'raw'/name).read_text().splitlines():
                if 'STAGE3_CONTROLLED_MECHANISM ' in line:
                    record=json.loads(line.split('STAGE3_CONTROLLED_MECHANISM ',1)[1])
                    self.assertEqual(exact_json(self.frozen['common_identity']),
                                     exact_json(record_identity(record,self.frozen['manifest_rules'])))

    def test_boundary_stencils_are_not_centered_by_fiat(self):
        coordinates=[-100.,-100.,-100.,-100.,0.,1.,273.15,300.,300.,300.,300.,.01,273.15,280.]
        base=dict(occupancies=1,soil_nodes=1,represented_snow=True,liquid_ground=True,
                  base_bits=list(map(bits,coordinates)))
        probes,stencils=enumerate_sweep(base,True)
        self.assertEqual(len(probes),24)
        self.assertEqual(stencils[4][3:],(False,True))
        self.assertEqual(stencils[5][3:],(True,False))
        self.assertEqual(sum(p[2]=='ComponentReplay' for p in probes),6)
        self.assertEqual(sum(p[2]=='IdentityAnchor' for p in probes),3)

    def test_pc1_independent_raw_beta_classification(self):
        import math
        for n in (1,2,4):
            for occupancy in range(n):
                for local in (6,7,8,9):
                    for beta in (1.,math.nextafter(1.,0.),0.,.5):
                        coordinates=[.5]*(10*n+9)
                        for offset in (4,5): coordinates[10*occupancy+offset]=beta
                        self.assertEqual(proof_closed_component(coordinates,10*occupancy+local,n),
                                         local in (8,9) or bits(beta)==bits(1.))
                for sun,shade in ((1.,math.nextafter(1.,0.)),
                                  (math.nextafter(1.,0.),1.)):
                    coordinates=[.5]*(10*n+9)
                    coordinates[10*occupancy+4]=sun
                    coordinates[10*occupancy+5]=shade
                    for local,beta in ((6,sun),(7,shade)):
                        self.assertEqual(proof_closed_component(coordinates,10*occupancy+local,n),
                                         bits(beta)==bits(1.))

    def test_memory_lifecycle_negative_controls(self):
        log=(ROOT/'raw/A-admission-01.log').read_text().splitlines()
        records=[json.loads(line.split('STAGE3_CONTROLLED_MECHANISM ',1)[1]) for line in log
                 if 'STAGE3_CONTROLLED_MECHANISM ' in line]
        phases=[json.loads(line.split('STAGE3_CONTROLLED_MEMORY ',1)[1]) for line in log
                if 'STAGE3_CONTROLLED_MEMORY ' in line]
        record=records[-1]
        pid=phases[0]['pid']
        sample={'pid':pid,'monotonic_ns':record['run_start_monotonic_ns']+1,
                'read_end_monotonic_ns':record['run_start_monotonic_ns']+2,'Cpus_allowed_list':'0'}
        maxima=[{'samples':1,'maximum_rss_kib':100}]
        validate_memory([record],phases,[sample],maxima,pid,1,0)
        for mutation in ('duplicate','pid','clock','affinity','active'):
            altered=copy.deepcopy(phases)
            samples=[dict(sample)]
            maxes=copy.deepcopy(maxima)
            if mutation=='duplicate': altered[2]['phase']='pre_run'
            if mutation=='pid': altered[2]['pid']+=1
            if mutation=='clock': altered[2]['monotonic_ns']=0
            if mutation=='affinity': samples[0]['Cpus_allowed_list']='1'
            if mutation=='active': maxes[0]['samples']=0
            with self.assertRaises(ValueError,msg=mutation):
                validate_memory([record],altered,samples,maxes,pid,1,0)

    def test_prespecified_statistics_and_failure_retention(self):
        rows=[]
        for pair in range(12):
            for arm,wall in (('A',1000000),('B',900000)):
                rows.append(dict(valid=True,label='measured',series='synthetic',pair=pair,
                                 arm=arm,records=[dict(run_wall_us=wall,cpu_ticks=wall//10000,clock_tick_hz=100)],
                                 process_user_s=wall/1e6,process_system_s=0.,lifetime_peak_rss_kib=100,
                                 observer_requested=False))
        result=summarize(rows)
        self.assertEqual(result['pair_count'],12)
        self.assertAlmostEqual(result['median_wall_improvement'],.1)
        self.assertAlmostEqual(result['median_seconds_saved'],.1)
        self.assertEqual(result['bootstrap']['median_improvement_95pct'],[1-.9,1-.9])
        self.assertTrue(result['timing_standalone_predicate'])
        rows[-1]['valid']=False
        with self.assertRaises(ValueError):
            summarize(rows)

    def test_mechanism_frame_work_is_not_volatile(self):
        rule=frame_work_rule(self.record)
        self.assertEqual(frame_work_comparison(rule)['noncarrier_constructions'],405)
        for field in ('ofe_count','provider_count','total_constructions','mechanism'):
            bad=copy.deepcopy(self.frozen['manifest_rules'])
            bad[FRAME_POINTER][field]='foreign'
            with self.assertRaises((ValueError,TypeError)):
                record_identity(self.record,bad)
        for mutation in ('counter','error','drop','carrier'):
            bad=copy.deepcopy(self.record)
            if mutation=='counter': bad['output_manifest']['direct_runtime_counters']['day_frame_constructions']+=1
            if mutation=='error': bad['carrier_counts']['counts'][2]['errors']=1
            if mutation=='drop': bad['carrier_counts']['dropped_records']=1
            if mutation=='carrier': bad['carrier_counts']['counts'][3]['completed']-=1
            with self.assertRaises(ValueError): frame_work_rule(bad)
        # Synthetic arithmetic controls only; these do not admit scale fixtures.
        for n in (10,19):
            a=dict(mechanism='F-two-full-lane-seeds',ofe_count=n,
                   provider_count=400*n,total_constructions=2*n*400*n+777)
            f=dict(a,provider_count=200*n,total_constructions=2*n*200*n+777)
            self.assertEqual(frame_work_comparison(a),frame_work_comparison(f))
            f['total_constructions']+=1
            self.assertNotEqual(frame_work_comparison(a),frame_work_comparison(f))

if __name__=='__main__':
    unittest.main()
