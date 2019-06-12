//!	# Substrate Grid Pike Module

//! // Add heading with custom module name
//! 
//! # <INSERT_CUSTOM_MODULE_NAME> Module
//! 
//! 
//! // Add simple description
//! 
//! 
//! // Include the following links that shows what trait needs to be implemented to use the module
//! // and the supported dispatchables that are documented in the Call enum.
//! 
//! 
//! 
//! [<INSERT_CUSTOM_MODULE_NAME>::Trait](./trait.Trait.html)
//! 
//! [Call](./enum.Call.html)
//! 
//! [Module](./struct.Module.html)
//! 
//! 
//! ## Overview
//! 
//! 
//!  
//! // Short description of module purpose.
//! // Links to Traits that should be implemented.
//! // What this module is for.
//! // What functionality the module provides.
//! // When to use the module (use case examples).
//! // How it is used.
//! // Inputs it uses and the source of each input.
//! // Outputs it produces.
//! 
//! 
//! ## Terminology
//! 
//! 
//! // Add terminology used in the custom module. Include concepts, storage items, or actions that you think
//! // deserve to be noted to give context to the rest of the documentation or module usage. The author needs to
//! // use some judgment about what is included. We don't want a list of every storage item nor types - the user
//! // can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
//! // "free balance" and "reserved balance" should be noted to give context to the module.
//! // Please do not link to outside resources. The reference docs should be the ultimate source of truth.
//! 
//! 
//! 
//! ## Goals
//! 
//! 
//! // Add goals that the custom module is designed to achieve.
//! 
//! 
//! 
//! ### Scenarios
//! 
//! 
//! 
//! #### <INSERT_SCENARIO_NAME>
//! 
//! 
//! // Describe requirements prior to interacting with the custom module.
//! // Describe the process of interacting with the custom module for this scenario and public API functions used.
//! 
//! 
//! ## Interface
//! 
//! 
//! ### Supported Origins
//! 
//! 
//! // What origins are used and supported in this module (root, signed, none)
//! // i.e. root when `ensure_root` used
//! // i.e. none when `ensure_none` used
//! // i.e. signed when `ensure_signed` used
//! 
//! 
//! `inherent` <INSERT_DESCRIPTION>
//! 
//! 
//! 
//! 
//! ### Types
//! 
//! 
//! // Type aliases. Include any associated types and where the user would typically define them.
//! 
//! 
//! `ExampleType` <INSERT_DESCRIPTION>
//! 
//! 
//! 
//! // Reference documentation of aspects such as storageItems and dispatchable functions should only be
//! // included in the https://docs.rs Rustdocs for Substrate and not repeated in the README file.
//! 
//! 
//! ### Dispatchable Functions
//! 
//! 
//! 
//! // A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.
//! 
//! 
//! // MUST have link to Call enum
//! // MUST have origin information included in function doc
//! // CAN have more info up to the user
//! 
//! 
//! ### Public Functions
//! 
//! 
//! 
//! // A link to the rustdoc and any notes about usage in the module, not for specific functions.
//! // For example, in the balances module: "Note that when using the publicly exposed functions,
//! // you (the runtime developer) are responsible for implementing any necessary checks
//! // (e.g. that the sender is the signer) before calling a function that will affect storage."
//! 
//! 
//! 
//! // It is up to the writer of the respective module (with respect to how much information to provide).
//! 
//! 
//! #### Public Inspection functions - Immutable (getters)
//! 
//! 
//! // Insert a subheading for each getter function signature
//! 
//! 
//! ##### `example_getter_name()`
//! 
//! 
//! // What it returns
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//! 
//! 
//! #### Public Mutable functions (changing state)
//! 
//! 
//! // Insert a subheading for each setter function signature
//! 
//! 
//! ##### `example_setter_name(origin, parameter_name: T::ExampleType)`
//! 
//! 
//! // What state it changes
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//! // What parameter values are valid and why
//! 
//! 
//! ### Storage Items
//! 
//! 
//! // Explain any storage items included in this module
//! 
//! 
//! ### Digest Items
//! 
//! 
//! // Explain any digest items included in this module
//! 
//! 
//! ### Inherent Data
//! 
//! 
//! // Explain what inherent data (if any) is defined in the module and any other related types
//! 
//! 
//! ### Events:
//! 
//! 
//! // Insert events for this module if any
//! 
//! 
//! ### Errors:
//! 
//! 
//! // Explain what generates errors
//! 
//! 
//! ## Usage
//! 
//! 
//! // Insert 2-3 examples of usage and code snippets that show how to use <INSERT_CUSTOM_MODULE_NAME> module in a custom module.
//! 
//! 
//! ### Prerequisites
//! 
//! 
//! // Show how to include necessary imports for <INSERT_CUSTOM_MODULE_NAME> and derive
//! // your module configuration trait with the INSERT_CUSTOM_MODULE_NAME trait.
//! 
//! 
//! ```rust
//! use <INSERT_CUSTOM_MODULE_NAME>;
//! 
//! 
//! pub trait Trait: <INSERT_CUSTOM_MODULE_NAME>::Trait { }
//! ```
//! 
//! 
//! ### Simple Code Snippet
//! 
//! 
//! // Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_MODULE_NAME>)
//! 
//! 
//! ### Example from SRML
//! 
//! 
//! // Show a usage example in an actual runtime
//! 
//! 
//! // See:
//! // - Substrate TCR https://github.com/parity-samples/substrate-tcr
//! // - Substrate Kitties https://shawntabrizi.github.io/substrate-collectables-workshop/#/
//! 
//! 
//! ## Genesis Config
//! 
//! 
//! 
//! ## Dependencies
//! 
//! 
//! // Dependencies on other SRML modules and the genesis config should be mentioned,
//! // but not the Rust Standard Library.
//! // Genesis configuration modifications that may be made to incorporate this module
//! // Interaction with other modules
//! 
//! 
//! 
//! ## Related Modules
//! 
//! 
//! // Interaction with other modules in the form of a bullet point list
//! 
//! 
//! ## References
//! 
//! 
//! 
//! // Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
//! // that the implementation is based on.
//! 


