create table epochs
(
    epoch                     bigint not null,
    blocks_count              int    not null default 0,
    proposer_slashings_count  int    not null,
    attester_slashings_count  int    not null,
    attestations_count        int    not null,
    deposits_count            int    not null,
    voluntary_exits_count     int    not null,
    validators_count          int    not null,
    average_validator_balance bigint not null,
    total_validator_balance   bigint not null,
    finalized                 bool,
    eligible_ether            bigint,
    global_participation_rate float,
    voted_ether               bigint,
    primary key (epoch)
);

create table blocks
(
    epoch                    bigint not null,
    slot                     bigint not null,
    block_root               bytea  not null,
    parent_root              bytea  not null,
    state_root               bytea  not null,
    signature                bytea  not null,
    randao_reveal            bytea,
    graffiti                 bytea,
    graffiti_text            text   null,
    eth1data_deposit_root    bytea,
    eth1data_deposit_count   int    not null,
    eth1data_block_hash      bytea,
    proposer_slashings_count int    not null,
    attester_slashings_count int    not null,
    attestations_count       int    not null,
    deposits_count           int    not null,
    voluntary_exits_count    int    not null,
    proposer                 int    not null,
    status                   text   not null, /* Can be 0 = scheduled, 1 proposed, 2 missed, 3 orphaned */
    primary key (slot, block_root)
);

create index idx_blocks_proposer on blocks (proposer);
create index idx_blocks_epoch on blocks (epoch);