// Copyright 2019 Steve Degosserie	
// Hyperledger Grid Pike compatible runtime module

use rstd::prelude::*;
use parity_codec::{Decode, Encode};
// use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
// use runtime_primitives::traits::Hash;``
use support::{
	decl_module, decl_storage, decl_event,
	ensure, fail, StorageMap,
	dispatch::Result
};
use system::ensure_signed;

const ERR_ORG_ID_REQUIRED : &str = "Organization ID required";
const ERR_ORG_ID_TOO_LONG : &str = "Organization ID too long";
const ERR_ORG_NAME_REQUIRED : &str = "Organization name required";
const ERR_ORG_NAME_TOO_LONG : &str = "Organization name too long";
const ERR_ORG_ALREADY_EXISTS : &str = "Organization already exists";

const BYTEARRAY_LIMIT: usize = 100;

// This could be a DID
pub type OrgId = Vec<u8>;
pub type OrgName = Vec<u8>;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Organization {
	pub id: OrgId,
	pub name: OrgName
}

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GridPike {
		Organizations get(org_by_id): map OrgId => Organization;
	}

	// add_extra_genesis {
	// 	config(orgs): Vec<(OrgId, OrgName)>;

	// 	build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
    // 		with_storage(storage, || {
	// 			for &(ref id, name) in &config.orgs {
	// 				let _ = <Module<T>>::create_org(id.clone(), name.clone());
	// 			}
	// 		});
	// 	});
	// }
}

decl_event!(
	pub enum Event
	{
		OrganizationCreated(OrgId, OrgName),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        pub fn create_org(origin, id: OrgId, name: OrgName) -> Result {
            let _origin = ensure_signed(origin)?;

			if let Err(err) = Self::validate_new_org(&id, &name) {
				fail!(err);
			}

			let org = Organization {id: id.clone(), name: name.clone()};
			<Organizations<T>>::insert(&id, org);

			Self::deposit_event(Event::OrganizationCreated(id, name));
			Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
	fn validate_new_org(id: &[u8], name: &[u8]) -> Result {
		ensure!(id.len() > 0, ERR_ORG_ID_REQUIRED);
		ensure!(id.len() <= BYTEARRAY_LIMIT, ERR_ORG_ID_TOO_LONG);
		ensure!(name.len() > 0, ERR_ORG_NAME_REQUIRED);
		ensure!(name.len() <= BYTEARRAY_LIMIT, ERR_ORG_NAME_TOO_LONG);
		ensure!(!<Organizations<T>>::exists::<Vec<u8>>(id.into()), ERR_ORG_ALREADY_EXISTS);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, assert_noop};
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

	type GridPike = super::Module<GridPikeTest>;

	fn build_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<GridPikeTest>::default().build_storage().unwrap().0;
		// t.extend(GenesisConfig::<GridPikeTest> {
		// 	orgs: vec![ (String::from(TEST_EXISTING_ORG).into_bytes(), String::from(TEST_EXISTING_ORG).into_bytes()) ],
		// }.build_storage().unwrap().0);
		t.into()
	}

	const TEST_ORG_ID : &str = "did:example:123456789abcdefghijk";
	const TEST_ORG_NAME : &str = "Parity Tech";
	const TEST_EXISTING_ORG : &str = "did:example:azertyuiop";
	const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

	#[test]
	fn create_org_with_valid_args() {
		with_externalities(&mut build_ext(), || {
			let id = String::from(TEST_ORG_ID).into_bytes();
			let name = String::from(TEST_ORG_NAME).into_bytes();

			assert_ok!(
				GridPike::create_org(
					Origin::signed(1),
					id.clone(),
					name.clone())
			);
			
			assert_eq!(
				GridPike::org_by_id(&id),
				Organization{id: id, name: name});
		})
	}

	#[test]
	fn create_org_with_missing_id() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					vec!(),
					String::from(TEST_ORG_NAME).into_bytes()),
				ERR_ORG_ID_REQUIRED
			);
		})
	}

	#[test]
	fn create_org_with_long_id() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					String::from(LONG_VALUE).into_bytes(),
					String::from(TEST_ORG_NAME).into_bytes()),
				ERR_ORG_ID_TOO_LONG
			);
		})
	}

	#[test]
	fn create_org_with_missing_name() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					String::from(TEST_ORG_ID).into_bytes(),
					vec!()),
				ERR_ORG_NAME_REQUIRED
			);
		})
	}

	#[test]
	fn create_org_with_long_name() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					String::from(TEST_ORG_ID).into_bytes(),
					String::from(LONG_VALUE).into_bytes()),
				ERR_ORG_NAME_TOO_LONG 
			);
		})
	}

	#[test]
	fn create_org_with_existing_id() {
		with_externalities(&mut build_ext(), || {
			let _ = 
				GridPike::create_org(
					Origin::signed(1),
					String::from(TEST_EXISTING_ORG).into_bytes(),
					String::from(TEST_ORG_NAME).into_bytes());

			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					String::from(TEST_EXISTING_ORG).into_bytes(),
					String::from(TEST_ORG_NAME).into_bytes()),
				ERR_ORG_ALREADY_EXISTS
			);
		})
	}
}
