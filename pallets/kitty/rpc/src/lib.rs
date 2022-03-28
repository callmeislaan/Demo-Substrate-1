pub use self::gen_client::Client as TransactionPaymentClient;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay},
};
use std::sync::Arc;

pub use pallet_kitty_rpc_runtime_api::PalletKittyRuntimeApi;

#[rpc]
pub trait PalletKittyApi<BlochHash> {
	#[rpc(name = "kitty_cnt")]
	fn kitty_cnt(&self, at: Option<BlochHash>) -> Result<u64>;

}

pub struct PalletKittyRpc<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> PalletKittyRpc<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> PalletKittyApi<<Block as BlockT>::Hash> for PalletKittyRpc<C, Block>
	where
		Block: BlockT,
		C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
		C::Api: PalletKittyRuntimeApi<Block>,
{
	fn kitty_cnt(&self, at: Option<<Block as BlockT>::Hash>, ) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		api.kitty_cnt(&at).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

}
