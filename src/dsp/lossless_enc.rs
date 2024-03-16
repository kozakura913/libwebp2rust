use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8LPredictor2_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor3_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor4_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor5_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor6_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor7_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor8_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor9_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor10_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor11_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor12_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LPredictor13_C(left: *const uint32_t, top: *const uint32_t) -> uint32_t;
    fn VP8LDspInit();
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogram {
    pub literal_: *mut uint32_t,
    pub red_: [uint32_t; 256],
    pub blue_: [uint32_t; 256],
    pub alpha_: [uint32_t; 256],
    pub distance_: [uint32_t; 40],
    pub palette_code_bits_: libc::c_int,
    pub trivial_symbol_: uint32_t,
    pub bit_cost_: libc::c_float,
    pub literal_cost_: libc::c_float,
    pub red_cost_: libc::c_float,
    pub blue_cost_: libc::c_float,
    pub is_used_: [uint8_t; 5],
}
pub type VP8LPredictorAddSubFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        libc::c_int,
        *mut uint32_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMultipliers {
    pub green_to_red_: uint8_t,
    pub green_to_blue_: uint8_t,
    pub red_to_blue_: uint8_t,
}
pub type VP8LProcessEncBlueAndRedFunc = Option::<
    unsafe extern "C" fn(*mut uint32_t, libc::c_int) -> (),
>;
pub type VP8LTransformColorFunc = Option::<
    unsafe extern "C" fn(*const VP8LMultipliers, *mut uint32_t, libc::c_int) -> (),
