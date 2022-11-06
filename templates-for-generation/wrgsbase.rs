unsafe {
    std::arch::asm!(
        "wrgsbase {lin_mem_base}",
        lin_mem_base = in(reg) wasm_module.get_memory(),
        options(nomem),
    );
}