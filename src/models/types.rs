// src/models/types.rs
dharitri_sc::derive_imports!();
use dharitri_sc::{
    api::ManagedTypeApi,
    types::{BigUint, TokenIdentifier, ManagedVec},
};

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct EconomicData<M: ManagedTypeApi> {
    pub gdp: BigUint<M>,
    pub inflation_rate: BigUint<M>,
    pub trade_balance: BigUint<M>,
    pub forex_reserves: BigUint<M>,
    pub last_update: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct MarketData<M: ManagedTypeApi> {
    pub price: BigUint<M>,
    pub volume: BigUint<M>,
    pub liquidity: BigUint<M>,
    pub depth: BigUint<M>,
    pub last_update: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct VolatilityData<M: ManagedTypeApi> {
    pub historical_volatility: BigUint<M>,
    pub current_volatility: BigUint<M>,
    pub trend: BigUint<M>,
    pub price_history: ManagedVec<M, BigUint<M>>,
    pub last_update: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct ContractStatus<M: ManagedTypeApi> {
    pub is_active: bool,
    pub token_id: TokenIdentifier<M>,
    pub last_update: u64,
    pub current_price: BigUint<M>,
}