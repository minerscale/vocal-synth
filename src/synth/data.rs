use crate::synth::NUM_OSCILLATORS;

pub const VOWELS: [[f32; NUM_OSCILLATORS]; 35] = [
    [
        0.21089221, 0.05501747, 0.04434347, 0.01116746, 0.00588869, 0.00771711, 0.00337172,
        0.00361026, 0.00039837, 0.00033421, 0.00020227, 0.00010707, 0.00007508, 0.00010708,
        0.00013568, 0.00039201, 0.00037185, 0.00011144, 0.00005597, 0.00006892, 0.00006761,
        0.00021855, 0.00030154, 0.00032582, 0.00013420, 0.00010090, 0.00010486, 0.00014783,
        0.00021827, 0.00048338, 0.00018863, 0.00016967, 0.00022244, 0.00039846, 0.00016885,
        0.00011529, 0.00007362, 0.00007215, 0.00008422, 0.00013718, 0.00017306, 0.00007799,
        0.00008421, 0.00004646, 0.00013362, 0.00011809, 0.00016484, 0.00003451, 0.00002908,
        0.00004523,
    ],
    [
        0.21069252, 0.04560676, 0.03070432, 0.01200106, 0.00641257, 0.00723191, 0.00470937,
        0.00198857, 0.00030099, 0.00048493, 0.00042651, 0.00047789, 0.00013104, 0.00009320,
        0.00010274, 0.00016906, 0.00036786, 0.00014304, 0.00003950, 0.00010370, 0.00003024,
        0.00015350, 0.00033640, 0.00054848, 0.00028439, 0.00014914, 0.00016505, 0.00018717,
        0.00022411, 0.00040958, 0.00012354, 0.00035153, 0.00037337, 0.00034145, 0.00034853,
        0.00028663, 0.00016026, 0.00011443, 0.00015169, 0.00013591, 0.00009113, 0.00006320,
        0.00003621, 0.00006745, 0.00009966, 0.00010735, 0.00014585, 0.00006128, 0.00004533,
        0.00005302,
    ],
    [
        0.17461915, 0.06718508, 0.05383880, 0.02344293, 0.01846074, 0.00939738, 0.01819362,
        0.00864854, 0.00202968, 0.00170378, 0.00055443, 0.00058097, 0.00029839, 0.00031772,
        0.00055240, 0.00069704, 0.00065047, 0.00018320, 0.00007818, 0.00003667, 0.00023976,
        0.00128519, 0.00169861, 0.00095496, 0.00037588, 0.00032106, 0.00033797, 0.00038889,
        0.00055955, 0.00043602, 0.00067880, 0.00060351, 0.00120228, 0.00109291, 0.00042628,
        0.00030035, 0.00027216, 0.00027615, 0.00044437, 0.00085357, 0.00039154, 0.00024196,
        0.00017853, 0.00007010, 0.00010582, 0.00018087, 0.00023396, 0.00011418, 0.00007350,
        0.00006134,
    ],
    [
        0.23525213, 0.09377283, 0.00995766, 0.00244611, 0.00190019, 0.00041603, 0.00037245,
        0.00019888, 0.00008748, 0.00028524, 0.00022346, 0.00089634, 0.00255805, 0.00172955,
        0.00067626, 0.00039511, 0.00019769, 0.00011146, 0.00009396, 0.00039720, 0.00203404,
        0.00115400, 0.00123840, 0.00052561, 0.00043617, 0.00023351, 0.00008173, 0.00010863,
        0.00010425, 0.00006758, 0.00007043, 0.00014082, 0.00014138, 0.00003101, 0.00003221,
        0.00006985, 0.00014147, 0.00013382, 0.00010241, 0.00013340, 0.00016736, 0.00023595,
        0.00035261, 0.00022454, 0.00005757, 0.00002777, 0.00002217, 0.00001024, 0.00000611,
        0.00000985,
    ],
    [
        0.17920118, 0.05402042, 0.02812875, 0.01751337, 0.01002414, 0.01055334, 0.00475030,
        0.00364272, 0.00526235, 0.00332281, 0.00073719, 0.00059447, 0.00014432, 0.00014614,
        0.00020880, 0.00033017, 0.00028213, 0.00017523, 0.00007906, 0.00006448, 0.00019837,
        0.00047886, 0.00043970, 0.00043449, 0.00024145, 0.00015752, 0.00032976, 0.00027666,
        0.00049211, 0.00031478, 0.00046494, 0.00061033, 0.00037134, 0.00031708, 0.00035910,
        0.00019846, 0.00013562, 0.00013110, 0.00014042, 0.00023273, 0.00013911, 0.00004198,
        0.00001463, 0.00001757, 0.00003171, 0.00004040, 0.00003421, 0.00004664, 0.00005009,
        0.00006345,
    ],
    [
        0.16194871, 0.08135933, 0.02654284, 0.00779381, 0.00199965, 0.00296346, 0.00130172,
        0.00155626, 0.00071886, 0.00217131, 0.00348450, 0.00455660, 0.00209087, 0.00102067,
        0.00115592, 0.00124498, 0.00078698, 0.00018782, 0.00011724, 0.00006370, 0.00010983,
        0.00036835, 0.00059373, 0.00102526, 0.00050043, 0.00041870, 0.00029280, 0.00060479,
        0.00100732, 0.00054744, 0.00038798, 0.00036603, 0.00029716, 0.00031688, 0.00053705,
        0.00025878, 0.00018692, 0.00011541, 0.00014953, 0.00008527, 0.00009083, 0.00022016,
        0.00032175, 0.00019626, 0.00019775, 0.00016257, 0.00007372, 0.00008277, 0.00006819,
        0.00007952,
    ],
    [
        0.22986023, 0.07115180, 0.00757323, 0.00142858, 0.00092067, 0.00073696, 0.00047855,
        0.00017485, 0.00012568, 0.00025204, 0.00018484, 0.00063635, 0.00304057, 0.00021174,
        0.00021245, 0.00032690, 0.00025613, 0.00041434, 0.00021975, 0.00030683, 0.00025885,
        0.00113880, 0.00184828, 0.00073027, 0.00031407, 0.00054881, 0.00042982, 0.00015143,
        0.00017041, 0.00013597, 0.00013759, 0.00010883, 0.00017540, 0.00037063, 0.00011260,
        0.00009678, 0.00006272, 0.00007127, 0.00006221, 0.00010469, 0.00027785, 0.00027026,
        0.00018630, 0.00007154, 0.00008541, 0.00004146, 0.00007081, 0.00009925, 0.00006485,
        0.00004511,
    ],
    [
        0.20393202, 0.05807269, 0.03674914, 0.03695682, 0.02757383, 0.01730984, 0.00672722,
        0.00828365, 0.00771731, 0.00510809, 0.00277660, 0.00088416, 0.00071189, 0.00094604,
        0.00056040, 0.00095104, 0.00149469, 0.00059414, 0.00027930, 0.00015877, 0.00059973,
        0.00093018, 0.00053071, 0.00079231, 0.00056936, 0.00046706, 0.00046843, 0.00053529,
        0.00088259, 0.00078865, 0.00091368, 0.00062905, 0.00058221, 0.00037875, 0.00045686,
        0.00023948, 0.00018221, 0.00026413, 0.00020915, 0.00024404, 0.00014961, 0.00011549,
        0.00011791, 0.00016067, 0.00014388, 0.00022210, 0.00018138, 0.00011422, 0.00009256,
        0.00005861,
    ],
    [
        0.13216389, 0.05759715, 0.05123776, 0.01242039, 0.00296077, 0.00201903, 0.00039097,
        0.00016985, 0.00007979, 0.00013473, 0.00006544, 0.00003241, 0.00003758, 0.00003761,
        0.00005438, 0.00006513, 0.00002896, 0.00003292, 0.00001423, 0.00001867, 0.00004650,
        0.00011551, 0.00019007, 0.00011533, 0.00007468, 0.00007825, 0.00012334, 0.00011096,
        0.00004603, 0.00005488, 0.00007793, 0.00006430, 0.00003899, 0.00000782, 0.00005130,
        0.00005495, 0.00004819, 0.00004546, 0.00006847, 0.00003655, 0.00003142, 0.00003666,
        0.00009664, 0.00005241, 0.00003389, 0.00000571, 0.00000574, 0.00000535, 0.00001693,
        0.00001095,
    ],
    [
        0.20451198, 0.05006571, 0.05165946, 0.01146941, 0.00965459, 0.00710247, 0.00198378,
        0.00259219, 0.00233789, 0.00827208, 0.00371555, 0.00034946, 0.00047550, 0.00030655,
        0.00019911, 0.00057693, 0.00024172, 0.00007620, 0.00020639, 0.00005114, 0.00021143,
        0.00042761, 0.00058420, 0.00015836, 0.00019736, 0.00013945, 0.00013146, 0.00045539,
        0.00064517, 0.00058356, 0.00066116, 0.00031017, 0.00038847, 0.00051818, 0.00022591,
        0.00020573, 0.00014851, 0.00012047, 0.00015079, 0.00016522, 0.00018742, 0.00012202,
        0.00007687, 0.00004817, 0.00004181, 0.00006164, 0.00006232, 0.00005852, 0.00002130,
        0.00003260,
    ],
    [
        0.18144482, 0.06968938, 0.04007317, 0.00829477, 0.00370719, 0.00104361, 0.00197425,
        0.00121234, 0.00301485, 0.00403748, 0.00467440, 0.00097251, 0.00116884, 0.00098871,
        0.00198920, 0.00091563, 0.00062605, 0.00025962, 0.00020264, 0.00023053, 0.00104466,
        0.00230869, 0.00080538, 0.00041121, 0.00050094, 0.00038661, 0.00043511, 0.00045747,
        0.00051822, 0.00031529, 0.00065443, 0.00048649, 0.00049577, 0.00085782, 0.00033393,
        0.00018361, 0.00023298, 0.00022978, 0.00041317, 0.00039511, 0.00041912, 0.00033640,
        0.00020355, 0.00016903, 0.00036433, 0.00013485, 0.00010183, 0.00008910, 0.00010910,
        0.00012836,
    ],
    [
        0.19813038, 0.07575665, 0.04290044, 0.01187569, 0.00721383, 0.00374744, 0.00159930,
        0.00098177, 0.00472779, 0.01484909, 0.00268142, 0.00104856, 0.00061808, 0.00051253,
        0.00126567, 0.00036118, 0.00013355, 0.00075540, 0.00023037, 0.00049044, 0.00171860,
        0.00130878, 0.00140581, 0.00041302, 0.00049628, 0.00035719, 0.00021654, 0.00067317,
        0.00093884, 0.00063081, 0.00063548, 0.00056421, 0.00018387, 0.00049007, 0.00031792,
        0.00026802, 0.00030238, 0.00033108, 0.00046903, 0.00044627, 0.00010130, 0.00027664,
        0.00010347, 0.00007661, 0.00011893, 0.00015348, 0.00010780, 0.00012506, 0.00011827,
        0.00003441,
    ],
    [
        0.14237332, 0.05952436, 0.05872193, 0.00480313, 0.00511602, 0.00309308, 0.00403098,
        0.00346138, 0.00542513, 0.00299963, 0.00052749, 0.00042720, 0.00045660, 0.00047323,
        0.00073964, 0.00073091, 0.00071676, 0.00040713, 0.00009929, 0.00121606, 0.00063011,
        0.00083057, 0.00030450, 0.00014946, 0.00009388, 0.00008690, 0.00024123, 0.00051747,
        0.00018270, 0.00015083, 0.00012646, 0.00018572, 0.00035901, 0.00011934, 0.00006093,
        0.00002159, 0.00001306, 0.00001744, 0.00002516, 0.00006387, 0.00006536, 0.00002437,
        0.00005566, 0.00004190, 0.00003083, 0.00004234, 0.00009096, 0.00002940, 0.00001234,
        0.00001323,
    ],
    [
        0.20373111, 0.07274172, 0.05957581, 0.00712015, 0.00860251, 0.00567498, 0.00742308,
        0.01494876, 0.00150232, 0.00135218, 0.00041201, 0.00035118, 0.00029932, 0.00020226,
        0.00037835, 0.00034990, 0.00038686, 0.00020614, 0.00009814, 0.00034459, 0.00012701,
        0.00044135, 0.00038837, 0.00019987, 0.00009319, 0.00007626, 0.00013600, 0.00031326,
        0.00019024, 0.00011858, 0.00007688, 0.00009603, 0.00016738, 0.00019372, 0.00007445,
        0.00006414, 0.00004634, 0.00002991, 0.00002675, 0.00001944, 0.00002074, 0.00008107,
        0.00008769, 0.00009257, 0.00018868, 0.00016214, 0.00012757, 0.00006460, 0.00000676,
        0.00000900,
    ],
    [
        0.17104384, 0.05351014, 0.02606882, 0.00279713, 0.00753681, 0.00341601, 0.00646770,
        0.00226337, 0.00087429, 0.00095002, 0.00026600, 0.00025882, 0.00056269, 0.00110666,
        0.00021311, 0.00014212, 0.00016391, 0.00010048, 0.00060992, 0.00040916, 0.00015824,
        0.00016487, 0.00017331, 0.00014396, 0.00018346, 0.00056530, 0.00012223, 0.00008673,
        0.00005472, 0.00004913, 0.00008261, 0.00012787, 0.00029956, 0.00012110, 0.00005345,
        0.00005114, 0.00007723, 0.00011550, 0.00014029, 0.00005715, 0.00004943, 0.00011740,
        0.00010861, 0.00013355, 0.00011356, 0.00006257, 0.00004089, 0.00004520, 0.00004339,
        0.00005334,
    ],
    [
        0.21873698, 0.03116985, 0.00767113, 0.00106604, 0.00408987, 0.00335511, 0.00042333,
        0.00020862, 0.00018965, 0.00015615, 0.00013887, 0.00009955, 0.00012453, 0.00047260,
        0.00006643, 0.00005186, 0.00004553, 0.00007411, 0.00014272, 0.00010930, 0.00003905,
        0.00004411, 0.00004046, 0.00004762, 0.00003896, 0.00010269, 0.00006695, 0.00003817,
        0.00003035, 0.00003680, 0.00003355, 0.00006443, 0.00005214, 0.00002695, 0.00002838,
        0.00002897, 0.00003104, 0.00004021, 0.00002683, 0.00002264, 0.00002560, 0.00002880,
        0.00002884, 0.00004570, 0.00005171, 0.00002548, 0.00002990, 0.00002274, 0.00002936,
        0.00002620,
    ],
    [
        0.15153821, 0.05729006, 0.01591827, 0.00127731, 0.00103209, 0.00102044, 0.00026963,
        0.00029767, 0.00024529, 0.00024649, 0.00071589, 0.00138505, 0.00070894, 0.00028236,
        0.00023649, 0.00035028, 0.00102023, 0.00033731, 0.00009590, 0.00034341, 0.00022075,
        0.00046066, 0.00080991, 0.00032284, 0.00021369, 0.00016360, 0.00023107, 0.00028979,
        0.00046674, 0.00033506, 0.00055884, 0.00059329, 0.00074199, 0.00036032, 0.00015803,
        0.00011859, 0.00008245, 0.00005467, 0.00004344, 0.00006735, 0.00012748, 0.00016405,
        0.00015928, 0.00007047, 0.00019294, 0.00007587, 0.00006411, 0.00009017, 0.00001559,
        0.00001465,
    ],
    [
        0.21982161, 0.06013175, 0.02327972, 0.00129561, 0.00385884, 0.00044540, 0.00029624,
        0.00009993, 0.00004618, 0.00008810, 0.00013640, 0.00035866, 0.00082153, 0.00151578,
        0.00067452, 0.00063661, 0.00150258, 0.00034423, 0.00038041, 0.00011242, 0.00018829,
        0.00059193, 0.00152731, 0.00113279, 0.00043771, 0.00061789, 0.00049022, 0.00113529,
        0.00014827, 0.00032776, 0.00032257, 0.00019240, 0.00009249, 0.00009481, 0.00015819,
        0.00025199, 0.00013363, 0.00003346, 0.00005938, 0.00005284, 0.00008213, 0.00014893,
        0.00021225, 0.00021596, 0.00022948, 0.00028684, 0.00026892, 0.00007209, 0.00004967,
        0.00004711,
    ],
    [
        0.20358987, 0.07316951, 0.04127993, 0.01035979, 0.01304799, 0.00931487, 0.00463132,
        0.01156618, 0.00086158, 0.00179849, 0.00054296, 0.00095894, 0.00058597, 0.00034156,
        0.00061832, 0.00048639, 0.00036113, 0.00014804, 0.00036548, 0.00023670, 0.00014922,
        0.00037642, 0.00058054, 0.00084557, 0.00046440, 0.00035403, 0.00053132, 0.00056018,
        0.00085606, 0.00077291, 0.00033401, 0.00037287, 0.00023354, 0.00067108, 0.00091821,
        0.00030813, 0.00030101, 0.00019359, 0.00019426, 0.00046938, 0.00048513, 0.00063535,
        0.00023496, 0.00023875, 0.00022180, 0.00021383, 0.00033229, 0.00025911, 0.00007558,
        0.00010384,
    ],
    [
        0.18737111, 0.05906043, 0.00471240, 0.00085601, 0.00023031, 0.00028293, 0.00010220,
        0.00013585, 0.00006766, 0.00010779, 0.00010078, 0.00023537, 0.00043841, 0.00007037,
        0.00006104, 0.00007261, 0.00014456, 0.00020196, 0.00073676, 0.00034827, 0.00048205,
        0.00073009, 0.00055022, 0.00033888, 0.00010031, 0.00019813, 0.00017780, 0.00019736,
        0.00025935, 0.00016830, 0.00019428, 0.00017852, 0.00006603, 0.00014133, 0.00008243,
        0.00005676, 0.00006930, 0.00006697, 0.00004971, 0.00007245, 0.00007548, 0.00011921,
        0.00014060, 0.00017415, 0.00009798, 0.00002845, 0.00003872, 0.00001711, 0.00002103,
        0.00001874,
    ],
    [
        0.15727339, 0.04909131, 0.05857877, 0.01038569, 0.01379919, 0.01115621, 0.00068593,
        0.00053946, 0.00028420, 0.00019284, 0.00017708, 0.00015095, 0.00009272, 0.00011090,
        0.00011025, 0.00028297, 0.00017040, 0.00008002, 0.00003893, 0.00002188, 0.00004206,
        0.00026404, 0.00057742, 0.00015223, 0.00012057, 0.00012005, 0.00039767, 0.00014882,
        0.00013426, 0.00008257, 0.00010525, 0.00012934, 0.00027156, 0.00022873, 0.00021993,
        0.00018360, 0.00012286, 0.00009983, 0.00004508, 0.00007877, 0.00010133, 0.00009372,
        0.00009718, 0.00016360, 0.00005485, 0.00001220, 0.00001763, 0.00001996, 0.00001549,
        0.00002732,
    ],
    [
        0.13384191, 0.11639326, 0.08392556, 0.02627279, 0.00589331, 0.00094422, 0.00049587,
        0.00028018, 0.00020044, 0.00026806, 0.00010040, 0.00010784, 0.00011215, 0.00010506,
        0.00013988, 0.00035273, 0.00020481, 0.00013198, 0.00016124, 0.00017082, 0.00019382,
        0.00039964, 0.00026157, 0.00014802, 0.00028302, 0.00015926, 0.00007321, 0.00005474,
        0.00004597, 0.00005591, 0.00007411, 0.00020318, 0.00019644, 0.00008189, 0.00006651,
        0.00017719, 0.00005802, 0.00004005, 0.00003766, 0.00004033, 0.00005046, 0.00002068,
        0.00001730, 0.00005162, 0.00013744, 0.00001338, 0.00001446, 0.00001699, 0.00001904,
        0.00002598,
    ],
    [
        0.24482490, 0.05603737, 0.01177357, 0.00201517, 0.00316008, 0.00091728, 0.00050320,
        0.00029763, 0.00026545, 0.00044946, 0.00066895, 0.00077166, 0.00314328, 0.00103342,
        0.00047546, 0.00032577, 0.00027722, 0.00021296, 0.00022605, 0.00039448, 0.00022398,
        0.00204280, 0.00066999, 0.00037369, 0.00019675, 0.00026461, 0.00080107, 0.00054675,
        0.00023003, 0.00019954, 0.00017998, 0.00020224, 0.00021076, 0.00007232, 0.00008668,
        0.00008282, 0.00006669, 0.00008473, 0.00009386, 0.00010835, 0.00026621, 0.00018202,
        0.00012400, 0.00012862, 0.00018892, 0.00036242, 0.00012363, 0.00005768, 0.00004261,
        0.00004315,
    ],
    [
        0.15190947, 0.04537146, 0.06065673, 0.00941668, 0.01343629, 0.00911570, 0.00062242,
        0.00056049, 0.00025551, 0.00016730, 0.00013613, 0.00013174, 0.00006864, 0.00008997,
        0.00010711, 0.00017450, 0.00015453, 0.00004849, 0.00003988, 0.00002184, 0.00004074,
        0.00036449, 0.00050143, 0.00014703, 0.00013793, 0.00009531, 0.00032314, 0.00018721,
        0.00015144, 0.00007812, 0.00009486, 0.00017694, 0.00025925, 0.00028669, 0.00014857,
        0.00021351, 0.00017770, 0.00009118, 0.00004018, 0.00006974, 0.00008082, 0.00004876,
        0.00012430, 0.00012008, 0.00010575, 0.00001081, 0.00002066, 0.00002360, 0.00002205,
        0.00001232,
    ],
    [
        0.16991041, 0.03479952, 0.00293553, 0.00073105, 0.00309477, 0.00032222, 0.00035289,
        0.00007928, 0.00013370, 0.00020725, 0.00032563, 0.00052162, 0.00071767, 0.00008946,
        0.00005094, 0.00004992, 0.00004347, 0.00003103, 0.00004726, 0.00011860, 0.00008672,
        0.00009365, 0.00007136, 0.00004414, 0.00006054, 0.00004821, 0.00002552, 0.00002462,
        0.00002397, 0.00002580, 0.00002858, 0.00002448, 0.00004461, 0.00002130, 0.00003162,
        0.00002013, 0.00002179, 0.00001617, 0.00001887, 0.00002000, 0.00003537, 0.00004238,
        0.00003242, 0.00001861, 0.00001681, 0.00001443, 0.00001629, 0.00001438, 0.00001292,
        0.00001444,
    ],
    [
        0.13866169, 0.06236434, 0.01644681, 0.00220776, 0.00165272, 0.00148504, 0.00435957,
        0.00128217, 0.00031880, 0.00049800, 0.00009639, 0.00013230, 0.00007163, 0.00014503,
        0.00022129, 0.00006882, 0.00005309, 0.00004113, 0.00002665, 0.00036211, 0.00048062,
        0.00007428, 0.00002840, 0.00002182, 0.00002445, 0.00006046, 0.00008619, 0.00006432,
        0.00004443, 0.00004250, 0.00006482, 0.00008992, 0.00011126, 0.00003586, 0.00002932,
        0.00002061, 0.00002259, 0.00002162, 0.00004070, 0.00002996, 0.00002703, 0.00001688,
        0.00002487, 0.00002212, 0.00002922, 0.00002862, 0.00001273, 0.00001046, 0.00001043,
        0.00001336,
    ],
    [
        0.21598109, 0.03107316, 0.00748963, 0.00071135, 0.00108340, 0.00101988, 0.00030685,
        0.00038412, 0.00028128, 0.00046505, 0.00030994, 0.00044362, 0.00028328, 0.00036325,
        0.00016974, 0.00013934, 0.00007671, 0.00005284, 0.00006707, 0.00004912, 0.00004874,
        0.00005198, 0.00005115, 0.00004881, 0.00004449, 0.00007113, 0.00007112, 0.00017247,
        0.00019722, 0.00013835, 0.00008219, 0.00022978, 0.00038132, 0.00020925, 0.00008947,
        0.00006739, 0.00007658, 0.00007430, 0.00005953, 0.00004109, 0.00003896, 0.00003836,
        0.00003186, 0.00003692, 0.00004434, 0.00005270, 0.00004848, 0.00009044, 0.00003031,
        0.00004387,
    ],
    [
        0.24640203, 0.01358907, 0.00406705, 0.00083158, 0.00157327, 0.00217550, 0.00100312,
        0.00044096, 0.00061413, 0.00080414, 0.00065525, 0.00078645, 0.00046541, 0.00130107,
        0.00077257, 0.00085677, 0.00076366, 0.00056537, 0.00059467, 0.00102003, 0.00083903,
        0.00089560, 0.00066029, 0.00061065, 0.00082518, 0.00078240, 0.00097685, 0.00125138,
        0.00086191, 0.00075676, 0.00054484, 0.00054240, 0.00072911, 0.00091490, 0.00057744,
        0.00087715, 0.00086103, 0.00068065, 0.00075631, 0.00062595, 0.00085735, 0.00106728,
        0.00077957, 0.00077497, 0.00048840, 0.00062467, 0.00055788, 0.00037185, 0.00042111,
        0.00055625,
    ],
    [
        0.16662833, 0.00829238, 0.00315512, 0.00033754, 0.00046782, 0.00071435, 0.00034082,
        0.00026813, 0.00038389, 0.00047208, 0.00043818, 0.00032680, 0.00022800, 0.00059983,
        0.00032486, 0.00030460, 0.00031373, 0.00020809, 0.00031415, 0.00022799, 0.00022554,
        0.00023231, 0.00054868, 0.00038813, 0.00031725, 0.00023415, 0.00028445, 0.00033113,
        0.00069394, 0.00071109, 0.00053298, 0.00029913, 0.00031387, 0.00044981, 0.00038950,
        0.00035090, 0.00027275, 0.00024792, 0.00059581, 0.00048403, 0.00051115, 0.00047974,
        0.00051579, 0.00048540, 0.00030206, 0.00047498, 0.00034766, 0.00031019, 0.00041819,
        0.00034979,
    ],
    [
        0.10984904, 0.00646748, 0.00250191, 0.00029448, 0.00046865, 0.00035410, 0.00028667,
        0.00023140, 0.00044220, 0.00073361, 0.00063011, 0.00048521, 0.00038132, 0.00034620,
        0.00049110, 0.00035224, 0.00046644, 0.00044545, 0.00033247, 0.00024384, 0.00033111,
        0.00035075, 0.00041005, 0.00047986, 0.00039469, 0.00049344, 0.00044456, 0.00064782,
        0.00127066, 0.00114737, 0.00301064, 0.00187610, 0.00236359, 0.00232745, 0.00325272,
        0.00301633, 0.00368357, 0.00193402, 0.00377440, 0.00293149, 0.00259826, 0.00281931,
        0.00421664, 0.00276635, 0.00494156, 0.00447983, 0.00396997, 0.00332227, 0.00352798,
        0.00422981,
    ],
    [
        0.14773924, 0.00607654, 0.00260678, 0.00049951, 0.00042477, 0.00031302, 0.00036615,
        0.00026431, 0.00032162, 0.00056106, 0.00065958, 0.00161109, 0.00122616, 0.00171169,
        0.00344535, 0.00669081, 0.00849391, 0.00417476, 0.00306852, 0.00264144, 0.00361402,
        0.00363457, 0.00205073, 0.00272799, 0.00228210, 0.00205272, 0.00203161, 0.00238678,
        0.00274962, 0.00192311, 0.00185291, 0.00160755, 0.00116576, 0.00124988, 0.00065124,
        0.00055751, 0.00054286, 0.00042863, 0.00045450, 0.00075551, 0.00082085, 0.00167595,
        0.00121193, 0.00128574, 0.00116521, 0.00082278, 0.00038933, 0.00035678, 0.00027892,
        0.00018616,
    ],
    [
        0.17627269, 0.07981539, 0.02929427, 0.00235293, 0.00329530, 0.00217220, 0.01192011,
        0.00480415, 0.00350182, 0.00045659, 0.00027050, 0.00019136, 0.00014980, 0.00014983,
        0.00007304, 0.00008240, 0.00007951, 0.00007597, 0.00007707, 0.00005668, 0.00006261,
        0.00006936, 0.00006331, 0.00007989, 0.00009877, 0.00007154, 0.00007400, 0.00010854,
        0.00010507, 0.00020223, 0.00027518, 0.00040802, 0.00028103, 0.00012145, 0.00012714,
        0.00004192, 0.00004566, 0.00003967, 0.00003357, 0.00005997, 0.00005089, 0.00004603,
        0.00002516, 0.00002832, 0.00004430, 0.00003952, 0.00003372, 0.00004292, 0.00003055,
        0.00002107,
    ],
    [
        0.20548090, 0.08201506, 0.00594778, 0.00123153, 0.00142240, 0.00040255, 0.00035514,
        0.00040913, 0.00027586, 0.00038487, 0.00041986, 0.00018431, 0.00009180, 0.00014282,
        0.00018810, 0.00018160, 0.00010497, 0.00005320, 0.00006147, 0.00006426, 0.00005089,
        0.00005252, 0.00004851, 0.00005876, 0.00006791, 0.00008438, 0.00023970, 0.00050539,
        0.00040552, 0.00025818, 0.00026660, 0.00015541, 0.00010333, 0.00008944, 0.00006478,
        0.00007239, 0.00008057, 0.00009733, 0.00006639, 0.00004294, 0.00007049, 0.00005735,
        0.00003980, 0.00003906, 0.00003028, 0.00002221, 0.00002249, 0.00002082, 0.00002197,
        0.00002476,
    ],
    [
        0.16094270, 0.06190847, 0.00739672, 0.00083685, 0.00185132, 0.00036041, 0.00039936,
        0.00113240, 0.00022448, 0.00019971, 0.00032152, 0.00016316, 0.00019853, 0.00021125,
        0.00023175, 0.00012959, 0.00030052, 0.00003848, 0.00002689, 0.00003077, 0.00004107,
        0.00003356, 0.00002904, 0.00003050, 0.00003777, 0.00004194, 0.00008153, 0.00009531,
        0.00008496, 0.00017994, 0.00028599, 0.00022411, 0.00019116, 0.00008395, 0.00005058,
        0.00006324, 0.00004954, 0.00010632, 0.00002590, 0.00003206, 0.00003801, 0.00004050,
        0.00002727, 0.00002489, 0.00003848, 0.00004182, 0.00004476, 0.00002708, 0.00001071,
        0.00001360,
    ],
    [
        0.23953842, 0.04790124, 0.02583336, 0.01074082, 0.00928890, 0.00201174, 0.00146396,
        0.00056032, 0.00022105, 0.00021807, 0.00014584, 0.00011799, 0.00015001, 0.00020225,
        0.00009833, 0.00006413, 0.00005281, 0.00006465, 0.00006263, 0.00023713, 0.00012372,
        0.00007009, 0.00005126, 0.00005450, 0.00005545, 0.00005090, 0.00004792, 0.00004991,
        0.00003134, 0.00003758, 0.00003142, 0.00004252, 0.00004864, 0.00003225, 0.00003177,
        0.00003424, 0.00003839, 0.00003182, 0.00004432, 0.00002824, 0.00002728, 0.00003038,
        0.00003173, 0.00004031, 0.00015240, 0.00003670, 0.00002076, 0.00002377, 0.00002047,
        0.00002140,
    ],
];