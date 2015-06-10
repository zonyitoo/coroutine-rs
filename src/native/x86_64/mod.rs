#[cfg(target_arch = "x86_64")]
use std::simd;

use libc;

#[cfg(windows)]
macro_rules! rustrt_off {
    (rbx) => (0);
    (rsp) => (1);
    (rbp) => (2);
    // RCX on Windows, RDI else where
    (arg0) => (3);
    (r12) => (4);
    (r13) => (5);
    (r14) => (6);
    (r15) => (7);
    (ip) => (8);

    (rdi) => (9);
    (rsi) => (10);
    (st1) => (11);
    (st2) => (12);
    (xmm6) => (14);
    (xmm7) => (15);
    (xmm8) => (18);
    (xmm9) => (20);
    (xmm10) => (22);
    (xmm11) => (24);
    (xmm12) => (26);
    (xmm13) => (28);
    (xmm14) => (30);
    (xmm15) => (32);
    (max) => (34);
}

#[cfg(not(windows))]
macro_rules! rustrt_off {
    (rbx) => (0);
    (rsp) => (1);
    (rbp) => (2);
    // RCX on Windows, RDI else where
    (arg0) => (3);
    (r12) => (4);
    (r13) => (5);
    (r14) => (6);
    (r15) => (7);
    (ip) => (8);

    // Not used, just padding
    (xxx) => (9);
    (xmm0) => (10);
    (xmm1) => (12);
    (xmm2) => (14);
    (xmm3) => (16);
    (xmm4) => (18);
    (xmm5) => (20);
    (max) => (22);
}

#[cfg(windows)]
macro_rules! rustrt_arg {
    (0) => ("%rcx");
    (1) => ("%rdx");
    (2) => ("%r8");
    (3) => ("%r9");
}

#[cfg(not(windows))]
macro_rules! rustrt_arg {
    (0) => ("%rdi");
    (1) => ("%rsi");
    (2) => ("%rdx");
    (3) => ("%rcx");
    (4) => ("%r8");
    (5) => ("%r9");
}

// windows requires saving more registers (both general and XMM), so the windows
// register context must be larger.
#[cfg(all(windows, target_arch = "x86_64"))]
#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    gpr: [libc::uintptr_t; 14],
    _xmm: [simd::u32x4; 10]
}

#[cfg(all(windows, target_arch = "x86_64"))]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            gpr: [0; 14],
            _xmm: [simd::u32x4(0,0,0,0); 10]
        }
    }
}

#[cfg(all(not(windows), target_arch = "x86_64"))]
#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    pub gpr: [libc::uintptr_t; 10],
    pub _xmm: [simd::u32x4; 6]
}

#[cfg(all(not(windows), target_arch = "x86_64"))]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            gpr: [0; 10],
            _xmm: [simd::u32x4(0,0,0,0); 6]
        }
    }
}

macro_rules! asm_raw {
    ( $($x:expr),+ ) => {
        asm!(concat!($($x),*) ::)
    };
}

