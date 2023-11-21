// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::stylus_proc::{entrypoint, external, sol_storage};
use stylus_toolkit::tokens::erc20::{Erc20, Erc20Params};

struct WethParams;

/// Immutable definitions
impl Erc20Params for WethParams {
    const NAME: &'static str = "Wrapped Ether Example";
    const SYMBOL: &'static str = "WETH";
    const DECIMALS: u8 = 18;
}

// The contract
sol_storage! {
    #[entrypoint] // Makes Weth the entrypoint
    struct Weth {
        #[borrow] // Allows erc20 to access Weth's storage and make calls
        Erc20<WethParams> erc20;
    }
}

#[external]
#[inherit(Erc20<WethParams>)]
impl Weth {}
