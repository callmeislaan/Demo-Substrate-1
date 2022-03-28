#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait PalletKittyRuntimeApi{
		fn kitty_cnt() -> u64;
	}
}
