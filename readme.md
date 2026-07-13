# Rust for Bitcoin Core JSON-RPC CLI (`rfb-cli`)

A modular, asynchronously architected command-line interface in Rust to interact with a Bitcoin Core node. This tool is built as part of the **Rust for Bitcoin Program 2.0 Technical Assessment**.

---

## Features

- **Asynchronous Architecture**: Built using `tokio` and `reqwest` for non-blocking HTTP requests.
- **Robust Configuration**: Supports multiple configuration vectors (CLI options, environment variables, or a `.env` file) with clean priority resolution.
- **Strongly-Typed & Custom RPC Client**: Deserializes core Bitcoin Core RPC outputs into structured Rust structs while maintaining a reusable client abstraction.
- **Dynamic Parameter Coercion**: A custom parameter parser (matching `bitcoin-cli` behavior) automatically detects and coerces CLI strings into JSON numbers, booleans, arrays, or objects before sending the JSON-RPC request.
- **Graceful Error Handling**: Elegant handling of authorization failures, connection timeouts, missing wallets, or invalid parameters, avoiding any runtime panics.
- **Visual Design**: Uses color-coded stdout logs (using the `colored` crate) to output data.

---

## Project Structure

The project has been organized into a modular design that isolates the CLI parsing, configuration resolution, network communication, and subcommand logic:

```text
rfb-assessment/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point and subcommand dispatcher
│   ├── cli.rs           # CLI structure and subcommands using clap
│   ├── config.rs        # Configuration priority and resolution
│   ├── rpc.rs           # Reusable async BitcoinRpcClient
│   ├── error.rs         # Structured error mapping using thiserror
│   └── commands/        # Subcommand implementations
│       ├── mod.rs       # Command runners and dynamic parameter parsing
│       ├── blockchain.rs# blockchain-info subcommand
│       ├── wallet.rs    # wallet-info and balance subcommands
│       └── address.rs   # new-address subcommand
└── .env.example         # Example configuration file
```

---

## Setup & Node Installation

This CLI connects to any Bitcoin Core node. The instructions below describe setting up a local Regtest node using **Polar**.

