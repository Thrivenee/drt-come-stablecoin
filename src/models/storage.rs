// src/models/storage.rs
dharitri_sc::imports!();

use crate::models::types::*;
use dharitri_sc::storage::mappers::SingleValueMapper;

#[dharitri_sc::module]
pub trait StorageModule {
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
    
    #[view(getEconomicData)]
    #[storage_mapper("economicData")]
    fn economic_data(&self) -> SingleValueMapper<EconomicData<Self::Api>>;
    
    #[view(getMarketData)]
    #[storage_mapper("marketData")]
    fn market_data(&self) -> SingleValueMapper<MarketData<Self::Api>>;
    
    #[view(getVolatilityData)]
    #[storage_mapper("volatilityData")]
    fn volatility_data(&self) -> SingleValueMapper<VolatilityData<Self::Api>>;
    
    #[view(isActive)]
    #[storage_mapper("isActive")]
    fn is_active(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("lastUpdate")]
    fn last_update(&self) -> SingleValueMapper<u64>;
}
