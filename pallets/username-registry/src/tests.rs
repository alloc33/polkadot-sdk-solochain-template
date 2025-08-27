use crate::{mock::*, Error, Event, UserNames};
use frame_support::{assert_noop, assert_ok};
use sp_core::H160;

#[test]
fn set_username_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let eth_address = H160::from_slice(&[1u8; 20]);
        let username = b"alice".to_vec();

        assert_ok!(UsernameRegistry::set_username(
            RuntimeOrigin::signed(1),
            eth_address,
            username.clone()
        ));

        let stored = UserNames::<Test>::get(eth_address).unwrap();
        assert_eq!(stored.into_inner(), username);

        System::assert_last_event(
            Event::UsernameSet {
                ethereum_address: eth_address,
                username,
            }
            .into(),
        );
    });
}

#[test]
fn get_username_from_storage() {
    new_test_ext().execute_with(|| {
        let eth_address = H160::from_slice(&[2u8; 20]);
        let username = b"bob".to_vec();

        assert_ok!(UsernameRegistry::set_username(
            RuntimeOrigin::signed(1),
            eth_address,
            username.clone()
        ));

        let stored = UserNames::<Test>::get(eth_address);
        assert_eq!(stored.unwrap().into_inner(), username);
    });
}

#[test]
fn get_nonexistent_username_returns_none() {
    new_test_ext().execute_with(|| {
        let eth_address = H160::from_slice(&[99u8; 20]);
        let stored = UserNames::<Test>::get(eth_address);
        assert_eq!(stored, None);
    });
}

#[test]
fn set_username_requires_signed_origin() {
    new_test_ext().execute_with(|| {
        let eth_address = H160::from_slice(&[1u8; 20]);
        let username = b"unauthorized".to_vec();

        assert_noop!(
            UsernameRegistry::set_username(RuntimeOrigin::none(), eth_address, username),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn username_length_limit_enforced() {
    new_test_ext().execute_with(|| {
        let eth_address = H160::from_slice(&[3u8; 20]);
        let long_username = vec![b'a'; 65];

        assert_noop!(
            UsernameRegistry::set_username(RuntimeOrigin::signed(1), eth_address, long_username),
            Error::<Test>::UsernameTooLong
        );
    });
}

#[test]
fn multiple_addresses_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let eth_address1 = H160::from_slice(&[6u8; 20]);
        let eth_address2 = H160::from_slice(&[7u8; 20]);
        let username1 = b"alice".to_vec();
        let username2 = b"bob".to_vec();

        assert_ok!(UsernameRegistry::set_username(
            RuntimeOrigin::signed(1),
            eth_address1,
            username1.clone()
        ));
        assert_ok!(UsernameRegistry::set_username(
            RuntimeOrigin::signed(1),
            eth_address2,
            username2.clone()
        ));

        let stored1 = UserNames::<Test>::get(eth_address1).unwrap();
        let stored2 = UserNames::<Test>::get(eth_address2).unwrap();

        assert_eq!(stored1.into_inner(), username1);
        assert_eq!(stored2.into_inner(), username2);
    });
}
