import unittest
import compare

class ComparativeBands(unittest.TestCase):
    def test_allowance_uses_reference_only(self):
        good=compare.compare_value('root_total_mm',100.0,100.5)
        self.assertEqual(good['allowances']['tight'],0.6)
        self.assertTrue(good['passes_available_value']['tight'])
        bad=compare.compare_value('root_total_mm',0.0,1000.0)
        self.assertEqual(bad['allowances']['tight'],0.1)
        self.assertFalse(bad['passes_available_value']['coarse'])
        self.assertIsNone(bad['relative_difference'])
        self.assertEqual(compare.band_key('final_snapshot_thaw_depth_m'),'frost_depth_m')
    def test_missing_and_unattributed_delivery_never_pass(self):
        r=compare.compare_value('snow_liquid_allocated_mm',1.0,1.0)
        self.assertNotIn('passes_available_value',r)
        self.assertEqual(r['band_status'],'UNBANDED_DIAGNOSTIC')
        with self.assertRaises(ValueError):compare.compare_value('swe_mm',0.0,float('nan'))

class CommonGenerationGrid(unittest.TestCase):
    def projection(self,rows,end):
        return dict(generation_coverage=[dict(day=0,lane=1,observed_start_s=0,observed_end_s=end)],
            generation_series=[dict(day=0,lane=1,start_s=start,duration_s=duration,generated_runoff_mm=depth) for start,duration,depth in rows])
    def test_different_partition_preserves_integral_without_tail_fill(self):
        r=self.projection([(0,600,2)],600)
        c=self.projection([(0,150,.5),(150,450,1.5)],600)
        row=compare.compare_observed_generation(r,c)['rows'][0]
        self.assertEqual(len(row['bins']),2)
        self.assertEqual(row['cumulative_signed_difference_mm'],0)
        self.assertEqual(row['maximum_absolute_rate_difference_mm_h'],0)
        self.assertFalse(row['full_common_day'])
        short=self.projection([(0,300,2)],300)
        row=compare.compare_observed_generation(r,short)['rows'][0]
        self.assertEqual(len(row['bins']),1)
        self.assertEqual(row['cumulative_signed_difference_mm'],1)
    def test_gap_or_overlap_is_rejected(self):
        r=self.projection([(0,600,2)],600)
        for rows in [[(0,300,1)],[(0,600,2),(0,300,1)]]:
            with self.assertRaises(ValueError):compare.compare_observed_generation(r,self.projection(rows,600))

class WorkBasis(unittest.TestCase):
    def test_batch_is_not_reported_as_zero_physical_work(self):
        def run(n):return dict(carrier=dict(counts=[dict(completed=0)]*4+[dict(completed=n)]),lse=dict(maps=dict(completions=n)))
        result=compare.work_comparison(run(64),run(36))
        self.assertIsNone(result['scalar_carrier_requests']['reduction_fraction'])
        self.assertEqual(result['batch_physical_requests']['reduction_fraction'],.4375)
        self.assertEqual(result['shared_physical_maps']['candidate'],36)

if __name__=='__main__':unittest.main()
