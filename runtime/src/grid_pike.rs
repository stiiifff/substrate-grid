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

const BYTEARRAY_LIMIT: usize = 100;

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

			if id.len() == 0 || id.len() > BYTEARRAY_LIMIT {
				fail!(Err(format!("Organization ID required (1-{} bytes)", BYTEARRAY_LIMIT))
			}
			if name.len() == 0 || name.len() > BYTEARRAY_LIMIT {
				Err(format!("Organization name required (1-{} bytes)", BYTEARRAY_LIMIT));
			}

			let hash = T::Hashing::hash(&id);
			ensure!(!<Organizations<T>>::exists(&hash), "");

			let org = Organization {id: id.clone(), name};
			<Organizations<T>>::insert(hash, org);

			Self::deposit_event(RawEvent::OrganizationCreated(hash, id));
			Ok(())
        }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for GridPikeTest {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct GridPikeTest;

	impl system::Trait for GridPikeTest {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for GridPikeTest {
		type Event = ();
	}

	type GridPike = Module<GridPikeTest>;

	fn build_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<GridPikeTest>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works() {
		with_externalities(&mut build_ext(), || {
			assert!(true);
		})
	}

	#[test]
	fn create_org_with_valid_args() {
		with_externalities(&mut build_ext(), || {
			assert_ok!(GridPike::create_org(Origin::signed(1),
				"parity", "Parity Tech"));
			
			//todo: test that org is stored by hash
		})
	}
}
