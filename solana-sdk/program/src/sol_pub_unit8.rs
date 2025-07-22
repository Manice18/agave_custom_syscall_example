use solana_pubkey::Pubkey;

pub fn sol_log_pubkey_as_unit8(pubkey: &Pubkey) {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_log_pubkey_as_unit8(pubkey.as_ref() as *const _ as *const u8)
    };

    #[cfg(not(target_os = "solana"))]
    crate::program_stubs::sol_log_pubkey_as_unit8(pubkey);
}
