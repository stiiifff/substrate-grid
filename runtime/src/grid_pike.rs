// Copyright 2019 Steve Degosserie
// Hyperledger Grid Pike compatible runtime module

use parity_codec::{Decode, Encode};
use rstd::prelude::*;
// use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
// use runtime_primitives::traits::Hash;``
use support::{decl_event, decl_module, decl_storage,
    dispatch::Result, ensure, fail, StorageValue, StorageMap};
use system::ensure_signed;

pub const ERR_ORG_ID_REQUIRED: &str = "Organization ID required";
pub const ERR_ORG_ID_TOO_LONG: &str = "Organization ID too long";
pub const ERR_ORG_NAME_REQUIRED: &str = "Organization name required";
pub const ERR_ORG_NAME_TOO_LONG: &str = "Organization name too long";
pub const ERR_ORG_ALREADY_EXISTS: &str = "Organization already exists";
pub const ERR_ORG_DOES_NOT_EXIST: &str = "Organization does not exist";
pub const ERR_AGENT_ALREADY_EXISTS: &str = "Agent already exists";
pub const ERR_SENDER_IS_NOT_AN_AGENT: &str = "Sender must be a known organization agent";
pub const ERR_SENDER_MUST_BE_ORG_AGENT: &str = "Sender must be agent of the specified organization";
pub const ERR_SENDER_MUST_BE_ORG_ADMIN: &str = "Sender must be organization admin";
pub const ERR_SENDER_MUST_BE_ACTIVE_ADMIN: &str = "Sender must be an active organization admin";

pub const BYTEARRAY_LIMIT: usize = 100;
pub const ROLE_ADMIN: &[u8; 5] = b"admin";

// This could be a DID
pub type OrgId = Vec<u8>;
pub type OrgName = Vec<u8>;
pub type Role = Vec<u8>;

