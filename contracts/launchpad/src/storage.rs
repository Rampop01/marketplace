use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::types::{CollectionKind, CollectionRecord, DataKey, Error};

const TTL_THRESHOLD: u32 = 50_000;
const TTL_BUMP: u32 = 100_000;

pub fn set_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
    env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_BUMP);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Initialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_platform_fee(env: &Env, receiver: &Address, bps: u32) {
    env.storage().instance().set(&DataKey::PlatformFeeReceiver, receiver);
    env.storage().instance().set(&DataKey::PlatformFeeBps, &bps);
}

pub fn get_platform_fee(env: &Env) -> (Address, u32) {
    (
        env.storage().instance().get(&DataKey::PlatformFeeReceiver).unwrap(),
        env.storage().instance().get(&DataKey::PlatformFeeBps).unwrap_or(0),
    )
}

pub fn set_wasm_hashes(
    env: &Env,
    normal_721: &BytesN<32>,
    normal_1155: &BytesN<32>,
    lazy_721: &BytesN<32>,
    lazy_1155: &BytesN<32>,
) {
    env.storage().instance().set(&DataKey::WasmNormal721, normal_721);
    env.storage().instance().set(&DataKey::WasmNormal1155, normal_1155);
    env.storage().instance().set(&DataKey::WasmLazy721, lazy_721);
    env.storage().instance().set(&DataKey::WasmLazy1155, lazy_1155);
}

pub fn get_wasm_normal_721(env: &Env) -> Option<BytesN<32>> {
    env.storage().instance().get(&DataKey::WasmNormal721)
}

pub fn get_wasm_normal_1155(env: &Env) -> Option<BytesN<32>> {
    env.storage().instance().get(&DataKey::WasmNormal1155)
}

pub fn get_wasm_lazy_721(env: &Env) -> Option<BytesN<32>> {
    env.storage().instance().get(&DataKey::WasmLazy721)
}

pub fn get_wasm_lazy_1155(env: &Env) -> Option<BytesN<32>> {
    env.storage().instance().get(&DataKey::WasmLazy1155)
}

pub fn collections_by_creator(env: &Env, creator: &Address) -> Vec<CollectionRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::ByCreator(creator.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn all_collections(env: &Env) -> Vec<CollectionRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::AllCollections)
        .unwrap_or(Vec::new(env))
}

// Counter for total collections ever deployed through this launchpad. 
pub fn collection_count(env: &Env) -> u64 {
    env.storage().instance().get(&DataKey::CollectionCount).unwrap_or(0)
}


// ── Private helpers ───────────────────────────────────────────────────

pub fn require_admin(env: &Env) -> Result<Address, Error> {
    let admin = get_admin(env).ok_or(Error::NotInitialized)?;
    admin.require_auth();
    Ok(admin)
}
pub fn record_collection(env: &Env, creator: &Address, address: &Address, kind: CollectionKind) {
    let rec = CollectionRecord {
        address: address.clone(),
        kind,
        creator: creator.clone(),
    };

    // Global indexed storage (#51) — each record in its own key, no Vec bloat
    let global_idx = collection_count(env);
    env.storage()
        .persistent()
        .set(&DataKey::CollectionByIndex(global_idx), &rec);
    env.storage()
        .persistent()
        .extend_ttl(&DataKey::CollectionByIndex(global_idx), TTL_THRESHOLD, TTL_BUMP);

    // Per-creator indexed storage (#51) — same pattern per creator
    let creator_count: u64 = env
        .storage()
        .persistent()
        .get(&DataKey::CreatorCollectionCount(creator.clone()))
        .unwrap_or(0);
    env.storage().persistent().set(
        &DataKey::CreatorCollectionByIndex(creator.clone(), creator_count),
        &rec,
    );
    env.storage().persistent().extend_ttl(
        &DataKey::CreatorCollectionByIndex(creator.clone(), creator_count),
        TTL_THRESHOLD,
        TTL_BUMP,
    );
    env.storage().persistent().set(
        &DataKey::CreatorCollectionCount(creator.clone()),
        &(creator_count + 1),
    );

    // Increment global counter
    let next = global_idx + 1;
    env.storage().instance().set(&DataKey::CollectionCount, &next);
}

/// Get a collection by global index.
pub fn collection_by_index(env: &Env, index: u64) -> Option<CollectionRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::CollectionByIndex(index))
}

/// Get per-creator collection count.
pub fn creator_collection_count(env: &Env, creator: &Address) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::CreatorCollectionCount(creator.clone()))
        .unwrap_or(0)
}

/// Get a creator's collection by index.
pub fn creator_collection_by_index(
    env: &Env,
    creator: &Address,
    index: u64,
) -> Option<CollectionRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::CreatorCollectionByIndex(creator.clone(), index))
}
