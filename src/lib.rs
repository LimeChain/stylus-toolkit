// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// The token module exporting Stylus implementations of ERC20 and ERC721 tokens.
pub mod tokens;

/// The crypto module exporting Stylus implementations of cryptographic primitives.
pub mod crypto;