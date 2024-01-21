# Astreum Blockchain CLI

The Astreum CLI is a command-line interface for interacting with the Astreum blockchain. It provides a variety of commands to manage accounts, blocks, chain synchronization, transactions, and access a shell interface.

## Installation

## Usage

The general syntax for using the Astreum CLI is:

    astreum <topic> [command]

### Topics

Astreum CLI supports the following topics:

- `account`: Manage blockchain accounts.
- `block`: Interact with individual blocks on the blockchain.
- `chain`: Manage and view the blockchain.
- `shell`: Access the Astreum interactive programming environment (coming soon).
- `tx`: Manage transactions on the blockchain.

#### Account Commands

- `astreum account all`: Display all accounts (coming soon).
- `astreum account new`: Create a new account (coming soon).
- `astreum account view <account_address>`: View details of an account (coming soon).

#### Block Commands

- `astreum block view <block_hash>`: View details of a block (coming soon).

#### Chain Commands

- `astreum chain sync`: Synchronize the local chain with the network (coming soon).
- `astreum chain view <block_hash>`: View details of the current chain (coming soon).

#### Transaction Commands

- `astreum tx new`: Create a new transaction (coming soon).
- `astreum tx view <tx_hash>`: View details of a transaction (coming soon).

## Error Handling

The CLI provides clear error messages for invalid commands or arguments. Make sure to follow the correct syntax as outlined in this document.
