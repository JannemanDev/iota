// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use better_any::{Tid, TidAble};
use move_binary_format::CompiledModule;
use move_bytecode_utils::module_cache::GetModule;
use move_core_types::{language_storage::ModuleId, resolver::ModuleResolver};

use crate::{
    base_types::{ObjectID, SequenceNumber, VersionNumber},
    committee::EpochId,
    error::{IotaError, IotaResult},
    inner_temporary_store::WrittenObjects,
    object::{Object, Owner},
    storage::{
        BackingPackageStore, ChildObjectResolver, ObjectStore, PackageObject, get_module,
        get_module_by_id, load_package_object_from_object_store,
    },
    transaction::{
        InputObjectKind, InputObjects, ObjectReadResult, Transaction, TransactionDataAPI,
    },
};

// TODO: We should use AuthorityTemporaryStore instead.
// Keeping this functionally identical to AuthorityTemporaryStore is a pain.
#[derive(Debug, Default, Tid)]
pub struct InMemoryStorage {
    persistent: BTreeMap<ObjectID, Object>,
}

impl BackingPackageStore for InMemoryStorage {
    fn get_package_object(&self, package_id: &ObjectID) -> IotaResult<Option<PackageObject>> {
        load_package_object_from_object_store(self, package_id)
    }
}

impl ChildObjectResolver for InMemoryStorage {
    fn read_child_object(
        &self,
        parent: &ObjectID,
        child: &ObjectID,
        child_version_upper_bound: SequenceNumber,
    ) -> IotaResult<Option<Object>> {
        let child_object = match self.persistent.get(child).cloned() {
            None => return Ok(None),
            Some(obj) => obj,
        };
        let parent = *parent;
        if child_object.owner != Owner::ObjectOwner(parent.into()) {
            return Err(IotaError::InvalidChildObjectAccess {
                object: *child,
                given_parent: parent,
                actual_owner: child_object.owner,
            });
        }
        if child_object.version() > child_version_upper_bound {
            return Err(IotaError::UnsupportedFeature {
                error: "TODO InMemoryStorage::read_child_object does not yet support bounded reads"
                    .to_owned(),
            });
        }
        Ok(Some(child_object))
    }

    fn get_object_received_at_version(
        &self,
        owner: &ObjectID,
        receiving_object_id: &ObjectID,
        receive_object_at_version: SequenceNumber,
        _epoch_id: EpochId,
    ) -> IotaResult<Option<Object>> {
        let recv_object = match self.persistent.get(receiving_object_id).cloned() {
            None => return Ok(None),
            Some(obj) => obj,
        };
        if recv_object.owner != Owner::AddressOwner((*owner).into()) {
            return Ok(None);
        }

        if recv_object.version() != receive_object_at_version {
            return Ok(None);
        }
        Ok(Some(recv_object))
    }
}

impl ModuleResolver for InMemoryStorage {
    type Error = IotaError;

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        get_module(self, module_id)
    }
}

impl ModuleResolver for &mut InMemoryStorage {
    type Error = IotaError;

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        (**self).get_module(module_id)
    }
}

impl ObjectStore for InMemoryStorage {
    fn get_object(&self, object_id: &ObjectID) -> crate::storage::error::Result<Option<Object>> {
        Ok(self.persistent.get(object_id).cloned())
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> crate::storage::error::Result<Option<Object>> {
        Ok(self
            .persistent
            .get(object_id)
            .and_then(|obj| {
                if obj.version() == version {
                    Some(obj)
                } else {
                    None
                }
            })
            .cloned())
    }
}

impl ObjectStore for &mut InMemoryStorage {
    fn get_object(&self, object_id: &ObjectID) -> crate::storage::error::Result<Option<Object>> {
        Ok(self.persistent.get(object_id).cloned())
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> crate::storage::error::Result<Option<Object>> {
        Ok(self
            .persistent
            .get(object_id)
            .and_then(|obj| {
                if obj.version() == version {
                    Some(obj)
                } else {
                    None
                }
            })
            .cloned())
    }
}

impl GetModule for InMemoryStorage {
    type Error = IotaError;
    type Item = CompiledModule;

    fn get_module_by_id(&self, id: &ModuleId) -> anyhow::Result<Option<Self::Item>, Self::Error> {
        get_module_by_id(self, id)
    }
}

impl InMemoryStorage {
    pub fn new(objects: Vec<Object>) -> Self {
        let mut persistent = BTreeMap::new();
        for o in objects {
            persistent.insert(o.id(), o);
        }
        Self { persistent }
    }

    pub fn read_input_objects_for_transaction(&self, transaction: &Transaction) -> InputObjects {
        let mut input_objects = Vec::new();
        for kind in transaction.transaction_data().input_objects().unwrap() {
            let obj: Object = match kind {
                InputObjectKind::MovePackage(id)
                | InputObjectKind::ImmOrOwnedMoveObject((id, _, _))
                | InputObjectKind::SharedMoveObject { id, .. } => {
                    self.get_object(&id).unwrap().clone()
                }
            };

            input_objects.push(ObjectReadResult::new(kind, obj.into()));
        }
        input_objects.into()
    }

    pub fn get_object(&self, id: &ObjectID) -> Option<&Object> {
        self.persistent.get(id)
    }

    pub fn get_objects(&self, objects: &[ObjectID]) -> Vec<Option<&Object>> {
        let mut result = Vec::new();
        for id in objects {
            result.push(self.get_object(id));
        }
        result
    }

    pub fn insert_object(&mut self, object: Object) {
        let id = object.id();
        self.persistent.insert(id, object);
    }

    pub fn remove_object(&mut self, object_id: ObjectID) -> Option<Object> {
        self.persistent.remove(&object_id)
    }

    pub fn objects(&self) -> &BTreeMap<ObjectID, Object> {
        &self.persistent
    }

    pub fn into_inner(self) -> BTreeMap<ObjectID, Object> {
        self.persistent
    }

    pub fn finish(&mut self, written: WrittenObjects) {
        for (_id, new_object) in written {
            debug_assert!(new_object.id() == _id);
            self.insert_object(new_object);
        }
    }
}
