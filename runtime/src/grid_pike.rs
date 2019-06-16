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
const ERR_ORG_DOES_NOT_EXIST : &str = "Organization does not exist";
const ERR_AGENT_ALREADY_EXISTS: &str = "Agent already exists";
const ERR_AGENT_MUST_BE_ORG_ADMIN: &str = "Agent must be organization admin";

const BYTEARRAY_LIMIT: usize = 100;
const ROLE_ADMIN : &'static [u8;5] = b"admin";

// This could be a DID
pub type OrgId = Vec<u8>;
pub type OrgName = Vec<u8>;
pub type Role = Vec<u8>;

fn validate_org_id(id: &[u8]) -> Result {
	ensure!(id.len() > 0, ERR_ORG_ID_REQUIRED);
	ensure!(id.len() <= BYTEARRAY_LIMIT, ERR_ORG_ID_TOO_LONG);
	Ok(())
}

fn validate_org_name(name: &[u8]) -> Result {
	ensure!(name.len() > 0, ERR_ORG_NAME_REQUIRED);
	ensure!(name.len() <= BYTEARRAY_LIMIT, ERR_ORG_NAME_TOO_LONG);
	Ok(())
}

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
	pub account: AccountId,
	pub active: bool,
	pub roles: Vec<Role>
}

#[derive(Default)]
pub struct OrganizationBuilder {
	id: OrgId,
	name: OrgName
}
impl OrganizationBuilder {
	pub fn with_id(mut self, id: OrgId) -> Self {
		self.id = id;
		self
	}

	pub fn with_name(mut self, name: OrgName) -> Self {
		self.name = name;
		self
	}

	pub fn build(self) -> rstd::result::Result<Organization, &'static str> {
		validate_org_id(&self.id)?;
		validate_org_name(&self.name)?;
		let mut org = Organization::default();
		org.id = self.id;
		org.name = self.name;
		Ok(org)
	}
}

#[derive(Default)]
pub struct AgentBuilder<AccountId> {
	org_id: OrgId,
	account: AccountId,
	active: bool,
	roles: Vec<Role>
}
impl<AccountId: Default> AgentBuilder<AccountId> {
	pub fn with_org(mut self, org_id: OrgId) -> Self {
		self.org_id = org_id;
		self
	}

	pub fn with_account(mut self, account: AccountId) -> Self {
		self.account = account;
		self
	}

	pub fn is_active(mut self, active: bool) -> Self {
		self.active = active;
		self
	}

	pub fn with_roles(mut self, roles: Vec<Role>) -> Self {
		self.roles = roles;
		self
	}

