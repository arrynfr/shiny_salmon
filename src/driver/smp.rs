use core::arch::asm;
const PSCI_0_2_FN64_CPU_ON: isize = 0xc4000003;

pub unsafe fn init_smp(start_addr: *const fn()) {
    let current_core: u8;
    asm!("mrs x2, MPIDR_EL1", out("x2") current_core);
    println!("Starting kernel at: {:x}", start_addr);
    for init_core in 1..crate::NUM_CORES {
        // x0 is PSCI command as input and return code as output
        let return_code: isize;
        asm!("hvc 0",
        inout("x0") PSCI_0_2_FN64_CPU_ON => return_code,
        in("x1") init_core,
        in("x2") start_addr as usize,
        in("x3") init_core,
        options(nostack, nomem));
        if return_code != 0 { kprintln!("Failed to initialize core {init_core}: {return_code}"); }
    }
}
