9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-25 15:32:05.577505
# Source Data: Surgo
# 
# Mukey: 464810
# Major Component: 26520036 (comppct_r = 45.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 79123232   H1             53.0    28.0        3.0        25.0         0.93    10.0    46.0    12.9     5.5
# 79123230   H2             84.0    28.0        3.0        45.0         0.93    10.0    46.0    12.9    0.75
# 79123233   H3     R      114.0    0.22        3.0        45.0          1.8    10.0    67.8     7.8    0.02
# 79123231   H4            160.0    28.0        3.0        48.0         1.58    10.0    67.8     7.8     0.3
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Duripan
# ksat (um/s): 0.22000
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
# 79123232::wilt_pt estimated from wfifteenbar_r and rock
# 79123232::field_cap estimated from wthirdbar_r and rock
# 79123230::wilt_pt estimated from wfifteenbar_r and rock
# 79123230::field_cap estimated from wthirdbar_r and rock
# 79123233::wilt_pt estimated from wfifteenbar_r and rock
# 79123233::field_cap estimated from rosetta3
# 79123231::wilt_pt estimated from wfifteenbar_r and rock
# 79123231::field_cap estimated from wthirdbar_r and rock
# res_lyr_i 2
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
# Build Date: 2026-06-25 15:32:05.931228
# Source File: :/wc1/runs/an/anaphylactic-vernacular/soils/464810.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> loam
# ki -> 400000
# kr -> 3.00E-05
# shcrit -> 1
# avke -> 50
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 2
# xmxlai -> 14
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
0	 'forest'	 'loam'	 1.5 	 0.3
'Tinker-Rock outcrop, granitic-Cryumbrepts, wet complex, 2 to 30 percent slopes'	 'CB-L'	 3	 0.23	 0.75	 400000	 3e-05	 1
	200.0	 0.93	 50	 1.0	 0.3441	 0.1219	 46.0	 10.0	 5.5	 15.0	 44.2	 0.07616	 0.4947	 0.005051	 1.524	 187.0	 0.1195	 0.2183
	530.0	 0.93	 100.8	 1.0	 0.3441	 0.1219	 46.0	 10.0	 5.5	 15.0	 44.2	 0.07616	 0.4947	 0.005051	 1.524	 187.0	 0.1195	 0.2183
	1000.0	 0.93	 100.8	 1.0	 0.16	 0.052	 46.0	 10.0	 0.75	 7.0	 67.5	 0.07616	 0.4947	 0.005051	 1.524	 187.0	 0.1195	 0.2183
1 10000.0 0.00079
