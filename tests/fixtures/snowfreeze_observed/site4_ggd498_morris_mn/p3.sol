9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-24 20:21:20.067387
# Source Data: Surgo
# 
# Mukey: 428368
# Major Component: 27599443 (comppct_r = 75.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82487874   Ap             18.0     9.0        0.0         0.0         1.45    21.0    42.0     5.0     2.5
# 82487875   Bw             38.0     9.0        0.0         0.0         1.45    18.0    45.0     6.0     1.0
# 82487873   2Bk1           50.0    92.0        0.0         2.0          1.7     5.0    85.0     2.0     0.0
# 82487876   2Bk2          200.0    92.0        0.0         2.0          1.7     5.0    85.0     2.0     0.0
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: -
# ksat: -
# 
# defaults applied to missing chorizon data:
# sandtotal_r  ->      66.800
# claytotal_r  ->       7.000
# om_r         ->       7.000
# cec7_r       ->      11.300
# sandvf_r     ->      10.000
# smr          ->      55.500
# 
# Build Notes:
# 82487874::wilt_pt estimated from wfifteenbar_r and rock
# 82487874::field_cap estimated from wthirdbar_r and rock
# 82487875::wilt_pt estimated from wfifteenbar_r and rock
# 82487875::field_cap estimated from wthirdbar_r and rock
# 82487873::wilt_pt estimated from wfifteenbar_r and rock
# 82487873::field_cap estimated from wthirdbar_r and rock
# 82487876::wilt_pt estimated from wfifteenbar_r and rock
# 82487876::field_cap estimated from wthirdbar_r and rock
# res_lyr_i None
# 
# THIS FILE AND THE CONTAINED DATA IS PROVIDED BY THE UNIVERSITY OF IDAHO
# 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
# TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
# PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL UNIVERSITY OF IDAHO
# BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHERE IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS FILE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
# 
# 
# If you change the original contexts of this file please
# indicate it by putting an 'X' in the box here -> [ ]
# 
# 
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::9002.0migration
# Build Date: 2026-06-24 20:21:20.888653
# Source File: :/wc1/runs/op/open-plan-conservatism/soils/428368.sol
# 
# Replacements
# --------------------------
# luse -> short grass
# stext -> loam
# ki -> 1000000
# kr -> 6.00E-05
# shcrit -> 1
# avke -> 25
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 1
0	 'short grass'	 'loam'	 1.5 	 0.3
'Renshaw loam, 0 to 2 percent slopes'	 'L'	 5	 0.16	 0.75	 1000000	 6e-05	 1
	180.0	 1.45	 25	 10.0	 0.3132	 0.1615	 42.0	 21.0	 2.5	 18.0	 9.0	 0.08685	 0.3946	 0.008083	 1.393	 13.39	 0.1335	 0.252
	200.0	 1.45	 25	 10.0	 0.2846	 0.1231	 45.0	 18.0	 1.0	 15.1	 9.0	 0.0809	 0.3898	 0.00858	 1.409	 16.29	 0.1232	 0.2403
	380.0	 1.45	 32.4	 10.0	 0.2846	 0.1231	 45.0	 18.0	 1.0	 15.1	 9.0	 0.0809	 0.3898	 0.00858	 1.409	 16.29	 0.1232	 0.2403
	500.0	 1.7	 331.2	 10.0	 0.1505	 0.045	 85.0	 5.0	 0.0	 2.6	 42.18	 0.0523	 0.3278	 0.02807	 1.865	 101.9	 0.05378	 0.1138
	2000.0	 1.7	 331.2	 1.0	 0.148	 0.044	 85.0	 5.0	 0.0	 2.6	 52.96	 0.0523	 0.3278	 0.02807	 1.865	 101.9	 0.05378	 0.1138
1 10000.0 0.01
