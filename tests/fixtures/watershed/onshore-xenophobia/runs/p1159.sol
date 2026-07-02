9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:07.851730
# Source Data: Surgo
#
# Mukey: 62663
# Major Component: 27032749 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723927   H1             38.0     9.0        0.0         3.0          0.9    28.5    26.9    17.2     5.0
# 80723928   H2            140.0     9.0        0.0         8.0         0.95    28.0    27.1    17.3    2.25
# 80723929   H3            165.0     9.0        0.0        33.0          1.2    24.0    32.6    24.2    0.25
#
# Restricting Layer:
# ksat threshold: 2.00000
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
# 80723927::wilt_pt estimated from wfifteenbar_r and rock
# 80723927::field_cap estimated from wthirdbar_r and rock
# 80723928::wilt_pt estimated from wfifteenbar_r and rock
# 80723928::field_cap estimated from wthirdbar_r and rock
# 80723929::wilt_pt estimated from wfifteenbar_r and rock
# 80723929::field_cap estimated from wthirdbar_r and rock
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
# Source soil utility: 9002.0migration
# Build Date: 2026-04-28 19:18:30.260307
# Source File: soils/62663.sol
#
# Replacements
# --------------------------
# luse -> forest high sev fire
# stext -> clay loam
# ki -> 1500000
# kr -> 6.00E-05
# shcrit -> 0.5
# avke -> 14
# bd ->
# ksflag -> 0
# ksatadj -> 1
# ksatfac -> 100
# ksatrec -> 0.3
# pmet_kcb -> 0.95
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
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
1	 'forest high sev fire'	 'clay loam'	 100 	 0.3
'Cruiser gravelly clay loam, 3 to 25 percent slopes'	 'GR-CL'	 4	 0.23	 0.75	 1500000	 6e-05	 0.5
	200.0	 0.9	 14	 10.0	 0.3834	 0.1909	 26.9	 28.5	 5.0	 50.0	 34.52	 0.1087	 0.5551	 0.004696	 1.431	 133.6	 0.1798	 0.2924
	380.0	 0.9	 32.4	 10.0	 0.3834	 0.1909	 26.9	 28.5	 5.0	 50.0	 34.52	 0.1087	 0.5551	 0.004696	 1.431	 133.6	 0.1798	 0.2924
	1400.0	 0.95	 32.4	 1.0	 0.3736	 0.1723	 27.1	 28.0	 2.25	 40.0	 37.9	 0.1066	 0.5392	 0.00473	 1.435	 106.5	 0.1744	 0.2875
	1800.0	 1.2	 32.4	 1.0	 0.326	 0.14	 32.6	 24.0	 0.25	 25.0	 59.8	 0.09576	 0.4603	 0.005568	 1.434	 33.72	 0.1492	 0.2657
1 10000.0 0.01