#[cfg(not(windows))]
pub unsafe fn swap_registers(_out_regs: *mut Registers, _in_regs: *const Registers) -> ! {
    // Save instruction pointer
    asm_raw!("pop %rax");
    asm_raw!("mov %rax, (", rustrt_off!(ip), "*8)(", rustrt_arg!(0), ")");

    // Save non-volatile integer registers
    // (including RSP)
    asm_raw!("mov %rbx, (", rustrt_off!(rbx), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %rsp, (", rustrt_off!(rsp), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %rbp, (", rustrt_off!(rbp), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %r12, (", rustrt_off!(r12), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %r13, (", rustrt_off!(r13), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %r14, (", rustrt_off!(r14), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("mov %r15, (", rustrt_off!(r15), "*8)(", rustrt_arg!(0), ")");

    // Save 0th argument register
    asm_raw!("mov ", rustrt_arg!(0), ", (", rustrt_off!(arg0), "*8)(", rustrt_arg!(0), ")");

    // Save non-volatile XMM registers
    asm_raw!("movapd %xmm0, (", rustrt_off!(xmm0), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("movapd %xmm1, (", rustrt_off!(xmm1), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("movapd %xmm2, (", rustrt_off!(xmm2), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("movapd %xmm3, (", rustrt_off!(xmm3), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("movapd %xmm4, (", rustrt_off!(xmm4), "*8)(", rustrt_arg!(0), ")");
    asm_raw!("movapd %xmm5, (", rustrt_off!(xmm5), "*8)(", rustrt_arg!(0), ")");

    // Restore non-volatile integer registers
    asm_raw!("mov (", rustrt_off!(rbx), "*8)(", rustrt_arg!(1), "), %rbx");
    asm_raw!("mov (", rustrt_off!(rsp), "*8)(", rustrt_arg!(1), "), %rsp");
    asm_raw!("mov (", rustrt_off!(rbp), "*8)(", rustrt_arg!(1), "), %rbp");
    asm_raw!("mov (", rustrt_off!(r12), "*8)(", rustrt_arg!(1), "), %r12");
    asm_raw!("mov (", rustrt_off!(r13), "*8)(", rustrt_arg!(1), "), %r13");
    asm_raw!("mov (", rustrt_off!(r14), "*8)(", rustrt_arg!(1), "), %r14");
    asm_raw!("mov (", rustrt_off!(r15), "*8)(", rustrt_arg!(1), "), %r15");

    // Restore 0th argument register
    asm_raw!("mov (", rustrt_off!(arg0), "*8)(", rustrt_arg!(1), "), ", rustrt_arg!(0));

    // Restore non-volatile XMM registers
    asm_raw!("movapd (", rustrt_off!(xmm0), "*8)(", rustrt_arg!(1), "), %xmm0");
    asm_raw!("movapd (", rustrt_off!(xmm1), "*8)(", rustrt_arg!(1), "), %xmm1");
    asm_raw!("movapd (", rustrt_off!(xmm2), "*8)(", rustrt_arg!(1), "), %xmm2");
    asm_raw!("movapd (", rustrt_off!(xmm3), "*8)(", rustrt_arg!(1), "), %xmm3");
    asm_raw!("movapd (", rustrt_off!(xmm4), "*8)(", rustrt_arg!(1), "), %xmm4");
    asm_raw!("movapd (", rustrt_off!(xmm5), "*8)(", rustrt_arg!(1), "), %xmm5");

    // Jump to the instruction pointer
    // found in regs:
    asm_raw!("jmp *(", rustrt_off!(ip), "*8)(", rustrt_arg!(1), ")");

    unreachable!();
}

// #[cfg(not(windows))]
// pub unsafe fn swap_registers(_out_regs: *mut Registers, _in_regs: *const Registers) -> ! {
//     // Save instruction pointer
//     asm_raw!("pop %rax");
//     asm_raw!("mov %rax, (", rustrt_off!(ip), "*8)(", rustrt_arg!(0), ")");

//     // Save non-volatile integer registers
//     // (including RSP)
//     asm_raw!("mov %rbx, (", rustrt_off!(rbx), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %rsp, (", rustrt_off!(rsp), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %rbp, (", rustrt_off!(rbp), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %r12, (", rustrt_off!(r12), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %r13, (", rustrt_off!(r13), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %r14, (", rustrt_off!(r14), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("mov %r15, (", rustrt_off!(r15), "*8)(", rustrt_arg!(0), ")");

//     // Save 0th argument register
//     asm_raw!("mov ", rustrt_arg!(0), ", (", rustrt_off!(arg0), "*8)(", rustrt_arg!(0), ")");

//     // Save non-volatile XMM registers
//     asm_raw!("movapd %xmm0, (", rustrt_off!(xmm0), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("movapd %xmm1, (", rustrt_off!(xmm1), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("movapd %xmm2, (", rustrt_off!(xmm2), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("movapd %xmm3, (", rustrt_off!(xmm3), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("movapd %xmm4, (", rustrt_off!(xmm4), "*8)(", rustrt_arg!(0), ")");
//     asm_raw!("movapd %xmm5, (", rustrt_off!(xmm5), "*8)(", rustrt_arg!(0), ")");

//     // Restore non-volatile integer registers
//     asm_raw!("mov (", rustrt_off!(rbx), "*8)(", rustrt_arg!(1), "), %rbx");
//     asm_raw!("mov (", rustrt_off!(rsp), "*8)(", rustrt_arg!(1), "), %rsp");
//     asm_raw!("mov (", rustrt_off!(rbp), "*8)(", rustrt_arg!(1), "), %rbp");
//     asm_raw!("mov (", rustrt_off!(r12), "*8)(", rustrt_arg!(1), "), %r12");
//     asm_raw!("mov (", rustrt_off!(r13), "*8)(", rustrt_arg!(1), "), %r13");
//     asm_raw!("mov (", rustrt_off!(r14), "*8)(", rustrt_arg!(1), "), %r14");
//     asm_raw!("mov (", rustrt_off!(r15), "*8)(", rustrt_arg!(1), "), %r15");

//     // Restore 0th argument register
//     asm_raw!("mov (", rustrt_off!(arg0), "*8)(", rustrt_arg!(1), "), ", rustrt_arg!(0));

//     // Restore non-volatile XMM registers
//     asm_raw!("movapd (", rustrt_off!(xmm0), "*8)(", rustrt_arg!(1), "), %xmm0");
//     asm_raw!("movapd (", rustrt_off!(xmm1), "*8)(", rustrt_arg!(1), "), %xmm1");
//     asm_raw!("movapd (", rustrt_off!(xmm2), "*8)(", rustrt_arg!(1), "), %xmm2");
//     asm_raw!("movapd (", rustrt_off!(xmm3), "*8)(", rustrt_arg!(1), "), %xmm3");
//     asm_raw!("movapd (", rustrt_off!(xmm4), "*8)(", rustrt_arg!(1), "), %xmm4");
//     asm_raw!("movapd (", rustrt_off!(xmm5), "*8)(", rustrt_arg!(1), "), %xmm5");

//     // Jump to the instruction pointer
//     // found in regs:
//     asm_raw!("jmp *(", rustrt_off!(ip), "*8)(", rustrt_arg!(1), ")");
// }
