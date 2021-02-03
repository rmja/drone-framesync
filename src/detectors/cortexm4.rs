use crate::comparators::{Exact16Comparator, Exact32Comparator, PopCount32Comparator, TwosComplement16Comparator, TwosComplement32Comparator};

use super::{Detector, Single16Detector, Single32Detector};

pub struct CortexmDetector<const TOL: usize> {
}

// Output from benchmark
// single16;exact;2288
// single16;lzc1;5581
// single16;lzc2;9308
// single16;lzc3;12188
// single16;lzc4;15068
// single16;popcnt1;3874
// single16;popcnt2;7726
// single16;popcnt3;7722
// single16;popcnt4;7727
// single16;twoscmpl1;3398
// single16;twoscmpl2;4412
// single16;twoscmpl3;5609
// single16;twoscmpl4;7718
// double16;exact;3696
// double16;lzc1;10874
// double16;lzc2;16145
// double16;lzc3;22358
// double16;lzc4;27188
// double16;popcnt1;6010
// double16;popcnt2;14566
// double16;popcnt3;14566
// double16;popcnt4;14566
// double16;twoscmpl1;6995
// double16;twoscmpl2;9398
// double16;twoscmpl3;10865
// double16;twoscmpl4;13715
// single32;exact;4204
// single32;lzc1;8705
// single32;lzc2;13985
// single32;lzc3;18548
// single32;lzc4;23105
// single32;lzc5;27665
// single32;lzc6;30785
// single32;popcnt1;6548
// single32;popcnt2;15314
// single32;popcnt3;15404
// single32;popcnt4;15314
// single32;popcnt5;15404
// single32;popcnt6;15315
// single32;twoscmpl1;6553
// single32;twoscmpl2;7627
// single32;twoscmpl3;9670
// single32;twoscmpl4;12787
// single32;twoscmpl5;14947
// single32;twoscmpl6;17348
// double32;exact;12040
// double32;lzc1;20678
// double32;lzc2;29346
// double32;lzc3;37988
// double32;lzc4;46600
// double32;lzc5;55238
// double32;lzc6;63906
// double32;popcnt1;15908
// double32;popcnt2;33252
// double32;popcnt3;33252
// double32;popcnt4;33250
// double32;popcnt5;33252
// double32;popcnt6;33250
// double32;twoscmpl1;16814
// double32;twoscmpl2;18728
// double32;twoscmpl3;23526
// double32;twoscmpl4;26466
// double32;twoscmpl5;32168
// double32;twoscmpl6;35106

macro_rules! impl_sync {
    ($name:ident<$type:ty>, $detector:ident, $comparator:ty) => {
        pub const fn $name<const SW: $type>() -> impl Detector<$type> {
            $detector::<$comparator>::new()
        }
    };
}

impl_sync!(sync16_tol0<u16>, Single16Detector, Exact16Comparator::<SW>);
impl_sync!(sync16_tol1<u16>, Single16Detector, TwosComplement16Comparator::<SW, 1>);
impl_sync!(sync16_tol2<u16>, Single16Detector, TwosComplement16Comparator::<SW, 2>);
impl_sync!(sync16_tol3<u16>, Single16Detector, TwosComplement16Comparator::<SW, 3>);
impl_sync!(sync16_tol4<u16>, Single16Detector, TwosComplement16Comparator::<SW, 4>);

impl_sync!(sync32_tol0<u32>, Single32Detector, Exact32Comparator::<SW>);
impl_sync!(sync32_tol1<u32>, Single32Detector, TwosComplement32Comparator::<SW, 1>);
impl_sync!(sync32_tol2<u32>, Single32Detector, TwosComplement32Comparator::<SW, 2>);
impl_sync!(sync32_tol3<u32>, Single32Detector, TwosComplement32Comparator::<SW, 3>);
impl_sync!(sync32_tol4<u32>, Single32Detector, TwosComplement32Comparator::<SW, 4>);
impl_sync!(sync32_tol5<u32>, Single32Detector, TwosComplement32Comparator::<SW, 5>);
impl_sync!(sync32_tol6<u32>, Single32Detector, PopCount32Comparator::<SW, 6>);
