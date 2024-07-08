use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol},
    evm, msg,
    prelude::*,
};
use stylus_sdk::call::MethodError;

/// The ERC20Info trait is used to define the name, symbol, and decimals of an ERC20 token.
pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

sol_storage! {
    /// Declares the storage layout of the ERC20 token.
    pub struct Erc20<T> {
        /// Mapping from user to balance
        mapping(address => uint256) balances;
        /// Mapping from user to each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;

        /// Used to specify the token's name, symbol, and decimals.
        PhantomData<T> params;
    }
}

sol! {
    /**
     * Emitted when `value` tokens are moved from `from` to `to`
     */
    event Transfer(address indexed from, address indexed to, uint256 value);
    /**
     * Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);

    /**
     * Error thrown in case of unsufficient balance during transfer
     */
    error InsufficientBalance(address from, uint256 have, uint256 want);
    /**
     * Error thrown in case of unsufficient allowance during transfer
     */
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

impl From<Erc20Error> for Vec<u8> {
    fn from(err: Erc20Error) -> Vec<u8> {
        match err {
            Erc20Error::InsufficientBalance(e) => e.encode(),
            Erc20Error::InsufficientAllowance(e) => e.encode(),
        }
    }
}

impl<T: Erc20Params> Erc20<T> {
    pub fn _transfer(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);
        evm::log(Transfer { from, to, value });
        Ok(true)
    }

    pub fn _mint(&mut self, address: Address, value: U256) {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);
        self.total_supply.set(self.total_supply.get() + value);
        evm::log(Transfer {
            from: Address::ZERO,
            to: address,
            value,
        });
    }
}

#[external]
impl<T: Erc20Params> Erc20<T> {
    /// Returns the name of the token.
    pub fn name() -> Result<String, Vec<u8>> {
        Ok(T::NAME.into())
    }

    /// Returns the symbol of the token.
    pub fn symbol() -> Result<String, Vec<u8>> {
        Ok(T::SYMBOL.into())
    }

    /// Returns the number of decimals the token uses.
    /// The information is used only for display purposes and does not affect
    /// the arithmetics of the contract.
    pub fn decimals() -> Result<u8, Vec<u8>> {
        Ok(T::DECIMALS)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    /// Returns the balance of the `account`.
    pub fn balance_of(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account))
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.get(owner).get(spender))
    }

    /// Moves a `value` amount of tokens from the caller's account to `to`.
    /// Returns a boolean value indicating whether the operation succeeded.
    /// Emits a {Transfer} event.
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self._transfer(msg::sender(), to, value)?;
        Ok(true)
    }

    /// Sets `value` as the allowance of `spender` over the caller's tokens.
    /// Returns a boolean value indicating whether the operation succeeded.
    /// Emits an {Approval} event.
    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Vec<u8>> {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        Ok(true)
    }

    /// Moves `value` amount of tokens from `from` to `to` using the allowance mechanism.
    /// `value` is then deducted from the caller's allowance.
    /// Returns a boolean value indicating whether the operation succeeded.
    /// Emits a {Transfer} event.
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: msg::sender(),
                have: old_allowance,
                want: value,
            }));
        }
        allowance.set(old_allowance - value);
        self._transfer(from, to, value)?;
        Ok(true)
    }
}
