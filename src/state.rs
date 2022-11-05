use std::sync::Arc;

use timed_locks::RwLock;

use crate::Config;

/// Shared state.
pub type State = Arc<RwLock<InnerState>>;

/// Shared state.
pub struct InnerState {
    pub config: Config,
}
