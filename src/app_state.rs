use crate::error::Res;

#[derive(Clone)]
pub struct AppState {

}

impl AppState {
    pub fn new() -> Res<Self> {
        Ok(Self {})
    }
}