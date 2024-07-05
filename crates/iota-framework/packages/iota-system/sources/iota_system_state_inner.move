// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_system::iota_system_state_inner {
    use iota::balance::{Self, Balance};
    use iota::coin::Coin;
    use iota_system::staking_pool::{stake_activation_epoch, StakedIota};
    use iota::iota::{IOTA, IotaTreasuryCap};
    use iota_system::validator::{Self, Validator};
    use iota_system::validator_set::{Self, ValidatorSet};
    use iota_system::validator_cap::{UnverifiedValidatorOperationCap, ValidatorOperationCap};
    use iota_system::stake_subsidy::StakeSubsidy;
    use iota_system::storage_fund::{Self, StorageFund};
    use iota_system::staking_pool::PoolTokenExchangeRate;
    use iota::vec_map::{Self, VecMap};
    use iota::vec_set::{Self, VecSet};
    use iota::event;
    use iota::table::Table;
    use iota::bag::Bag;
    use iota::bag;

    /* friend iota_system::genesis; */
    /* friend iota_system::iota_system; */

    /* #[test_only] */
    /* friend iota_system::governance_test_utils; */

    // same as in validator_set
    const ACTIVE_VALIDATOR_ONLY: u8 = 1;
    const ACTIVE_OR_PENDING_VALIDATOR: u8 = 2;
    const ANY_VALIDATOR: u8 = 3;

    const SYSTEM_STATE_VERSION_V1: u64 = 1;

    /// A list of system config parameters.
    public struct SystemParameters has store {
        /// The duration of an epoch, in milliseconds.
        epoch_duration_ms: u64,

        /// The starting epoch in which stake subsidies start being paid out
        stake_subsidy_start_epoch: u64,

        /// Maximum number of active validators at any moment.
        /// We do not allow the number of validators in any epoch to go above this.
        max_validator_count: u64,

        /// Lower-bound on the amount of stake required to become a validator.
        min_validator_joining_stake: u64,

        /// Validators with stake amount below `validator_low_stake_threshold` are considered to
        /// have low stake and will be escorted out of the validator set after being below this
        /// threshold for more than `validator_low_stake_grace_period` number of epochs.
        validator_low_stake_threshold: u64,

        /// Validators with stake below `validator_very_low_stake_threshold` will be removed
        /// immediately at epoch change, no grace period.
        validator_very_low_stake_threshold: u64,

        /// A validator can have stake below `validator_low_stake_threshold`
        /// for this many epochs before being kicked out.
        validator_low_stake_grace_period: u64,

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Added min_validator_count.
    public struct SystemParametersV2 has store {
        /// The duration of an epoch, in milliseconds.
        epoch_duration_ms: u64,

        /// The starting epoch in which stake subsidies start being paid out
        stake_subsidy_start_epoch: u64,

        /// Minimum number of active validators at any moment.
        min_validator_count: u64,

        /// Maximum number of active validators at any moment.
        /// We do not allow the number of validators in any epoch to go above this.
        max_validator_count: u64,

        /// Lower-bound on the amount of stake required to become a validator.
        min_validator_joining_stake: u64,

        /// Validators with stake amount below `validator_low_stake_threshold` are considered to
        /// have low stake and will be escorted out of the validator set after being below this
        /// threshold for more than `validator_low_stake_grace_period` number of epochs.
        validator_low_stake_threshold: u64,

        /// Validators with stake below `validator_very_low_stake_threshold` will be removed
        /// immediately at epoch change, no grace period.
        validator_very_low_stake_threshold: u64,

        /// A validator can have stake below `validator_low_stake_threshold`
        /// for this many epochs before being kicked out.
        validator_low_stake_grace_period: u64,

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// The top-level object containing all information of the Iota system.
    public struct IotaSystemStateInner has store {
        /// The current epoch ID, starting from 0.
        epoch: u64,
        /// The current protocol version, starting from 1.
        protocol_version: u64,
        /// The current version of the system state data structure type.
        /// This is always the same as IotaSystemState.version. Keeping a copy here so that
        /// we know what version it is by inspecting IotaSystemStateInner as well.
        system_state_version: u64,
        /// The IOTA's TreasuryCap.
        iota_treasury_cap: IotaTreasuryCap,
        /// Contains all information about the validators.
        validators: ValidatorSet,
        /// The storage fund.
        storage_fund: StorageFund,
        /// A list of system config parameters.
        parameters: SystemParameters,
        /// The reference gas price for the current epoch.
        reference_gas_price: u64,
        /// A map storing the records of validator reporting each other.
        /// There is an entry in the map for each validator that has been reported
        /// at least once. The entry VecSet contains all the validators that reported
        /// them. If a validator has never been reported they don't have an entry in this map.
        /// This map persists across epoch: a peer continues being in a reported state until the
        /// reporter doesn't explicitly remove their report.
        /// Note that in case we want to support validator address change in future,
        /// the reports should be based on validator ids
        validator_report_records: VecMap<address, VecSet<address>>,
        /// Schedule of stake subsidies given out each epoch.
        stake_subsidy: StakeSubsidy,

        /// Whether the system is running in a downgraded safe mode due to a non-recoverable bug.
        /// This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode.
        /// It can be reset once we are able to successfully execute advance_epoch.
        /// The rest of the fields starting with `safe_mode_` are accmulated during safe mode
        /// when advance_epoch_safe_mode is executed. They will eventually be processed once we
        /// are out of safe mode.
        safe_mode: bool,
        safe_mode_storage_rewards: Balance<IOTA>,
        safe_mode_computation_rewards: Balance<IOTA>,
        safe_mode_storage_rebates: u64,
        safe_mode_non_refundable_storage_fee: u64,

        /// Unix timestamp of the current epoch start
        epoch_start_timestamp_ms: u64,
        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Uses SystemParametersV2 as the parameters.
    public struct IotaSystemStateInnerV2 has store {
        /// The current epoch ID, starting from 0.
        epoch: u64,
        /// The current protocol version, starting from 1.
        protocol_version: u64,
        /// The current version of the system state data structure type.
        /// This is always the same as IotaSystemState.version. Keeping a copy here so that
        /// we know what version it is by inspecting IotaSystemStateInner as well.
        system_state_version: u64,
        /// The IOTA's TreasuryCap.
        iota_treasury_cap: IotaTreasuryCap,
        /// Contains all information about the validators.
        validators: ValidatorSet,
        /// The storage fund.
        storage_fund: StorageFund,
        /// A list of system config parameters.
        parameters: SystemParametersV2,
        /// The reference gas price for the current epoch.
        reference_gas_price: u64,
        /// A map storing the records of validator reporting each other.
        /// There is an entry in the map for each validator that has been reported
        /// at least once. The entry VecSet contains all the validators that reported
        /// them. If a validator has never been reported they don't have an entry in this map.
        /// This map persists across epoch: a peer continues being in a reported state until the
        /// reporter doesn't explicitly remove their report.
        /// Note that in case we want to support validator address change in future,
        /// the reports should be based on validator ids
        validator_report_records: VecMap<address, VecSet<address>>,
        /// Schedule of stake subsidies given out each epoch.
        stake_subsidy: StakeSubsidy,

        /// Whether the system is running in a downgraded safe mode due to a non-recoverable bug.
        /// This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode.
        /// It can be reset once we are able to successfully execute advance_epoch.
        /// The rest of the fields starting with `safe_mode_` are accmulated during safe mode
        /// when advance_epoch_safe_mode is executed. They will eventually be processed once we
        /// are out of safe mode.
        safe_mode: bool,
        safe_mode_storage_rewards: Balance<IOTA>,
        safe_mode_computation_rewards: Balance<IOTA>,
        safe_mode_storage_rebates: u64,
        safe_mode_non_refundable_storage_fee: u64,

        /// Unix timestamp of the current epoch start
        epoch_start_timestamp_ms: u64,
        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Event containing system-level epoch information, emitted during
    /// the epoch advancement transaction.
    public struct SystemEpochInfoEvent has copy, drop {
        epoch: u64,
        protocol_version: u64,
        reference_gas_price: u64,
        total_stake: u64,
        storage_charge: u64,
        storage_rebate: u64,
        storage_fund_balance: u64,
        stake_subsidy_amount: u64,
        total_gas_fees: u64,
        total_stake_rewards_distributed: u64,
        leftover_storage_fund_inflow: u64,
    }

    // Errors
    const ENotValidator: u64 = 0;
    const ELimitExceeded: u64 = 1;
    #[allow(unused_const)]
    const ENotSystemAddress: u64 = 2;
    const ECannotReportOneself: u64 = 3;
    const EReportRecordNotFound: u64 = 4;
    const EBpsTooLarge: u64 = 5;
    const EStakeWithdrawBeforeActivation: u64 = 6;
    const ESafeModeGasNotProcessed: u64 = 7;
    const EAdvancedToWrongEpoch: u64 = 8;

    const BASIS_POINT_DENOMINATOR: u128 = 10000;

    // ==== functions that can only be called by genesis ====

    /// Create a new IotaSystemState object and make it shared.
    /// This function will be called only once in genesis.
    public(package) fun create(
        iota_treasury_cap: IotaTreasuryCap,
        validators: vector<Validator>,
        initial_storage_fund: Balance<IOTA>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        parameters: SystemParameters,
        stake_subsidy: StakeSubsidy,
        ctx: &mut TxContext,
    ): IotaSystemStateInner {
        let validators = validator_set::new(validators, ctx);
        let reference_gas_price = validators.derive_reference_gas_price();
        // This type is fixed as it's created at genesis. It should not be updated during type upgrade.
        let system_state = IotaSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            iota_treasury_cap,
            validators,
            storage_fund: storage_fund::new(initial_storage_fund),
            parameters,
            reference_gas_price,
            validator_report_records: vec_map::empty(),
            stake_subsidy,
            safe_mode: false,
            safe_mode_storage_rewards: balance::zero(),
            safe_mode_computation_rewards: balance::zero(),
            safe_mode_storage_rebates: 0,
            safe_mode_non_refundable_storage_fee: 0,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(package) fun create_system_parameters(
        epoch_duration_ms: u64,
        stake_subsidy_start_epoch: u64,

        // Validator committee parameters
        max_validator_count: u64,
        min_validator_joining_stake: u64,
        validator_low_stake_threshold: u64,
        validator_very_low_stake_threshold: u64,
        validator_low_stake_grace_period: u64,
        ctx: &mut TxContext,
    ): SystemParameters {
        SystemParameters {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields: bag::new(ctx),
        }
    }

    public(package) fun v1_to_v2(self: IotaSystemStateInner): IotaSystemStateInnerV2 {
        let IotaSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: _,
            iota_treasury_cap,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields: state_extra_fields,
        } = self;
        let SystemParameters {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields: param_extra_fields,
        } = parameters;
        IotaSystemStateInnerV2 {
            epoch,
            protocol_version,
            system_state_version: 2,
            iota_treasury_cap,
            validators,
            storage_fund,
            parameters: SystemParametersV2 {
                epoch_duration_ms,
                stake_subsidy_start_epoch,
                min_validator_count: 4,
                max_validator_count,
                min_validator_joining_stake,
                validator_low_stake_threshold,
                validator_very_low_stake_threshold,
                validator_low_stake_grace_period,
                extra_fields: param_extra_fields,
            },
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields: state_extra_fields
        }
    }

    // ==== public(friend) functions ====

    /// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
    /// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
    /// can call `request_add_validator` to officially become an active validator at the next epoch.
    /// Aborts if the caller is already a pending or active validator, or a validator candidate.
    /// Note: `proof_of_possession` MUST be a valid signature using iota_address and protocol_pubkey_bytes.
    /// To produce a valid PoP, run [fn test_proof_of_possession].
    public(package) fun request_add_validator_candidate(
        self: &mut IotaSystemStateInnerV2,
        pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        proof_of_possession: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        image_url: vector<u8>,
        project_url: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        gas_price: u64,
        commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let validator = validator::new(
            ctx.sender(),
            pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            gas_price,
            commission_rate,
            ctx
        );

        self.validators.request_add_validator_candidate(validator, ctx);
    }

    /// Called by a validator candidate to remove themselves from the candidacy. After this call
    /// their staking pool becomes deactivate.
    public(package) fun request_remove_validator_candidate(
        self: &mut IotaSystemStateInnerV2,
        ctx: &mut TxContext,
    ) {
        self.validators.request_remove_validator_candidate(ctx);
    }

    /// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
    /// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
    /// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
    /// epoch has already reached the maximum.
    public(package) fun request_add_validator(
        self: &mut IotaSystemStateInnerV2,
        ctx: &TxContext,
    ) {
        assert!(
            self.validators.next_epoch_validator_count() < self.parameters.max_validator_count,
            ELimitExceeded,
        );

        self.validators.request_add_validator(self.parameters.min_validator_joining_stake, ctx);
    }

    /// A validator can call this function to request a removal in the next epoch.
    /// We use the sender of `ctx` to look up the validator
    /// (i.e. sender must match the iota_address in the validator).
    /// At the end of the epoch, the `validator` object will be returned to the iota_address
    /// of the validator.
    public(package) fun request_remove_validator(
        self: &mut IotaSystemStateInnerV2,
        ctx: &TxContext,
    ) {
        // Only check min validator condition if the current number of validators satisfy the constraint.
        // This is so that if we somehow already are in a state where we have less than min validators, it no longer matters
        // and is ok to stay so. This is useful for a test setup.
        if (self.validators.active_validators().length() >= self.parameters.min_validator_count) {
            assert!(
                self.validators.next_epoch_validator_count() > self.parameters.min_validator_count,
                ELimitExceeded,
            );
        };

        self.validators.request_remove_validator(ctx)
    }

    /// A validator can call this function to submit a new gas price quote, to be
    /// used for the reference gas price calculation at the end of the epoch.
    public(package) fun request_set_gas_price(
        self: &mut IotaSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        // Verify the represented address is an active or pending validator, and the capability is still valid.
        let verified_cap = self.validators.verify_cap(cap, ACTIVE_OR_PENDING_VALIDATOR);
        let validator = self.validators.get_validator_mut_with_verified_cap(&verified_cap, false /* include_candidate */);

        validator.request_set_gas_price(verified_cap, new_gas_price);
    }

    /// This function is used to set new gas price for candidate validators
    public(package) fun set_candidate_validator_gas_price(
        self: &mut IotaSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        // Verify the represented address is an active or pending validator, and the capability is still valid.
        let verified_cap = self.validators.verify_cap(cap, ANY_VALIDATOR);
        let candidate = self.validators.get_validator_mut_with_verified_cap(&verified_cap, true /* include_candidate */);
        candidate.set_candidate_gas_price(verified_cap, new_gas_price)
    }

    /// A validator can call this function to set a new commission rate, updated at the end of
    /// the epoch.
    public(package) fun request_set_commission_rate(
        self: &mut IotaSystemStateInnerV2,
        new_commission_rate: u64,
        ctx: &TxContext,
    ) {
        self.validators.request_set_commission_rate(
            new_commission_rate,
            ctx
        )
    }

    /// This function is used to set new commission rate for candidate validators
    public(package) fun set_candidate_validator_commission_rate(
        self: &mut IotaSystemStateInnerV2,
        new_commission_rate: u64,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.set_candidate_commission_rate(new_commission_rate)
    }

    /// Add stake to a validator's staking pool.
    public(package) fun request_add_stake(
        self: &mut IotaSystemStateInnerV2,
        stake: Coin<IOTA>,
        validator_address: address,
        ctx: &mut TxContext,
    ) : StakedIota {
        self.validators.request_add_stake(
            validator_address,
            stake.into_balance(),
            ctx,
        )
    }

    /// Add stake to a validator's staking pool using multiple coins.
    public(package) fun request_add_stake_mul_coin(
        self: &mut IotaSystemStateInnerV2,
        stakes: vector<Coin<IOTA>>,
        stake_amount: option::Option<u64>,
        validator_address: address,
        ctx: &mut TxContext,
    ) : StakedIota {
        let balance = extract_coin_balance(stakes, stake_amount, ctx);
        self.validators.request_add_stake(validator_address, balance, ctx)
    }

    /// Withdraw some portion of a stake from a validator's staking pool.
    public(package) fun request_withdraw_stake(
        self: &mut IotaSystemStateInnerV2,
        staked_iota: StakedIota,
        ctx: &TxContext,
    ) : Balance<IOTA> {
        assert!(
            stake_activation_epoch(&staked_iota) <= ctx.epoch(),
            EStakeWithdrawBeforeActivation
        );
        self.validators.request_withdraw_stake(staked_iota, ctx)
    }

    /// Report a validator as a bad or non-performant actor in the system.
    /// Succeeds if all the following are satisfied:
    /// 1. both the reporter in `cap` and the input `reportee_addr` are active validators.
    /// 2. reporter and reportee not the same address.
    /// 3. the cap object is still valid.
    /// This function is idempotent.
    public(package) fun report_validator(
        self: &mut IotaSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        // Reportee needs to be an active validator
        assert!(self.validators.is_active_validator_by_iota_address(reportee_addr), ENotValidator);
        // Verify the represented reporter address is an active validator, and the capability is still valid.
        let verified_cap = self.validators.verify_cap(cap, ACTIVE_VALIDATOR_ONLY);
        report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
    }


    /// Undo a `report_validator` action. Aborts if
    /// 1. the reportee is not a currently active validator or
    /// 2. the sender has not previously reported the `reportee_addr`, or
    /// 3. the cap is not valid
    public(package) fun undo_report_validator(
        self: &mut IotaSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        let verified_cap = self.validators.verify_cap(cap, ACTIVE_VALIDATOR_ONLY);
        undo_report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
    }

    fun report_validator_impl(
        verified_cap: ValidatorOperationCap,
        reportee_addr: address,
        validator_report_records: &mut VecMap<address, VecSet<address>>,
    ) {
        let reporter_address = *verified_cap.verified_operation_cap_address();
        assert!(reporter_address != reportee_addr, ECannotReportOneself);
        if (!validator_report_records.contains(&reportee_addr)) {
            validator_report_records.insert(reportee_addr, vec_set::singleton(reporter_address));
        } else {
            let reporters = validator_report_records.get_mut(&reportee_addr);
            if (!reporters.contains(&reporter_address)) {
                reporters.insert(reporter_address);
            }
        }
    }

    fun undo_report_validator_impl(
        verified_cap: ValidatorOperationCap,
        reportee_addr: address,
        validator_report_records: &mut VecMap<address, VecSet<address>>,
    ) {
        assert!(validator_report_records.contains(&reportee_addr), EReportRecordNotFound);
        let reporters = validator_report_records.get_mut(&reportee_addr);

        let reporter_addr = *verified_cap.verified_operation_cap_address();
        assert!(reporters.contains(&reporter_addr), EReportRecordNotFound);

        reporters.remove(&reporter_addr);
        if (reporters.is_empty()) {
            validator_report_records.remove(&reportee_addr);
        }
    }

    // ==== validator metadata management functions ====

    /// Create a new `UnverifiedValidatorOperationCap`, transfer it to the
    /// validator and registers it. The original object is thus revoked.
    public(package) fun rotate_operation_cap(
        self: &mut IotaSystemStateInnerV2,
        ctx: &mut TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        validator.new_unverified_validator_operation_cap_and_transfer(ctx);
    }

    /// Update a validator's name.
    public(package) fun update_validator_name(
        self: &mut IotaSystemStateInnerV2,
        name: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);

        validator.update_name(name);
    }

    /// Update a validator's description
    public(package) fun update_validator_description(
        self: &mut IotaSystemStateInnerV2,
        description: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        validator.update_description(description);
    }

    /// Update a validator's image url
    public(package) fun update_validator_image_url(
        self: &mut IotaSystemStateInnerV2,
        image_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        validator.update_image_url(image_url);
    }

    /// Update a validator's project url
    public(package) fun update_validator_project_url(
        self: &mut IotaSystemStateInnerV2,
        project_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        validator.update_project_url(project_url);
    }

    /// Update a validator's network address.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_network_address(
        self: &mut IotaSystemStateInnerV2,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_network_address(network_address);
        let validator :&Validator = validator; // Force immutability for the following call
        self.validators.assert_no_pending_or_active_duplicates(validator);
    }

    /// Update candidate validator's network address.
    public(package) fun update_candidate_validator_network_address(
        self: &mut IotaSystemStateInnerV2,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_network_address(network_address);
    }

    /// Update a validator's p2p address.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_p2p_address(
        self: &mut IotaSystemStateInnerV2,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_p2p_address(p2p_address);
        let validator :&Validator = validator; // Force immutability for the following call
        self.validators.assert_no_pending_or_active_duplicates(validator);
    }

    /// Update candidate validator's p2p address.
    public(package) fun update_candidate_validator_p2p_address(
        self: &mut IotaSystemStateInnerV2,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_p2p_address(p2p_address);
    }

    /// Update a validator's narwhal primary address.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_primary_address(
        self: &mut IotaSystemStateInnerV2,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_primary_address(primary_address);
    }

    /// Update candidate validator's narwhal primary address.
    public(package) fun update_candidate_validator_primary_address(
        self: &mut IotaSystemStateInnerV2,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_primary_address(primary_address);
    }

    /// Update a validator's narwhal worker address.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_worker_address(
        self: &mut IotaSystemStateInnerV2,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_worker_address(worker_address);
    }

    /// Update candidate validator's narwhal worker address.
    public(package) fun update_candidate_validator_worker_address(
        self: &mut IotaSystemStateInnerV2,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_worker_address(worker_address);
    }

    /// Update a validator's public key of protocol key and proof of possession.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_protocol_pubkey(
        self: &mut IotaSystemStateInnerV2,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_protocol_pubkey(protocol_pubkey, proof_of_possession);
        let validator :&Validator = validator; // Force immutability for the following call
        self.validators.assert_no_pending_or_active_duplicates(validator);
    }

    /// Update candidate validator's public key of protocol key and proof of possession.
    public(package) fun update_candidate_validator_protocol_pubkey(
        self: &mut IotaSystemStateInnerV2,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_protocol_pubkey(protocol_pubkey, proof_of_possession);
    }

    /// Update a validator's public key of worker key.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_worker_pubkey(
        self: &mut IotaSystemStateInnerV2,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_worker_pubkey(worker_pubkey);
        let validator :&Validator = validator; // Force immutability for the following call
        self.validators.assert_no_pending_or_active_duplicates(validator);
    }

    /// Update candidate validator's public key of worker key.
    public(package) fun update_candidate_validator_worker_pubkey(
        self: &mut IotaSystemStateInnerV2,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_worker_pubkey(worker_pubkey);
    }

    /// Update a validator's public key of network key.
    /// The change will only take effects starting from the next epoch.
    public(package) fun update_validator_next_epoch_network_pubkey(
        self: &mut IotaSystemStateInnerV2,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = self.validators.get_validator_mut_with_ctx(ctx);
        validator.update_next_epoch_network_pubkey(network_pubkey);
        let validator :&Validator = validator; // Force immutability for the following call
        self.validators.assert_no_pending_or_active_duplicates(validator);
    }

    /// Update candidate validator's public key of network key.
    public(package) fun update_candidate_validator_network_pubkey(
        self: &mut IotaSystemStateInnerV2,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
        candidate.update_candidate_network_pubkey(network_pubkey);
    }

    /// This function should be called at the end of an epoch, and advances the system to the next epoch.
    /// It does the following things:
    /// 1. Add storage charge to the storage fund.
    /// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
    ///    gas coins.
    /// 3. Distribute computation charge to validator stake.
    /// 4. Update all validators.
    public(package) fun advance_epoch(
        self: &mut IotaSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        mut storage_reward: Balance<IOTA>,
        mut computation_reward: Balance<IOTA>,
        mut storage_rebate_amount: u64,
        mut non_refundable_storage_fee_amount: u64,
        reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<IOTA> {
        let prev_epoch_start_timestamp = self.epoch_start_timestamp_ms;
        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;

        let bps_denominator_u64 = BASIS_POINT_DENOMINATOR as u64;
        // Rates can't be higher than 100%.
        assert!(reward_slashing_rate <= bps_denominator_u64, EBpsTooLarge);

        // Accumulate the gas summary during safe_mode before processing any rewards:
        let safe_mode_storage_rewards = self.safe_mode_storage_rewards.withdraw_all();
        storage_reward.join(safe_mode_storage_rewards);
        let safe_mode_computation_rewards = self.safe_mode_computation_rewards.withdraw_all();
        computation_reward.join(safe_mode_computation_rewards);
        storage_rebate_amount = storage_rebate_amount + self.safe_mode_storage_rebates;
        self.safe_mode_storage_rebates = 0;
        non_refundable_storage_fee_amount = non_refundable_storage_fee_amount + self.safe_mode_non_refundable_storage_fee;
        self.safe_mode_non_refundable_storage_fee = 0;

        let storage_charge = storage_reward.value();
        let computation_charge = computation_reward.value();

        // Include stake subsidy in the rewards given out to validators and stakers.
        // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
        // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
        let stake_subsidy =
            if (ctx.epoch() >= self.parameters.stake_subsidy_start_epoch  &&
                epoch_start_timestamp_ms >= prev_epoch_start_timestamp + self.parameters.epoch_duration_ms)
            {
                self.stake_subsidy.advance_epoch()
            } else {
                balance::zero()
            };

        let stake_subsidy_amount = stake_subsidy.value();
        computation_reward.join(stake_subsidy);


        self.epoch = self.epoch + 1;
        // Sanity check to make sure we are advancing to the right epoch.
        assert!(new_epoch == self.epoch, EAdvancedToWrongEpoch);

        let computation_reward_amount_before_distribution = computation_reward.value();

        self.validators.advance_epoch(
            &mut computation_reward,
            &mut self.validator_report_records,
            reward_slashing_rate,
            self.parameters.validator_low_stake_threshold,
            self.parameters.validator_very_low_stake_threshold,
            self.parameters.validator_low_stake_grace_period,
            ctx,
        );

        let new_total_stake = self.validators.total_stake();

        let computation_reward_amount_after_distribution = computation_reward.value();
        let computation_reward_distributed = computation_reward_amount_before_distribution - computation_reward_amount_after_distribution;

        self.protocol_version = next_protocol_version;

        // Derive the reference gas price for the new epoch
        self.reference_gas_price = self.validators.derive_reference_gas_price();
        // Because of precision issues with integer divisions, we expect that there will be some
        // remaining balance in `storage_fund_reward` and `computation_reward`.
        // All of these go to the storage fund.
        let leftover_staking_rewards = computation_reward;
        let leftover_storage_fund_inflow = leftover_staking_rewards.value();

        // Burning leftover rewards
        self.iota_treasury_cap.burn_balance(leftover_staking_rewards, ctx);

        let refunded_storage_rebate =
            self.storage_fund.advance_epoch(
                storage_reward,
                storage_rebate_amount,
                non_refundable_storage_fee_amount,
            );

        event::emit(
            //TODO: Add additional information (e.g., how much was burned, how much was leftover, etc.)
            SystemEpochInfoEvent {
                epoch: self.epoch,
                protocol_version: self.protocol_version,
                reference_gas_price: self.reference_gas_price,
                total_stake: new_total_stake,
                storage_charge,
                storage_rebate: storage_rebate_amount,
                storage_fund_balance: self.storage_fund.total_balance(),
                stake_subsidy_amount,
                total_gas_fees: computation_charge,
                total_stake_rewards_distributed: computation_reward_distributed,
                leftover_storage_fund_inflow,
            }
        );
        self.safe_mode = false;
        // Double check that the gas from safe mode has been processed.
        assert!(self.safe_mode_storage_rebates == 0
            && self.safe_mode_storage_rewards.value() == 0
            && self.safe_mode_computation_rewards.value() == 0, ESafeModeGasNotProcessed);

        // Return the storage rebate split from storage fund that's already refunded to the transaction senders.
        // This will be burnt at the last step of epoch change programmable transaction.
        refunded_storage_rebate
    }

    /// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
    /// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
    public(package) fun epoch(self: &IotaSystemStateInnerV2): u64 {
        self.epoch
    }

    public(package) fun protocol_version(self: &IotaSystemStateInnerV2): u64 {
        self.protocol_version
    }

    public(package) fun system_state_version(self: &IotaSystemStateInnerV2): u64 {
        self.system_state_version
    }

    /// This function always return the genesis system state version, which is used to create the system state in genesis.
    /// It should never change for a given network.
    public(package) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    /// Returns unix timestamp of the start of current epoch
    public(package) fun epoch_start_timestamp_ms(self: &IotaSystemStateInnerV2): u64 {
        self.epoch_start_timestamp_ms
    }

    /// Returns the total amount staked with `validator_addr`.
    /// Aborts if `validator_addr` is not an active validator.
    public(package) fun validator_stake_amount(self: &IotaSystemStateInnerV2, validator_addr: address): u64 {
        self.validators.validator_total_stake_amount(validator_addr)
    }

    /// Returns the staking pool id of a given validator.
    /// Aborts if `validator_addr` is not an active validator.
    public(package) fun validator_staking_pool_id(self: &IotaSystemStateInnerV2, validator_addr: address): ID {

        self.validators.validator_staking_pool_id(validator_addr)
    }

    /// Returns reference to the staking pool mappings that map pool ids to active validator addresses
    public(package) fun validator_staking_pool_mappings(self: &IotaSystemStateInnerV2): &Table<ID, address> {

        self.validators.staking_pool_mappings()
    }

    /// Returns all the validators who are currently reporting `addr`
    public(package) fun get_reporters_of(self: &IotaSystemStateInnerV2, addr: address): VecSet<address> {

        if (self.validator_report_records.contains(&addr)) {
            self.validator_report_records[&addr]
        } else {
            vec_set::empty()
        }
    }

    public(package) fun get_storage_fund_total_balance(self: &IotaSystemStateInnerV2): u64 {
        self.storage_fund.total_balance()
    }

    public(package) fun get_storage_fund_object_rebates(self: &IotaSystemStateInnerV2): u64 {
        self.storage_fund.total_object_storage_rebates()
    }

    public(package) fun pool_exchange_rates(
        self: &mut IotaSystemStateInnerV2,
        pool_id: &ID
    ): &Table<u64, PoolTokenExchangeRate>  {
        let validators = &mut self.validators;
        validators.pool_exchange_rates(pool_id)
    }

    public(package) fun active_validator_addresses(self: &IotaSystemStateInnerV2): vector<address> {
        let validator_set = &self.validators;
        validator_set.active_validator_addresses()
    }

    #[allow(lint(self_transfer))]
    /// Extract required Balance from vector of Coin<IOTA>, transfer the remainder back to sender.
    fun extract_coin_balance(mut coins: vector<Coin<IOTA>>, amount: option::Option<u64>, ctx: &mut TxContext): Balance<IOTA> {
        let mut merged_coin = coins.pop_back();
        merged_coin.join_vec(coins);

        let mut total_balance = merged_coin.into_balance();
        // return the full amount if amount is not specified
        if (amount.is_some()) {
            let amount = amount.destroy_some();
            let balance = total_balance.split(amount);
            // transfer back the remainder if non zero.
            if (total_balance.value() > 0) {
                transfer::public_transfer(total_balance.into_coin(ctx), ctx.sender());
            } else {
                total_balance.destroy_zero();
            };
            balance
        } else {
            total_balance
        }
    }

    #[test_only]
    /// Return the current validator set
    public(package) fun validators(self: &IotaSystemStateInnerV2): &ValidatorSet {
        &self.validators
    }

    #[test_only]
    /// Return the currently active validator by address
    public(package) fun active_validator_by_address(self: &IotaSystemStateInnerV2, validator_address: address): &Validator {
        self.validators().get_active_validator_ref(validator_address)
    }

    #[test_only]
    /// Return the currently pending validator by address
    public(package) fun pending_validator_by_address(self: &IotaSystemStateInnerV2, validator_address: address): &Validator {
        self.validators().get_pending_validator_ref(validator_address)
    }

    #[test_only]
    /// Return the currently candidate validator by address
    public(package) fun candidate_validator_by_address(self: &IotaSystemStateInnerV2, validator_address: address): &Validator {
        validators(self).get_candidate_validator_ref(validator_address)
    }

    #[test_only]
    public(package) fun get_stake_subsidy_distribution_counter(self: &IotaSystemStateInnerV2): u64 {
        self.stake_subsidy.get_distribution_counter()
    }

    #[test_only]
    public(package) fun set_epoch_for_testing(self: &mut IotaSystemStateInnerV2, epoch_num: u64) {
        self.epoch = epoch_num
    }

    #[test_only]
    public(package) fun request_add_validator_for_testing(
        self: &mut IotaSystemStateInnerV2,
        min_joining_stake_for_testing: u64,
        ctx: &TxContext,
    ) {
        assert!(
            self.validators.next_epoch_validator_count() < self.parameters.max_validator_count,
            ELimitExceeded,
        );

        self.validators.request_add_validator(min_joining_stake_for_testing, ctx);
    }

    // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
    // candidate validator - bypassing the proof of possession check and other metadata validation
    // in the process.
    #[test_only]
    public(package) fun request_add_validator_candidate_for_testing(
        self: &mut IotaSystemStateInnerV2,
        pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        proof_of_possession: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        image_url: vector<u8>,
        project_url: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        gas_price: u64,
        commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let validator = validator::new_for_testing(
            ctx.sender(),
            pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            option::none(),
            gas_price,
            commission_rate,
            false, // not an initial validator active at genesis
            ctx
        );

        self.validators.request_add_validator_candidate(validator, ctx);
    }

}
