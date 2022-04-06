use crate::memory::{consume_region, Region};
use crate::serde::from_slice;
use crate::{OwnedDeps, Deps, DepsMut, Env};
use crate::imports::{ExternalApi, ExternalQuerier, ExternalStorage};


extern "C" {
    fn global_env() -> u32;
}

pub struct GlobalApi {}
impl GlobalApi {
    pub fn env() -> Env {
        let env_ptr = unsafe { global_env() };
        let vec_env = unsafe { consume_region(env_ptr as *mut Region) };
        from_slice(&vec_env).unwrap()
    }

    // By existing design, ownership of Deps is intended to be held outside the contract logic.
    // So, it is not possible to provide a simple getter style without changing the existing design.
    pub fn with_deps<C, R>(callback: C) -> R
    where
        C: FnOnce(Deps) -> R,
    {
        let deps = make_dependencies();
        callback(deps.as_ref())
    }

    pub fn with_deps_mut<C, R>(callback: C) -> R
    where
        C: FnOnce(DepsMut) -> R,
    {
        let mut deps = make_dependencies();
        callback(deps.as_mut())
    }
}

/// Makes all bridges to external dependencies (i.e. Wasm imports) that are injected by the VM
fn make_dependencies() -> OwnedDeps<ExternalStorage, ExternalApi, ExternalQuerier> {
    OwnedDeps {
        storage: ExternalStorage::new(),
        api: ExternalApi::new(),
        querier: ExternalQuerier::new(),
    }
}
