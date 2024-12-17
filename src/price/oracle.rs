// src/price/oracle.rs
dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait OracleModule: 
    crate::models::storage::StorageModule + 
    crate::price::calculator::PriceCalculatorModule 
{
    #[endpoint]
    fn update_oracle_price(&self, oracle_price: BigUint) -> SCResult<()> {
        require!(
            self.oracles().contains(&self.blockchain().get_caller()),
            "Not authorized oracle"
        );
        require!(
            self.check_oracle_price_validity(&oracle_price),
            "Invalid oracle price"
        );
        
        self.oracle_price().set(&oracle_price);
        Ok(())
    }

    #[endpoint]
    fn add_oracle(&self, oracle_address: ManagedAddress) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Caller is not admin"
        );
        self.oracles().insert(oracle_address);
        Ok(())
    }

    #[endpoint]
    fn remove_oracle(&self, oracle_address: ManagedAddress) -> SCResult<()> {
        require!(
            self.blockchain().get_caller() == self.blockchain().get_owner_address(),
            "Caller is not admin"
        );
        self.oracles().swap_remove(&oracle_address);
        Ok(())
    }
    
    #[storage_mapper("oraclePrice")]
    fn oracle_price(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("oracles")]
    fn oracles(&self) -> UnorderedSetMapper<ManagedAddress>;
    
    fn check_oracle_price_validity(&self, price: &BigUint) -> bool {
        let current_price = self.current_price().get();
        let max_deviation = current_price.clone() / 10u64;
        
        if price > &current_price {
            price - &current_price <= max_deviation
        } else {
            &current_price - price <= max_deviation
        }
    }
}
