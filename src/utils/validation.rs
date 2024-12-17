// src/utils/validation.rs
use dharitri_sc::{
    api::ManagedTypeApi,
    types::BigUint,
};

pub fn validate_price<M: ManagedTypeApi>(price: &BigUint<M>) -> bool {
    price > &BigUint::<M>::from(0u64)
}

pub fn validate_parameters<M: ManagedTypeApi>(params: &[BigUint<M>]) -> bool {
    params.iter().all(|p| p > &BigUint::<M>::from(0u64))
}
