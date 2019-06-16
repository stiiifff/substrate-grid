// Copyright 2019 Steve Degosserie	
// Hyperledger Grid Schema compatible runtime module

use crate::grid_pike::{OrgId, validate_org_id};
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

const ERR_SCHEMA_NAME_REQUIRED: &str = "Schema name required";
const ERR_SCHEMA_NAME_TOO_LONG: &str = "Schema name too long";

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
	required: bool,
	// description: String,
	// number_exponent: i32,
	// enum_options: Vec<String>,
    // struct_properties: Vec<PropertyDefinition>,
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

pub trait Trait: system::Trait {
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
			<Schemas<T>>::insert(&name, schema);

			Self::deposit_event(Event::SchemaCreated(name, owner));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
}

#[cfg(test)]
mod tests {
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
    // const TEST_ORG_NAME: &str = "Parity Tech";
    // const TEST_EXISTING_ORG: &str = "did:example:azertyuiop";
    const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

	const TYPE_PROP: &[u8] = b"type";
	const WEIGHT_PROP: &[u8] = b"weight";
	const TEMP_PROP: &[u8] = b"temperature";
	const LOCATION_PROP: &[u8] = b"location";

	// fn store_test_org(id: OrgId, name: OrgName) {
	// 	Organizations::<GridPikeTest>::insert(
	// 		id.clone(),
	// 		Organization {
	// 			id: id,
	// 			name: name,
	// 		},
	// 	);
	// }

	// fn store_test_agent(
	// 	account: u64, org_id: OrgId,
	// 	active: bool, roles: Vec<Role>) {
	// 	Agents::<GridPikeTest>::insert(
	// 		&account,
	// 		Agent {
	// 			org_id: org_id,
	// 			account: account,
	// 			active: active,
	// 			roles: roles,
	// 		},
	// 	);
	// }

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
}
