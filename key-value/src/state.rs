use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static BUCKET_NAME: &[u8] = b"keyvalue_bucket";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: String
}

pub fn resolver(storage: &mut dyn Storage) -> Bucket<Value> {
    bucket(storage, BUCKET_NAME)
}

pub fn resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Value> {
    bucket_read(storage, BUCKET_NAME)
}
