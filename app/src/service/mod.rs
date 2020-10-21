/*
release
- search, info, get, list, download, install, uninstall

datastore
- create, delete, get, list, update

cache
-

executor (local, remote via ssh)
-

context
- create, delete, get, list, update
- switch to different executor

 */
use huber_common::result::Result;

pub(crate) mod cache;
pub(crate) mod context;
pub(crate) mod datastore;
pub(crate) mod release;

trait ItemOperationTrait {
    type Item;
    type ItemInstance;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance>;
    fn delete(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<Self::ItemInstance>>;
    fn get(&self, name: &str) -> Result<Self::ItemInstance>;
}

trait ItemSearchTrait {
    type Item;

    fn search(&self, pattern: &str) -> Result<Vec<Self::Item>>;
    fn search_unmanaged(&self, obj: &Self::Item) -> Result<Self::Item>;
    fn get(&self, name: &str) -> Result<Self::Item>;
}