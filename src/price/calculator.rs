// src/price/calculator.rs
dharitri_sc::imports!();
use dharitri_sc::storage::mappers::SingleValueMapper;

#[dharitri_sc::module]
pub trait PriceCalculatorModule: crate::models::storage::StorageModule {
    #[storage_mapper("currentPrice")]
    fn current_price(&self) -> SingleValueMapper<BigUint>;

    fn calculate_weighted_price(
        &self,
        economic_price: &BigUint,
        market_price: &BigUint,
        volatility_price: &BigUint,
        economic_weight: &BigUint,
        market_weight: &BigUint,
        volatility_weight: &BigUint,
    ) -> BigUint {
        let total_weight = economic_weight + market_weight + volatility_weight;
        
        let weighted_economic = economic_price * economic_weight;
        let weighted_market = market_price * market_weight;
        let weighted_volatility = volatility_price * volatility_weight;
        
        (weighted_economic + weighted_market + weighted_volatility) / total_weight
    }
}
