# NFT Map Pallet for Substrate

A pallet that implements NFT value mapping functionality for blockchain networks.

This pallet provides a simple way to associate NFT values with accounts, managed by a root user.

## Overview

The NFT Map pallet provides functionality for:
- Adding NFT values for accounts
- Updating existing NFT values
- Removing NFT mappings
- Value bounds validation
- Root-level access control

## Key Components

### Roles
- **Root**: Administrator who can add, update, and remove NFT mappings
- **Account**: Entity that can have an associated NFT value

### Storage Items
```rust
pub type NFTs<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::NFTValue>;
```

### Types

#### NFTValue
```rust
type NFTValue: Member + Parameter + From<u32> + Into<u32> + MaxEncodedLen + Copy;
```

#### Config Constants
```rust
type MinValue: Get<u32>;
type MaxValue: Get<u32>;
```

## Extrinsics

### add_nft
Adds a new NFT value mapping for an account.
```rust
fn add_nft(
    origin: OriginFor<T>,
    account: T::AccountId,
    val: T::NFTValue
) -> DispatchResult
```

### update_nft
Updates an existing NFT value for an account.
```rust
fn update_nft(
    origin: OriginFor<T>,
    account: T::AccountId,
    val: T::NFTValue
) -> DispatchResult
```

### delete_nft
Removes an NFT mapping for an account.
```rust
fn delete_nft(
    origin: OriginFor<T>,
    account: T::AccountId
) -> DispatchResult
```

## Events
- `NFTAdded`: Emitted when a new NFT mapping is added
- `NFTUpdated`: Emitted when an NFT value is updated
- `NFTRemoved`: Emitted when an NFT mapping is removed

## Errors
- `NFTAlreadyExists`
- `NFTNotFound`
- `InvalidNFTValue`
- `ValueTooLow`
- `ValueTooHigh`

## Testing

The pallet includes comprehensive tests covering:
- NFT addition
- NFT value updates
- NFT deletion
- Value validation
- Access control
- Error conditions
- Event emission

Run tests with:
```bash
cargo test
```

## Configuration

To use this pallet, include it in your runtime's `construct_runtime!` macro:
```rust
construct_runtime!(
    pub enum Runtime {
        NFTMap: pallet_nft_map,
    }
);
```

### Runtime Configuration
```rust
parameter_types! {
    pub const MinNFTValue: u32 = 1;
    pub const MaxNFTValue: u32 = 1_000_000;
}

impl pallet_nft_map::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NFTValue = u32;
    type MinValue = MinNFTValue;
    type MaxValue = MaxNFTValue;
}
```

## License
[Add your license information here]

## Contributing
[Add contributing guidelines here]

---

Note: This pallet is part of the Substrate framework and follows FRAME conventions for pallet development.