// ------------------------------------------------------------
// test.rs — Unit tests for the Soroban marketplace contract
// ------------------------------------------------------------

#![cfg(test)]

use super::*;
use crate::events::*;
use soroban_sdk::{testutils::Events, Symbol, Val};
use soroban_sdk::{
    bytes,
    symbol_short,
    testutils::Address as _,
    Address, Env,
};

/// Helper — deploy the contract and return (env, client, token_admin, token_id).
fn setup() -> (Env, MarketplaceContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MarketplaceContract, ());
    let client = MarketplaceContractClient::new(&env, &contract_id);

    let artist = Address::generate(&env);
    let buyer  = Address::generate(&env);

    (env, client, artist, buyer, contract_id)
}

// ── create_listing ───────────────────────────────────────────

#[test]
fn test_create_listing_success() {
    let (env, client, artist, _, _) = setup();
    let cid = bytes!(&env, 0x516d546573744349444f6e495046533132333435);
    let price: i128 = 10_000_000; // 1 XLM

    let listing_id = client.create_listing(
        &artist,
        &cid,
        &price,
        &symbol_short!("XLM"),
    );

    assert_eq!(listing_id, 1u64);

    let listing = client.get_listing(&listing_id);
    assert_eq!(listing.artist, artist);
    assert_eq!(listing.price, price);
    assert_eq!(listing.status, ListingStatus::Active);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_create_listing_zero_price() {
    let (env, client, artist, _, _) = setup();
    let cid = bytes!(&env, 0x516d74657374);
    client.create_listing(&artist, &cid, &0_i128, &symbol_short!("XLM"));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_create_listing_empty_cid() {
    let (env, client, artist, _, _) = setup();
    client.create_listing(
        &artist,
        &bytes!(&env,),
        &10_000_000_i128,
        &symbol_short!("XLM"),
    );
}

// ── cancel_listing ───────────────────────────────────────────

#[test]
fn test_cancel_listing_success() {
    let (env, client, artist, _, _) = setup();
    let cid = bytes!(&env, 0x516d74657374);
    let id = client.create_listing(&artist, &cid, &5_000_000_i128, &symbol_short!("XLM"));

    let result = client.cancel_listing(&artist, &id);
    assert!(result);

    let listing = client.get_listing(&id);
    assert_eq!(listing.status, ListingStatus::Cancelled);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_cancel_listing_wrong_artist() {
    let (env, client, artist, buyer, _) = setup();
    let cid = bytes!(&env, 0x516d74657374);
    let id = client.create_listing(&artist, &cid, &5_000_000_i128, &symbol_short!("XLM"));
    client.cancel_listing(&buyer, &id);
}

// ── get_artist_listings ──────────────────────────────────────

#[test]
fn test_get_artist_listings() {
    let (env, client, artist, _, _) = setup();
    let cid = bytes!(&env, 0x516d74657374);

    client.create_listing(&artist, &cid, &1_000_000_i128, &symbol_short!("XLM"));
    client.create_listing(&artist, &cid, &2_000_000_i128, &symbol_short!("XLM"));
    client.create_listing(&artist, &cid, &3_000_000_i128, &symbol_short!("XLM"));

    let ids = client.get_artist_listings(&artist);
    assert_eq!(ids.len(), 3);
    assert_eq!(ids.get(0).unwrap(), 1_u64);
    assert_eq!(ids.get(1).unwrap(), 2_u64);
    assert_eq!(ids.get(2).unwrap(), 3_u64);
}

#[test]
fn test_buy_artwork_success() {
    let (env, client, artist, buyer, _) = setup();

    let cid = bytes!(&env, 0x516d74657374);
    let price = 10_000_000_i128;
    let id = client.create_listing(&artist, &cid, &price, &symbol_short!("XLM"));

    let result = client.buy_artwork(&buyer, &id);
    assert!(result);

    let listing = client.get_listing(&id);
    assert_eq!(listing.status, ListingStatus::Sold);
    assert_eq!(listing.owner, Some(buyer.clone()));
}

// ── get_listing not found ────────────────────────────────────

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_get_listing_not_found() {
    let (_env, client, _, _, _) = setup();
    client.get_listing(&999);
}