### 1. Installing Polar
1. Download Polar from the official website: [lightninglabs.github.io/polar/](https://lightninglabs.github.io/polar/).
2. Install and launch the application. Ensure Docker Desktop is running in the background as Polar utilizes Docker containers to provision nodes.

### 2. Creating a Bitcoin Core Node
1. Inside Polar, click **Create a Network**.
2. Give your network a name (e.g., `Regtest Local`) and add **1 Bitcoin Core node** (e.g., version `24.0` or higher). You can set LND or other nodes to `0` for this assessment.
3. Click **Create Network**.

### 3. Starting the Network
1. Click **Start** in the top right corner of the Polar dashboard.
2. Wait a few seconds for the status indicators to turn green.

### 4. Obtaining RPC Credentials
1. Click on your active Bitcoin node (e.g., `backend1`).
2. Go to the **Connect** tab in the sidebar.
3. Copy the following connection properties:
   - **RPC URL** (e.g., `http://127.0.0.1:18443`)
   - **Username** (e.g., `polaruser`)
   - **Password** (e.g., `polarpass`)

---

## Configuration

You can configure the application in three ways, listed in order of priority:

1. **CLI Flags**: Pass credentials directly during invocation.
   ```bash
   cargo run -- --rpc-url http://127.0.0.1:18443 --rpc-user polaruser --rpc-pass polarpass blockchain-info
   ```
2. **Environment Variables**: Set temporary or permanent system environment variables:
   - `RPC_URL`
   - `RPC_USER`
   - `RPC_PASS`
   - `RPC_WALLET`
3. **Configuration File (`.env`)**: Create a file named `.env` in the root folder of the project:
   ```env
   RPC_URL=http://127.0.0.1:18443
   RPC_USER=alice
   RPC_PASS=password
   RPC_WALLET=Miner
   ```

---

## Usage & Command Examples

First, compile the application using Cargo:
```bash
cargo build
```

Below are example outputs for each of the implemented commands.

### 1. `blockchain-info`
Retrieves and displays chain metrics.
```bash
cargo run -- blockchain-info
```
**Example Output:**
```text
Fetching blockchain information...

=== Blockchain Info ===
Chain                    : regtest
Blocks                   : 101
Headers                  : 101
Difficulty               : 0.00000000
Verification Progress    : 100.0000%
=======================
```

---

### 2. `wallet-info`
Displays details of the loaded wallet. If no wallet is specified or loaded, it prints a clean error.
```bash
cargo run -- --wallet Miner wallet-info
```
**Example Output:**
```text
Fetching wallet information...

=== Wallet Info ===
Wallet Name              : Miner
Balance                  : 50.00000000 BTC
Unconfirmed Balance      : 0.00000000 BTC
Transaction Count        : 410
===================
```

---

### 3. `balance`
Prints the active wallet's balance directly.
```bash
cargo run -- --wallet Miner balance
```
**Example Output:**
```text
Balance: 50.00000000 BTC
```

---

### 4. `new-address`
Generates a new receiving address (default: Bech32 format).
```bash
cargo run -- --wallet Miner new-address
```
**Example Output:**
```text
Generating a new receiving address...
New Address: bcrt1q5qamaunh97wxl4l3nm34a7hgct4vsz9z3sljft
```

---

### 5. `rpc` (Generic JSON-RPC Command)
Runs any arbitrary Bitcoin Core RPC command. The CLI automatically coerces arguments to their correct type (numbers, booleans, arrays, objects, strings).

#### Fetch block count (No parameters):
```bash
cargo run -- rpc getblockcount
```
**Example Output:**
```text
Executing RPC: getblockcount with 0 parameter(s)...

=== RPC Response ===
101
====================
```

#### Fetch block hash (Numeric parameter coerced):
```bash
cargo run -- rpc getblockhash 100
```
**Example Output:**
```text
Executing RPC: getblockhash with 1 parameter(s)...

=== RPC Response ===
"15434ac5b0e0f5d4b420c3683e79cf546184eb1379375f04672c80b28f0b982b"
====================
```

#### Fetch block (String parameter coerced):
```bash
cargo run -- rpc getblock "15434ac5b0e0f5d4b420c3683e79cf546184eb1379375f04672c80b28f0b982b"
```
**Example Output:**
```text
Executing RPC: getblock with 1 parameter(s)...

=== RPC Response ===
{
  "bits": "207fffff",
  "chainwork": "00000000000000000000000000000000000000000000000000000000000000ca",
  "confirmations": 2,
  "difficulty": 4.656542373906925e-10,
  "hash": "15434ac5b0e0f5d4b420c3683e79cf546184eb1379375f04672c80b28f0b982b",
  "height": 100,
  ...
}
====================
```

---

## Error Handling Scenarios

The CLI catches failures at the boundary layer and prints descriptive terminal messages:

- **Invalid Credentials (401 Unauthorized)**:
  ```text
  Execution Error: Authentication failed: Invalid RPC username or password.
  ```
- **Connection Failures (Offline Node)**:
  ```text
  Execution Error: Connection failure: Could not connect to the Bitcoin Core node. Check if it is running.
  Details: ...
  ```
- **Missing / Unloaded Wallet**:
  ```text
  Execution Error: Wallet error: No wallet is loaded. Load a wallet using loadwallet or create a new one with createwallet.
  ```
- **Invalid RPC Method / Bad Arguments**:
  ```text
  Execution Error: RPC Error (code -32601): Method not found
  ```

---

## Unit Testing

Run unit tests (covering URL building and dynamic JSON-RPC parameter parser types):
```bash
cargo test
```
**Example Output:**
```text
running 2 tests
test commands::tests::test_parse_param_types ... ok
test rpc::tests::test_rpc_client_url_resolution ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
```

---

## Assumptions & Design Decisions

1. **Parameter Typing**: Unlike standard CLI inputs that are strictly text, Bitcoin Core JSON-RPC expects typed values (e.g. `200` as an integer rather than `"200"`, `true` as a boolean, etc.). We assume that arguments matching valid JSON are intended to be parsed as JSON, falling back to a raw string if parsing fails.
2. **Wallet URLs**: Bitcoin Core endpoints change depending on whether a wallet is specified (e.g. `/` vs `/wallet/<name>`). The CLI resolves this cleanly by dynamically appending the path suffix when a wallet configuration is present.
3. **Tokio Runtime**: An async model using Tokio was selected to provide a highly scalable, non-blocking foundations, matching production client libraries.
