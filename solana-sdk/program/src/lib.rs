//! The base library for all Solana on-chain Rust programs.
//!
//! All Solana Rust programs that run on-chain will link to this crate, which
//! acts as a standard library for Solana programs. Solana programs also link to
//! the [Rust standard library][std], though it is [modified][sstd] for the
//! Solana runtime environment. While off-chain programs that interact with the
//! Solana network _can_ link to this crate, they typically instead use the
//! [`solana-sdk`] crate, which reexports all modules from `solana-program`.
//!
//! [std]: https://doc.rust-lang.org/stable/std/
//! [sstd]: https://solana.com/docs/programs/limitations#rust-libraries
//! [`solana-sdk`]: https://docs.rs/solana-sdk/latest/solana_sdk/
//!
//! This library defines
//!
//! - macros for declaring the [program entrypoint][pe],
//! - [core data types][cdt],
//! - [logging] macros,
//! - [serialization] methods,
//! - methods for [cross-program instruction execution][cpi],
//! - program IDs and instruction constructors for the system program and other
//!   [native programs][np],
//! - [sysvar] accessors.
//!
//! [pe]: #defining-a-solana-program
//! [cdt]: #core-data-types
//! [logging]: crate::log
//! [serialization]: #serialization
//! [np]: #native-programs
//! [cpi]: #cross-program-instruction-execution
//! [sysvar]: crate::sysvar
//!
//! Idiomatic examples of `solana-program` usage can be found in
//! [the Solana Program Library][spl].
//!
//! [spl]: https://github.com/solana-labs/solana-program-library
//!
//! # Defining a solana program
//!
//! Solana program crates have some unique properties compared to typical Rust
//! programs:
//!
//! - They are often compiled for both on-chain use and off-chain use. This is
//!   primarily because off-chain clients may need access to data types
//!   defined by the on-chain program.
//! - They do not define a `main` function, but instead define their entrypoint
//!   with the [`entrypoint!`] macro.
//! - They are compiled as the ["cdylib"] crate type for dynamic loading
//!   by the Solana runtime.
//! - They run in a constrained VM environment, and while they do have access to
//!   the [Rust standard library][std], many features of the standard library,
//!   particularly related to OS services, will fail at runtime, will silently
//!   do nothing, or are not defined. See the [restrictions to the Rust standard
//!   library][sstd] in the Solana documentation for more.
//!
//! [std]: https://doc.rust-lang.org/std/index.html
//! ["cdylib"]: https://doc.rust-lang.org/reference/linkage.html
//!
//! Because multiple crates that are linked together cannot all define
//! program entrypoints (see the [`entrypoint!`] documentation) a common
//! convention is to use a [Cargo feature] called `no-entrypoint` to allow
//! the program entrypoint to be disabled.
//!
//! [Cargo feature]: https://doc.rust-lang.org/cargo/reference/features.html
//!
//! The skeleton of a Solana program typically looks like:
//!
//! ```
//! #[cfg(not(feature = "no-entrypoint"))]
//! pub mod entrypoint {
//!     use solana_program::{
//!         account_info::AccountInfo,
//!         entrypoint,
//!         entrypoint::ProgramResult,
//!         pubkey::Pubkey,
//!     };
//!
//!     entrypoint!(process_instruction);
//!
//!     pub fn process_instruction(
//!         program_id: &Pubkey,
//!         accounts: &[AccountInfo],
//!         instruction_data: &[u8],
//!     ) -> ProgramResult {
//!         // Decode and dispatch instructions here.
//!         todo!()
//!     }
//! }
//!
//! // Additional code goes here.
//! ```
//!
//! With a `Cargo.toml` file that contains
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [features]
//! no-entrypoint = []
//! ```
//!
//! Note that a Solana program must specify its crate-type as "cdylib", to
//! be discovered and built by the `cargo-build-sbf` command as a deployable program.
//! Solana programs also often have crate-type "rlib" so they can be linked to other Rust crates.
//! Avoid using "rlib" and "cdylib" crates together, since their combined usage precludes
//! compiler optimizations that may decrease program size and CU usage.
//!
//! Prefer writing a separate package if it is supposed to be used as a library for other Solana
//! programs (i.e. a "rlib" only crate). This would be normally the case for defining account
//! types and helpers that are used by both clients and program. When creating a Rust project
//! intended to be a program ready for deployment, use only the "cdylib" crate type.
//!
//! # On-chain vs. off-chain compilation targets
//!
//! Solana programs run on the [rbpf] VM, which implements a variant of the
//! [eBPF] instruction set. Because this crate can be compiled for both on-chain
//! and off-chain execution, the environments of which are significantly
//! different, it extensively uses [conditional compilation][cc] to tailor its
//! implementation to the environment. The `cfg` predicate used for identifying
//! compilation for on-chain programs is `target_os = "solana"`, as in this
//! example from the `solana-program` codebase that logs a message via a
//! syscall when run on-chain, and via a library call when offchain:
//!
//! [rbpf]: https://github.com/solana-labs/rbpf
//! [eBPF]: https://ebpf.io/
//! [cc]: https://doc.rust-lang.org/reference/conditional-compilation.html
//!
//! ```
//! pub fn sol_log(message: &str) {
//!     #[cfg(target_os = "solana")]
//!     unsafe {
//!         sol_log_(message.as_ptr(), message.len() as u64);
//!     }
//!
//!     #[cfg(not(target_os = "solana"))]
//!     program_stubs::sol_log(message);
//! }
//! # mod program_stubs {
//! #     pub(crate) fn sol_log(message: &str) { }
//! # }
//! ```
//!
//! This `cfg` pattern is suitable as well for user code that needs to work both
//! on-chain and off-chain.
//!
//! `solana-program` and `solana-sdk` were previously a single crate. Because of
//! this history, and because of the dual-usage of `solana-program` for two
//! different environments, it contains some features that are not available to
//! on-chain programs at compile-time. It also contains some on-chain features
//! that will fail in off-chain scenarios at runtime. This distinction is not
//! well-reflected in the documentation.
//!
//! For a more complete description of Solana's implementation of eBPF and its
//! limitations, see the main Solana documentation for [on-chain programs][ocp].
//!
//! [ocp]: https://solana.com/docs/programs
//!
//! # Core data types
//!
//! - [`Pubkey`] &mdash; The address of a [Solana account][acc]. Some account
//!   addresses are [ed25519] public keys, with corresponding secret keys that
//!   are managed off-chain. Often, though, account addresses do not have
//!   corresponding secret keys &mdash; as with [_program derived
//!   addresses_][pdas] &mdash; or the secret key is not relevant to the
//!   operation of a program, and may have even been disposed of. As running
//!   Solana programs can not safely create or manage secret keys, the full
//!   [`Keypair`] is not defined in `solana-program` but in `solana-sdk`.
//! - [`Hash`] &mdash; A cryptographic hash. Used to uniquely identify blocks,
//!   and also for general purpose hashing.
//! - [`AccountInfo`] &mdash; A description of a single Solana account. All accounts
//!   that might be accessed by a program invocation are provided to the program
//!   entrypoint as `AccountInfo`.
//! - [`Instruction`] &mdash; A directive telling the runtime to execute a program,
//!   passing it a set of accounts and program-specific data.
//! - [`ProgramError`] and [`ProgramResult`] &mdash; The error type that all programs
//!   must return, reported to the runtime as a `u64`.
//! - [`Sol`] &mdash; The Solana native token type, with conversions to and from
//!   [_lamports_], the smallest fractional unit of SOL, in the [`native_token`]
//!   module.
//!
//! [acc]: https://solana.com/docs/core/accounts
//! [`Pubkey`]: pubkey::Pubkey
//! [`Hash`]: hash::Hash
//! [`Instruction`]: instruction::Instruction
//! [`AccountInfo`]: account_info::AccountInfo
//! [`ProgramError`]: program_error::ProgramError
//! [`ProgramResult`]: entrypoint::ProgramResult
//! [ed25519]: https://ed25519.cr.yp.to/
//! [`Keypair`]: https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html
//! [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
//! [`Sol`]: native_token::Sol
//! [_lamports_]: https://solana.com/docs/intro#what-are-sols
//!
//! # Serialization
//!
//! Within the Solana runtime, programs, and network, at least three different
//! serialization formats are used, and `solana-program` provides access to
//! those needed by programs.
//!
//! In user-written Solana program code, serialization is primarily used for
//! accessing [`AccountInfo`] data and [`Instruction`] data, both of which are
//! program-specific binary data. Every program is free to decide their own
//! serialization format, but data received from other sources &mdash;
//! [sysvars][sysvar] for example &mdash; must be deserialized using the methods
//! indicated by the documentation for that data or data type.
//!
//! [`AccountInfo`]: account_info::AccountInfo
//! [`Instruction`]: instruction::Instruction
//!
//! The three serialization formats in use in Solana are:
//!
//! - __[Borsh]__, a compact and well-specified format developed by the [NEAR]
//!   project, suitable for use in protocol definitions and for archival storage.
//!   It has a [Rust implementation][brust] and a [JavaScript implementation][bjs]
//!   and is recommended for all purposes.
//!
//!   Users need to import the [`borsh`] crate themselves &mdash; it is not
//!   re-exported by `solana-program`, though this crate provides several useful
//!   utilities in its [`borsh1` module][borshmod] that are not available in the
//!   `borsh` library.
//!
//!   The [`Instruction::new_with_borsh`] function creates an `Instruction` by
//!   serializing a value with borsh.
//!
//!   [Borsh]: https://borsh.io/
//!   [NEAR]: https://near.org/
//!   [brust]: https://docs.rs/borsh
//!   [bjs]: https://github.com/near/borsh-js
//!   [`borsh`]: https://docs.rs/borsh
//!   [borshmod]: crate::borsh1
//!   [`Instruction::new_with_borsh`]: instruction::Instruction::new_with_borsh
//!
//! - __[Bincode]__, a compact serialization format that implements the [Serde]
//!   Rust APIs. As it does not have a specification nor a JavaScript
//!   implementation, and uses more CPU than borsh, it is not recommend for new
//!   code.
//!
//!   Many system program and native program instructions are serialized with
//!   bincode, and it is used for other purposes in the runtime. In these cases
//!   Rust programmers are generally not directly exposed to the encoding format
//!   as it is hidden behind APIs.
//!
//!   The [`Instruction::new_with_bincode`] function creates an `Instruction` by
//!   serializing a value with bincode.
//!
//!   [Bincode]: https://docs.rs/bincode
//!   [Serde]: https://serde.rs/
//!   [`Instruction::new_with_bincode`]: instruction::Instruction::new_with_bincode
//!
//! - __[`Pack`]__, a Solana-specific serialization API that is used by many
//!   older programs in the [Solana Program Library][spl] to define their
//!   account format. It is difficult to implement and does not define a
//!   language-independent serialization format. It is not generally recommended
//!   for new code.
//!
//!   [`Pack`]: https://docs.rs/solana-program-pack/latest/trait.Pack.html
//!
//! Developers should carefully consider the CPU cost of serialization, balanced
//! against the need for correctness and ease of use: off-the-shelf
//! serialization formats tend to be more expensive than carefully hand-written
//! application-specific formats; but application-specific formats are more
//! difficult to ensure the correctness of, and to provide multi-language
//! implementations for. It is not uncommon for programs to pack and unpack
//! their data with hand-written code.
//!
//! # Cross-program instruction execution
//!
//! Solana programs may call other programs, termed [_cross-program
//! invocation_][cpi] (CPI), with the [`invoke`] and [`invoke_signed`]
//! functions. When calling another program the caller must provide the
//! [`Instruction`] to be invoked, as well as the [`AccountInfo`] for every
//! account required by the instruction. Because the only way for a program to
//! acquire `AccountInfo` values is by receiving them from the runtime at the
//! [program entrypoint][entrypoint!], any account required by the callee
//! program must transitively be required by the caller program, and provided by
//! _its_ caller.
//!
//! [`invoke`]: program::invoke
//! [`invoke_signed`]: program::invoke_signed
//! [cpi]: https://solana.com/docs/core/cpi
//!
//! A simple example of transferring lamports via CPI:
//!
//! ```
//! use solana_account_info::{next_account_info, AccountInfo};
//! use solana_program_entrypoint::entrypoint;
//! use solana_program_error::ProgramResult;
//! use solana_cpi::invoke;
//! use solana_pubkey::Pubkey;
//! use solana_system_interface::instruction::transfer;
//!
//! entrypoint!(process_instruction);
//!
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!
//!     let payer = next_account_info(account_info_iter)?;
//!     let recipient = next_account_info(account_info_iter)?;
//!
//!     assert!(payer.is_writable);
//!     assert!(payer.is_signer);
//!     assert!(recipient.is_writable);
//!
//!     let lamports = 1000000;
//!
//!     invoke(
//!         &transfer(payer.key, recipient.key, lamports),
//!         &[payer.clone(), recipient.clone()],
//!     )
//! }
//! ```
//!
//! Solana also includes a mechanism to let programs control and sign for
//! accounts without needing to protect a corresponding secret key, called
//! [_program derived addresses_][pdas]. PDAs are derived with the
//! [`Pubkey::find_program_address`] function. With a PDA, a program can call
//! `invoke_signed` to call another program while virtually "signing" for the
//! PDA.
//!
//! [pdas]: https://solana.com/docs/core/cpi#program-derived-addresses
//! [`Pubkey::find_program_address`]: pubkey::Pubkey::find_program_address
//!
//! A simple example of creating an account for a PDA:
//!
//! ```
//! use solana_account_info::{next_account_info, AccountInfo};
//! use solana_program_entrypoint::entrypoint;
//! use solana_program_error::ProgramResult;
//! use solana_cpi::invoke_signed;
//! use solana_pubkey::Pubkey;
//! use solana_system_interface::instruction::create_account;
//!
//! entrypoint!(process_instruction);
//!
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let payer = next_account_info(account_info_iter)?;
//!     let vault_pda = next_account_info(account_info_iter)?;
//!     let system_program = next_account_info(account_info_iter)?;
//!
//!     assert!(payer.is_writable);
//!     assert!(payer.is_signer);
//!     assert!(vault_pda.is_writable);
//!     assert_eq!(vault_pda.owner, &solana_system_interface::program::ID);
//!     assert!(solana_system_interface::program::check_id(system_program.key));
//!
//!     let vault_bump_seed = instruction_data[0];
//!     let vault_seeds = &[b"vault", payer.key.as_ref(), &[vault_bump_seed]];
//!     let expected_vault_pda = Pubkey::create_program_address(vault_seeds, program_id)?;
//!
//!     assert_eq!(vault_pda.key, &expected_vault_pda);
//!
//!     let lamports = 10000000;
//!     let vault_size = 16;
//!
//!     invoke_signed(
//!         &create_account(
//!             &payer.key,
//!             &vault_pda.key,
//!             lamports,
//!             vault_size,
//!             &program_id,
//!         ),
//!         &[
//!             payer.clone(),
//!             vault_pda.clone(),
//!         ],
//!         &[
//!             &[
//!                 b"vault",
//!                 payer.key.as_ref(),
//!                 &[vault_bump_seed],
//!             ],
//!         ]
//!     )?;
//!     Ok(())
//! }
//! ```
//!
//! # Native programs
//!
//! Some solana programs are [_native programs_][np2], running native machine
//! code that is distributed with the runtime, with well-known program IDs.
//!
//! [np2]: https://docs.solanalabs.com/runtime/programs
//!
//! Some native programs can be [invoked][cpi] by other programs, but some can
//! only be executed as "top-level" instructions included by off-chain clients
//! in a [`Transaction`].
//!
//! [`Transaction`]: https://docs.rs/solana-sdk/latest/solana_sdk/transaction/struct.Transaction.html
//!
//! This crate defines the program IDs for most native programs. Even though
//! some native programs cannot be invoked by other programs, a Solana program
//! may need access to their program IDs. For example, a program may need to
//! verify that an ed25519 signature verification instruction was included in
//! the same transaction as its own instruction. For many native programs, this
//! crate also defines enums that represent the instructions they process, and
//! constructors for building the instructions.
//!
//! Locations of program IDs and instruction constructors are noted in the list
//! below, as well as whether they are invokable by other programs.
//!
//! While some native programs have been active since the genesis block, others
//! are activated dynamically after a specific [slot], and some are not yet
//! active. This documentation does not distinguish which native programs are
//! active on any particular network. The `solana feature status` CLI command
//! can help in determining active features.
//!
//! [slot]: https://solana.com/docs/terminology#slot
//!
//! Native programs important to Solana program authors include:
//!
//! - __System Program__: Creates new accounts, allocates account data, assigns
//!   accounts to owning programs, transfers lamports from System Program owned
//!   accounts and pays transaction fees.
//!   - ID: [`solana_system_interface::program::ID`](https://docs.rs/solana-system-interface/latest/solana_system_interface/program/constant.ID.html)
//!   - Instruction: [`solana_system_interface::instruction`](https://docs.rs/solana-system-interface/latest/solana_system_interface/instruction/index.html)
//!   - Invokable by programs? yes
//!
//! - __Compute Budget Program__: Requests additional CPU or memory resources
//!   for a transaction. This program does nothing when called from another
//!   program.
//!   - ID: [`solana_compute_budget_interface::ID`](https://docs.rs/solana-compute-budget-interface/latest/solana_compute_budget_interface/constant.ID.html)
//!   - Instruction: [`solana_compute_budget_interface::ComputeBudgetInstruction`](https://docs.rs/solana-compute-budget-interface/latest/solana_compute_budget_interface/enum.ComputeBudgetInstruction.html)
//!   - Invokable by programs? no
//!
//! - __ed25519 Program__: Verifies an ed25519 signature.
//!   - ID: [`solana_sdk_ids::ed25519_program::ID`](https://docs.rs/solana-sdk-ids/latest/solana_sdk_ids/ed25519_program/constant.ID.html)
//!   - Instruction: [`solana_ed25519_program::new_ed25519_instruction_with_signature`](https://docs.rs/solana-ed25519-program/latest/solana_ed25519_program/fn.new_ed25519_instruction_with_signature.html)
//!   - Invokable by programs? no
//!
//! - __secp256k1 Program__: Verifies secp256k1 public key recovery operations.
//!   - ID: [`solana_sdk_ids::secp256k1_program::ID`](https://docs.rs/solana-sdk-ids/latest/solana_sdk_ids/secp256k1_program/constant.ID.html)
//!   - Instruction: [`solana_secp256k1_program::new_secp256k1_instruction_with_signature`](https://docs.rs/solana-secp256k1-program/latest/solana_secp256k1_program/fn.new_secp256k1_instruction_with_signature.html)
//!   - Invokable by programs? no
//!
//! - __BPF Loader__: Deploys, and executes immutable programs on the chain.
//!   - ID: [`solana_sdk_ids::bpf_loader::ID`](https://docs.rs/solana-sdk-ids/latest/solana_sdk_ids/bpf_loader/constant.ID.html)
//!   - Instruction: [`solana_loader_v2_interface::instruction`](https://docs.rs/solana-loader-v2-interface/latest/solana_loader_v2_interface/instruction/index.html)
//!   - Invokable by programs? yes
//!
//! - __Upgradable BPF Loader__: Deploys, upgrades, and executes upgradable
//!   programs on the chain.
//!   - ID: [`solana_sdk_ids::bpf_loader_upgradeable::ID`](https://docs.rs/solana-sdk-ids/latest/solana_sdk_ids/bpf_loader_upgradeable/constant.ID.html)
//!   - Instruction: [`solana_loader_v3_interface::instruction`](https://docs.rs/solana-loader-v3-interface/latest/solana_loader_v3_interface/instruction/index.html)
//!   - Invokable by programs? yes
//!
//! - __Deprecated BPF Loader__: Deploys, and executes immutable programs on the
//!   chain.
//!   - ID: [`solana_sdk_ids::bpf_loader_deprecated::ID`](https://docs.rs/solana-sdk-ids/latest/solana_sdk_ids/bpf_loader_deprecated/constant.ID.html)
//!   - Instruction: [`solana_loader_v2_interface::instruction`](https://docs.rs/solana-loader-v2-interface/latest/solana_loader_v2_interface/instruction/index.html)
//!   - Invokable by programs? yes
//!
//! [lut]: https://docs.solanalabs.com/proposals/versioned-transactions

