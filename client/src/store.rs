use yewdux::Store;

use crate::types::DtoPost;
#[derive(Default, Debug, Clone, PartialEq, Eq, Store)]
pub struct AppStore {
    pub posts: Vec<DtoPost>,
}
