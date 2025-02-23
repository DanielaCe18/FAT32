use core::arch::asm;

/// Save the current context and switch to the next one.
pub unsafe fn switch_context(current: *mut usize, next: *const usize) {
    asm!(
        "mov [{}], rsp",   // Save current stack pointer
        "mov rsp, [{}]",   // Load new stack pointer
        in(reg) current,
        in(reg) next,
        options(nostack)
    );
}

