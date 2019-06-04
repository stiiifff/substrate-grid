/// Copyright 2019 Steve Degosserie
/// Hyperledger Grid Pike compatible runtime module

use parity_codec::{Decode, Encode};
use primitives::{H256};
use runtime_primitives::traits::Hash;
use support::{
	decl_module, decl_storage, decl_event,
	ensure, StorageValue, StorageMap,
	dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

const BYTEARRAY_LIMIT: usize = 100;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Organization {
	pub id: Vec<u8>,
	pub name: Vec<u8>
}

decl_storage! {
	trait Store for Module<T: Trait> as GridPike {
		Organizations get(organizations): map H256 => Option<Organization>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        pub fn create_org(origin, id: Vec<u8>, name: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
			
			ensure!(id.len() > 0 && id.len() <= BYTEARRAY_LIMIT, format!("Organization ID required (1-{} bytes)".into(), BYTEARRAY_LIMIT));
			ensure!(name.len() > 0 && name.len() <= BYTEARRAY_LIMIT, format!("Organization name required (1-{} bytes)".into(), BYTEARRAY_LIMIT));

			let digest = <<T as system::Trait>::Hashing as Hash>::hash(&id);
			ensure!(Organizations::get(digest.clone()) == None, format!("Organization already exists: {}", String::from(id)));

			let org = Organization {id, name};
			Organizations::insert(&digest, org);

			Self::deposit_event(RawEvent::OrganizationCreated(org));
			Ok(())
        }

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		// pub fn do_something(origin, something: u32) -> Result {
		// 	// TODO: You only need this if you want to check it was signed.
		// 	let who = ensure_signed(origin)?;

		// 	// TODO: Code to execute when something calls this.
		// 	// For example: the following line stores the passed in u32 in the storage
		// 	<Something<T>>::put(something);

		// 	// here we are raising the Something event
		// 	Self::deposit_event(RawEvent::SomethingStored(something, who));
		// 	Ok(())
		// }
	}
}

decl_event!(
	pub enum Event {
		OrganizationCreated(Organization),
	}
);

impl<T: Trait> Module<T> {
}

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