pub fn validate_org_id(id: &[u8]) -> Result {
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
    pub name: OrgName,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Agent<AccountId> {
    pub org_id: OrgId,
    pub account: AccountId,
    pub active: bool,
    pub role_ids: Vec<u32>,
}

#[derive(Default)]
pub struct OrganizationBuilder {
    id: OrgId,
    name: OrgName,
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
    role_ids: Vec<u32>,
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

    pub fn build(self) -> rstd::result::Result<Agent<AccountId>, &'static str> {
        validate_org_id(&self.org_id)?;
        let mut agent = Agent::<AccountId>::default();
        agent.org_id = self.org_id;
        agent.account = self.account;
        agent.active = self.active;
        agent.role_ids = self.role_ids;
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

        Roles get(role_by_index): map u32 => Role;
        RolesCount get(roles_count): u32;
        RolesIndex get(role_index): map Role => u32;
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

            let mut agent = AgentBuilder::<T::AccountId>::default()
                .with_org(id.clone())
                .with_account(sender.clone())
                .is_active(true)
                .build()?;
            Self::validate_new_agent(&sender)?;

            // Do this after all valitadions cause we're potentially mutating state
            let admin_role_id = Self::get_or_add_role_id(ROLE_ADMIN.to_vec())?;
            agent.role_ids = vec![admin_role_id];

			<Organizations<T>>::insert(&id, org);
            <Agents<T>>::insert(&sender, agent);

            Self::deposit_event(RawEvent::OrganizationCreated(id.clone(), name));
            Self::deposit_event(RawEvent::AgentCreated(id, sender));

            Ok(())
        }

        pub fn create_agent(
            origin, org_id: OrgId, account: T::AccountId,
            active: bool, roles: Vec<Role>) -> Result {
            let sender = ensure_signed(origin)?;

            let mut agent = AgentBuilder::<T::AccountId>::default()
                .with_org(org_id.clone())
                .with_account(account.clone())
                .is_active(active)
                .build()?;
            Self::validate_new_agent(&account)?;
			Self::validate_existing_org(&org_id)?;

            // verify the signer of the transaction is authorized to create agent
            Self::validate_is_org_active_agent(&sender, org_id.clone())?;
            Self::validate_is_agent_admin(&sender)?;

            // Do this after all valitadions cause we're potentially mutating state
            agent.role_ids = Self::get_or_add_roles(roles)?;

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
        match Self::validate_is_org_active_agent(account, org_id.clone()) {
            Ok(_) => match Self::validate_is_agent_admin(account) {
                Ok(_) => true,
                Err(_) => false
            }
            Err(_) => false
        }
    }

    // Helpers
    pub fn validate_new_org(id: &[u8]) -> Result {
        ensure!(
            !<Organizations<T>>::exists::<Vec<u8>>(id.into()),
            ERR_ORG_ALREADY_EXISTS
        );
        Ok(())
    }

	pub fn validate_existing_org(id: &[u8]) -> Result {
		ensure!(<Organizations<T>>::exists::<Vec<u8>>(id.into()), ERR_ORG_DOES_NOT_EXIST);
		Ok(())
	}

    pub fn validate_new_agent(agent: &T::AccountId) -> Result {
        ensure!(!<Agents<T>>::exists(agent), ERR_AGENT_ALREADY_EXISTS);
        Ok(())
    }

    pub fn validate_is_org_active_agent(account: &T::AccountId, org_id: OrgId) -> Result {
		match <Agents<T>>::get(account) {
            Some(agent) => {
				if agent.org_id != org_id {
					fail!(ERR_SENDER_MUST_BE_ORG_AGENT);
				}
				if !agent.active {
					fail!(ERR_SENDER_MUST_BE_ACTIVE_ADMIN);
				}
				Ok(())
            },
            None => fail!(ERR_SENDER_IS_NOT_AN_AGENT)
        }
    }

    pub fn validate_is_agent_admin(account: &T::AccountId) -> Result {
        let admin_role_id = <RolesIndex<T>>::get(ROLE_ADMIN.to_vec());
		match <Agents<T>>::get(account) {
            Some(agent) => {
				if !agent.role_ids.contains(&admin_role_id) {
					fail!(ERR_SENDER_MUST_BE_ORG_ADMIN);
				}
				Ok(())
            },
            None => fail!(ERR_SENDER_IS_NOT_AN_AGENT)
        }
    }

    // PRIVATE MUTABLES
    fn get_or_add_roles(roles: Vec<Role>) -> rstd::result::Result<Vec<u32>, &'static str> {
        let mut role_ids: Vec<u32> = vec!();
        for role in roles {
            match Self::get_or_add_role_id(role) {
                Ok(role_id) => role_ids.push(role_id),
                Err(err) => fail!(err)
            }
        }
        Ok(role_ids)
    }

    fn get_or_add_role_id(role: Role) -> rstd::result::Result<u32, &'static str> {
        match <RolesIndex<T>>::exists(&role) {
            true => Ok(<RolesIndex<T>>::get(&role)),
            false => {
                let roles_count = Self::roles_count();
                let new_role_idx = roles_count.checked_add(1)
                    .ok_or("Overflow adding a new role")?;
                
                <Roles<T>>::insert(new_role_idx, role.clone());
                <RolesCount<T>>::put(new_role_idx);
                <RolesIndex<T>>::insert(role, new_role_idx);

                Ok(new_role_idx)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_noop, assert_ok, impl_outer_origin};

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
        let t = system::GenesisConfig::<GridPikeTest>::default()
            .build_storage()
            .unwrap()
            .0;
        //FIXME: doesn't work, see add_extra_genesis above
        // t.extend(GenesisConfig::<GridPikeTest> {
        // 	orgs: vec![ (String::from(TEST_EXISTING_ORG).into_bytes(), String::from(TEST_EXISTING_ORG).into_bytes()) ],
        // }.build_storage().unwrap().0);
        t.into()
    }

    const TEST_ORG_ID: &str = "did:example:123456789abcdefghijk";
    const TEST_ORG_NAME: &str = "Parity Tech";
    const TEST_EXISTING_ORG: &str = "did:example:azertyuiop";
    const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

	pub fn store_test_org(id: OrgId, name: OrgName) {
		Organizations::<GridPikeTest>::insert(
			id.clone(),
			Organization {
				id: id,
				name: name,
			},
		);
	}

	pub fn store_test_agent(
		account: u64, org_id: OrgId,
		active: bool, role_ids: Vec<u32>) {
		Agents::<GridPikeTest>::insert(
			&account,
			Agent {
				org_id: org_id,
				account: account,
				active: active,
				role_ids: role_ids,
			},
		);
	}

    pub fn store_admin_role() -> u32 {
        let admin_role = ROLE_ADMIN.to_vec();

        let roles_count = <RolesCount<GridPikeTest>>::get();
        let new_role_idx = roles_count.checked_add(1)
            .expect("Overflow adding a new role");
        
        <Roles<GridPikeTest>>::insert(new_role_idx, admin_role.clone());
        <RolesCount<GridPikeTest>>::put(new_role_idx);
        <RolesIndex<GridPikeTest>>::insert(admin_role, new_role_idx);

        new_role_idx
    }

    // create_org tests
    #[test]
    fn create_org_with_valid_args() {
        with_externalities(&mut build_ext(), || {
            // Arrange
            const ADMIN_ROLE_ID: u32 = 1;
            let sender = 1;
            let id = String::from(TEST_ORG_ID).into_bytes();
            let name = String::from(TEST_ORG_NAME).into_bytes();

            // Act
            let result = GridPike::create_org(Origin::signed(sender), id.clone(), name.clone());

            // Assert
            assert_ok!(result);

            assert_eq!(
                GridPike::org_by_id(&id),
                Some(Organization {
                    id: id.clone(),
                    name: name
                })
            );

            assert_eq!(
                Roles::<GridPikeTest>::get(1),
                ROLE_ADMIN.to_vec()
            );

            assert_eq!(
                RolesCount::<GridPikeTest>::get(),
                1
            );

            assert_eq!(
                RolesIndex::<GridPikeTest>::get(ROLE_ADMIN.to_vec()),
                1
            );

            assert_eq!(
                GridPike::agent_by_account(&sender),
                Some(Agent {
                    org_id: id.clone(),
                    account: sender,
                    active: true,
                    role_ids: vec![ADMIN_ROLE_ID]
                })
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
                    String::from(TEST_ORG_NAME).into_bytes()
                ),
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
                    String::from(TEST_ORG_NAME).into_bytes()
                ),
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
                    vec!()
                ),
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
                    String::from(LONG_VALUE).into_bytes()
                ),
                ERR_ORG_NAME_TOO_LONG
            );
        })
    }

    #[test]
    fn create_org_with_existing_id() {
        with_externalities(&mut build_ext(), || {
            let existing_org = String::from(TEST_EXISTING_ORG).into_bytes();
            let name = String::from(TEST_ORG_NAME).into_bytes();

			store_test_org(existing_org.clone(), name.clone());

            assert_noop!(
                GridPike::create_org(Origin::signed(1), existing_org, name),
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

			store_test_agent(
				sender.clone(), String::from(TEST_EXISTING_ORG).into_bytes(), true, vec!());

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
            const ROLE_OPERATOR: &[u8] = b"operator";
            let admin = 1;
            let agent = 2;
            let id = String::from(TEST_ORG_ID).into_bytes();

            // Create org & admin agent
            let _admin_role_id = store_admin_role();
			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(admin.clone(), id.clone(), true, vec![1]);

            // Send tx to create non-admin agent for org
            let result =
                GridPike::create_agent(Origin::signed(admin), id.clone(), agent, true, vec![ROLE_OPERATOR.to_vec()]);

            assert_ok!(result);

            assert_eq!(
                GridPike::agent_by_account(&agent),
                Some(Agent {
                    org_id: id.clone(),
                    account: agent,
                    active: true,
                    role_ids: vec!(2)
                })
            );

            assert_eq!(
                Roles::<GridPikeTest>::get(2),
                ROLE_OPERATOR.to_vec()
            );

            assert_eq!(
                RolesCount::<GridPikeTest>::get(),
                2
            );

            assert_eq!(
                RolesIndex::<GridPikeTest>::get(ROLE_OPERATOR.to_vec()),
                2
            );
        })
    }

    #[test]
    fn create_agent_with_missing_org_id() {
        with_externalities(&mut build_ext(), || {
            assert_noop!(
                GridPike::create_agent(Origin::signed(1), vec!(), 2, true, vec!()),
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
                    2,
                    true,
                    vec!()
                ),
                ERR_ORG_ID_TOO_LONG
            );
        })
    }

	#[test]
	fn create_agent_with_unknown_sender() {
        with_externalities(&mut build_ext(), || {
			let id = String::from(TEST_ORG_ID).into_bytes();

			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());

            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(1),
                    id,
                    2,
                    true,
                    vec!()
                ),
                ERR_SENDER_IS_NOT_AN_AGENT
            );
        })
    }

	#[test]
    fn create_agent_with_unknown_org() {
        with_externalities(&mut build_ext(), || {
            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(1),
                    String::from(TEST_ORG_ID).into_bytes(),
                    2,
                    true,
                    vec!()
                ),
                ERR_ORG_DOES_NOT_EXIST
            );
        })
    }

    #[test]
    fn create_agent_with_existing_account() {
        with_externalities(&mut build_ext(), || {
            let agent = 2;
            let id = String::from(TEST_ORG_ID).into_bytes();
			
			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(agent.clone(), id.clone(), true, vec!());

            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(1),
                    id,
                    agent,
                    true,
                    vec!()
                ),
                ERR_AGENT_ALREADY_EXISTS
            );
        })
    }

	#[test]
	fn create_agent_with_invalid_sender() {
		with_externalities(&mut build_ext(), || {
			let admin = 1;
			let agent = 2;
            let id = String::from(TEST_ORG_ID).into_bytes();
            let other_org = String::from(TEST_EXISTING_ORG).into_bytes();
			let org_name = String::from(TEST_ORG_NAME).into_bytes();

			store_test_org(id.clone(), org_name.clone());
			store_test_org(other_org.clone(), org_name.clone());
			store_test_agent(admin.clone(), other_org.clone(), true, vec!());

            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(admin),
                    id,
                    agent,
                    true,
                    vec!()
                ),
                ERR_SENDER_MUST_BE_ORG_AGENT
            );
		})
	}

	#[test]
    fn create_agent_with_non_admin_account() {
        with_externalities(&mut build_ext(), || {
			let non_admin = 1;
			let agent = 2;
            let id = String::from(TEST_ORG_ID).into_bytes();

			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(non_admin.clone(), id.clone(), true, vec!());

            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(non_admin.clone()),
                    id,
                    agent,
                    true,
                    vec!()
                ),
                ERR_SENDER_MUST_BE_ORG_ADMIN
            );
        })
    }

	#[test]
    fn create_agent_with_inactive_admin_account() {
        with_externalities(&mut build_ext(), || {
			let admin = 1;
			let agent = 2;
            let id = String::from(TEST_ORG_ID).into_bytes();

            let admin_role_id = store_admin_role();
			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(admin.clone(), id.clone(), false, vec![admin_role_id]);

            assert_noop!(
                GridPike::create_agent(
                    Origin::signed(admin.clone()),
                    id,
                    agent,
                    true,
                    vec!()
                ),
                ERR_SENDER_MUST_BE_ACTIVE_ADMIN
            );
        })
    }

    // is_admin tests
    #[test]
    fn is_admin_for_actual_org_admin() {
        with_externalities(&mut build_ext(), || {
            let agent = 1;
            let org_id = String::from(TEST_ORG_ID).into_bytes();

			// Insert test data directly into storage to test public immutable func
            let admin_role_id = store_admin_role();
			store_test_org(org_id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(agent, org_id.clone(), true, vec![admin_role_id]);

            // Test is_admin method
            assert_eq!(GridPike::is_admin(&agent, org_id), true);
        })
    }

    #[test]
    fn is_admin_for_unknown_agent() {
        with_externalities(&mut build_ext(), || {
            let agent = 1;
            let org_id = String::from(TEST_ORG_ID).into_bytes();

            assert_eq!(GridPike::is_admin(&agent, org_id), false);
        })
    }

    #[test]
    fn is_admin_for_inactive_org_agent() {
        with_externalities(&mut build_ext(), || {
            let agent = 1;
            let org_id = String::from(TEST_ORG_ID).into_bytes();

			// Insert test data directly into storage to test public immutable func
            let admin_role_id = store_admin_role();
			store_test_org(org_id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(agent, org_id.clone(), false, vec![admin_role_id]); // <-- /!\ Agent is inactive

            assert_eq!(GridPike::is_admin(&agent, org_id), false);
        })
    }

    #[test]
    fn is_admin_for_non_admin_org_agent() {
        with_externalities(&mut build_ext(), || {
            let agent = 1;
            let org_id = String::from(TEST_ORG_ID).into_bytes();

			// Insert test data directly into storage to test public immutable func
			store_test_org(org_id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(agent, org_id.clone(), true, vec!()); // <-- /!\ Agent is not admin

            assert_eq!(GridPike::is_admin(&agent, org_id), false);
        })
    }

    #[test]
    fn is_admin_for_agent_org_mismatch() {
        with_externalities(&mut build_ext(), || {
            let agent = 1;
            let id = String::from(TEST_ORG_ID).into_bytes();
            let other_org = String::from(TEST_EXISTING_ORG).into_bytes();

			// Insert test data directly into storage to test public immutable func
            let admin_role_id = store_admin_role();
			store_test_org(id.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(agent, id.clone(), true, vec![admin_role_id]);

            assert_eq!(
                GridPike::is_admin(&agent, other_org), // <-- /!\ Agent is admin for another org
                false
            );
        })
    }
}
