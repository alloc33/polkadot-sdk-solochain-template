# Username Registry Pallet

A secure, production-ready Substrate pallet for managing username registrations associated with Ethereum addresses.

## Overview

The Username Registry pallet provides on-chain storage and management of usernames mapped to Ethereum addresses. It ensures security through signed transaction requirements and implements proper storage bounds to prevent bloat.

## Features

- **Secure Registration**: Only signed transactions can register or update usernames
- **Ethereum Integration**: Maps usernames to Ethereum addresses (H160 format)
- **Storage Efficiency**: Usernames are limited to 64 bytes to prevent storage bloat
- **Event Emission**: Emits events for successful username registrations
- **RPC Support**: Provides runtime API for external JSON-RPC queries

## API

### Dispatchables

#### `set_username(ethereum_address: H160, username: Vec<u8>)`
Register or update a username for the specified Ethereum address.

**Parameters:**
- `ethereum_address`: The Ethereum address to associate with the username
- `username`: The username to register (maximum 64 bytes)

**Errors:**
- `UsernameTooLong`: Username exceeds the 64-byte limit

### Events

#### `UsernameSet { ethereum_address: H160, username: Vec<u8> }`
Emitted when a username is successfully registered or updated.

### Storage

#### `UserNames: Map<H160, BoundedVec<u8, 64>>`
Maps Ethereum addresses to their registered usernames.

## Usage

```rust
use pallet_username_registry::{Config, Pallet};
use sp_core::H160;

// Register a username (in a dispatchable context)
let eth_addr = H160::from_slice(&[1u8; 20]);
let username = b"alice".to_vec();
Pallet::<T>::set_username(origin, eth_addr, username)?;

// Query username via runtime API
let stored_username = UserNames::<T>::get(&eth_addr);
```

## Testing

The pallet includes comprehensive unit tests covering:
- Successful username registration
- Storage retrieval
- Signed transaction requirements
- Length limit enforcement
- Multiple address handling

Run tests with:
```bash
cargo test -p pallet-username-registry
```

## License

MIT-0