# Anchor Vault Program

A simple SOL vault built with [Anchor](https://www.anchor-lang.com/) on Solana. Each user gets their own PDA-owned vault to deposit SOL into, withdraw from, and close when they're done.

## How It Works

The program exposes four instructions:

### `initialize`
Sets up a user's vault. Creates a `vault_state` PDA to track vault metadata (and bumps) and a separate `vault` system account PDA that will actually hold the deposited SOL.

### `deposit`
Transfers SOL from the user's wallet into their `vault` PDA.

### `withdraw`
Transfers SOL from the `vault` PDA back to the user's wallet. The vault PDA signs the transfer using its stored bump seeds.

### `close`
Withdraws any remaining balance from the vault back to the user and closes the `vault_state` account, refunding its rent.

## Program Structure

```
programs/q3_26_vault/
├── src/
│   ├── instructions/
│   │   ├── initialize.rs   # Set up a user's vault_state + vault PDAs
│   │   ├── deposit.rs      # Move SOL from user into vault
│   │   ├── withdraw.rs     # Move SOL from vault back to user
│   │   └── close.rs        # Withdraw remaining balance and close the vault
│   ├── instructions.rs     # Instruction module re-exports
│   ├── state.rs            # Vault state account definition
│   ├── constants.rs        # PDA seeds and other shared constants
│   ├── error.rs             # Custom error codes
│   └── lib.rs               # Instruction entrypoints
└── tests/
    └── test_initialize.rs   # LiteSVM integration test for `initialize`
```

## Testing

Tests are written with [LiteSVM](https://github.com/LiteSVM/litesvm), which runs a full Solana runtime in-process for fast, dependency-free integration testing (no local validator required).

Currently covered:

- **`test_initialize`** — confirms `vault_state` and `vault` are created correctly when a user initializes their vault.

Not yet covered (planned):

- `deposit` — SOL moves from user to vault
- `withdraw` — SOL moves from vault back to user, vault PDA signs correctly
- `close` — remaining balance is withdrawn and `vault_state` is closed with rent refunded

Run the test suite with:

```bash
anchor test --skip-build
```

or, to build and test together:

```bash
anchor test
```

### Tests Passing

```
running 1 test
test test_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_initialize.rs (target/debug/deps/test_initialize-77fb349cc6e99968)

running 1 test
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s
```

![Initialize test passing](./img/vault%20screenshot.png)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)

## Building

```bash
anchor build
```