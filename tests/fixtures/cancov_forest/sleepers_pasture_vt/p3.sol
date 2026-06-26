9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 22:21:55.454068
# Source Data: Surgo
# 
# Mukey: 282829
# Major Component: 27656439 (comppct_r = 50.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 82709671   H1             20.0  9.1722        0.0         2.0          1.1     4.0    60.0    34.3     5.0
# 82709672   H2             48.0  9.1722        1.0         1.0          1.4     4.0    60.0    34.3     2.1
# 82709673   H3             56.0  9.1722        0.0         0.0         1.55     2.0    68.0    40.8     1.2
# 82709670   R      R       81.0   0.215         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 82709671::wilt_pt estimated from wfifteenbar_r and rock
# 82709671::field_cap estimated from wthirdbar_r and rock
# 82709672::wilt_pt estimated from wfifteenbar_r and rock
# 82709672::field_cap estimated from wthirdbar_r and rock
# 82709673::wilt_pt estimated from wfifteenbar_r and rock
# 82709673::field_cap estimated from wthirdbar_r and rock
# 82709670::using default rock content of 55.5%
# 82709670::wilt_pt estimated from rosetta2
# 82709670::field_cap estimated from rosetta2
# 82709670::bd estimated from sand, vfs, and clay
# res_lyr_i 3
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
# Build Date: 2026-06-26 22:21:56.660920
# Source File: :/wc1/runs/in/interconnected-fit/soils/282829.sol
# 
# Replacements
# --------------------------
# luse -> agriculture crops
# stext -> sand loam
# ki ->
# kr ->
# shcrit ->
# avke ->
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 0
# ksatrec -> 0
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax ->
# xmxlai ->
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct ->
# plant.data.dropfc ->
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 1
0	 'agriculture crops'	 'sand loam'	 0 	 0
'Vershire-Lombard complex, 15 to 25 percent slopes, rocky'	 'GR-SIL'	 3	 0.23	 0.75	 9317030.0	 0.0123	 0.9406
	200.0	 1.1	 33.0199	 10.0	 0.1877	 0.0809	 60.0	 4.0	 5.0	 11.3	 14.74	 0.06149	 0.4445	 0.009153	 1.547	 159.4	 0.08746	 0.189
	480.0	 1.4	 33.0199	 10.0	 0.1607	 0.0586	 60.0	 4.0	 2.1	 2.2	 14.74	 0.05582	 0.3803	 0.01261	 1.536	 64.88	 0.07536	 0.1774
	600.0	 1.55	 33.0199	 1.0	 0.118	 0.033	 68.0	 2.0	 1.2	 1.1	 0.0	 0.05051	 0.3497	 0.01827	 1.577	 60.59	 0.06222	 0.1569
1 10000.0 0.00077
