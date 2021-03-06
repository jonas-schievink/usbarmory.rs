//! Low level access to Cortex-A processors

#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod register;

/// "No OPeration" instruction
///
/// Use this in "for loop" delays or the compiler will optimize away your delay
pub fn nop() {
    extern "C" {
        fn __nop();
    }

    unsafe { __nop() }
}

/// Performs `n` CPU instructions to add a delay to the program
///
/// Note that this function may result in a delay of *more* than `n` CPU clock
/// cycles due to time spent in higher priority interrupt handlers
pub fn delay(n: u32) {
    extern "C" {
        fn __delay(n: u32);

    }

    unsafe { __delay(n) }
}

/// Enables IRQ interrupts
///
/// # Safety
///
/// This operation can break critical sections based on masking IRQ
pub unsafe fn enable_irq() {
    extern "C" {
        fn __enable_irq();
    }

    __enable_irq()
}

/// Disable IRQ interrupts
pub fn disable_irq() {
    extern "C" {
        fn __disable_irq();
    }

    unsafe { __disable_irq() }
}

/// Data Memory Barrier
pub fn dmb() {
    extern "C" {
        fn __dmb();
    }

    unsafe { __dmb() }
}

/// Data Synchronization Barrier
pub fn dsb() {
    extern "C" {
        fn __dsb();
    }

    unsafe { __dsb() }
}

/// Instruction Synchronization Barrier
pub fn isb() {
    extern "C" {
        fn __isb();
    }

    unsafe { __isb() }
}

/// Wait For Interrupt
pub fn wfi() {
    extern "C" {
        fn __wfi();
    }

    unsafe { __wfi() }
}

/// UnDeFined instruction
///
/// Calling this will result in a `UndefinedInstrution` exception
pub fn udf() -> ! {
    extern "C" {
        fn __udf() -> !;
    }

    unsafe { __udf() }
}

/// Runs the given closure in a critical section
///
/// The closure code won't be preempted by interrupts
pub fn no_interrupts<T>(f: impl FnOnce() -> T) -> T {
    // IRQ mask bit
    const I: u32 = 1 << 7;

    let cpsr = register::cpsr::read();
    disable_irq();
    let r = f();
    if cpsr & I == 0 {
        // re-enable interrupts
        unsafe { enable_irq() }
    }
    r
}
