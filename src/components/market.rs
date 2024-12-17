// src/components/market.rs
dharitri_sc::imports!();
use crate::models::types::MarketData;

#[dharitri_sc::module]
pub trait MarketModule: crate::models::storage::StorageModule {
    #[endpoint]
    fn update_market_data(
        &self,
        price: BigUint,
        volume: BigUint,
        liquidity: BigUint,
        depth: BigUint,
    ) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Caller is not admin"
        );

        require!(
            liquidity >= self.min_liquidity_threshold().get(),
            "Liquidity below threshold"
        );

        let market_data = MarketData {
            price,
            volume,
            liquidity,
            depth,
            last_update: self.blockchain().get_block_timestamp(),
        };

        self.market_data().set(&market_data);
        Ok(())
    }
    
    #[storage_mapper("minLiquidityThreshold")]
    fn min_liquidity_threshold(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("maxPriceImpact")]
    fn max_price_impact(&self) -> SingleValueMapper<BigUint>;
}
