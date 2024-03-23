mod constants;
mod sbor;
mod state;
mod types;

use constants::*;
use radix_engine::system::system_modules::execution_trace::*;
use radix_engine_interface::prelude::*;
use state::*;
use std::cmp::*;
use transaction::prelude::*;
use types::*;

fn main() {
    // Defining the addresses of the account that has the liquidity receipt NFT,
    // the liquidity receipt resource address, and the non-fungible local ID of
    // the liquidity position.
    let position_non_fungible_local_id = NonFungibleLocalId::from_str(
        "{29de6fbdb0ba2dda-4c3c88c857022ead-a5c6381a54f02f2c-bd1e1eea22df0ea8}",
    )
    .unwrap();

    // Reading the liquidity receipt non-fungible data of the position. This is
    // done to get the pool address and user resource address.
    let liquidity_receipt_data = liquidity_receipt_data(
        EXCHANGE_LIQUIDITY_RECEIPT_RESOURCE_ADDRESS,
        &position_non_fungible_local_id,
    );

    // Creating the manifest that will get us the information that we're after.
    let manifest = ManifestBuilder::new()
        // Step 1: Withdraw the underlying liquidity receipt resources from
        // Ignition.
        .create_proof_from_account_of_amount(
            PROTOCOL_OWNER_ACCOUNT_COMPONENT_ADDRESS,
            PROTOCOL_OWNER_BADGE_RESOURCE_ADDRESS,
            1,
        )
        .call_method(
            IGNITION_COMPONENT_ADDRESS,
            "withdraw_pool_units",
            (NonFungibleGlobalId::new(
                EXCHANGE_LIQUIDITY_RECEIPT_RESOURCE_ADDRESS,
                position_non_fungible_local_id.clone(),
            ),),
        )
        // Step 2: Close the liquidity position through the adapter and not
        // through the Ignition component.
        .call_method(
            EXCHANGE_ADAPTER_COMPONENT_ADDRESS,
            "close_liquidity_position",
            (
                liquidity_receipt_data.pool_address,
                ManifestExpression::EntireWorktop,
                crate::sbor::manifest_value_from_scrypto_value(
                    &liquidity_receipt_data.adapter_specific_information,
                ),
            ),
        )
        // Step 3: Deposit the resources into an account - we do this just so
        // that the execution does not fail due to the dangling buckets.
        .deposit_batch(ComponentAddress::virtual_account_from_public_key(
            &Ed25519PrivateKey::from_u64(1).unwrap().public_key(),
        ))
        // Step 4: Get the price of the user resource from the oracle. This is
        // used later on in the Ignition settlement logic.
        .call_method(
            IGNITION_ORACLE_COMPONENT_ADDRESS,
            "get_price",
            (liquidity_receipt_data.user_resource_address, XRD),
        )
        .build();
    let receipt = preview_manifest(manifest);

    // Assert that the preview succeeded.
    let commit_result = receipt.expect_commit_success();

    // Getting the oracle reported price of the user resource from the receipt
    // output.
    let (price, _) = commit_result.output::<(Decimal, Instant)>(4);
    let oracle_reported_price = Price {
        base: liquidity_receipt_data.user_resource_address,
        quote: XRD,
        price,
    };

    // We need to get the data that was reported by the adapter when closing the
    // position which we can get from the outputs in the receipt.
    let CloseLiquidityPositionOutput { fees, .. } = commit_result.output(2);

    // Determine the amounts of the protocol and user resources returned when
    // the position was closed.
    let resources_returned_from_closing_liquidity_position = commit_result
        .execution_trace
        .as_ref()
        .unwrap()
        .worktop_changes()
        .get(&2)
        .unwrap()
        .iter()
        .filter_map(|worktop_change| match worktop_change {
            WorktopChange::Put(ResourceSpecifier::Amount(
                resource_address,
                amount,
            )) => Some((*resource_address, *amount)),
            WorktopChange::Take(_) | WorktopChange::Put(_) => None,
        })
        .collect::<IndexMap<_, _>>();

    let [user_resource_bucket_amount, protocol_resource_bucket_amount] =
        [liquidity_receipt_data.user_resource_address, XRD].map(|address| {
            resources_returned_from_closing_liquidity_position
                .get(&address)
                .copied()
                .unwrap()
        });

    let [user_resource_fees, _] =
        [liquidity_receipt_data.user_resource_address, XRD].map(|address| {
            fees.get(&address)
                .copied()
                .unwrap_or(Decimal::ZERO)
                .max(Decimal::ZERO)
        });

    // Determine the amount of resources that the user should be given back.
    //
    // Branch 1: There is enough of the user asset to give the user back the
    // same amount that they put in. So, we give them their initial amount +
    // the fees.
    let (
        amount_of_protocol_resource_to_give_user,
        amount_of_user_resource_to_give_user,
    ) = if user_resource_bucket_amount
        >= liquidity_receipt_data.user_contribution_amount
    {
        let amount_of_protocol_resource_to_give_user = dec!(0);
        let amount_of_user_resource_to_give_user = min(
            user_resource_bucket_amount,
            liquidity_receipt_data
                .user_contribution_amount
                .checked_add(user_resource_fees)
                .unwrap(),
        );

        (
            amount_of_protocol_resource_to_give_user,
            amount_of_user_resource_to_give_user,
        )
    }
    // Branch 2: There is not enough of the user token to given them back the
    // same amount that they put in. IL protection kicks in here.
    else {
        let amount_of_protocol_resource_to_give_user = {
            let user_amount_missing = liquidity_receipt_data
                .user_contribution_amount
                .checked_sub(user_resource_bucket_amount)
                .unwrap();
            let (_, protocol_resources_required_for_buy_back) =
                oracle_reported_price
                    .exchange(
                        liquidity_receipt_data.user_resource_address,
                        user_amount_missing,
                    )
                    .unwrap();
            min(
                protocol_resources_required_for_buy_back,
                protocol_resource_bucket_amount,
            )
        };
        let amount_of_user_resource_to_give_user = user_resource_bucket_amount;

        (
            amount_of_protocol_resource_to_give_user,
            amount_of_user_resource_to_give_user,
        )
    };

    // Printing the amount of the user and protocol resources that will be given
    // to the user if this position was closed now. If the XRD given to the user
    // is 0 then it means that there was no need for IL protection to kick in so
    // they were just given their initial amount + user resource fees.
    println!(
        "Ignition liquidity position global id: {}",
        NonFungibleGlobalId::new(
            EXCHANGE_LIQUIDITY_RECEIPT_RESOURCE_ADDRESS,
            position_non_fungible_local_id
        )
        .to_canonical_string(&AddressBech32Encoder::new(
            &NetworkDefinition::mainnet()
        ))
    );
    println!(
        "Amount of user resources contributed: {}",
        liquidity_receipt_data.user_contribution_amount
    );
    println!(
        "XRD that goes to user: {amount_of_protocol_resource_to_give_user}",
    );
    println!(
        "User resource that goes to user: {amount_of_user_resource_to_give_user}",
    );

    // Printing the amount that they got in fees. As mentioned in the previous
    // comments, this is dependent on whether IL protection needed to kick in or
    // not. If IL protection was needed then no fees are awarded. Else, fees are
    // awarded.
    //
    // Note: The above `amount_of_user_resource_to_give_user` is the initial
    // amount plus the fees. Do not add the below variable again.
    let user_resource_fees = if user_resource_bucket_amount
        >= liquidity_receipt_data.user_contribution_amount
    {
        user_resource_fees
    } else {
        dec!(0)
    };
    println!("User resource fees given to user: {user_resource_fees}")
}