	pub fn build(self) -> rstd::result::Result<Agent<AccountId>, &'static str> {
		validate_org_id(&self.org_id)?;
		let mut agent = Agent::<AccountId>::default();
		agent.org_id = self.org_id;
		agent.account = self.account;
		agent.active = self.active;
		agent.roles = self.roles;
		Ok(agent)
	}
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GridPike {
		Organizations get(org_by_id): map OrgId => Option<Organization>;
		Agents get(agent_by_account): map T::AccountId => Option<Agent<T::AccountId>>;
	}

	//FIXME: does not compile -> tests setup storage data inline instead
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
		AgentCreated(OrgId, AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

        pub fn create_org(origin, id: OrgId, name: OrgName) -> Result {
            let sender = ensure_signed(origin)?;
			
			let org = OrganizationBuilder::default()
				.with_id(id.clone())
				.with_name(name.clone())
				.build()?;
			Self::validate_new_org(&id)?;
			<Organizations<T>>::insert(&id, org);

			let agent = AgentBuilder::<T::AccountId>::default()
				.with_org(id.clone())
				.with_account(sender.clone())
				.is_active(true)
				.with_roles(vec![ROLE_ADMIN.to_vec()])
				.build()?;
			Self::validate_new_agent(&sender)?;
			<Agents<T>>::insert(&sender, agent);

			Self::deposit_event(RawEvent::OrganizationCreated(id.clone(), name));
			Self::deposit_event(RawEvent::AgentCreated(id, sender));

			Ok(())
        }

		pub fn create_agent(
			origin, org_id: OrgId, account: T::AccountId,
			active: bool, roles: Vec<Role>) -> Result {
			let sender = ensure_signed(origin)?;
			
			let agent = AgentBuilder::<T::AccountId>::default()
				.with_org(org_id.clone())
				.with_account(account.clone())
				.is_active(active)
				.with_roles(roles)
				.build()?;
			Self::validate_new_agent(&account)?;

			// verify the signer of the transaction is authorized to create agent
			Self::validate_is_org_admin(&sender, org_id.clone())?;

			<Agents<T>>::insert(&account, agent);

			Self::deposit_event(RawEvent::AgentCreated(org_id, account));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	// PUBLIC IMMUTABLES

	/// Checks whether an account has the 'Admin' role for the specified organization.
	pub fn is_admin(account: &T::AccountId, org_id: OrgId) -> bool {
		match <Agents<T>>::get(account) {
			Some(agent) => {
				agent.org_id == org_id && agent.active &&
				agent.roles.contains(&ROLE_ADMIN.to_vec())
			},
			None => false
		}
	}

	// PRIVATE MUTABLES

	// Helpers
	fn validate_new_org(id: &[u8]) -> Result {
		ensure!(!<Organizations<T>>::exists::<Vec<u8>>(id.into()), ERR_ORG_ALREADY_EXISTS);
		Ok(())
	}

	fn validate_new_agent(agent: &T::AccountId) -> Result {
		ensure!(!<Agents<T>>::exists(agent), ERR_AGENT_ALREADY_EXISTS);
		Ok(())
	}

	fn validate_is_org_admin(account: &T::AccountId, org_id: OrgId) -> Result {
		match Self::is_admin(account, org_id) {
			true => Ok(()),
			false => Err(ERR_AGENT_MUST_BE_ORG_ADMIN)
		}
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
		let t = system::GenesisConfig::<GridPikeTest>::default().build_storage().unwrap().0;
		//FIXME: doesn't work, see add_extra_genesis above
		// t.extend(GenesisConfig::<GridPikeTest> {
		// 	orgs: vec![ (String::from(TEST_EXISTING_ORG).into_bytes(), String::from(TEST_EXISTING_ORG).into_bytes()) ],
		// }.build_storage().unwrap().0);
		t.into()
	}

	const TEST_ORG_ID : &str = "did:example:123456789abcdefghijk";
	const TEST_ORG_NAME : &str = "Parity Tech";
	const TEST_EXISTING_ORG : &str = "did:example:azertyuiop";
	const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

	// create_org tests
	#[test]
	fn create_org_with_valid_args() {
		with_externalities(&mut build_ext(), || {
			// Arrange
			let sender = 1;
			let id = String::from(TEST_ORG_ID).into_bytes();
			let name = String::from(TEST_ORG_NAME).into_bytes();

			// Act
			let result = GridPike::create_org(
				Origin::signed(sender),
				id.clone(),
				name.clone()
			);
			
			// Assert
			assert_ok!(result);

			assert_eq!(
				GridPike::org_by_id(&id),
				Some(
					Organization {
						id: id.clone(),
						name: name
					}
				)
			);
			
			assert_eq!(
				GridPike::agent_by_account(&sender),
				Some(
					Agent {
						org_id: id.clone(),
						account: sender,
						active: true,
						roles: vec![ROLE_ADMIN.to_vec()]
					}
				)
			);
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
			let existing_org = String::from(TEST_EXISTING_ORG).into_bytes();
			let name = String::from(TEST_ORG_NAME).into_bytes();

			Organizations::<GridPikeTest>::insert(&existing_org,
				Organization {
					id: existing_org.clone(),
					name: name.clone()
				}
			);

			assert_noop!(
				GridPike::create_org(
					Origin::signed(1),
					existing_org.clone(),
					name),
				ERR_ORG_ALREADY_EXISTS
			);
		})
	}

	#[test]
	fn create_org_with_existing_agent() {
		with_externalities(&mut build_ext(), || {
			// Arrange
			let sender = 1;
			let id = String::from(TEST_ORG_ID).into_bytes();
			let name = String::from(TEST_ORG_NAME).into_bytes();

			Agents::<GridPikeTest>::insert(sender,
				Agent {
					org_id: String::from(TEST_EXISTING_ORG).into_bytes(),
					account: sender,
					active: true,
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);
			
			assert_noop!(
				GridPike::create_org(Origin::signed(sender), id, name),
				ERR_AGENT_ALREADY_EXISTS
			);
		})
	}

	// create_agent tests
	#[test]
	fn create_agent_with_valid_args() {
		with_externalities(&mut build_ext(), || {
			let admin = 1;
			let agent = 2;
			let id = String::from(TEST_ORG_ID).into_bytes();

			// Create org & admin agent
			Organizations::<GridPikeTest>::insert(&id,
				Organization {
					id: id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(admin,
				Agent {
					org_id: id.clone(),
					account: admin,
					active: true,	
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);

			// Send tx to create non-admin agent for org
			let result = GridPike::create_agent(
				Origin::signed(admin),
				id.clone(),
				agent, true, vec!()
			);

			assert_ok!(result);
			
			assert_eq!(
				GridPike::agent_by_account(&agent),
				Some(
					Agent {
						org_id: id.clone(),
						account: agent,
						active: true,
						roles: vec!()
					}
				)
			);
		})
	}

	#[test]
	fn create_agent_with_missing_org_id() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_agent(
					Origin::signed(1),
					vec!(),
					2, true, vec!()),
				ERR_ORG_ID_REQUIRED
			);
		})
	}

	#[test]
	fn create_agent_with_long_org_id() {
		with_externalities(&mut build_ext(), || {
			assert_noop!(
				GridPike::create_agent(
					Origin::signed(1),
					String::from(LONG_VALUE).into_bytes(),
					2, true, vec!()),
				ERR_ORG_ID_TOO_LONG
			);
		})
	}

	#[test]
	fn create_agent_with_existing_account() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let id = String::from(TEST_ORG_ID).into_bytes();
			let other_org = String::from(TEST_EXISTING_ORG).into_bytes();

			// Insert test data directly into storage to test public immutable func
			Organizations::<GridPikeTest>::insert(&id,
				Organization {
					id: id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(agent,
				Agent {
					org_id: id.clone(),
					account: agent,
					active: true,	
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);

			assert_noop!(
				GridPike::create_agent(
					Origin::signed(agent.clone()),
					String::from(TEST_ORG_ID).into_bytes(),
					agent, true, vec!()),
				ERR_AGENT_ALREADY_EXISTS
			);
		})
	}

	// #[test]
	// fn create_org_with_long_name() {
	// 	with_externalities(&mut build_ext(), || {
	// 		assert_noop!(
	// 			GridPike::create_org(
	// 				Origin::signed(1),
	// 				String::from(TEST_ORG_ID).into_bytes(),
	// 				String::from(LONG_VALUE).into_bytes()),
	// 			ERR_ORG_NAME_TOO_LONG 
	// 		);
	// 	})
	// }

	// #[test]
	// fn create_org_with_existing_id() {
	// 	with_externalities(&mut build_ext(), || {
	// 		let existing_org = String::from(TEST_EXISTING_ORG).into_bytes();

	// 		Organizations::<GridPikeTest>::insert(&existing_org,
	// 			Organization {
	// 				id: existing_org.clone(),
	// 				name: String::from(TEST_ORG_NAME).into_bytes()
	// 			}
	// 		);

	// 		assert_noop!(
	// 			GridPike::create_org(
	// 				Origin::signed(1),
	// 				existing_org.clone(),
	// 				String::from(TEST_ORG_NAME).into_bytes()),
	// 			ERR_ORG_ALREADY_EXISTS
	// 		);
	// 	})
	// }

	// #[test]
	// fn create_org_with_existing_agent() {
	// 	with_externalities(&mut build_ext(), || {
	// 		// Arrange
	// 		let sender = 1;
	// 		let id = String::from(TEST_ORG_ID).into_bytes();
	// 		let name = String::from(TEST_ORG_NAME).into_bytes();

	// 		Agents::<GridPikeTest>::insert(sender,
	// 			Agent {
	// 				org_id: String::from(TEST_EXISTING_ORG).into_bytes(),
	// 				account: sender,
	// 				active: true,
	// 				roles: vec![ROLE_ADMIN.to_vec()]
	// 			}
	// 		);
			
	// 		assert_noop!(
	// 			GridPike::create_org(Origin::signed(sender), id, name),
	// 			ERR_AGENT_ALREADY_EXISTS
	// 		);
	// 	})
	// }

	// is_admin tests
	#[test]
	fn is_admin_for_actual_org_admin() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let org_id = String::from(TEST_ORG_ID).into_bytes();
			
			// Insert test data directly into storage to test public immutable func
			Organizations::<GridPikeTest>::insert(&org_id,
				Organization {
					id: org_id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(agent,
				Agent {
					org_id: org_id.clone(),
					account: agent,
					active: true,
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);

			// Test is_admin method
			assert_eq!(
				GridPike::is_admin(&agent, org_id),
				true
			);
		})
	}

	#[test]
	fn is_admin_for_unknown_agent() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let org_id = String::from(TEST_ORG_ID).into_bytes();
			
			assert_eq!(
				GridPike::is_admin(&agent, org_id),
				false
			);
		})
	}

	#[test]
	fn is_admin_for_inactive_org_agent() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let org_id = String::from(TEST_ORG_ID).into_bytes();

			// Insert test data directly into storage to test public immutable func
			Organizations::<GridPikeTest>::insert(&org_id,
				Organization {
					id: org_id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(agent,
				Agent {
					org_id: org_id.clone(),
					account: agent,
					active: false,	// <-- /!\ Agent is inactive
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);
			
			assert_eq!(
				GridPike::is_admin(&agent, org_id),
				false
			);
		})
	}

	#[test]
	fn is_admin_for_non_admin_org_agent() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let org_id = String::from(TEST_ORG_ID).into_bytes();

			// Insert test data directly into storage to test public immutable func
			Organizations::<GridPikeTest>::insert(&org_id,
				Organization {
					id: org_id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(agent,
				Agent {
					org_id: org_id.clone(),
					account: agent,
					active: true,	
					roles: vec!()	// <-- /!\ Agent is not admin
				}
			);
			
			assert_eq!(
				GridPike::is_admin(&agent, org_id),
				false
			);
		})
	}

	#[test]
	fn is_admin_for_agent_org_mismatch() {
		with_externalities(&mut build_ext(), || {
			let agent = 1;
			let id = String::from(TEST_ORG_ID).into_bytes();
			let other_org = String::from(TEST_EXISTING_ORG).into_bytes();

			// Insert test data directly into storage to test public immutable func
			Organizations::<GridPikeTest>::insert(&id,
				Organization {
					id: id.clone(),
					name: String::from(TEST_ORG_NAME).into_bytes()
				}
			);

			Agents::<GridPikeTest>::insert(agent,
				Agent {
					org_id: id.clone(),
					account: agent,
					active: true,	
					roles: vec![ROLE_ADMIN.to_vec()]
				}
			);
			
			assert_eq!(
				GridPike::is_admin(&agent, other_org),  // <-- /!\ Agent is admin for another org
				false
			);
		})
	}
}
