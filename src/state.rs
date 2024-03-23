use crate::types::*;
use gateway_client::apis::configuration::*;
use gateway_client::apis::state_api::*;
use gateway_client::apis::transaction_api::*;
use gateway_client::models::*;
use radix_engine::transaction::*;
use radix_engine_interface::prelude::*;
use transaction::manifest::*;
use transaction::prelude::*;

pub fn preview_manifest(
    manifest: TransactionManifestV1,
) -> TransactionReceiptV1 {
    // Configuration to use for connections to the gateway - this is nothing
    // special, just the base url of the gateway API.
    let gateway_config = Configuration {
        base_path: "https://mainnet.radixdlt.com".to_owned(),
        ..Default::default()
    };

    // Based on the above used base url path, we're using mainnet and therefore
    // this is the network definition to use.
    let network_definition = NetworkDefinition::mainnet();

    // Decompile the manifest into a string manifest.
    let manifest_string =
        decompile(&manifest.instructions, &network_definition)
            .expect("Should not fail!");

    // Construct the preview request.
    let request = TransactionPreviewRequest {
        manifest: manifest_string,
        blobs_hex: Some(manifest.blobs.values().map(hex::encode).collect()),
        // Start and end epoch does not matter here since we will disable the
        // check in the preview.
        start_epoch_inclusive: 200,
        end_epoch_exclusive: 210,
        // No need to think of the notary here.
        notary_public_key: None,
        notary_is_signatory: None,
        tip_percentage: 0,
        nonce: 1,
        signer_public_keys: Default::default(),
        flags: Box::new(TransactionPreviewRequestFlags {
            use_free_credit: true,
            assume_all_signature_proofs: true,
            skip_epoch_check: true,
        }),
    };

    // Do the preview and get the response.
    let response = transaction_preview(&gateway_config, request);

    // We're assuming here that the HTTP request won't fail and we do unwraps.
    // You should think about how to handle the case where they do fail and
    // perhaps return a `Result` from this function.
    let response = response.unwrap();

    // The response contains an SBOR encoded transaction receipt which we can
    // just get and decode.
    let receipt = scrypto_decode::<VersionedTransactionReceipt>(
        &response.encoded_receipt,
    )
    .expect("Decoding of receipt must succeed");

    // Convert the receipt into a v1 receipt and return it back. We do not do
    // any assertions here on whether the transaction was successful or not.
    receipt.into_latest()
}

pub fn liquidity_receipt_data(
    resource_address: ResourceAddress,
    local_id: &NonFungibleLocalId,
) -> LiquidityReceipt<AnyValue> {
    // Configuration to use for connections to the gateway - this is nothing
    // special, just the base url of the gateway API.
    let gateway_config = Configuration {
        base_path: "https://mainnet.radixdlt.com".to_owned(),
        ..Default::default()
    };

    // Based on the above used base url path, we're using mainnet and therefore
    // this is the network definition to use.
    let network_definition = NetworkDefinition::mainnet();

    // Constructing the request.
    let request = StateNonFungibleDataRequest {
        at_ledger_state: None,
        resource_address: AddressBech32Encoder::new(&network_definition)
            .encode(&resource_address.as_node_id().0)
            .unwrap(),
        non_fungible_ids: vec![local_id.to_string()],
    };

    // Get the data of the NFT.
    let response = non_fungible_data(&gateway_config, request);

    // We're assuming here that the HTTP request won't fail and we do unwraps.
    // You should think about how to handle the case where they do fail and
    // perhaps return a `Result` from this function.
    let mut response = response.unwrap();

    let liquidity_receipt_encoded_data = hex::decode(
        response
            .non_fungible_ids
            .pop()
            .unwrap()
            .data
            .unwrap()
            .raw_hex,
    )
    .unwrap();
    scrypto_decode(&liquidity_receipt_encoded_data).unwrap()
}
