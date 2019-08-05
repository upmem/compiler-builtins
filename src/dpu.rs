use core::intrinsics;

#[naked]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe fn __mul32() {
    // todo(UPMEM)
    asm!("fault 66" :::: "volatile");
    intrinsics::unreachable();
}

#[naked]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe fn __udiv32() {
    // todo(UPMEM)
    asm!("fault 67" :::: "volatile");
    intrinsics::unreachable();
}

#[naked]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe fn __sdiv32() {
    // todo(UPMEM)
    asm!("fault 68" :::: "volatile");
    intrinsics::unreachable();
}

