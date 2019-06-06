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

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Agent<AccountId> {
	pub org_id: OrgId,
	pub pub_key: AccountId,
	pub active: bool,
	// pub roles:
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GridPike {
		Organizations get(org_by_id): map OrgId => Organization;
		Agents get(agent_by_pubkey): map T::AccountId => Agent<T::AccountId>;
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
	pub enum Event<T>
	where <T as system::Trait>::AccountId
	{
		OrganizationCreated(OrgId, OrgName),
		AgentCreated(AccountId, OrgId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

        pub fn create_org(origin, id: OrgId, name: OrgName) -> Result {
            let sender = ensure_signed(origin)?;

			if let Err(err) = Self::validate_new_org(&sender, &id, &name) {
				fail!(err);
			}

			let org = Organization {id: id.clone(), name: name.clone()};
			<Organizations<T>>::insert(&id, org);

			let agent = Agent {org_id: id.clone(), pub_key: sender.clone(), active: true};
			<Agents<T>>::insert(&sender, agent);

			Self::deposit_event(RawEvent::OrganizationCreated(id.clone(), name));
			Self::deposit_event(RawEvent::AgentCreated(sender, id));

			Ok(())
        }

		// pub fn create_agent(origin, org_id: OrgId, agent: T::AccountId, active: bool) -> Result {
		// 	let _sender = ensure_signed(origin)?;

		// 	let agent = Agent {org_id, pub_key}


		// 	Ok(())
		// }
	}
}

impl<T: Trait> Module<T> {
	fn validate_new_org(agent: &T::AccountId, id: &[u8], name: &[u8]) -> Result {
		// ensure!(<Agents<T>>::exists(agent), "Agent does not exist");
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
