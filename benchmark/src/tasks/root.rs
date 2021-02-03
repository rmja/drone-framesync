//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use cortexm4::sync16_tol0;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_framesync::{comparators::*, detectors::*, detectors::cortexm4};
use drone_stm32f4_hal::dwt::Stopwatch;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");

    // Make sure to perform benchmarks using a _release_ build.

    run_test("best16;exact", cortexm4::sync16_tol0::<0xFFFF>());
    run_test("best16;tol1", cortexm4::sync16_tol1::<0xFFFF>());
    run_test("best16;tol2", cortexm4::sync16_tol2::<0xFFFF>());
    run_test("best16;tol3", cortexm4::sync16_tol3::<0xFFFF>());
    run_test("best16;tol4", cortexm4::sync16_tol4::<0xFFFF>());

    run_test("single16;exact", Single16Detector::<Exact16Comparator::<0xFFFF>>::new());
    run_test("single16;lzc1", Single16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 1>>::new());
    run_test("single16;lzc2", Single16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 2>>::new());
    run_test("single16;lzc3", Single16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 3>>::new());
    run_test("single16;lzc4", Single16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 4>>::new());
    run_test("single16;popcnt1", Single16Detector::<PopCount16Comparator::<0xFFFF, 1>>::new());
    run_test("single16;popcnt2", Single16Detector::<PopCount16Comparator::<0xFFFF, 2>>::new());
    run_test("single16;popcnt3", Single16Detector::<PopCount16Comparator::<0xFFFF, 3>>::new());
    run_test("single16;popcnt4", Single16Detector::<PopCount16Comparator::<0xFFFF, 4>>::new());
    run_test("single16;twoscmpl1", Single16Detector::<TwosComplement16Comparator::<0xFFFF, 1>>::new());
    run_test("single16;twoscmpl2", Single16Detector::<TwosComplement16Comparator::<0xFFFF, 2>>::new());
    run_test("single16;twoscmpl3", Single16Detector::<TwosComplement16Comparator::<0xFFFF, 3>>::new());
    run_test("single16;twoscmpl4", Single16Detector::<TwosComplement16Comparator::<0xFFFF, 4>>::new());

    run_test("double16;exact", Double16Detector::<Exact16Comparator::<0xFFFF>>::new());
    run_test("double16;lzc1", Double16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 1>>::new());
    run_test("double16;lzc2", Double16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 2>>::new());
    run_test("double16;lzc3", Double16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 3>>::new());
    run_test("double16;lzc4", Double16Detector::<LeadingZeroCount16Comparator::<0xFFFF, 4>>::new());
    run_test("double16;popcnt1", Double16Detector::<PopCount16Comparator::<0xFFFF, 1>>::new());
    run_test("double16;popcnt2", Double16Detector::<PopCount16Comparator::<0xFFFF, 2>>::new());
    run_test("double16;popcnt3", Double16Detector::<PopCount16Comparator::<0xFFFF, 3>>::new());
    run_test("double16;popcnt4", Double16Detector::<PopCount16Comparator::<0xFFFF, 4>>::new());
    run_test("double16;twoscmpl1", Double16Detector::<TwosComplement16Comparator::<0xFFFF, 1>>::new());
    run_test("double16;twoscmpl2", Double16Detector::<TwosComplement16Comparator::<0xFFFF, 2>>::new());
    run_test("double16;twoscmpl3", Double16Detector::<TwosComplement16Comparator::<0xFFFF, 3>>::new());
    run_test("double16;twoscmpl4", Double16Detector::<TwosComplement16Comparator::<0xFFFF, 4>>::new());

    run_test("best32;exact", cortexm4::sync32_tol0::<0xFFFFFFFF>());
    run_test("best32;tol1", cortexm4::sync32_tol1::<0xFFFFFFFF>());
    run_test("best32;tol2", cortexm4::sync32_tol2::<0xFFFFFFFF>());
    run_test("best32;tol3", cortexm4::sync32_tol3::<0xFFFFFFFF>());
    run_test("best32;tol4", cortexm4::sync32_tol4::<0xFFFFFFFF>());
    run_test("best32;tol5", cortexm4::sync32_tol5::<0xFFFFFFFF>());
    run_test("best32;tol6", cortexm4::sync32_tol6::<0xFFFFFFFF>());

    run_test("single32;exact", Single32Detector::<Exact32Comparator::<0xFFFFFFFF>>::new());
    run_test("single32;lzc1", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("single32;lzc2", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("single32;lzc3", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("single32;lzc4", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("single32;lzc5", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("single32;lzc6", Single32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 6>>::new());
    run_test("single32;popcnt1", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("single32;popcnt2", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("single32;popcnt3", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("single32;popcnt4", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("single32;popcnt5", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("single32;popcnt6", Single32Detector::<PopCount32Comparator::<0xFFFFFFFF, 6>>::new());
    run_test("single32;twoscmpl1", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("single32;twoscmpl2", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("single32;twoscmpl3", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("single32;twoscmpl4", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("single32;twoscmpl5", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("single32;twoscmpl6", Single32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 6>>::new());

    run_test("double32;exact", Double32Detector::<Exact32Comparator::<0xFFFFFFFF>>::new());
    run_test("double32;lzc1", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("double32;lzc2", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("double32;lzc3", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("double32;lzc4", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("double32;lzc5", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("double32;lzc6", Double32Detector::<LeadingZeroCount32Comparator::<0xFFFFFFFF, 6>>::new());
    run_test("double32;popcnt1", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("double32;popcnt2", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("double32;popcnt3", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("double32;popcnt4", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("double32;popcnt5", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("double32;popcnt6", Double32Detector::<PopCount32Comparator::<0xFFFFFFFF, 6>>::new());
    run_test("double32;twoscmpl1", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 1>>::new());
    run_test("double32;twoscmpl2", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 2>>::new());
    run_test("double32;twoscmpl3", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 3>>::new());
    run_test("double32;twoscmpl4", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 4>>::new());
    run_test("double32;twoscmpl5", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 5>>::new());
    run_test("double32;twoscmpl6", Double32Detector::<TwosComplement32Comparator::<0xFFFFFFFF, 6>>::new());

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}

fn run_test<D: Detector<T>, T>(name: &str, detector: D) {
    const HAYSTACK_SIZE: usize = 16;
    const TRIALS: usize = 200;

    let haystack = [0u8; HAYSTACK_SIZE];
    
    let mut sw = Stopwatch::start_new();
    for _ in 0..TRIALS {
        detector.position(&haystack);
    }
    sw.stop();
    println!("{};{}", name, sw.elapsed());
}