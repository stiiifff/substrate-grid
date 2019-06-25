// Copyright 2019 Steve Degosserie	
// Hyperledger Grid Schema compatible runtime module

use crate::grid_pike::{OrgId, validate_org_id};
use crate::grid_pike::Trait as PikeTrait;
use crate::grid_pike::Module as PikeModule;
use rstd::prelude::*;
use parity_codec::{Decode, Encode};
// use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
// use runtime_primitives::traits::Hash;
use support::{
	decl_module, decl_storage, decl_event,
	ensure, StorageMap,
	dispatch::Result
};
use system::ensure_signed;

const ERR_SCHEMA_NAME_REQUIRED: &str = "Schema name required";
const ERR_SCHEMA_NAME_TOO_LONG: &str = "Schema name too long";
const ERR_SCHEMA_ALREADY_EXISTS: &str = "Schema already exists";

const BYTEARRAY_LIMIT: usize = 100;

pub type Name = Vec<u8>;

fn validate_schema_name(name: &[u8]) -> Result {
	ensure!(name.len() > 0, ERR_SCHEMA_NAME_REQUIRED);
    ensure!(name.len() <= BYTEARRAY_LIMIT, ERR_SCHEMA_NAME_TOO_LONG);
    Ok(())
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Schema {
	pub name: Name,
	pub owner: OrgId,
	pub properties: Vec<PropertyDefinition>
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct PropertyDefinition {
	pub name: Name,
	pub data_type: DataType,
	pub required: bool,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub enum DataType {
    Bytes,
    Boolean,
    Number,
    String,
    Enum,
    Struct,
    LatLong,
}

impl Default for PropertyDefinition {
	fn default() -> Self {
		Self {
			name: vec!(),
			data_type: DataType::Bytes,
			required: false,
			// number_exponent: 0
		} 
	}
}

#[derive(Default)]
pub struct SchemaBuilder {
	name: Name,
	owner: OrgId,
	properties: Vec<PropertyDefinition>
}
impl SchemaBuilder {
	pub fn with_name(mut self, name: Name) -> Self {
		self.name = name;
		self
	}

	pub fn with_owner(mut self, owner: OrgId) -> Self {
		self.owner = owner;
		self
	}

	pub fn with_properties(mut self, properties: Vec<PropertyDefinition>) -> Self {
		self.properties = properties;
		self
	} 

	pub fn build(self) -> rstd::result::Result<Schema, &'static str> {
		validate_schema_name(&self.name)?;
		validate_org_id(&self.owner)?;
		let mut schema = Schema::default();
		schema.name = self.name;
		schema.owner = self.owner;
		schema.properties = self.properties;
		Ok(schema)
	}
}

pub trait Trait: PikeTrait {
	type Event: From<Event /*<Self> */> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GridSchema {
		Schemas get(schema_by_name): map Name => Option<Schema>;
	}
}

decl_event!(
	pub enum Event //<T>
	// where <T as system::Trait>::AccountId
	{
		SchemaCreated(Name, OrgId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event /*<T>*/ () = default;

		pub fn create_schema(
			origin, name: Name, owner: OrgId,
			properties: Vec<PropertyDefinition>) -> Result {
			let sender = ensure_signed(origin)?;

			let schema = SchemaBuilder::default()
				.with_name(name.clone())
				.with_owner(owner.clone())
				.with_properties(properties)
				.build()?;
			Self::validate_new_schema(&name)?;

			// Validate org exists
			<PikeModule<T>>::validate_existing_org(&owner)?;
			// Validate signer is an active agent of the specified org
			<PikeModule<T>>::validate_is_org_active_agent(&sender, owner.clone())?;

			// Check whether signer has the right to create schema
			// TODO: add has_permission fn to grid_pike module (permission = role)
			// check_permission(perm_checker, signer, "can_create_schema")?;

			//TODO: add properties validation (name, data_type & related props)

			<Schemas<T>>::insert(&name, schema);

			Self::deposit_event(Event::SchemaCreated(name, owner));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {

	// Helpers
    fn validate_new_schema(name: &[u8]) -> Result {
        ensure!(!<Schemas<T>>::exists::<Vec<u8>>(name.into()), ERR_SCHEMA_ALREADY_EXISTS);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use crate::grid_pike::{
		ERR_ORG_DOES_NOT_EXIST, ERR_SENDER_IS_NOT_AN_AGENT,
		ERR_SENDER_MUST_BE_ORG_AGENT, ERR_SENDER_MUST_BE_ACTIVE_ADMIN};
	use crate::grid_pike::tests::{store_test_org, store_test_agent, store_admin_role};

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_noop, assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for GridSchemaTest {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct GridSchemaTest;

    impl system::Trait for GridSchemaTest {
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
    impl Trait for GridSchemaTest {
        type Event = ();
    }
	impl PikeTrait for GridSchemaTest {
		type Event = ();
	}

    type GridSchema = super::Module<GridSchemaTest>;

    fn build_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        let t = system::GenesisConfig::<GridSchemaTest>::default()
            .build_storage()
            .unwrap()
            .0;
        t.into()
    }

    const TEST_ORG_ID: &str = "did:example:123456789abcdefghijk";
	const TEST_SCHEMA_NAME: &str = "asset";
    const TEST_ORG_NAME: &str = "Parity Tech";
	const TEST_EXISTING_ORG: &str = "did:example:azertyuiop";
    const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

	const TYPE_PROP: &[u8] = b"type";
	const WEIGHT_PROP: &[u8] = b"weight";
	const TEMP_PROP: &[u8] = b"temperature";
	const LOCATION_PROP: &[u8] = b"location";

	fn store_test_schema(name: Name, owner: OrgId) {
		Schemas::<GridSchemaTest>::insert(
			name.clone(),
			Schema {
				name,
				owner,
				properties: vec!()
			},
		);
	}

    // create_schema tests
    #[test]
    fn create_schema_with_valid_args() {
        with_externalities(&mut build_ext(), || {
            // Arrange
            let sender = 1;
            let schema = String::from(TEST_SCHEMA_NAME).into_bytes();
			let owner = String::from(TEST_ORG_ID).into_bytes();

			let properties = vec![
				PropertyDefinition { name: TYPE_PROP.to_vec(), data_type: DataType::String, required: true },
				PropertyDefinition { name: WEIGHT_PROP.to_vec(), data_type: DataType::Number, required: false },
				PropertyDefinition { name: TEMP_PROP.to_vec(), data_type: DataType::Number, required: false },
				PropertyDefinition { name: LOCATION_PROP.to_vec(), data_type: DataType::LatLong, required: false },
			];

			// Store org & agent
			let admin_role_id = store_admin_role();
			store_test_org(owner.clone(), String::from(TEST_ORG_NAME).into_bytes());
			store_test_agent(sender, owner.clone(), true, vec![admin_role_id]);

            // Act
            let result = GridSchema::create_schema(
				Origin::signed(sender), schema.clone(), owner.clone(), properties.clone());

            // Assert
            assert_ok!(result);

            assert_eq!(
                GridSchema::schema_by_name(&schema),
                Some(Schema {
                    name: schema,
                    owner: owner,
					properties: properties
                })
            );
        })
    }

	#[test]
    fn create_schema_with_missing_name() {
        with_externalities(&mut build_ext(), || {
            assert_noop!(
                GridSchema::create_schema(
					Origin::signed(1), vec!(), String::from(TEST_ORG_ID).into_bytes(), vec!()),
                ERR_SCHEMA_NAME_REQUIRED
            );
        })
    }

    #[test]
    fn create_schema_with_long_name() {
        with_externalities(&mut build_ext(), || {
            assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(1),
					String::from(LONG_VALUE).into_bytes(),
                    String::from(TEST_ORG_ID).into_bytes(),
                    vec!()
                ),
                ERR_SCHEMA_NAME_TOO_LONG
            );
        })
    }

    #[test]
    fn create_schema_with_existing_name() {
        with_externalities(&mut build_ext(), || {
            let existing = String::from(TEST_SCHEMA_NAME).into_bytes();
            let owner = String::from(TEST_ORG_NAME).into_bytes();

			store_test_schema(existing.clone(), owner.clone());

			assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(1),
					existing,
                    owner,
                    vec!()
                ),
                ERR_SCHEMA_ALREADY_EXISTS
            );
        })
    }

	#[test]
    fn create_schema_with_unknown_org() {
        with_externalities(&mut build_ext(), || {
			let sender = 1;
            let schema = String::from(TEST_SCHEMA_NAME).into_bytes();
            let owner = String::from(TEST_ORG_NAME).into_bytes();

			assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(sender),
					schema,
                    owner,
                    vec!()
                ),
                ERR_ORG_DOES_NOT_EXIST
            );
        })
    }

	#[test]
    fn create_schema_with_unknown_sender() {
        with_externalities(&mut build_ext(), || {
			let sender = 1;
            let schema = String::from(TEST_SCHEMA_NAME).into_bytes();
            let owner = String::from(TEST_ORG_NAME).into_bytes();

			let _admin_role_id = store_admin_role();
			store_test_org(owner.clone(), String::from(TEST_ORG_NAME).into_bytes());

			assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(sender),
					schema,
                    owner,
                    vec!()
                ),
                ERR_SENDER_IS_NOT_AN_AGENT
            );
        })
    }

	#[test]
    fn create_schema_with_invalid_sender() {
        with_externalities(&mut build_ext(), || {
			let admin = 1;
            let owner = String::from(TEST_ORG_ID).into_bytes();
            let other_org = String::from(TEST_EXISTING_ORG).into_bytes();
			let org_name = String::from(TEST_ORG_NAME).into_bytes();
			let schema = String::from(TEST_SCHEMA_NAME).into_bytes();

			store_test_org(owner.clone(), org_name.clone());
			store_test_org(other_org.clone(), org_name.clone());
			store_test_agent(admin.clone(), other_org.clone(), true, vec!());

			assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(admin),
					schema,
                    owner,
                    vec!()
                ),
                ERR_SENDER_MUST_BE_ORG_AGENT
            );
        })
    }

	#[test]
    fn create_schema_with_inactive_agent() {
        with_externalities(&mut build_ext(), || {
			let admin = 1;
            let owner = String::from(TEST_ORG_ID).into_bytes();
			let org_name = String::from(TEST_ORG_NAME).into_bytes();
			let schema = String::from(TEST_SCHEMA_NAME).into_bytes();

			store_test_org(owner.clone(), org_name.clone());
			store_test_agent(admin.clone(), owner.clone(), false, vec!());

			assert_noop!(
                GridSchema::create_schema(
                    Origin::signed(admin),
					schema,
                    owner,
                    vec!()
                ),
                ERR_SENDER_MUST_BE_ACTIVE_ADMIN
            );
        })
    }
}
