// src/components/base_trait.rs
use dharitri_sc::types::SCResult;

pub trait BaseModule {
    fn require_caller_is_admin(&self);
    fn check_circuit_breakers(&self, new_price: &BigUint) -> bool;
}
