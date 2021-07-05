table! {
    blocks (slot, block_root) {
        epoch -> Int8,
        slot -> Int8,
        block_root -> Bytea,
        parent_root -> Bytea,
        state_root -> Bytea,
        signature -> Bytea,
        randao_reveal -> Nullable<Bytea>,
        graffiti -> Nullable<Bytea>,
        graffiti_text -> Nullable<Text>,
        eth1data_deposit_root -> Nullable<Bytea>,
        eth1data_deposit_count -> Int4,
        eth1data_block_hash -> Nullable<Bytea>,
        proposer_slashings_count -> Int4,
        attester_slashings_count -> Int4,
        attestations_count -> Int4,
        deposits_count -> Int4,
        voluntary_exits_count -> Int4,
        proposer -> Int4,
        status -> Text,
    }
}

table! {
    epochs (epoch) {
        epoch -> Int8,
        blocks_count -> Int4,
        proposer_slashings_count -> Int4,
        attester_slashings_count -> Int4,
        attestations_count -> Int4,
        deposits_count -> Int4,
        voluntary_exits_count -> Int4,
        validators_count -> Int4,
        average_validator_balance -> Int8,
        total_validator_balance -> Int8,
        finalized -> Nullable<Bool>,
        eligible_ether -> Nullable<Int8>,
        global_participation_rate -> Nullable<Float8>,
        voted_ether -> Nullable<Int8>,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    epochs,
);
