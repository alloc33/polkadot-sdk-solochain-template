# Username Registry

Substrate blockchain that maps Ethereum addresses to usernames with a custom JSON-RPC interface.

## Build the project

```bash
cargo build --release
```

## Setup two nodes

### Generate network keys

```bash
rm -rf /tmp/alice /tmp/bob
mkdir -p /tmp/alice /tmp/bob

./target/release/solochain-template-node key generate-node-key --chain dev --base-path /tmp/alice
./target/release/solochain-template-node key generate-node-key --chain dev --base-path /tmp/bob
```

### Start Node A

```bash
./target/release/solochain-template-node \
  --alice \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --base-path /tmp/alice \
  --chain dev
```

Copy the peer ID from the startup logs (line starting with "Local node identity is:").

### Start Node B

```bash
./target/release/solochain-template-node \
  --bob \
  --port 30334 \
  --rpc-port 9945 \
  --base-path /tmp/bob \
  --chain dev \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PASTE_PEER_ID_HERE
```

Both nodes should show "💤 Idle (1 peers)" indicating they're connected.

## Store username data

Open [Polkadot JS Apps](https://polkadot.js.org/apps/?rpc=ws://localhost:9944) and go to Developer → Extrinsics:

- Pallet: `usernameRegistry`
- Function: `setUsername`
- ethereum_address: `0x1111111111111111111111111111111111111111`
- username: `alice`

Submit with Alice's account.

## Retrieve data via JSON-RPC

Query from Node A:
```bash
curl -H "Content-Type: application/json" -d '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "usernameRegistry_getUsername",
  "params": ["0x1111111111111111111111111111111111111111"]
}' http://localhost:9944
```

Query from Node B (proves sync works):
```bash
curl -H "Content-Type: application/json" -d '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "usernameRegistry_getUsername",
  "params": ["0x1111111111111111111111111111111111111111"]
}' http://localhost:9945
```

Both should return: `{"jsonrpc":"2.0","result":"alice","id":1}`

## Run tests

```bash
cargo test
```

## Implementation approach

I built this as a standard Substrate pallet with these components:

**Storage**: Uses a StorageMap to link H160 Ethereum addresses to BoundedVec usernames (64 byte limit to prevent bloat).

**Security**: The `set_username` extrinsic requires signed transactions via `ensure_signed()` - no anonymous updates allowed.

**RPC Layer**: Created a runtime API trait that the RPC server calls to fetch usernames. The RPC endpoint handles Ethereum address parsing and returns clean JSON.

**Multi-node**: Standard Substrate networking handles block sync between nodes. Set data on one node, query from another - the blockchain state is shared.

The design keeps the username registry logic isolated in its own pallet while following Substrate conventions. Added comprehensive unit tests covering the main user flows and edge cases.
