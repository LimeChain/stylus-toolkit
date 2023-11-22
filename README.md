# stylus-toolkit
Building blocks for Stylus smart contract development

## How to use

1. Create new project
```shell
cargo stylus new my-cool-project --minimal
```

2. Import `stylus-toolkit` as dependency in the `Cargo.toml`
```toml
stylus-toolkit = { git = "https://github.com/LimeChain/stylus-toolkit.git", branch = "main" }
```

3. Import the desired contracts into your source files

ERC20 Example

```rust
use stylus_toolkit::tokens::erc20::{Erc20, Erc20Params};

struct MyParams;
impl Erc20Params for MyParams {
    const NAME: &'static str = "Dummy ERC20 token";
    const SYMBOL: &'static str = "DERC20";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct DummyErc20 {
        #[borrow] // Allows erc20 to access Dummy Erc20's storage and make calls
        Erc20<MyParams> erc20;
    }
}
#[external]
#[inherit(Erc20<MyParams>)]
impl DummyrErc20 {}
```

ED25519 Signature Verification

```rust
use stylus_toolkit::crypto::ed25519::ed25519_verify;

sol_storage! {
    #[entrypoint]
    struct Ed25519Verify { }
}

#[external]
impl Ed25519Verify {
    pub fn verify(
        &mut self,
        msg: Bytes,
        signature: Bytes,
        public_key: FixedBytes<32>,
    ) -> Result<(bool), Vec<u8>> {
        Ok(ed25519_verify(public_key, signature, msg))
    }
}
```

4. Build the project

```shell
cargo build --package my-cool-project --release
```

5. Sanity check the source code for activation prior to deployment

```shell
cargo stylus check
```

6. Deploy contracts
```shell
cargo stylus deploy -e $RPC --private-key $PK
```