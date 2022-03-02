mod signature_validation;
mod utility_functions;

pub mod prelude {
	pub use super::{minimal_test_ext, offchain_test_ext, samples};
	pub use crate as pallet_airdrop;
	pub use frame_support::{
		assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_noop, assert_ok,
		assert_storage_noop,
	};
	pub use pallet_airdrop::mock::{self, new_test_ext, AirdropModule, Origin, Test};
	pub use pallet_airdrop::types;
}

pub mod samples {
	use super::prelude::types::ServerResponse;
	use sp_core::sr25519;

	pub const ACCOUNT_ID: &[sr25519::Public] = &[
		sr25519::Public([1; 32]),
		sr25519::Public([2; 32]),
		sr25519::Public([3; 32]),
		sr25519::Public([4; 32]),
		sr25519::Public([5; 32]),
	];

	pub const SERVER_DATA: &[ServerResponse] = &[
		ServerResponse {
			omm: 1234443_u128,
			amount: 345323_u128,
			stake: 8437566_u128,
			defi_user: true,
		},
		ServerResponse {
			omm: 8548467_u128,
			amount: 928333_u128,
			stake: 298329_u128,
			defi_user: false,
		},
	];
}

// Build genesis storage according to the mock runtime.
pub fn minimal_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<prelude::Test>()
		.unwrap()
		.into()
}

pub fn offchain_test_ext() -> (
	sp_io::TestExternalities,
	std::sync::Arc<parking_lot::RwLock<sp_core::offchain::testing::OffchainState>>,
) {
	use sp_core::offchain::TransactionPoolExt;
	use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
	use sp_runtime::RuntimeAppPublic;
	use std::sync::Arc;

	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
	let keystore = KeyStore::new();
	SyncCryptoStore::sr25519_generate_new(
		&keystore,
		crate::airdrop_crypto::Public::ID,
		Some(&format!("{}/abcdefg", PHRASE)),
	)
	.unwrap();

	let mut test_ext = sp_io::TestExternalities::default();
	let (pool, _pool_state) = sp_core::offchain::testing::TestTransactionPoolExt::new();
	let (offchain, state) = sp_core::offchain::testing::TestOffchainExt::new();

	test_ext.register_extension(sp_core::offchain::OffchainWorkerExt::new(offchain));
	test_ext.register_extension(TransactionPoolExt::new(pool));
	test_ext.register_extension(KeystoreExt(Arc::new(keystore)));

	(test_ext, state)
}
