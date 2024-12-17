// src/components/risk.rs
dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait RiskModule: crate::models::storage::StorageModule {
    #[storage_mapper("priceDeviationThreshold")]
    fn price_deviation_threshold(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("consecutiveBreachLimit")]
    fn consecutive_breach_limit(&self) -> SingleValueMapper<u32>;
}
