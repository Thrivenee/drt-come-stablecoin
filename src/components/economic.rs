// src/components/economic.rs
dharitri_sc::imports!();
use crate::models::types::EconomicData;

#[dharitri_sc::module]
pub trait EconomicModule: crate::models::storage::StorageModule {
    #[endpoint]
    fn update_economic_data(
        &self,
        gdp: BigUint,
        inflation_rate: BigUint,
        trade_balance: BigUint,
        forex_reserves: BigUint,
    ) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Caller is not admin"
        );
        
        require!(
            gdp >= self.min_gdp_threshold().get(),
            "GDP below threshold"
        );
        require!(
            inflation_rate <= self.max_inflation_rate().get(),
            "Inflation rate too high"
        );
        require!(
            forex_reserves >= self.min_forex_reserves().get(),
            "Forex reserves too low"
        );

        let economic_data = EconomicData {
            gdp,
            inflation_rate,
            trade_balance,
            forex_reserves,
            last_update: self.blockchain().get_block_timestamp(),
        };

        self.economic_data().set(&economic_data);
        Ok(())
    }
    
    #[storage_mapper("minGdpThreshold")]
    fn min_gdp_threshold(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("maxInflationRate")]
    fn max_inflation_rate(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("minForexReserves")]
    fn min_forex_reserves(&self) -> SingleValueMapper<BigUint>;
}
