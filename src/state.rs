use crate::database::connect::DB;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DB,
}
