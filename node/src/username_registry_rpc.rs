//! Username Registry RPC API
//!
//! Provides JSON-RPC endpoints for querying usernames by Ethereum address.

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    types::ErrorObjectOwned,
};
use pallet_username_registry::UsernameRegistryApi as UsernameRegistryRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_core::H160;
use sp_runtime::codec::Codec;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};
use std::sync::Arc;

#[rpc(client, server)]
pub trait UsernameRegistryApi<BlockHash, AccountId, ResponseType> {
    #[method(name = "usernameRegistry_getUsername")]
    fn get_username(
        &self,
        ethereum_address: String,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<ResponseType>>;
}

pub struct UsernameRegistryRpc<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> UsernameRegistryRpc<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

#[async_trait]
impl<C, Block, AccountId> UsernameRegistryApiServer<Block::Hash, AccountId, String>
    for UsernameRegistryRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
    C::Api: UsernameRegistryRuntimeApi<Block, AccountId>,
    AccountId: Clone + std::fmt::Display + Codec + MaybeDisplay,
{
    fn get_username(
        &self,
        ethereum_address: String,
        at: Option<Block::Hash>,
    ) -> RpcResult<Option<String>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        let eth_addr: H160 = ethereum_address
            .parse()
            .map_err(|_| ErrorObjectOwned::owned(1, "Invalid Ethereum address", None::<String>))?;

        let username_bytes = api
            .get_username(at, eth_addr)
            .map_err(|e| ErrorObjectOwned::owned(2, "Runtime API error", Some(e.to_string())))?;

        let username = username_bytes
            .map(|bytes| String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string()));

        Ok(username)
    }
}

