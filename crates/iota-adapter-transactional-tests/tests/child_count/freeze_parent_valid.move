// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests valid freezing of an object that has children

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use iota::dynamic_object_field as ofield;

    public struct S has key, store {
        id: iota::object::UID,
    }

    public struct R has key, store {
        id: iota::object::UID,
        s: S,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = iota::object::new(ctx);
        iota::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: iota::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun remove(parent: &mut S, idx: u64) {
        let S { id } = ofield::remove(&mut parent.id, idx);
        iota::object::delete(id)
    }

    public entry fun remove_and_add(parent: &mut S, idx: u64) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, child)
    }

    public entry fun remove_and_wrap(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, R { id: iota::object::new(ctx), s: child })
    }

    public entry fun delete(s: S) {
        let S { id } = s;
        iota::object::delete(id)
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: iota::object::new(ctx), s };
        iota::transfer::public_transfer(r, tx_context::sender(ctx))
    }

    public entry fun freeze_object(s: S) {
        iota::transfer::public_freeze_object(s)
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# view-object 2,0

//# run test::m::remove --sender A --args object(2,0) 0

//# run test::m::freeze_object --sender A --args object(2,0)
