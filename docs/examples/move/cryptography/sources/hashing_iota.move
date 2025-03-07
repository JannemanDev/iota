module example::hashing_iota {
    use iota::hash;

    /// Object that holds the output hash value.
    public struct Output has key, store {
        id: UID,
        value: vector<u8>
    }

    public fun hash_data(data: vector<u8>, recipient: address, ctx: &mut TxContext) {
        let hashed = Output {
            id: object::new(ctx),
            value: hash::keccak256(&data),
        };
        // Transfer an output data object holding the hashed data to the recipient.
        transfer::public_transfer(hashed, recipient)
    }
}