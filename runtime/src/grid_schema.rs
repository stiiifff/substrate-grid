// Copyright 2019 Steve Degosserie	
// Hyperledger Grid Schema compatible runtime module

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

pub type SchemaName = Vec<u8>;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Schema {
	pub name: SchemaName
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GridSchema {
		Schemas get(name): map SchemaName => Option<Schema>;
	}
}

decl_event!(
	pub enum Event<T>
	where <T as system::Trait>::AccountId
	{
		SchemaCreated(SchemaName),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_schema(origin, id: OrgId, name: OrgName) -> Result {
		}
	}
}

impl<T: Trait> Module<T> {
}
