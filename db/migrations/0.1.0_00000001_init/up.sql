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

create table validators
(
    validator_index              int         not null,
    pubkey                       bytea       not null,
    pubkey_hex                   text        not null default '',
    withdrawable_epoch           bigint      not null,
    withdrawal_credentials       bytea       not null,
    balance                      bigint      not null,
    balance_activation           bigint,
    effective_balance            bigint      not null,
    slashed                      bool        not null,
    activation_eligibility_epoch bigint      not null,
    activation_epoch             bigint      not null,
    exit_epoch                   bigint      not null,
    status                     varchar(20) not null default '',
    primary key (validator_index)
);

create index idx_validators_pubkey on validators (pubkey);
create index idx_validators_pubkeyhex on validators (pubkey_hex);
create index idx_validators_pubkeyhex_pattern_pos on validators (pubkey_hex varchar_pattern_ops);
create index idx_validators_status on validators (status);
create index idx_validators_balanceactivation on validators (balance_activation);
create index idx_validators_activationepoch on validators (activation_epoch);