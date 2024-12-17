// src/components/volatility.rs
dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait VolatilityModule: crate::models::storage::StorageModule {
    #[storage_mapper("baseVolatilityThreshold")]
    fn base_volatility_threshold(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("maxVolatilityLevel")]
    fn max_volatility_level(&self) -> SingleValueMapper<BigUint>;
}