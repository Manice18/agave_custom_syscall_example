use {super::*, solana_sbpf::vm::ContextObject};

declare_builtin_function!(
    /// Log a user's info message
    SyscallLog,
    fn rust(
        invoke_context: &mut InvokeContext,
        addr: u64,
        len: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let cost = invoke_context
            .get_execution_cost()
            .syscall_base_cost
            .max(len);
        consume_compute_meter(invoke_context, cost)?;

        translate_string_and_do(
            memory_mapping,
            addr,
            len,
            invoke_context.get_check_aligned(),
            &mut |string: &str| {
                stable_log::program_log(&invoke_context.get_log_collector(), string);
                Ok(0)
            },
        )?;
        Ok(0)
    }
);

declare_builtin_function!(
    /// Log 5 64-bit values
    SyscallLogU64,
    fn rust(
        invoke_context: &mut InvokeContext,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        _memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let cost = invoke_context.get_execution_cost().log_64_units;
        consume_compute_meter(invoke_context, cost)?;

        stable_log::program_log(
            &invoke_context.get_log_collector(),
            &format!("{arg1:#x}, {arg2:#x}, {arg3:#x}, {arg4:#x}, {arg5:#x}"),
        );
        Ok(0)
    }
);

declare_builtin_function!(
    /// Log current compute consumption
    SyscallLogBpfComputeUnits,
    fn rust(
        invoke_context: &mut InvokeContext,
        _arg1: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        _memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let cost = invoke_context.get_execution_cost().syscall_base_cost;
        consume_compute_meter(invoke_context, cost)?;

        ic_logger_msg!(
            invoke_context.get_log_collector(),
            "Program consumption: {} units remaining",
            invoke_context.get_remaining(),
        );
        Ok(0)
    }
);

declare_builtin_function!(
    /// Log a [`Pubkey`] as a base58 string
    SyscallLogPubkey,
    fn rust(
        invoke_context: &mut InvokeContext,
        pubkey_addr: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let cost = invoke_context.get_execution_cost().log_pubkey_units;
        consume_compute_meter(invoke_context, cost)?;

        let pubkey = translate_type::<Pubkey>(
            memory_mapping,
            pubkey_addr,
            invoke_context.get_check_aligned(),
        )?;
        stable_log::program_log(&invoke_context.get_log_collector(), &pubkey.to_string());
        Ok(0)
    }
);

declare_builtin_function!(
    /// Log data handling
    SyscallLogData,
    fn rust(
        invoke_context: &mut InvokeContext,
        addr: u64,
        len: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let execution_cost = invoke_context.get_execution_cost();

        consume_compute_meter(invoke_context, execution_cost.syscall_base_cost)?;

        let untranslated_fields = translate_slice::<VmSlice<u8>>(
            memory_mapping,
            addr,
            len,
            invoke_context.get_check_aligned(),
        )?;

        consume_compute_meter(
            invoke_context,
            execution_cost
                .syscall_base_cost
                .saturating_mul(untranslated_fields.len() as u64),
        )?;
        consume_compute_meter(
            invoke_context,
            untranslated_fields
                .iter()
                .fold(0, |total, e| total.saturating_add(e.len())),
        )?;

        let mut fields = Vec::with_capacity(untranslated_fields.len());

        for untranslated_field in untranslated_fields {
            fields.push(untranslated_field.translate(memory_mapping, invoke_context.get_check_aligned())?);
        }

        let log_collector = invoke_context.get_log_collector();

        stable_log::program_data(&log_collector, &fields);

        Ok(0)
    }
);

// Custom Syscall
declare_builtin_function!(
    /// Log a [`Pubkey`] as a Unit8 array
    SyscallLogPubkeyAsUnit8,
    fn rust(
        invoke_context: &mut InvokeContext,
        pubkey_addr: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        let cost = invoke_context.get_execution_cost().log_pubkey_units;
        consume_compute_meter(invoke_context, cost)?;

        let pubkey = translate_type::<Pubkey>(
            memory_mapping,
            pubkey_addr,
            invoke_context.get_check_aligned(),
        )?;
        let bytes = pubkey.to_bytes();
        let byte_str = bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ");
        let log_msg = format!("[{}]", byte_str);
        stable_log::program_log(&invoke_context.get_log_collector(), &log_msg);
        Ok(0)
    }
);
