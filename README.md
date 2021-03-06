# Substrate Grid &emsp; [![build]][codeship] [![rustc]][rustc_nightly] [![license]][license_mit]

[build]: https://app.codeship.com/projects/2663ec20-7322-0137-6fec-5af050f70adb/status?branch=master
[codeship]: https://app.codeship.com/projects/348677

[rustc]: https://img.shields.io/badge/rustc-1.35+-lightgray.svg
[rustc_nightly]: https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html

[license]: https://img.shields.io/badge/license-MIT-blue.svg
[license_mit]: https://github.com/stiiifff/substrate-grid/blob/master/MIT_LICENSE

**Substrate Grid** is an exploration into re-implementing parts of the [Hyperledger Grid](https://grid.hyperledger.org/about/) project onto the [Parity Substrate](https://www.parity.io/substrate/) technology.

> Hyperledger Grid is a platform for building supply chain solutions that include distributed ledger components. The project provides a set of modular components for developing smart contracts and client interfaces, including domain-specific data models (such as GS1 product definitions), smart-contract business logic, libraries, and SDKs.

**Substrate Grid** differs (for now) from the original implementation on the following points:
* It strives to use native Substrate data formats & libraries as much as possible (e.g. **Parity Codec vs Google Protobuf**).
* The **Pike**, **Schema** and **Track&Trace** contracts are re-implemented as [**Substrate Runtime Modules**](https://substrate.dev/docs/en/runtime/substrate-runtime-module-library), instead of **Wasm contracts**.
* The Substrate WASM runtime is leveraged as-is, no attempt is made (for now) to re-implement [Hyperledger Sawtooth](https://github.com/hyperledger/sawtooth-core) and [Sawtooth Sabre](https://github.com/hyperledger/sawtooth-sabre) on top of Substrate (if that makes any sense ..).
* The original implementation tends to store a lot of **String** data on-chain. While this is a design decision that can be debatted, and is generally frowned upon, **Substrate Grid** retains *some* of those to remain somewhat faithfull to the original design, but otherwise offload most of the data off-chain.

# Todos / Improvements

* Pike: implement update_org & update_agent transaction types.
* Schema: implement missing validations & permission checks, and schema update.
* Product and Track&Trace contracts: todo, not started yet.
* Storage: Avoid using Vec<u8> as key in maps, used u32 indexed lists instead.
* Storage: Offload unnecessary on-chain storage to either DID claims or decentralized storage (e.g. IPFS).
* Substrate: Integration with existing governance-related Sustrate runtime modules into Grid Pike's features (mgmt of the consortium's organizations & agents).
* Sawtooth: Investigate full port of Sawtooth / Sabre as alternative runtime storage, so that the Hyperledger Grid contracts can run as-is, avoiding a re-implementation (and tedious work for keeping in sync).

# Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build the WebAssembly binary:

```bash
./scripts/build.sh
```

Build all native code:

```bash
cargo build
```

# Test

Run tests for the Pike & Schema modules:

```bash
cargo test -p grid-runtime grid_pike
cargo test -p grid-runtime grid_schema
```

# Run

Note: this project is based on the [Substrate Node template](https://github.com/paritytech/substrate/tree/master/node-template).

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units. Give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet). You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmQZ8TjTqeDj3ciwr93EJ95hxfDsb9pEYDizUAbWpigtQN`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmQZ8TjTqeDj3ciwr93EJ95hxfDsb9pEYDizUAbWpigtQN \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

# Test with Substrate UI

Once a local node is running, open the [Substrate UI](https://substrate-ui.parity.io) in your browser.

Then, in the **Settings** section:
* **Developer** tab: Paste the content of the [type_defs.json](type_defs.json) file in the text area, and **Save**. This will ensure **Substrate UI** is able to recognize the custom types of our custom **Substrate Runtime modules**.
* **General** tab: select the Local Node in the list.

If configured correctly, the **Substrate UI** should display several UI components in the sidebar such as **Explorer**, **Accounts**, **Address book** among others.

Creation of an organization & agent in the **Extrinsics** tab:

* Create an organization
  * Select the **gridPike** module and **createOrg** function
  * Specify an organization ID and name in hexadecimal formats (e.g. 0x67726964 for 'grid' in ASCII characters).
  * Click on the **Submit Transaction** button.
  * If all goes well, the transaction is mined in a block, and the chain state is mutated.
* Create an admin agent
  * Select the **gridPike** module and **createAgent** function
  * Specify the ID of the previously created organization (in hexa).
  * Choose an account in the list e.g. ALICE in the dev chain.
  * Choose whether the agent should be active or not.
  * Optionally assign one or more role(s) to the agent (role name in hexa).
  * Click on the **Submit Transaction** button.
  * If all goes well, the transaction is mined in a block, and the chain state is mutated.
* Verify that the organization & agent were stored on-chain, using the **Chain state** tab
  * Select the **gridPike** module, then call the **agents** and **organizations** immutable functions (and optionnally the role-related ones) to verify that the organization & agent were indeed recorded.

## License

The `Substrate Node template` is is free and unencumbered software released into the public domain. Please read the [UNLICENSE](UNLICENSE) file in this repository for more information.

The `Substrate Grid runtime` is licensed under the MIT license. Please read the [MIT_LICENSE](MIT_LICENSE) file in this repository for more information.