#![allow(incomplete_features)]
#![cfg_attr(feature = "frozen-abi", feature(specialization))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Allows macro expansion of `use ::solana_program::*` to work within this crate
extern crate self as solana_program;

pub mod bpf_loader;
pub mod bpf_loader_deprecated;
pub mod compute_units;
pub mod ed25519_program;
pub mod entrypoint_deprecated;
pub mod epoch_schedule;
pub mod epoch_stake;
pub mod hash;
pub mod incinerator;
pub mod instruction;
pub mod lamports;
pub mod log;
pub mod program;
pub mod program_error;
pub mod secp256k1_program;
pub mod slot_hashes;
pub mod slot_history;
pub mod sol_pub_unit8;
pub mod syscalls;
pub mod sysvar;

#[deprecated(since = "2.2.0", note = "Use `solana-big-mod-exp` crate instead")]
pub use solana_big_mod_exp as big_mod_exp;
#[deprecated(since = "2.2.0", note = "Use `solana-blake3-hasher` crate instead")]
pub use solana_blake3_hasher as blake3;
#[cfg(feature = "borsh")]
#[deprecated(since = "2.1.0", note = "Use `solana-borsh` crate instead")]
pub use solana_borsh::v1 as borsh1;
#[deprecated(since = "2.1.0", note = "Use `solana-epoch-rewards` crate instead")]
pub use solana_epoch_rewards as epoch_rewards;
#[deprecated(since = "2.1.0", note = "Use `solana-fee-calculator` crate instead")]
pub use solana_fee_calculator as fee_calculator;
#[deprecated(since = "2.2.0", note = "Use `solana-keccak-hasher` crate instead")]
pub use solana_keccak_hasher as keccak;
#[deprecated(since = "2.1.0", note = "Use `solana-last-restart-slot` crate instead")]
pub use solana_last_restart_slot as last_restart_slot;
#[deprecated(since = "2.1.0", note = "Use `solana-program-memory` crate instead")]
pub use solana_program_memory as program_memory;
#[deprecated(since = "2.1.0", note = "Use `solana-program-pack` crate instead")]
pub use solana_program_pack as program_pack;
#[deprecated(since = "2.1.0", note = "Use `solana-secp256k1-recover` crate instead")]
pub use solana_secp256k1_recover as secp256k1_recover;
#[deprecated(since = "2.1.0", note = "Use `solana-serde-varint` crate instead")]
pub use solana_serde_varint as serde_varint;
#[deprecated(since = "2.1.0", note = "Use `solana-serialize-utils` crate instead")]
pub use solana_serialize_utils as serialize_utils;
#[deprecated(since = "2.1.0", note = "Use `solana-short-vec` crate instead")]
pub use solana_short_vec as short_vec;
#[deprecated(since = "2.1.0", note = "Use `solana-stable-layout` crate instead")]
pub use solana_stable_layout as stable_layout;
#[cfg(not(target_os = "solana"))]
pub use solana_sysvar::program_stubs;
pub use {
    solana_account_info::{self as account_info, debug_account_data},
    solana_clock as clock,
    solana_msg::msg,
    solana_native_token as native_token,
    solana_program_entrypoint::{
        self as entrypoint, custom_heap_default, custom_panic_default, entrypoint,
        entrypoint_no_alloc,
    },
    solana_program_option as program_option, solana_pubkey as pubkey, solana_rent as rent,
    solana_sysvar::impl_sysvar_get,
};
/// The [config native program][np].
///
/// [np]: https://docs.solanalabs.com/runtime/programs#config-program
pub mod config {
    pub mod program {
        pub use solana_sdk_ids::config::{check_id, id, ID};
    }
}

