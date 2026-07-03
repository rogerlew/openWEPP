9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-06-23 16:32:08.807652
# Source Data: Surgo
#
# Mukey: 620349
# Major Component: 27227471 (comppct_r = 35.0)
# Texture: loam
#
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 81295765   Oi     X        5.0   350.0        0.0         0.0          0.2     7.0    66.8    10.0    75.0
# 81295766   A              15.0    10.0        0.0         0.0         1.47    20.0    40.0    11.5    1.25
# 81295767   Bt1            38.0    10.0        0.0         0.0         1.43    26.0    35.0    10.4    1.25
# 81295768   Bt2            81.0    10.0        0.0         0.0         1.55    26.0    35.0    10.4     0.4
# 81295769   Bt3           100.0    30.0        0.0         0.0         1.62    15.0    55.0     9.0    0.15
# 81295770   R      R      150.0     0.4         -           -        1.5196     7.0    66.8    10.0     7.0
#
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Lithic bedrock
# ksat (um/s): 0.40000
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
# 81295766::wilt_pt estimated from wfifteenbar_r and rock
# 81295766::field_cap estimated from wthirdbar_r and rock
# 81295767::wilt_pt estimated from wfifteenbar_r and rock
# 81295767::field_cap estimated from wthirdbar_r and rock
# 81295768::wilt_pt estimated from wfifteenbar_r and rock
# 81295768::field_cap estimated from wthirdbar_r and rock
# 81295769::wilt_pt estimated from wfifteenbar_r and rock
# 81295769::field_cap estimated from wthirdbar_r and rock
# 81295770::using default rock content of 55.5%
# 81295770::wilt_pt estimated from rosetta2
# 81295770::field_cap estimated from rosetta2
# 81295770::bd estimated from sand, vfs, and clay
# res_lyr_i 5
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
# Build Date: 2026-06-23 16:32:10.852679
# Source File: :/wc1/runs/ho/honeyed-marathoner/soils/620349.sol
#
# Replacements
# --------------------------
# luse -> forest high sev fire
# stext -> loam
# ki -> 1000000
# kr -> 0.001
# shcrit -> 1
# avke -> 15
# bd ->
# ksflag -> 0
# ksatadj -> 1
# ksatfac -> 100
# ksatrec -> 0.3
# pmet_kcb -> 0.45
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 2
# keffflag -> 1
# lkeff -> 0.1
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_kslast(kslast=100.0)
Any comments:
1 0
1	 'forest high sev fire'	 'loam'	 100 	 0.3
'Ericson-Nooney-Bigcoulee families, association, 8 to 35 percent slopes, Broadly Defined'	 'L'	 5	 0.16	 0.75	 1000000	 0.001	 1
	150.0	 1.47	 15	 10.0	 0.2978	 0.1407	 40.0	 20.0	 1.25	 11.3	 9.0	 0.08466	 0.3877	 0.007674	 1.4	 12.33	 0.1301	 0.2471
	200.0	 1.43	 15	 10.0	 0.3337	 0.1783	 35.0	 26.0	 1.25	 21.2	 17.0	 0.09628	 0.4078	 0.007093	 1.378	 11.05	 0.1495	 0.2697
	380.0	 1.43	 36.0	 10.0	 0.3337	 0.1783	 35.0	 26.0	 1.25	 21.2	 17.0	 0.09628	 0.4078	 0.007093	 1.378	 11.05	 0.1495	 0.2697
	810.0	 1.55	 36.0	 1.0	 0.331	 0.1786	 35.0	 26.0	 0.4	 20.3	 16.0	 0.09493	 0.3789	 0.007551	 1.349	 6.338	 0.1495	 0.2667
	1000.0	 1.62	 108.0	 1.0	 0.2129	 0.1106	 55.0	 15.0	 0.15	 11.9	 15.0	 0.07179	 0.351	 0.01289	 1.375	 12.79	 0.1105	 0.2259
1 10000.0 100.0
