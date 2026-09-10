import unittest
from project_outputs import daily_projection, validate_generation_rows, add_final_front_depths

class SpatialProjection(unittest.TestCase):
    def rows(self):
        return [dict(sim_day_index=d,ofe_id=l,OFE=l,Area=a,Q=9/l,frdp=10*l,
            **{'Snow-Water':2*l,'SoilWaterTotal':30*l})
            for d in (1,2) for l,a in ((1,100),(2,300))]
    def passes(self):
        return [dict(sim_day_index=d,runvol=4,peakro=.001) for d in (1,2)]
    def test_nonuniform_area_and_order(self):
        rows=daily_projection(list(reversed(self.rows())),self.passes(),2,2,400)
        self.assertEqual(rows[0]['values']['daily_routed_outlet_runoff_mm'],10)
        self.assertEqual(rows[3]['values']['cumulative_routed_outlet_runoff_mm'],20)
        self.assertEqual(rows[2]['values']['published_cumulative_length_normalized_Q_mm'],4.5)
        self.assertNotIn('daily_generated_runoff_mm',rows[2]['values'])
        self.assertEqual(rows[2]['values']['frost_depth_m'],.02)
    def test_incomplete_duplicate_foreign_and_changing_area(self):
        good=self.rows()
        variants=[good[:-1],good+[good[0]],good+[dict(good[0],ofe_id=3,OFE=3)],
            [dict(r,Area=200) if r['sim_day_index']==2 else r for r in good]]
        for rows in variants:
            with self.subTest(rows=rows),self.assertRaises(ValueError):
                daily_projection(rows,self.passes(),2,2,400)
        with self.assertRaises(ValueError):daily_projection(good,self.passes()*2,2,2,400)
        with self.assertRaises(ValueError):daily_projection(good,self.passes(),2,2,500)

    def test_generation_foreign_or_beyond_day(self):
        row=dict(sim_day_index=1,ofe_id=1,interval_start_s=0,interval_duration_s=300)
        validate_generation_rows([row],1,2)
        for changes in [dict(ofe_id=3),dict(sim_day_index=2),dict(interval_start_s=-1),
                dict(interval_start_s=86300),dict(interval_duration_s=0)]:
            with self.subTest(changes=changes),self.assertRaises(ValueError):
                validate_generation_rows([dict(row,**changes)],1,2)

    def test_front_depths_are_final_only_with_exact_lane_coverage(self):
        rows=daily_projection(self.rows(),self.passes(),2,2,400)
        snapshot=dict(committed_day_count=2,lanes=[dict(lane_id=l,frost_depth_m=.1*l,thaw_depth_m=.2*l) for l in (1,2)])
        add_final_front_depths(rows,snapshot,2,2)
        self.assertNotIn('final_snapshot_thaw_depth_m',rows[1]['values'])
        self.assertEqual(rows[5]['values']['final_snapshot_thaw_depth_m'],.4)
        with self.assertRaises(ValueError):add_final_front_depths(rows,dict(snapshot,lanes=snapshot['lanes']*2),2,2)

if __name__=='__main__':unittest.main()