pub use solana_pubkey::{declare_deprecated_id, declare_id, pubkey};
#[deprecated(since = "2.1.0", note = "Use `solana-sysvar-id` crate instead")]
pub use solana_sysvar_id::{declare_deprecated_sysvar_id, declare_sysvar_id};

/// Convenience macro for doing integer division where the operation's safety
/// can be checked at compile-time.
///
/// Since `unchecked_div_by_const!()` is supposed to fail at compile-time, abuse
/// doctests to cover failure modes
///
/// # Examples
///
/// Literal denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// let _ = unchecked_div_by_const!(10, 0);
/// # }
/// ```
///
/// Const denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// const D: u64 = 0;
/// let _ = unchecked_div_by_const!(10, D);
/// # }
/// ```
///
/// Non-const denominator fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// let d = 0;
/// let _ = unchecked_div_by_const!(10, d);
/// # }
/// ```
///
/// Literal denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// const N: u64 = 10;
/// let _ = unchecked_div_by_const!(N, 0);
/// # }
/// ```
///
/// Const denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// const N: u64 = 10;
/// const D: u64 = 0;
/// let _ = unchecked_div_by_const!(N, D);
/// # }
/// ```
///
/// Non-const denominator fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// # const N: u64 = 10;
/// let d = 0;
/// let _ = unchecked_div_by_const!(N, d);
/// # }
/// ```
///
/// Literal denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// let n = 10;
/// let _ = unchecked_div_by_const!(n, 0);
/// # }
/// ```
///
/// Const denominator div-by-zero fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// let n = 10;
/// const D: u64 = 0;
/// let _ = unchecked_div_by_const!(n, D);
/// # }
/// ```
///
/// Non-const denominator fails:
///
/// ```compile_fail
/// # use solana_program::unchecked_div_by_const;
/// # fn main() {
/// let n = 10;
/// let d = 0;
/// let _ = unchecked_div_by_const!(n, d);
/// # }
/// ```
#[macro_export]
macro_rules! unchecked_div_by_const {
    ($num:expr, $den:expr) => {{
        // Ensure the denominator is compile-time constant
        let _ = [(); ($den - $den) as usize];
        // Compile-time constant integer div-by-zero passes for some reason
        // when invoked from a compilation unit other than that where this
        // macro is defined. Do an explicit zero-check for now. Sorry about the
        // ugly error messages!
        // https://users.rust-lang.org/t/unexpected-behavior-of-compile-time-integer-div-by-zero-check-in-declarative-macro/56718
        let _ = [(); ($den as usize) - 1];
        #[allow(clippy::arithmetic_side_effects)]
        let quotient = $num / $den;
        quotient
    }};
}

// This re-export is purposefully listed after all other exports: because of an
// interaction within rustdoc between the reexports inside this module of
// `solana_program`'s top-level modules, and `solana_sdk`'s glob re-export of
// `solana_program`'s top-level modules, if this re-export is not lexically last
// rustdoc fails to generate documentation for the re-exports within
// `solana_sdk`.
#[deprecated(since = "2.2.0", note = "Use solana-example-mocks instead")]
#[cfg(not(target_os = "solana"))]
pub use solana_example_mocks as example_mocks;

#[cfg(test)]
mod tests {
    use super::unchecked_div_by_const;

    #[test]
    fn test_unchecked_div_by_const() {
        const D: u64 = 2;
        const N: u64 = 10;
        let n = 10;
        assert_eq!(unchecked_div_by_const!(10, 2), 5);
        assert_eq!(unchecked_div_by_const!(N, 2), 5);
        assert_eq!(unchecked_div_by_const!(n, 2), 5);
        assert_eq!(unchecked_div_by_const!(10, D), 5);
        assert_eq!(unchecked_div_by_const!(N, D), 5);
        assert_eq!(unchecked_div_by_const!(n, D), 5);
    }
}
