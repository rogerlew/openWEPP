9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-24 20:08:05.178171
# Source Data: Surgo
# 
# Mukey: 282863
# Major Component: 27656702 (comppct_r = 50.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82710408   O1     X        5.0 28.2222        0.0         0.0         0.34     4.0    60.0    34.3    85.0
# 82710410   H1             13.0  9.1722        0.0         2.0          1.1     4.0    60.0    34.3     5.0
# 82710411   H2             48.0  9.1722        1.0         1.0          1.4     4.0    60.0    34.3     2.1
# 82710412   H3             56.0  9.1722        0.0         0.0         1.55     2.0    68.0    40.8     1.2
# 82710409   R      R       81.0   0.215         -           -        1.5196     7.0    66.8    10.0     7.0
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Lithic bedrock
# ksat (um/s): 0.21500
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
# 82710410::wilt_pt estimated from wfifteenbar_r and rock
# 82710410::field_cap estimated from wthirdbar_r and rock
# 82710411::wilt_pt estimated from wfifteenbar_r and rock
# 82710411::field_cap estimated from wthirdbar_r and rock
# 82710412::wilt_pt estimated from wfifteenbar_r and rock
# 82710412::field_cap estimated from wthirdbar_r and rock
# 82710409::using default rock content of 55.5%
# 82710409::wilt_pt estimated from rosetta2
# 82710409::field_cap estimated from rosetta2
# 82710409::bd estimated from sand, vfs, and clay
# res_lyr_i 4
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
# Build Date: 2026-06-24 20:08:06.098236
# Source File: :/wc1/runs/ba/baseless-salesmanship/soils/282863.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> sand loam
# ki -> 400000
# kr -> 8.00E-05
# shcrit -> 2
# avke -> 60
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
0	 'forest'	 'sand loam'	 1.5 	 0.3
'Vershire-Lombard complex, 15 to 35 percent slopes, very stony'	 'VFSL'	 4	 0.23	 0.75	 400000	 8e-05	 2
	130.0	 1.1	 60	 10.0	 0.1828	 0.0796	 60.0	 4.0	 5.0	 11.3	 10.82	 0.06149	 0.4445	 0.009153	 1.547	 159.4	 0.08746	 0.189
	200.0	 1.4	 60	 10.0	 0.1607	 0.0586	 60.0	 4.0	 2.1	 2.2	 14.74	 0.05582	 0.3803	 0.01261	 1.536	 64.88	 0.07536	 0.1774
	480.0	 1.4	 33.0199	 10.0	 0.1607	 0.0586	 60.0	 4.0	 2.1	 2.2	 14.74	 0.05582	 0.3803	 0.01261	 1.536	 64.88	 0.07536	 0.1774
	600.0	 1.55	 33.0199	 1.0	 0.118	 0.033	 68.0	 2.0	 1.2	 1.1	 0.0	 0.05051	 0.3497	 0.01827	 1.577	 60.59	 0.06222	 0.1569
1 10000.0 0.00077
