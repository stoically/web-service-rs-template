use std::{ops::Deref, sync::Arc};

use timed_locks::RwLock;

use crate::Config;

/// Shared state.
#[derive(Clone)]
pub struct State {
    pub inner: Arc<RwLock<InnerState>>,
}

impl Deref for State {
    type Target = Arc<RwLock<InnerState>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct InnerState {
    pub config: Config,
}
