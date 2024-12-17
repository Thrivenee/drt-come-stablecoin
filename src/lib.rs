// src/lib.rs
#![no_std]

dharitri_sc::imports!();

pub mod components;
pub mod models;
pub mod price;
pub mod utils;

use crate::models::types::*;
use dharitri_sc::{
    types::{BigUint, TokenIdentifier},
};

pub const STABLE_COIN_NUM_DECIMALS: usize = 18;
pub const PRECISION: u64 = 1_000_000;

#[dharitri_sc::contract]
pub trait ComeStablecoin:
    crate::models::storage::StorageModule +
    crate::components::economic::EconomicModule +
    crate::components::market::MarketModule +
    crate::components::risk::RiskModule +
    crate::components::volatility::VolatilityModule +
    crate::price::calculator::PriceCalculatorModule +
    crate::price::oracle::OracleModule
{
    #[init]
    fn init(
        &self,
        initial_supply: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
    ) {
        let issue_cost = self.call_value().rewa_value().clone_value();
        
        self.send()
            .dcdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    can_burn: true,
                    can_mint: true,
                    num_decimals: STABLE_COIN_NUM_DECIMALS,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback());

        self.set_initial_parameters();
    }

    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set(&token_id);
                self.is_active().set(true);
            },
            ManagedAsyncCallResult::Err(_) => {
                sc_panic!("Token issuance failed");
            }
        }
    }

    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Only owner can set roles"
        );
        
        let roles = [
            DcdtLocalRole::Mint,
            DcdtLocalRole::Burn,
        ];
        
        self.send()
            .dcdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_id().get(),
                roles.iter().cloned(),
            )
            .async_call()
            .call_and_exit();
    }

    #[endpoint(updatePrice)]
    fn update_price(&self) -> SCResult<()> {
        require!(self.is_active().get(), "Contract is paused");

        let economic_weight = BigUint::from(40u64);
        let market_weight = BigUint::from(35u64);
        let volatility_weight = BigUint::from(25u64);

        let market_data = self.market_data().get();
        let volatility_data = self.volatility_data().get();

        let new_price = self.calculate_weighted_price(
            &market_data.price,
            &market_data.price,
            &volatility_data.current_volatility,
            &economic_weight,
            &market_weight,
            &volatility_weight,
        );

        require!(
            self.check_circuit_breakers(&new_price),
            "Circuit breakers triggered"
        );

        self.current_price().set(&new_price);
        Ok(())
    }

    fn check_circuit_breakers(&self, new_price: &BigUint) -> bool {
        let current_price = self.current_price().get();
        let max_deviation = current_price.clone() / 10u64;
        
        if new_price > &current_price {
            new_price - &current_price <= max_deviation
        } else {
            &current_price - new_price <= max_deviation
        }
    }

    fn set_initial_parameters(&self) {
        self.min_gdp_threshold().set(&BigUint::from(1_000_000_000u64));
        self.max_inflation_rate().set(&BigUint::from(20u64));
        self.min_forex_reserves().set(&BigUint::from(10_000_000u64));
        self.min_liquidity_threshold().set(&BigUint::from(1_000_000u64));
        self.max_price_impact().set(&BigUint::from(10u64));
        self.base_volatility_threshold().set(&BigUint::from(10u64));
        self.max_volatility_level().set(&BigUint::from(20u64));
        self.price_deviation_threshold().set(&BigUint::from(7u64));
        self.consecutive_breach_limit().set(&u32::from(3u32));
        self.is_active().set(true);
        self.last_update().set(&self.blockchain().get_block_timestamp());
    }
}