>;
pub type VP8LCollectColorBlueTransformsFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
>;
pub type VP8LCollectColorRedTransformsFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
>;
pub type VP8LCostFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, libc::c_int) -> uint32_t,
>;
pub type VP8LCostCombinedFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t, libc::c_int) -> uint32_t,
>;
pub type VP8LCombinedShannonEntropyFunc = Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> libc::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LStreaks {
    pub counts: [libc::c_int; 2],
    pub streaks: [[libc::c_int; 2]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitEntropy {
    pub entropy: libc::c_float,
    pub sum: uint32_t,
    pub nonzeros: libc::c_int,
    pub max_val: uint32_t,
    pub nonzero_code: uint32_t,
}
pub type VP8LGetCombinedEntropyUnrefinedFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        libc::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
pub type VP8LGetEntropyUnrefinedFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
pub type VP8LFastLog2SlowFunc = Option::<
    unsafe extern "C" fn(uint32_t) -> libc::c_float,
>;
pub type VP8LAddVectorFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        *mut uint32_t,
        libc::c_int,
    ) -> (),
>;
pub type VP8LAddVectorEqFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *mut uint32_t, libc::c_int) -> (),
>;
pub type VP8LVectorMismatchFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t, libc::c_int) -> libc::c_int,
>;
pub type VP8LBundleColorMapFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, libc::c_int, libc::c_int, *mut uint32_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
#[inline]
unsafe extern "C" fn VP8LHistogramNumCodes(
    mut palette_code_bits: libc::c_int,
) -> libc::c_int {
    return 256 as libc::c_int + 24 as libc::c_int
        + (if palette_code_bits > 0 as libc::c_int {
            (1 as libc::c_int) << palette_code_bits
        } else {
            0 as libc::c_int
        });
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LFastSLog2(mut v: uint32_t) -> libc::c_float {
    return if v < 256 as libc::c_int as libc::c_uint {
        kSLog2Table[v as usize]
    } else {
        VP8LFastSLog2Slow.expect("non-null function pointer")(v)
    };
}
#[inline]
unsafe extern "C" fn VP8LSubPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t = (0xff00ff as libc::c_uint)
        .wrapping_add(a & 0xff00ff00 as libc::c_uint)
        .wrapping_sub(b & 0xff00ff00 as libc::c_uint);
    let red_and_blue: uint32_t = (0xff00ff00 as libc::c_uint)
        .wrapping_add(a & 0xff00ff as libc::c_uint)
        .wrapping_sub(b & 0xff00ff as libc::c_uint);
    return alpha_and_green & 0xff00ff00 as libc::c_uint
        | red_and_blue & 0xff00ff as libc::c_uint;
}
#[no_mangle]
pub static mut kLog2Table: [libc::c_float; 256] = [
    0.0000000000000000f32,
    0.0000000000000000f32,
    1.0000000000000000f32,
    1.5849625007211560f32,
    2.0000000000000000f32,
    2.3219280948873621f32,
    2.5849625007211560f32,
    2.8073549220576041f32,
    3.0000000000000000f32,
    3.1699250014423121f32,
    3.3219280948873621f32,
    3.4594316186372973f32,
    3.5849625007211560f32,
    3.7004397181410921f32,
    3.8073549220576041f32,
    3.9068905956085187f32,
    4.0000000000000000f32,
    4.0874628412503390f32,
    4.1699250014423121f32,
    4.2479275134435852f32,
    4.3219280948873626f32,
    4.3923174227787606f32,
    4.4594316186372973f32,
    4.5235619560570130f32,
    4.5849625007211560f32,
    4.6438561897747243f32,
    4.7004397181410917f32,
    4.7548875021634682f32,
    4.8073549220576037f32,
    4.8579809951275718f32,
    4.9068905956085187f32,
    4.9541963103868749f32,
    5.0000000000000000f32,
    5.0443941193584533f32,
    5.0874628412503390f32,
    5.1292830169449663f32,
    5.1699250014423121f32,
    5.2094533656289501f32,
    5.2479275134435852f32,
    5.2854022188622487f32,
    5.3219280948873626f32,
    5.3575520046180837f32,
    5.3923174227787606f32,
    5.4262647547020979f32,
    5.4594316186372973f32,
    5.4918530963296747f32,
    5.5235619560570130f32,
    5.5545888516776376f32,
    5.5849625007211560f32,
    5.6147098441152083f32,
    5.6438561897747243f32,
    5.6724253419714951f32,
    5.7004397181410917f32,
    5.7279204545631987f32,
    5.7548875021634682f32,
    5.7813597135246599f32,
    5.8073549220576037f32,
    5.8328900141647412f32,
    5.8579809951275718f32,
    5.8826430493618415f32,
    5.9068905956085187f32,
    5.9307373375628866f32,
    5.9541963103868749f32,
    5.9772799234999167f32,
    6.0000000000000000f32,
    6.0223678130284543f32,
    6.0443941193584533f32,
    6.0660891904577720f32,
    6.0874628412503390f32,
    6.1085244567781691f32,
    6.1292830169449663f32,
    6.1497471195046822f32,
    6.1699250014423121f32,
    6.1898245588800175f32,
    6.2094533656289501f32,
    6.2288186904958804f32,
    6.2479275134435852f32,
    6.2667865406949010f32,
    6.2854022188622487f32,
    6.3037807481771030f32,
    6.3219280948873626f32,
    6.3398500028846243f32,
    6.3575520046180837f32,
    6.3750394313469245f32,
    6.3923174227787606f32,
    6.4093909361377017f32,
    6.4262647547020979f32,
    6.4429434958487279f32,
    6.4594316186372973f32,
    6.4757334309663976f32,
    6.4918530963296747f32,
    6.5077946401986963f32,
    6.5235619560570130f32,
    6.5391588111080309f32,
    6.5545888516776376f32,
    6.5698556083309478f32,
    6.5849625007211560f32,
    6.5999128421871278f32,
    6.6147098441152083f32,
    6.6293566200796094f32,
    6.6438561897747243f32,
    6.6582114827517946f32,
    6.6724253419714951f32,
    6.6865005271832185f32,
    6.7004397181410917f32,
    6.7142455176661224f32,
    6.7279204545631987f32,
    6.7414669864011464f32,
    6.7548875021634682f32,
    6.7681843247769259f32,
    6.7813597135246599f32,
    6.7944158663501061f32,
    6.8073549220576037f32,
    6.8201789624151878f32,
    6.8328900141647412f32,
    6.8454900509443747f32,
    6.8579809951275718f32,
    6.8703647195834047f32,
    6.8826430493618415f32,
    6.8948177633079437f32,
    6.9068905956085187f32,
    6.9188632372745946f32,
    6.9307373375628866f32,
    6.9425145053392398f32,
    6.9541963103868749f32,
    6.9657842846620869f32,
    6.9772799234999167f32,
    6.9886846867721654f32,
    7.0000000000000000f32,
    7.0112272554232539f32,
    7.0223678130284543f32,
    7.0334230015374501f32,
    7.0443941193584533f32,
    7.0552824355011898f32,
    7.0660891904577720f32,
    7.0768155970508308f32,
    7.0874628412503390f32,
    7.0980320829605263f32,
    7.1085244567781691f32,
    7.1189410727235076f32,
    7.1292830169449663f32,
    7.1395513523987936f32,
    7.1497471195046822f32,
    7.1598713367783890f32,
    7.1699250014423121f32,
    7.1799090900149344f32,
    7.1898245588800175f32,
    7.1996723448363644f32,
    7.2094533656289501f32,
    7.2191685204621611f32,
    7.2288186904958804f32,
    7.2384047393250785f32,
    7.2479275134435852f32,
    7.2573878426926521f32,
    7.2667865406949010f32,
    7.2761244052742375f32,
    7.2854022188622487f32,
    7.2946207488916270f32,
    7.3037807481771030f32,
    7.3128829552843557f32,
    7.3219280948873626f32,
    7.3309168781146167f32,
    7.3398500028846243f32,
    7.3487281542310771f32,
    7.3575520046180837f32,
    7.3663222142458160f32,
    7.3750394313469245f32,
    7.3837042924740519f32,
    7.3923174227787606f32,
    7.4008794362821843f32,
    7.4093909361377017f32,
    7.4178525148858982f32,
    7.4262647547020979f32,
    7.4346282276367245f32,
    7.4429434958487279f32,
    7.4512111118323289f32,
    7.4594316186372973f32,
    7.4676055500829976f32,
    7.4757334309663976f32,
    7.4838157772642563f32,
    7.4918530963296747f32,
    7.4998458870832056f32,
    7.5077946401986963f32,
    7.5156998382840427f32,
    7.5235619560570130f32,
    7.5313814605163118f32,
    7.5391588111080309f32,
    7.5468944598876364f32,
    7.5545888516776376f32,
    7.5622424242210728f32,
    7.5698556083309478f32,
    7.5774288280357486f32,
    7.5849625007211560f32,
    7.5924570372680806f32,
    7.5999128421871278f32,
    7.6073303137496104f32,
    7.6147098441152083f32,
    7.6220518194563764f32,
    7.6293566200796094f32,
    7.6366246205436487f32,
    7.6438561897747243f32,
    7.6510516911789281f32,
    7.6582114827517946f32,
    7.6653359171851764f32,
    7.6724253419714951f32,
    7.6794800995054464f32,
    7.6865005271832185f32,
    7.6934869574993252f32,
    7.7004397181410917f32,
    7.7073591320808825f32,
    7.7142455176661224f32,
    7.7210991887071855f32,
    7.7279204545631987f32,
    7.7347096202258383f32,
    7.7414669864011464f32,
    7.7481928495894605f32,
    7.7548875021634682f32,
    7.7615512324444795f32,
    7.7681843247769259f32,
    7.7747870596011736f32,
    7.7813597135246599f32,
    7.7879025593914317f32,
    7.7944158663501061f32,
    7.8008998999203047f32,
    7.8073549220576037f32,
    7.8137811912170374f32,
    7.8201789624151878f32,
    7.8265484872909150f32,
    7.8328900141647412f32,
    7.8392037880969436f32,
    7.8454900509443747f32,
    7.8517490414160571f32,
    7.8579809951275718f32,
    7.8641861446542797f32,
    7.8703647195834047f32,
    7.8765169465649993f32,
    7.8826430493618415f32,
    7.8887432488982591f32,
    7.8948177633079437f32,
    7.9008668079807486f32,
    7.9068905956085187f32,
    7.9128893362299619f32,
    7.9188632372745946f32,
    7.9248125036057812f32,
    7.9307373375628866f32,
    7.9366379390025709f32,
    7.9425145053392398f32,
    7.9483672315846778f32,
    7.9541963103868749f32,
    7.9600019320680805f32,
    7.9657842846620869f32,
    7.9715435539507719f32,
    7.9772799234999167f32,
    7.9829935746943103f32,
    7.9886846867721654f32,
    7.9943534368588577f32,
];
#[no_mangle]
pub static mut kSLog2Table: [libc::c_float; 256] = [
    0.00000000f32,
    0.00000000f32,
    2.00000000f32,
    4.75488750f32,
    8.00000000f32,
    11.60964047f32,
    15.50977500f32,
    19.65148445f32,
    24.00000000f32,
    28.52932501f32,
    33.21928095f32,
    38.05374781f32,
    43.01955001f32,
    48.10571634f32,
    53.30296891f32,
    58.60335893f32,
    64.00000000f32,
    69.48686830f32,
    75.05865003f32,
    80.71062276f32,
    86.43856190f32,
    92.23866588f32,
    98.10749561f32,
    104.04192499f32,
    110.03910002f32,
    116.09640474f32,
    122.21143267f32,
    128.38196256f32,
    134.60593782f32,
    140.88144886f32,
    147.20671787f32,
    153.58008562f32,
    160.00000000f32,
    166.46500594f32,
    172.97373660f32,
    179.52490559f32,
    186.11730005f32,
    192.74977453f32,
    199.42124551f32,
    206.13068654f32,
    212.87712380f32,
    219.65963219f32,
    226.47733176f32,
    233.32938445f32,
    240.21499122f32,
    247.13338933f32,
    254.08384998f32,
    261.06567603f32,
    268.07820003f32,
    275.12078236f32,
    282.19280949f32,
    289.29369244f32,
    296.42286534f32,
    303.57978409f32,
    310.76392512f32,
    317.97478424f32,
    325.21187564f32,
    332.47473081f32,
    339.76289772f32,
    347.07593991f32,
    354.41343574f32,
    361.77497759f32,
    369.16017124f32,
    376.56863518f32,
    384.00000000f32,
    391.45390785f32,
    398.93001188f32,
    406.42797576f32,
    413.94747321f32,
    421.48818752f32,
    429.04981119f32,
    436.63204548f32,
    444.23460010f32,
    451.85719280f32,
    459.49954906f32,
    467.16140179f32,
    474.84249102f32,
    482.54256363f32,
    490.26137307f32,
    497.99867911f32,
    505.75424759f32,
    513.52785023f32,
    521.31926438f32,
    529.12827280f32,
    536.95466351f32,
    544.79822957f32,
    552.65876890f32,
    560.53608414f32,
    568.42998244f32,
    576.34027536f32,
    584.26677867f32,
    592.20931226f32,
    600.16769996f32,
    608.14176943f32,
    616.13135206f32,
    624.13628279f32,
    632.15640007f32,
    640.19154569f32,
    648.24156472f32,
    656.30630539f32,
    664.38561898f32,
    672.47935976f32,
    680.58738488f32,
    688.70955430f32,
    696.84573069f32,
    704.99577935f32,
    713.15956818f32,
    721.33696754f32,
    729.52785023f32,
    737.73209140f32,
    745.94956849f32,
    754.18016116f32,
    762.42375127f32,
    770.68022275f32,
    778.94946161f32,
    787.23135586f32,
    795.52579543f32,
    803.83267219f32,
    812.15187982f32,
    820.48331383f32,
    828.82687147f32,
    837.18245171f32,
    845.54995518f32,
    853.92928416f32,
    862.32034249f32,
    870.72303558f32,
    879.13727036f32,
    887.56295522f32,
    896.00000000f32,
    904.44831595f32,
    912.90781569f32,
    921.37841320f32,
    929.86002376f32,
    938.35256392f32,
    946.85595152f32,
    955.37010560f32,
    963.89494641f32,
    972.43039537f32,
    980.97637504f32,
    989.53280911f32,
    998.09962237f32,
    1006.67674069f32,
    1015.26409097f32,
    1023.86160116f32,
    1032.46920021f32,
    1041.08681805f32,
    1049.71438560f32,
    1058.35183469f32,
    1066.99909811f32,
    1075.65610955f32,
    1084.32280357f32,
    1092.99911564f32,
    1101.68498204f32,
    1110.38033993f32,
    1119.08512727f32,
    1127.79928282f32,
    1136.52274614f32,
    1145.25545758f32,
    1153.99735821f32,
    1162.74838989f32,
    1171.50849518f32,
    1180.27761738f32,
    1189.05570047f32,
    1197.84268914f32,
    1206.63852876f32,
    1215.44316535f32,
    1224.25654560f32,
    1233.07861684f32,
    1241.90932703f32,
    1250.74862473f32,
    1259.59645914f32,
    1268.45278005f32,
    1277.31753781f32,
    1286.19068338f32,
    1295.07216828f32,
    1303.96194457f32,
    1312.85996488f32,
    1321.76618236f32,
    1330.68055071f32,
    1339.60302413f32,
    1348.53355734f32,
    1357.47210556f32,
    1366.41862452f32,
    1375.37307041f32,
    1384.33539991f32,
    1393.30557020f32,
    1402.28353887f32,
    1411.26926400f32,
    1420.26270412f32,
    1429.26381818f32,
    1438.27256558f32,
    1447.28890615f32,
    1456.31280014f32,
    1465.34420819f32,
    1474.38309138f32,
    1483.42941118f32,
    1492.48312945f32,
    1501.54420843f32,
    1510.61261078f32,
    1519.68829949f32,
    1528.77123795f32,
    1537.86138993f32,
    1546.95871952f32,
    1556.06319119f32,
    1565.17476976f32,
    1574.29342040f32,
    1583.41910860f32,
    1592.55180020f32,
    1601.69146137f32,
    1610.83805860f32,
    1619.99155871f32,
    1629.15192882f32,
    1638.31913637f32,
    1647.49314911f32,
    1656.67393509f32,
    1665.86146266f32,
    1675.05570047f32,
    1684.25661744f32,
    1693.46418280f32,
    1702.67836605f32,
    1711.89913698f32,
    1721.12646563f32,
    1730.36032233f32,
    1739.60067768f32,
    1748.84750254f32,
    1758.10076802f32,
    1767.36044551f32,
    1776.62650662f32,
    1785.89892323f32,
    1795.17766747f32,
    1804.46271172f32,
    1813.75402857f32,
    1823.05159087f32,
    1832.35537170f32,
    1841.66534438f32,
    1850.98148244f32,
    1860.30375965f32,
    1869.63214999f32,
    1878.96662767f32,
    1888.30716711f32,
    1897.65374295f32,
    1907.00633003f32,
    1916.36490342f32,
    1925.72943838f32,
    1935.09991037f32,
    1944.47629506f32,
    1953.85856831f32,
    1963.24670620f32,
    1972.64068498f32,
    1982.04048108f32,
    1991.44607117f32,
    2000.85743204f32,
    2010.27454072f32,
    2019.69737440f32,
    2029.12591044f32,
    2038.56012640f32,
];
#[no_mangle]
pub static mut kPrefixEncodeCode: [VP8LPrefixCode; 512] = [
    {
        let mut init = VP8LPrefixCode {
            code_: 0 as libc::c_int as int8_t,
            extra_bits_: 0 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 0 as libc::c_int as int8_t,
            extra_bits_: 0 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 1 as libc::c_int as int8_t,
            extra_bits_: 0 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 2 as libc::c_int as int8_t,
            extra_bits_: 0 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 3 as libc::c_int as int8_t,
            extra_bits_: 0 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 4 as libc::c_int as int8_t,
            extra_bits_: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 4 as libc::c_int as int8_t,
            extra_bits_: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 5 as libc::c_int as int8_t,
            extra_bits_: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 5 as libc::c_int as int8_t,
            extra_bits_: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 6 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 6 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 6 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 6 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 7 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 7 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 7 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 7 as libc::c_int as int8_t,
            extra_bits_: 2 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 8 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 9 as libc::c_int as int8_t,
            extra_bits_: 3 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 10 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 11 as libc::c_int as int8_t,
            extra_bits_: 4 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 12 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 13 as libc::c_int as int8_t,
            extra_bits_: 5 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 14 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 15 as libc::c_int as int8_t,
            extra_bits_: 6 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 16 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = VP8LPrefixCode {
            code_: 17 as libc::c_int as int8_t,
            extra_bits_: 7 as libc::c_int as int8_t,
        };
        init
    },
];
#[no_mangle]
pub static mut kPrefixEncodeExtraBitsValue: [uint8_t; 512] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    64 as libc::c_int as uint8_t,
    65 as libc::c_int as uint8_t,
    66 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
    68 as libc::c_int as uint8_t,
    69 as libc::c_int as uint8_t,
    70 as libc::c_int as uint8_t,
    71 as libc::c_int as uint8_t,
    72 as libc::c_int as uint8_t,
    73 as libc::c_int as uint8_t,
    74 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    77 as libc::c_int as uint8_t,
    78 as libc::c_int as uint8_t,
    79 as libc::c_int as uint8_t,
    80 as libc::c_int as uint8_t,
    81 as libc::c_int as uint8_t,
    82 as libc::c_int as uint8_t,
    83 as libc::c_int as uint8_t,
    84 as libc::c_int as uint8_t,
    85 as libc::c_int as uint8_t,
    86 as libc::c_int as uint8_t,
    87 as libc::c_int as uint8_t,
    88 as libc::c_int as uint8_t,
    89 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    91 as libc::c_int as uint8_t,
    92 as libc::c_int as uint8_t,
    93 as libc::c_int as uint8_t,
    94 as libc::c_int as uint8_t,
    95 as libc::c_int as uint8_t,
    96 as libc::c_int as uint8_t,
    97 as libc::c_int as uint8_t,
    98 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    100 as libc::c_int as uint8_t,
    101 as libc::c_int as uint8_t,
    102 as libc::c_int as uint8_t,
    103 as libc::c_int as uint8_t,
    104 as libc::c_int as uint8_t,
    105 as libc::c_int as uint8_t,
    106 as libc::c_int as uint8_t,
    107 as libc::c_int as uint8_t,
    108 as libc::c_int as uint8_t,
    109 as libc::c_int as uint8_t,
    110 as libc::c_int as uint8_t,
    111 as libc::c_int as uint8_t,
    112 as libc::c_int as uint8_t,
    113 as libc::c_int as uint8_t,
    114 as libc::c_int as uint8_t,
    115 as libc::c_int as uint8_t,
    116 as libc::c_int as uint8_t,
    117 as libc::c_int as uint8_t,
    118 as libc::c_int as uint8_t,
    119 as libc::c_int as uint8_t,
    120 as libc::c_int as uint8_t,
    121 as libc::c_int as uint8_t,
    122 as libc::c_int as uint8_t,
    123 as libc::c_int as uint8_t,
    124 as libc::c_int as uint8_t,
    125 as libc::c_int as uint8_t,
    126 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    64 as libc::c_int as uint8_t,
    65 as libc::c_int as uint8_t,
    66 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
    68 as libc::c_int as uint8_t,
    69 as libc::c_int as uint8_t,
    70 as libc::c_int as uint8_t,
    71 as libc::c_int as uint8_t,
    72 as libc::c_int as uint8_t,
    73 as libc::c_int as uint8_t,
    74 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    77 as libc::c_int as uint8_t,
    78 as libc::c_int as uint8_t,
    79 as libc::c_int as uint8_t,
    80 as libc::c_int as uint8_t,
    81 as libc::c_int as uint8_t,
    82 as libc::c_int as uint8_t,
    83 as libc::c_int as uint8_t,
    84 as libc::c_int as uint8_t,
    85 as libc::c_int as uint8_t,
    86 as libc::c_int as uint8_t,
    87 as libc::c_int as uint8_t,
    88 as libc::c_int as uint8_t,
    89 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    91 as libc::c_int as uint8_t,
    92 as libc::c_int as uint8_t,
    93 as libc::c_int as uint8_t,
    94 as libc::c_int as uint8_t,
    95 as libc::c_int as uint8_t,
    96 as libc::c_int as uint8_t,
    97 as libc::c_int as uint8_t,
    98 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    100 as libc::c_int as uint8_t,
    101 as libc::c_int as uint8_t,
    102 as libc::c_int as uint8_t,
    103 as libc::c_int as uint8_t,
    104 as libc::c_int as uint8_t,
    105 as libc::c_int as uint8_t,
    106 as libc::c_int as uint8_t,
    107 as libc::c_int as uint8_t,
    108 as libc::c_int as uint8_t,
    109 as libc::c_int as uint8_t,
    110 as libc::c_int as uint8_t,
    111 as libc::c_int as uint8_t,
    112 as libc::c_int as uint8_t,
    113 as libc::c_int as uint8_t,
    114 as libc::c_int as uint8_t,
    115 as libc::c_int as uint8_t,
    116 as libc::c_int as uint8_t,
    117 as libc::c_int as uint8_t,
    118 as libc::c_int as uint8_t,
    119 as libc::c_int as uint8_t,
    120 as libc::c_int as uint8_t,
    121 as libc::c_int as uint8_t,
    122 as libc::c_int as uint8_t,
    123 as libc::c_int as uint8_t,
    124 as libc::c_int as uint8_t,
    125 as libc::c_int as uint8_t,
    126 as libc::c_int as uint8_t,
];
unsafe extern "C" fn FastSLog2Slow_C(mut v: uint32_t) -> libc::c_float {
    if v < 65536 as libc::c_int as libc::c_uint {
        let log_cnt: libc::c_int = BitsLog2Floor(v) - 7 as libc::c_int;
        let y: uint32_t = ((1 as libc::c_int) << log_cnt) as uint32_t;
        let mut correction: libc::c_int = 0 as libc::c_int;
        let v_f: libc::c_float = v as libc::c_float;
        let orig_v: uint32_t = v;
        v >>= log_cnt;
        correction = ((23 as libc::c_int as libc::c_uint)
            .wrapping_mul(orig_v & y.wrapping_sub(1 as libc::c_int as libc::c_uint))
            >> 4 as libc::c_int) as libc::c_int;
        return v_f * (kLog2Table[v as usize] + log_cnt as libc::c_float)
            + correction as libc::c_float;
    } else {
        return (1.44269504088896338700465094007086f64 * v as libc::c_double
            * log(v as libc::c_double)) as libc::c_float
    };
}
unsafe extern "C" fn FastLog2Slow_C(mut v: uint32_t) -> libc::c_float {
    if v < 65536 as libc::c_int as libc::c_uint {
        let log_cnt: libc::c_int = BitsLog2Floor(v) - 7 as libc::c_int;
        let y: uint32_t = ((1 as libc::c_int) << log_cnt) as uint32_t;
        let orig_v: uint32_t = v;
        let mut log_2: libc::c_double = 0.;
        v >>= log_cnt;
        log_2 = (kLog2Table[v as usize] + log_cnt as libc::c_float) as libc::c_double;
        if orig_v >= 4096 as libc::c_int as libc::c_uint {
            let correction: libc::c_int = ((23 as libc::c_int as libc::c_uint)
                .wrapping_mul(orig_v & y.wrapping_sub(1 as libc::c_int as libc::c_uint))
                >> 4 as libc::c_int) as libc::c_int;
            log_2 += correction as libc::c_double / orig_v as libc::c_double;
        }
        return log_2 as libc::c_float;
    } else {
        return (1.44269504088896338700465094007086f64 * log(v as libc::c_double))
            as libc::c_float
    };
}
unsafe extern "C" fn CombinedShannonEntropy_C(
    mut X: *const libc::c_int,
    mut Y: *const libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_float = 0.0f32;
    let mut sumX: libc::c_int = 0 as libc::c_int;
    let mut sumXY: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let x: libc::c_int = *X.offset(i as isize);
        if x != 0 as libc::c_int {
            let xy: libc::c_int = x + *Y.offset(i as isize);
            sumX += x;
            retval -= VP8LFastSLog2(x as uint32_t);
            sumXY += xy;
            retval -= VP8LFastSLog2(xy as uint32_t);
        } else if *Y.offset(i as isize) != 0 as libc::c_int {
            sumXY += *Y.offset(i as isize);
            retval -= VP8LFastSLog2(*Y.offset(i as isize) as uint32_t);
        }
        i += 1;
        i;
    }
    retval += VP8LFastSLog2(sumX as uint32_t) + VP8LFastSLog2(sumXY as uint32_t);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitEntropyInit(entropy: *mut VP8LBitEntropy) {
    (*entropy).entropy = 0.0f64 as libc::c_float;
    (*entropy).sum = 0 as libc::c_int as uint32_t;
    (*entropy).nonzeros = 0 as libc::c_int;
    (*entropy).max_val = 0 as libc::c_int as uint32_t;
    (*entropy).nonzero_code = 0xffffffff as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitsEntropyUnrefined(
    array: *const uint32_t,
    mut n: libc::c_int,
    entropy: *mut VP8LBitEntropy,
) {
    let mut i: libc::c_int = 0;
    VP8LBitEntropyInit(entropy);
    i = 0 as libc::c_int;
    while i < n {
        if *array.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            (*entropy)
                .sum = ((*entropy).sum as libc::c_uint)
                .wrapping_add(*array.offset(i as isize)) as uint32_t as uint32_t;
            (*entropy).nonzero_code = i as uint32_t;
            (*entropy).nonzeros += 1;
            (*entropy).nonzeros;
            (*entropy).entropy -= VP8LFastSLog2(*array.offset(i as isize));
            if (*entropy).max_val < *array.offset(i as isize) {
                (*entropy).max_val = *array.offset(i as isize);
            }
        }
        i += 1;
        i;
    }
    (*entropy).entropy += VP8LFastSLog2((*entropy).sum);
}
#[inline]
unsafe extern "C" fn GetEntropyUnrefinedHelper(
    mut val: uint32_t,
    mut i: libc::c_int,
    val_prev: *mut uint32_t,
    i_prev: *mut libc::c_int,
    bit_entropy: *mut VP8LBitEntropy,
    stats: *mut VP8LStreaks,
) {
    let streak: libc::c_int = i - *i_prev;
    if *val_prev != 0 as libc::c_int as libc::c_uint {
        (*bit_entropy)
            .sum = ((*bit_entropy).sum as libc::c_uint)
            .wrapping_add((*val_prev).wrapping_mul(streak as libc::c_uint)) as uint32_t
            as uint32_t;
        (*bit_entropy).nonzeros += streak;
        (*bit_entropy).nonzero_code = *i_prev as uint32_t;
        (*bit_entropy).entropy -= VP8LFastSLog2(*val_prev) * streak as libc::c_float;
        if (*bit_entropy).max_val < *val_prev {
            (*bit_entropy).max_val = *val_prev;
        }
    }
    (*stats)
        .counts[(*val_prev != 0 as libc::c_int as libc::c_uint) as libc::c_int as usize]
        += (streak > 3 as libc::c_int) as libc::c_int;
    (*stats)
        .streaks[(*val_prev != 0 as libc::c_int as libc::c_uint) as libc::c_int
        as usize][(streak > 3 as libc::c_int) as libc::c_int as usize] += streak;
    *val_prev = val;
    *i_prev = i;
}
unsafe extern "C" fn GetEntropyUnrefined_C(
    mut X: *const uint32_t,
    mut length: libc::c_int,
    bit_entropy: *mut VP8LBitEntropy,
    stats: *mut VP8LStreaks,
) {
    let mut i: libc::c_int = 0;
    let mut i_prev: libc::c_int = 0 as libc::c_int;
    let mut x_prev: uint32_t = *X.offset(0 as libc::c_int as isize);
    memset(
        stats as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LStreaks>() as libc::c_ulong,
    );
    VP8LBitEntropyInit(bit_entropy);
    i = 1 as libc::c_int;
    while i < length {
        let x: uint32_t = *X.offset(i as isize);
        if x != x_prev {
            GetEntropyUnrefinedHelper(
                x,
                i,
                &mut x_prev,
                &mut i_prev,
                bit_entropy,
                stats,
            );
        }
        i += 1;
        i;
    }
    GetEntropyUnrefinedHelper(
        0 as libc::c_int as uint32_t,
        i,
        &mut x_prev,
        &mut i_prev,
        bit_entropy,
        stats,
    );
    (*bit_entropy).entropy += VP8LFastSLog2((*bit_entropy).sum);
}
unsafe extern "C" fn GetCombinedEntropyUnrefined_C(
    mut X: *const uint32_t,
    mut Y: *const uint32_t,
    mut length: libc::c_int,
    bit_entropy: *mut VP8LBitEntropy,
    stats: *mut VP8LStreaks,
) {
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut i_prev: libc::c_int = 0 as libc::c_int;
    let mut xy_prev: uint32_t = (*X.offset(0 as libc::c_int as isize))
        .wrapping_add(*Y.offset(0 as libc::c_int as isize));
    memset(
        stats as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LStreaks>() as libc::c_ulong,
    );
    VP8LBitEntropyInit(bit_entropy);
    i = 1 as libc::c_int;
    while i < length {
        let xy: uint32_t = (*X.offset(i as isize)).wrapping_add(*Y.offset(i as isize));
        if xy != xy_prev {
            GetEntropyUnrefinedHelper(
                xy,
                i,
                &mut xy_prev,
                &mut i_prev,
                bit_entropy,
                stats,
            );
        }
        i += 1;
        i;
    }
    GetEntropyUnrefinedHelper(
        0 as libc::c_int as uint32_t,
        i,
        &mut xy_prev,
        &mut i_prev,
        bit_entropy,
        stats,
    );
    (*bit_entropy).entropy += VP8LFastSLog2((*bit_entropy).sum);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LSubtractGreenFromBlueAndRed_C(
    mut argb_data: *mut uint32_t,
    mut num_pixels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let argb: libc::c_int = *argb_data.offset(i as isize) as libc::c_int;
        let green: libc::c_int = argb >> 8 as libc::c_int & 0xff as libc::c_int;
        let new_r: uint32_t = ((argb >> 16 as libc::c_int & 0xff as libc::c_int) - green
            & 0xff as libc::c_int) as uint32_t;
        let new_b: uint32_t = ((argb >> 0 as libc::c_int & 0xff as libc::c_int) - green
            & 0xff as libc::c_int) as uint32_t;
        *argb_data
            .offset(
                i as isize,
            ) = argb as uint32_t & 0xff00ff00 as libc::c_uint
            | new_r << 16 as libc::c_int | new_b;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ColorTransformDelta(
    mut color_pred: int8_t,
    mut color: int8_t,
) -> libc::c_int {
    return color_pred as libc::c_int * color as libc::c_int >> 5 as libc::c_int;
}
#[inline]
unsafe extern "C" fn U32ToS8(mut v: uint32_t) -> int8_t {
    return (v & 0xff as libc::c_int as libc::c_uint) as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LTransformColor_C(
    m: *const VP8LMultipliers,
    mut data: *mut uint32_t,
    mut num_pixels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let argb: uint32_t = *data.offset(i as isize);
        let green: int8_t = U32ToS8(argb >> 8 as libc::c_int);
        let red: int8_t = U32ToS8(argb >> 16 as libc::c_int);
        let mut new_red: libc::c_int = red as libc::c_int & 0xff as libc::c_int;
        let mut new_blue: libc::c_int = (argb & 0xff as libc::c_int as libc::c_uint)
            as libc::c_int;
        new_red -= ColorTransformDelta((*m).green_to_red_ as int8_t, green);
        new_red &= 0xff as libc::c_int;
        new_blue -= ColorTransformDelta((*m).green_to_blue_ as int8_t, green);
        new_blue -= ColorTransformDelta((*m).red_to_blue_ as int8_t, red);
        new_blue &= 0xff as libc::c_int;
        *data
            .offset(
                i as isize,
            ) = argb & 0xff00ff00 as libc::c_uint
            | (new_red << 16 as libc::c_int) as libc::c_uint | new_blue as libc::c_uint;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn TransformColorRed(
    mut green_to_red: uint8_t,
    mut argb: uint32_t,
) -> uint8_t {
    let green: int8_t = U32ToS8(argb >> 8 as libc::c_int);
    let mut new_red: libc::c_int = (argb >> 16 as libc::c_int) as libc::c_int;
    new_red -= ColorTransformDelta(green_to_red as int8_t, green);
    return (new_red & 0xff as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn TransformColorBlue(
    mut green_to_blue: uint8_t,
    mut red_to_blue: uint8_t,
    mut argb: uint32_t,
) -> uint8_t {
    let green: int8_t = U32ToS8(argb >> 8 as libc::c_int);
    let red: int8_t = U32ToS8(argb >> 16 as libc::c_int);
    let mut new_blue: libc::c_int = (argb & 0xff as libc::c_int as libc::c_uint)
        as libc::c_int;
    new_blue -= ColorTransformDelta(green_to_blue as int8_t, green);
    new_blue -= ColorTransformDelta(red_to_blue as int8_t, red);
    return (new_blue & 0xff as libc::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LCollectColorRedTransforms_C(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut green_to_red: libc::c_int,
    mut histo: *mut libc::c_int,
) {
    loop {
        let fresh0 = tile_height;
        tile_height = tile_height - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < tile_width {
            let ref mut fresh1 = *histo
                .offset(
                    TransformColorRed(green_to_red as uint8_t, *argb.offset(x as isize))
                        as isize,
                );
            *fresh1 += 1;
            *fresh1;
            x += 1;
            x;
        }
        argb = argb.offset(stride as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LCollectColorBlueTransforms_C(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut green_to_blue: libc::c_int,
    mut red_to_blue: libc::c_int,
    mut histo: *mut libc::c_int,
) {
    loop {
        let fresh2 = tile_height;
        tile_height = tile_height - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < tile_width {
            let ref mut fresh3 = *histo
                .offset(
                    TransformColorBlue(
                        green_to_blue as uint8_t,
                        red_to_blue as uint8_t,
                        *argb.offset(x as isize),
                    ) as isize,
                );
            *fresh3 += 1;
            *fresh3;
            x += 1;
            x;
        }
        argb = argb.offset(stride as isize);
    };
}
unsafe extern "C" fn VectorMismatch_C(
    array1: *const uint32_t,
    array2: *const uint32_t,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut match_len: libc::c_int = 0 as libc::c_int;
    while match_len < length
        && *array1.offset(match_len as isize) == *array2.offset(match_len as isize)
    {
        match_len += 1;
        match_len;
    }
    return match_len;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBundleColorMap_C(
    row: *const uint8_t,
    mut width: libc::c_int,
    mut xbits: libc::c_int,
    mut dst: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    if xbits > 0 as libc::c_int {
        let bit_depth: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int - xbits;
        let mask: libc::c_int = ((1 as libc::c_int) << xbits) - 1 as libc::c_int;
        let mut code: uint32_t = 0xff000000 as libc::c_uint;
        x = 0 as libc::c_int;
        while x < width {
            let xsub: libc::c_int = x & mask;
            if xsub == 0 as libc::c_int {
                code = 0xff000000 as libc::c_uint;
            }
            code
                |= ((*row.offset(x as isize) as libc::c_int)
                    << 8 as libc::c_int + bit_depth * xsub) as libc::c_uint;
            *dst.offset((x >> xbits) as isize) = code;
            x += 1;
            x;
        }
    } else {
        x = 0 as libc::c_int;
        while x < width {
            *dst
                .offset(
                    x as isize,
                ) = 0xff000000 as libc::c_uint
                | ((*row.offset(x as isize) as libc::c_int) << 8 as libc::c_int)
                    as libc::c_uint;
            x += 1;
            x;
        }
    };
}
unsafe extern "C" fn ExtraCost_C(
    mut population: *const uint32_t,
    mut length: libc::c_int,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut cost: uint32_t = (*population.offset(4 as libc::c_int as isize))
        .wrapping_add(*population.offset(5 as libc::c_int as isize));
    i = 2 as libc::c_int;
    while i < length / 2 as libc::c_int - 1 as libc::c_int {
        cost = (cost as libc::c_uint)
            .wrapping_add(
                (i as libc::c_uint)
                    .wrapping_mul(
                        (*population
                            .offset((2 as libc::c_int * i + 2 as libc::c_int) as isize))
                            .wrapping_add(
                                *population
                                    .offset((2 as libc::c_int * i + 3 as libc::c_int) as isize),
                            ),
                    ),
            ) as uint32_t as uint32_t;
        i += 1;
        i;
    }
    return cost;
}
unsafe extern "C" fn ExtraCostCombined_C(
    mut X: *const uint32_t,
    mut Y: *const uint32_t,
    mut length: libc::c_int,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut cost: uint32_t = (*X.offset(4 as libc::c_int as isize))
        .wrapping_add(*Y.offset(4 as libc::c_int as isize))
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(*Y.offset(5 as libc::c_int as isize));
    i = 2 as libc::c_int;
    while i < length / 2 as libc::c_int - 1 as libc::c_int {
        let xy0: libc::c_int = (*X
            .offset((2 as libc::c_int * i + 2 as libc::c_int) as isize))
            .wrapping_add(*Y.offset((2 as libc::c_int * i + 2 as libc::c_int) as isize))
            as libc::c_int;
        let xy1: libc::c_int = (*X
            .offset((2 as libc::c_int * i + 3 as libc::c_int) as isize))
            .wrapping_add(*Y.offset((2 as libc::c_int * i + 3 as libc::c_int) as isize))
            as libc::c_int;
        cost = (cost as libc::c_uint).wrapping_add((i * (xy0 + xy1)) as libc::c_uint)
            as uint32_t as uint32_t;
        i += 1;
        i;
    }
    return cost;
}
unsafe extern "C" fn AddVector_C(
    mut a: *const uint32_t,
    mut b: *const uint32_t,
    mut out: *mut uint32_t,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *out
            .offset(
                i as isize,
            ) = (*a.offset(i as isize)).wrapping_add(*b.offset(i as isize));
        i += 1;
        i;
    }
}
unsafe extern "C" fn AddVectorEq_C(
    mut a: *const uint32_t,
    mut out: *mut uint32_t,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let ref mut fresh4 = *out.offset(i as isize);
        *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(*a.offset(i as isize))
            as uint32_t as uint32_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramAdd(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    out: *mut VP8LHistogram,
) {
    let mut i: libc::c_int = 0;
    let literal_size: libc::c_int = VP8LHistogramNumCodes((*a).palette_code_bits_);
    if b != out as *const VP8LHistogram {
        if (*a).is_used_[0 as libc::c_int as usize] != 0 {
            if (*b).is_used_[0 as libc::c_int as usize] != 0 {
                VP8LAddVector
                    .expect(
                        "non-null function pointer",
                    )((*a).literal_, (*b).literal_, (*out).literal_, literal_size);
            } else {
                memcpy(
                    &mut *((*out).literal_).offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &mut *((*a).literal_).offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *const libc::c_void,
                    (literal_size as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        } else if (*b).is_used_[0 as libc::c_int as usize] != 0 {
            memcpy(
                &mut *((*out).literal_).offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                &mut *((*b).literal_).offset(0 as libc::c_int as isize) as *mut uint32_t
                    as *const libc::c_void,
                (literal_size as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memset(
                &mut *((*out).literal_).offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                0 as libc::c_int,
                (literal_size as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        if (*a).is_used_[1 as libc::c_int as usize] != 0 {
            if (*b).is_used_[1 as libc::c_int as usize] != 0 {
                VP8LAddVector
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).red_).as_ptr(),
                    ((*b).red_).as_ptr(),
                    ((*out).red_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).red_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).red_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        } else if (*b).is_used_[1 as libc::c_int as usize] != 0 {
            memcpy(
                &mut *((*out).red_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                &*((*b).red_).as_ptr().offset(0 as libc::c_int as isize)
                    as *const uint32_t as *const libc::c_void,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memset(
                &mut *((*out).red_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                0 as libc::c_int,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        if (*a).is_used_[2 as libc::c_int as usize] != 0 {
            if (*b).is_used_[2 as libc::c_int as usize] != 0 {
                VP8LAddVector
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).blue_).as_ptr(),
                    ((*b).blue_).as_ptr(),
                    ((*out).blue_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).blue_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).blue_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        } else if (*b).is_used_[2 as libc::c_int as usize] != 0 {
            memcpy(
                &mut *((*out).blue_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                &*((*b).blue_).as_ptr().offset(0 as libc::c_int as isize)
                    as *const uint32_t as *const libc::c_void,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memset(
                &mut *((*out).blue_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                0 as libc::c_int,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        if (*a).is_used_[3 as libc::c_int as usize] != 0 {
            if (*b).is_used_[3 as libc::c_int as usize] != 0 {
                VP8LAddVector
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).alpha_).as_ptr(),
                    ((*b).alpha_).as_ptr(),
                    ((*out).alpha_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).alpha_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).alpha_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        } else if (*b).is_used_[3 as libc::c_int as usize] != 0 {
            memcpy(
                &mut *((*out).alpha_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                &*((*b).alpha_).as_ptr().offset(0 as libc::c_int as isize)
                    as *const uint32_t as *const libc::c_void,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memset(
                &mut *((*out).alpha_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                0 as libc::c_int,
                (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        if (*a).is_used_[4 as libc::c_int as usize] != 0 {
            if (*b).is_used_[4 as libc::c_int as usize] != 0 {
                VP8LAddVector
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).distance_).as_ptr(),
                    ((*b).distance_).as_ptr(),
                    ((*out).distance_).as_mut_ptr(),
                    40 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).distance_)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint32_t
                        as *mut libc::c_void,
                    &*((*a).distance_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (40 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        } else if (*b).is_used_[4 as libc::c_int as usize] != 0 {
            memcpy(
                &mut *((*out).distance_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                &*((*b).distance_).as_ptr().offset(0 as libc::c_int as isize)
                    as *const uint32_t as *const libc::c_void,
                (40 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memset(
                &mut *((*out).distance_).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint32_t as *mut libc::c_void,
                0 as libc::c_int,
                (40 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            (*out)
                .is_used_[i
                as usize] = ((*a).is_used_[i as usize] as libc::c_int
                | (*b).is_used_[i as usize] as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
    } else {
        if (*a).is_used_[0 as libc::c_int as usize] != 0 {
            if (*out).is_used_[0 as libc::c_int as usize] != 0 {
                VP8LAddVectorEq
                    .expect(
                        "non-null function pointer",
                    )((*a).literal_, (*out).literal_, literal_size);
            } else {
                memcpy(
                    &mut *((*out).literal_).offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &mut *((*a).literal_).offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *const libc::c_void,
                    (literal_size as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        if (*a).is_used_[1 as libc::c_int as usize] != 0 {
            if (*out).is_used_[1 as libc::c_int as usize] != 0 {
                VP8LAddVectorEq
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).red_).as_ptr(),
                    ((*out).red_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).red_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).red_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        if (*a).is_used_[2 as libc::c_int as usize] != 0 {
            if (*out).is_used_[2 as libc::c_int as usize] != 0 {
                VP8LAddVectorEq
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).blue_).as_ptr(),
                    ((*out).blue_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).blue_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).blue_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        if (*a).is_used_[3 as libc::c_int as usize] != 0 {
            if (*out).is_used_[3 as libc::c_int as usize] != 0 {
                VP8LAddVectorEq
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).alpha_).as_ptr(),
                    ((*out).alpha_).as_mut_ptr(),
                    256 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).alpha_).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut uint32_t as *mut libc::c_void,
                    &*((*a).alpha_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        if (*a).is_used_[4 as libc::c_int as usize] != 0 {
            if (*out).is_used_[4 as libc::c_int as usize] != 0 {
                VP8LAddVectorEq
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*a).distance_).as_ptr(),
                    ((*out).distance_).as_mut_ptr(),
                    40 as libc::c_int,
                );
            } else {
                memcpy(
                    &mut *((*out).distance_)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint32_t
                        as *mut libc::c_void,
                    &*((*a).distance_).as_ptr().offset(0 as libc::c_int as isize)
                        as *const uint32_t as *const libc::c_void,
                    (40 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            (*out)
                .is_used_[i
                as usize] = ((*out).is_used_[i as usize] as libc::c_int
                | (*a).is_used_[i as usize] as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn PredictorSub0_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        *out
            .offset(
                i as isize,
            ) = VP8LSubPixels(*in_0.offset(i as isize), 0xff000000 as libc::c_uint);
        i += 1;
        i;
    }
}
unsafe extern "C" fn PredictorSub1_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        *out
            .offset(
                i as isize,
            ) = VP8LSubPixels(
            *in_0.offset(i as isize),
            *in_0.offset((i - 1 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn PredictorSub2_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor2_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub3_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor3_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub4_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor4_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub5_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor5_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub6_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor6_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub7_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor7_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub8_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor8_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub9_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor9_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub10_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor10_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub11_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor11_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub12_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor12_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorSub13_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor13_C(
            &*in_0.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LSubPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
#[no_mangle]
pub static mut VP8LSubtractGreenFromBlueAndRed: VP8LProcessEncBlueAndRedFunc = None;
#[no_mangle]
pub static mut VP8LTransformColor: VP8LTransformColorFunc = None;
#[no_mangle]
pub static mut VP8LCollectColorBlueTransforms: VP8LCollectColorBlueTransformsFunc = None;
#[no_mangle]
pub static mut VP8LCollectColorRedTransforms: VP8LCollectColorRedTransformsFunc = None;
#[no_mangle]
pub static mut VP8LFastLog2Slow: VP8LFastLog2SlowFunc = None;
#[no_mangle]
pub static mut VP8LFastSLog2Slow: VP8LFastLog2SlowFunc = None;
#[no_mangle]
pub static mut VP8LExtraCost: VP8LCostFunc = None;
#[no_mangle]
pub static mut VP8LExtraCostCombined: VP8LCostCombinedFunc = None;
#[no_mangle]
pub static mut VP8LCombinedShannonEntropy: VP8LCombinedShannonEntropyFunc = None;
#[no_mangle]
pub static mut VP8LGetEntropyUnrefined: VP8LGetEntropyUnrefinedFunc = None;
#[no_mangle]
pub static mut VP8LGetCombinedEntropyUnrefined: VP8LGetCombinedEntropyUnrefinedFunc = None;
#[no_mangle]
pub static mut VP8LAddVector: VP8LAddVectorFunc = None;
#[no_mangle]
pub static mut VP8LAddVectorEq: VP8LAddVectorEqFunc = None;
#[no_mangle]
pub static mut VP8LVectorMismatch: VP8LVectorMismatchFunc = None;
#[no_mangle]
pub static mut VP8LBundleColorMap: VP8LBundleColorMapFunc = None;
#[no_mangle]
pub static mut VP8LPredictorsSub: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LPredictorsSub_C: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub unsafe extern "C" fn VP8LEncDspInit() {
    static mut VP8LEncDspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    if !(pthread_mutex_lock(&mut VP8LEncDspInit_body_lock) != 0) {
        VP8LEncDspInit_body();
        pthread_mutex_unlock(&mut VP8LEncDspInit_body_lock);
    }
}
unsafe extern "C" fn VP8LEncDspInit_body() {
    VP8LDspInit();
    VP8LSubtractGreenFromBlueAndRed = Some(
        VP8LSubtractGreenFromBlueAndRed_C
            as unsafe extern "C" fn(*mut uint32_t, libc::c_int) -> (),
    );
    VP8LTransformColor = Some(
        VP8LTransformColor_C
            as unsafe extern "C" fn(
                *const VP8LMultipliers,
                *mut uint32_t,
                libc::c_int,
            ) -> (),
    );
    VP8LCollectColorBlueTransforms = Some(
        VP8LCollectColorBlueTransforms_C
            as unsafe extern "C" fn(
                *const uint32_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
            ) -> (),
    );
    VP8LCollectColorRedTransforms = Some(
        VP8LCollectColorRedTransforms_C
            as unsafe extern "C" fn(
                *const uint32_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
            ) -> (),
    );
    VP8LFastLog2Slow = Some(
        FastLog2Slow_C as unsafe extern "C" fn(uint32_t) -> libc::c_float,
    );
    VP8LFastSLog2Slow = Some(
        FastSLog2Slow_C as unsafe extern "C" fn(uint32_t) -> libc::c_float,
    );
    VP8LExtraCost = Some(
        ExtraCost_C as unsafe extern "C" fn(*const uint32_t, libc::c_int) -> uint32_t,
    );
    VP8LExtraCostCombined = Some(
        ExtraCostCombined_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
            ) -> uint32_t,
    );
    VP8LCombinedShannonEntropy = Some(
        CombinedShannonEntropy_C
            as unsafe extern "C" fn(
                *const libc::c_int,
                *const libc::c_int,
            ) -> libc::c_float,
    );
    VP8LGetEntropyUnrefined = Some(
        GetEntropyUnrefined_C
            as unsafe extern "C" fn(
                *const uint32_t,
                libc::c_int,
                *mut VP8LBitEntropy,
                *mut VP8LStreaks,
            ) -> (),
    );
    VP8LGetCombinedEntropyUnrefined = Some(
        GetCombinedEntropyUnrefined_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut VP8LBitEntropy,
                *mut VP8LStreaks,
            ) -> (),
    );
    VP8LAddVector = Some(
        AddVector_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                *mut uint32_t,
                libc::c_int,
            ) -> (),
    );
    VP8LAddVectorEq = Some(
        AddVectorEq_C
            as unsafe extern "C" fn(*const uint32_t, *mut uint32_t, libc::c_int) -> (),
    );
    VP8LVectorMismatch = Some(
        VectorMismatch_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
            ) -> libc::c_int,
    );
    VP8LBundleColorMap = Some(
        VP8LBundleColorMap_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[0 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[1 as libc::c_int
        as usize] = Some(
        PredictorSub1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[2 as libc::c_int
        as usize] = Some(
        PredictorSub2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[3 as libc::c_int
        as usize] = Some(
        PredictorSub3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[4 as libc::c_int
        as usize] = Some(
        PredictorSub4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[5 as libc::c_int
        as usize] = Some(
        PredictorSub5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[6 as libc::c_int
        as usize] = Some(
        PredictorSub6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[7 as libc::c_int
        as usize] = Some(
        PredictorSub7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[8 as libc::c_int
        as usize] = Some(
        PredictorSub8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[9 as libc::c_int
        as usize] = Some(
        PredictorSub9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[10 as libc::c_int
        as usize] = Some(
        PredictorSub10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[11 as libc::c_int
        as usize] = Some(
        PredictorSub11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[12 as libc::c_int
        as usize] = Some(
        PredictorSub12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[13 as libc::c_int
        as usize] = Some(
        PredictorSub13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[14 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub[15 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[0 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[1 as libc::c_int
        as usize] = Some(
        PredictorSub1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[2 as libc::c_int
        as usize] = Some(
        PredictorSub2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[3 as libc::c_int
        as usize] = Some(
        PredictorSub3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[4 as libc::c_int
        as usize] = Some(
        PredictorSub4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[5 as libc::c_int
        as usize] = Some(
        PredictorSub5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[6 as libc::c_int
        as usize] = Some(
        PredictorSub6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[7 as libc::c_int
        as usize] = Some(
        PredictorSub7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[8 as libc::c_int
        as usize] = Some(
        PredictorSub8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[9 as libc::c_int
        as usize] = Some(
        PredictorSub9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[10 as libc::c_int
        as usize] = Some(
        PredictorSub10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[11 as libc::c_int
        as usize] = Some(
        PredictorSub11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[12 as libc::c_int
        as usize] = Some(
        PredictorSub12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[13 as libc::c_int
        as usize] = Some(
        PredictorSub13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[14 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsSub_C[15 as libc::c_int
        as usize] = Some(
        PredictorSub0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
}
