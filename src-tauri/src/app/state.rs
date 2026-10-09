use std::sync::Mutex;

use crate::infra::ovpn_engine::OvpnHandle;
use crate::infra::wg_engine::EngineHandle;

#[derive(Default)]
pub struct AppState {
    pub engine: Mutex<Option<EngineHandle>>,
    pub ovpn: Mutex<Option<OvpnHandle>>,
}
