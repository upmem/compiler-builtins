use core::intrinsics;

intrinsics! {
    pub extern "C" fn __mulsi3(a: u32, b: u32) -> u32 {
        let c: u32;
        unsafe {
            asm!("  jgtu $2, $1, __mulsi3_swap
                    move r2, $1
                    move r0, $2, true, __mulsi3_start
                  __mulsi3_swap:
                    move r2, $2
                    move r0, $1
                  __mulsi3_start:
                    move r1, zero
                    mul_step d0, r2, d0, 0 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 1 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 2 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 3 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 4 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 5 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 6 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 7 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 8 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 9 , z, __mulsi3_exit
                    mul_step d0, r2, d0, 10, z, __mulsi3_exit
                    mul_step d0, r2, d0, 11, z, __mulsi3_exit
                    mul_step d0, r2, d0, 12, z, __mulsi3_exit
                    mul_step d0, r2, d0, 13, z, __mulsi3_exit
                    mul_step d0, r2, d0, 14, z, __mulsi3_exit
                    mul_step d0, r2, d0, 15, z, __mulsi3_exit
                    mul_step d0, r2, d0, 16, z, __mulsi3_exit
                    mul_step d0, r2, d0, 17, z, __mulsi3_exit
                    mul_step d0, r2, d0, 18, z, __mulsi3_exit
                    mul_step d0, r2, d0, 19, z, __mulsi3_exit
                    mul_step d0, r2, d0, 20, z, __mulsi3_exit
                    mul_step d0, r2, d0, 21, z, __mulsi3_exit
                    mul_step d0, r2, d0, 22, z, __mulsi3_exit
                    mul_step d0, r2, d0, 23, z, __mulsi3_exit
                    mul_step d0, r2, d0, 24, z, __mulsi3_exit
                    mul_step d0, r2, d0, 25, z, __mulsi3_exit
                    mul_step d0, r2, d0, 26, z, __mulsi3_exit
                    mul_step d0, r2, d0, 27, z, __mulsi3_exit
                    mul_step d0, r2, d0, 28, z, __mulsi3_exit
                    mul_step d0, r2, d0, 29, z, __mulsi3_exit
                    mul_step d0, r2, d0, 30, z, __mulsi3_exit
                    mul_step d0, r2, d0, 31, z, __mulsi3_exit
                  __mulsi3_exit:
                    move $0, r1" : "=r"(c) : "r"(a), "r"(b));
        }
        c
    }
}

#[naked]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe fn __udiv32() {
    asm!("  clz r3, r1, max, __udiv32_division_by_zero
            clz r4, r0
            sub r3, r4, r3, gtu, __udiv32_result_0
            move r4, r1
            move.u d0, r0
            jump r3, __udiv32_base
            div_step d0, r4, d0, 3
            div_step d0, r4, d0, 3
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 1
            div_step d0, r4, d0, 9
            div_step d0, r4, d0, 8
            div_step d0, r4, d0, 7
            div_step d0, r4, d0, 6
            div_step d0, r4, d0, 5
            div_step d0, r4, d0, 4
            div_step d0, r4, d0, 3
            div_step d0, r4, d0, 2
            div_step d0, r4, d0, 1
          __udiv32_base:
            div_step d0, r4, d0, 0
            __udiv32_exit:
            jump r23
          __udiv32_result_0:
            move.u d0, r0, true, __udiv32_exit
          __udiv32_division_by_zero:
            fault 2" :::: "volatile");
    intrinsics::unreachable();
}

#[naked]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe fn __div32() {
    asm!("  sd r22, 0, d22
            add r22, r22, 8
            clo r3, r0, z, __div32_pos_dividend
            clo r3, r1, z, __div32_neg_dividend_pos_divider
          __div32_neg_dividend_neg_divider:
            neg r0, r0
            neg r1, r1
            call r23, __udiv32
            neg r1, r1, true, __div32_exit
          __div32_neg_dividend_pos_divider:
            neg r0, r0
            call r23, __udiv32
            neg r1, r1
            neg r0, r0, true, __div32_exit
          __div32_pos_dividend:
            clo r3, r1, z, __div32_pos_dividend_pos_divider
            neg r1, r1
            call r23, __udiv32
            neg r0, r0, true, __div32_exit
          __div32_pos_dividend_pos_divider:
            call r23, __udiv32
          __div32_exit:
            ld d22, r22, -8
            jump r23" ::: "memory" : "volatile");
    intrinsics::unreachable();
}

#[link_name = ".atomic"]
static LLVM_ATOMIC_LOCK: u8 = 0;

macro_rules! atomic_cmpxchg {
    ($name:ident, $ty:ty) => {
        #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
        pub unsafe extern "C" fn $name(ptr: *mut $ty, oldval: $ty, newval: $ty) -> $ty {
            asm!("acquire zero, $0, nz, ." :: "i" (&LLVM_ATOMIC_LOCK) :: "volatile");
            let curval = *ptr;
            if curval == oldval {
                *ptr = newval;
            }
            asm!("release zero, $0, nz, . + 1" :: "i" (&LLVM_ATOMIC_LOCK) :: "volatile");
            return curval;
        }
    };
}

macro_rules! atomic_rmw {
    ($name:ident, $ty:ty, $op:expr) => {
        #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
        pub unsafe extern "C" fn $name(ptr: *mut $ty, val: $ty) -> $ty {
            asm!("acquire zero, $0, nz, ." :: "i" (&LLVM_ATOMIC_LOCK) :: "volatile");
            let curval = *ptr;
            *ptr = $op(curval, val);
            asm!("release zero, $0, nz, . + 1" :: "i" (&LLVM_ATOMIC_LOCK) :: "volatile");
            return curval; 
        }
    };
}

atomic_cmpxchg!(__dpu_sync_val_compare_and_swap_1, u8);
atomic_cmpxchg!(__dpu_sync_val_compare_and_swap_2, u16);
atomic_cmpxchg!(__dpu_sync_val_compare_and_swap_4, u32);
atomic_cmpxchg!(__dpu_sync_val_compare_and_swap_8, u64);

atomic_rmw!(__dpu_sync_fetch_and_add_1, u8, |a: u8, b: u8| a.wrapping_add(b));
atomic_rmw!(__dpu_sync_fetch_and_add_2, u16, |a: u16, b: u16| a
    .wrapping_add(b));
atomic_rmw!(__dpu_sync_fetch_and_add_4, u32, |a: u32, b: u32| a
    .wrapping_add(b));
atomic_rmw!(__dpu_sync_fetch_and_add_8, u64, |a: u64, b: u64| a
    .wrapping_add(b));

atomic_rmw!(__dpu_sync_fetch_and_sub_1, u8, |a: u8, b: u8| a.wrapping_sub(b));
atomic_rmw!(__dpu_sync_fetch_and_sub_2, u16, |a: u16, b: u16| a
    .wrapping_sub(b));
atomic_rmw!(__dpu_sync_fetch_and_sub_4, u32, |a: u32, b: u32| a
    .wrapping_sub(b));
atomic_rmw!(__dpu_sync_fetch_and_sub_8, u64, |a: u64, b: u64| a
    .wrapping_sub(b));

atomic_rmw!(__dpu_sync_fetch_and_and_1, u8, |a: u8, b: u8| a & b);
atomic_rmw!(__dpu_sync_fetch_and_and_2, u16, |a: u16, b: u16| a & b);
atomic_rmw!(__dpu_sync_fetch_and_and_4, u32, |a: u32, b: u32| a & b);
atomic_rmw!(__dpu_sync_fetch_and_and_8, u64, |a: u64, b: u64| a & b);

atomic_rmw!(__dpu_sync_fetch_and_or_1, u8, |a: u8, b: u8| a | b);
atomic_rmw!(__dpu_sync_fetch_and_or_2, u16, |a: u16, b: u16| a | b);
atomic_rmw!(__dpu_sync_fetch_and_or_4, u32, |a: u32, b: u32| a | b);
atomic_rmw!(__dpu_sync_fetch_and_or_8, u64, |a: u64, b: u64| a | b);

atomic_rmw!(__dpu_sync_fetch_and_xor_1, u8, |a: u8, b: u8| a ^ b);
atomic_rmw!(__dpu_sync_fetch_and_xor_2, u16, |a: u16, b: u16| a ^ b);
atomic_rmw!(__dpu_sync_fetch_and_xor_4, u32, |a: u32, b: u32| a ^ b);
atomic_rmw!(__dpu_sync_fetch_and_xor_8, u64, |a: u64, b: u64| a ^ b);

atomic_rmw!(__dpu_sync_fetch_and_nand_1, u8, |a: u8, b: u8| !(a & b));
atomic_rmw!(__dpu_sync_fetch_and_nand_2, u16, |a: u16, b: u16| !(a & b));
atomic_rmw!(__dpu_sync_fetch_and_nand_4, u32, |a: u32, b: u32| !(a & b));
atomic_rmw!(__dpu_sync_fetch_and_nand_8, u64, |a: u64, b: u64| !(a & b));

atomic_rmw!(__dpu_sync_fetch_and_max_1, i8, |a: i8, b: i8| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_max_2, i16, |a: i16, b: i16| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_max_4, i32, |a: i32, b: i32| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_max_8, i64, |a: i64, b: i64| if a > b {
    a
} else {
    b
});

atomic_rmw!(__dpu_sync_fetch_and_umax_1, u8, |a: u8, b: u8| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umax_2, u16, |a: u16, b: u16| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umax_4, u32, |a: u32, b: u32| if a > b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umax_8, u64, |a: u64, b: u64| if a > b {
    a
} else {
    b
});

atomic_rmw!(__dpu_sync_fetch_and_min_1, i8, |a: i8, b: i8| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_min_2, i16, |a: i16, b: i16| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_min_4, i32, |a: i32, b: i32| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_min_8, i64, |a: i64, b: i64| if a < b {
    a
} else {
    b
});

atomic_rmw!(__dpu_sync_fetch_and_umin_1, u8, |a: u8, b: u8| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umin_2, u16, |a: u16, b: u16| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umin_4, u32, |a: u32, b: u32| if a < b {
    a
} else {
    b
});
atomic_rmw!(__dpu_sync_fetch_and_umin_8, u64, |a: u64, b: u64| if a < b {
    a
} else {
    b
});

atomic_rmw!(__dpu_sync_lock_test_and_set_1, u8, |_: u8, b: u8| b);
atomic_rmw!(__dpu_sync_lock_test_and_set_2, u16, |_: u16, b: u16| b);
atomic_rmw!(__dpu_sync_lock_test_and_set_4, u32, |_: u32, b: u32| b);
atomic_rmw!(__dpu_sync_lock_test_and_set_8, u64, |_: u64, b: u64| b);

