use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_core::H160;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_username() {
        let caller: T::AccountId = whitelisted_caller();
        let eth_address = H160::from_slice(&[1u8; 20]);
        let username = b"benchmarkuser".to_vec();

        #[extrinsic_call]
        set_username(RawOrigin::Signed(caller), eth_address, username.clone());

        assert_eq!(
            UserNames::<T>::get(eth_address).map(|v| v.into_inner()),
            Some(username)
        );
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

