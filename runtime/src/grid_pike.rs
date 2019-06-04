// Copyright 2019 Steve Degosserie
// Hyperledger Grid Pike compatible runtime module

use rstd::prelude::*;
use parity_codec::{Decode, Encode};
use runtime_primitives::traits::Hash;
use support::{
	decl_module, decl_storage, decl_event,
	ensure, StorageMap,
	dispatch::Result
};
use system::ensure_signed;
// use lazy_static::lazy_static;

// lazy_static! {
// 	ref BYTEARRAY_LIMIT: usize = 100;
// 	ref ORG_ID_INVALID_MSG: &str = format!("Organization ID required (1-{} bytes)", BYTEARRAY_LIMIT);
// 	ref ORG_NAME_INVALID_MSG: &str = format!("Organization name required (1-{} bytes)", BYTEARRAY_LIMIT);
// }

pub type OrgHash<T> = <T as system::Trait>::Hash;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Organization {
	pub id: Vec<u8>,
	pub name: Vec<u8>
}

decl_storage! {
	trait Store for Module<T: Trait> as GridPike {
		// Mapping from organization hash to Organization data
		Organizations: map OrgHash<T> => Organization;
	}
}

decl_event!(
	pub enum Event<T>
	where <T as system::Trait>::Hash
	{
		OrganizationCreated(Hash, Vec<u8>),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

        pub fn create_org(origin, id: Vec<u8>, name: Vec<u8>) -> Result {
            let _origin = ensure_signed(origin)?;
			
			// ensure!(id.len() > 0 && id.len() <= BYTEARRAY_LIMIT, ORG_ID_INVALID_MSG);
			// ensure!(name.len() > 0 && name.len() <= BYTEARRAY_LIMIT, ORG_NAME_INVALID_MSG);

			let hash = T::Hashing::hash(&id);
			ensure!(!<Organizations<T>>::exists(&hash), "");

			let org = Organization {id: id.clone(), name};
			<Organizations<T>>::insert(hash, org);

			Self::deposit_event(RawEvent::OrganizationCreated(hash, id));
			Ok(())
        }
	}
}

// impl<T: Trait> Module<T> {
// }

// tests for this module
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use runtime_io::with_externalities;
// 	use primitives::{H256, Blake2Hasher};
// 	use support::{impl_outer_origin, assert_ok};
// 	use runtime_primitives::{
// 		BuildStorage,
// 		traits::{BlakeTwo256, IdentityLookup},
// 		testing::{Digest, DigestItem, Header}
// 	};

// 	impl_outer_origin! {
// 		pub enum Origin for Test {}
// 	}

// 	// For testing the module, we construct most of a mock runtime. This means
// 	// first constructing a configuration type (`Test`) which `impl`s each of the
// 	// configuration traits of modules we want to use.
// 	#[derive(Clone, Eq, PartialEq)]
// 	pub struct Test;
// 	impl system::Trait for Test {
// 		type Origin = Origin;
// 		type Index = u64;
// 		type BlockNumber = u64;
// 		type Hash = H256;
// 		type Hashing = BlakeTwo256;
// 		type Digest = Digest;
// 		type AccountId = u64;
// 		type Lookup = IdentityLookup<Self::AccountId>;
// 		type Header = Header;
// 		type Event = ();
// 		type Log = DigestItem;
// 	}
// 	impl Trait for Test {
// 		type Event = ();
// 	}
// 	type TemplateModule = Module<Test>;

// 	// This function basically just builds a genesis storage key/value store according to
// 	// our desired mockup.
// 	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
// 		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
// 	}

// 	#[test]
// 	fn it_works_for_default_value() {
// 		with_externalities(&mut new_test_ext(), || {
// 			// Just a dummy test for the dummy funtion `do_something`
// 			// calling the `do_something` function with a value 42
// 			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 			// asserting that the stored value is equal to what we stored
// 			assert_eq!(TemplateModule::something(), Some(42));
// 		});
// 	}
// }
