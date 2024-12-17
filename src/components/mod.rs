// src/components/mod.rs
pub mod economic;
pub mod market;
pub mod risk;
pub mod volatility;

pub use economic::EconomicModule;
pub use market::MarketModule;
pub use risk::RiskModule;
pub use volatility::VolatilityModule;