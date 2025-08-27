use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

pub trait WeightInfo {
    fn set_username() -> Weight;
}

// Simple weight implementation for development
pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_username() -> Weight {
        // One storage write + event
        Weight::from_parts(10_000_000, 0).saturating_add(T::DbWeight::get().writes(1))
    }
}

impl WeightInfo for () {
    fn set_username() -> Weight {
        Weight::from_parts(10_000_000, 0).saturating_add(RocksDbWeight::get().writes(1))
    }
}