#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

#[cfg(feature = "std")]
use serde_derive::{Serialize, Deserialize};
use parity_codec::{Encode, Decode};
use rstd::prelude::*;
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use runtime_primitives::{
	ApplyResult, transaction_validity::TransactionValidity, generic, create_runtime_str,
	traits::{self, NumberFor, BlakeTwo256, Block as BlockT, StaticLookup, Verify}
};
use client::{
	block_builder::api::{CheckInherentsResult, InherentData, self as block_builder_api},
	runtime_api, impl_runtime_apis
};
use version::RuntimeVersion;
#[cfg(feature = "std")]
use version::NativeVersion;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use consensus::Call as ConsensusCall;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use runtime_primitives::{Permill, Perbill};
pub use timestamp::BlockPeriod;
pub use support::{StorageValue, construct_runtime};

/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;

/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Index of a block number in the chain.
pub type BlockNumber = u64;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

/// Used for the module template in `./template.rs`
// mod template;
mod grid_pike;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	/// Opaque, encoded, unchecked extrinsic.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);
	#[cfg(feature = "std")]
	impl std::fmt::Debug for UncheckedExtrinsic {
		fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(fmt, "{}", primitives::hexdisplay::HexDisplay::from(&self.0))
		}
	}
	impl traits::Extrinsic for UncheckedExtrinsic {
		fn is_signed(&self) -> Option<bool> {
			None
		}
	}
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, AuthorityId, AuthoritySignature>>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque session key type.
	pub type SessionKey = AuthorityId;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("eternitrace"),
	impl_name: create_runtime_str!("eternitrace"),
	authoring_version: 3,
	spec_version: 3,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = Indices;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Nonce;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header digest type.
	type Digest = generic::Digest<Log>;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous log type.
	type Log = Log;
	/// The ubiquitous origin type.
	type Origin = Origin;
}

impl aura::Trait for Runtime {
	type HandleReport = ();
}

impl consensus::Trait for Runtime {
	/// The identifier we use to refer to authorities.
	type SessionKey = AuthorityId;
	// The aura module handles offline-reports internally
	// rather than using an explicit report system.
	type InherentOfflineReport = ();
	/// The ubiquitous log type.
	type Log = Log;
}

impl indices::Trait for Runtime {
	/// The type for recording indexing into the account enumeration. If this ever overflows, there
	/// will be problems!
	type AccountIndex = u32;
	/// Use the standard means of resolving an index hint from an id.
	type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
	/// Determine whether an account is dead.
	type IsDeadAccount = Balances;
	/// The uniquitous event type.
	type Event = Event;
}

impl timestamp::Trait for Runtime {
	/// A timestamp: seconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// What to do if a new account is created.
	type OnNewAccount = Indices;
	/// The uniquitous event type.
	type Event = Event;

	type TransactionPayment = ();
	type DustRemoval = ();
	type TransferPayment = ();
}

impl sudo::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
	type Proposal = Call;
}

impl grid_pike::Trait for Runtime {
	type Event = Event;
}

construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Indices: indices,
		Balances: balances,
		Sudo: sudo,
		GridPike: grid_pike::{Module, Call, Storage, Event<T>},
	}
);

/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
	impl runtime_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}

		fn authorities() -> Vec<AuthorityId> {
			panic!("Deprecated, please use `AuthoritiesApi`.")
		}
	}

	impl runtime_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl block_builder_api::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl consensus_aura::AuraApi<Block> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}
	}

	impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(n: NumberFor<Block>) {
			Executive::offchain_worker(n)
		}
	}

	impl consensus_authorities::AuthoritiesApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityId> {
			Consensus::authorities()
		}
	}
}
