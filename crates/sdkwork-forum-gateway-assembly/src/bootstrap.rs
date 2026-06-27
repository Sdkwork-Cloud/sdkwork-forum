//! Generated gateway bootstrap for sdkwork-forum.
//! Route crates currently expose route descriptors; assembly stays empty until Router gateway_mount ships.

use axum::Router;

pub struct ApplicationAssembly {
    pub router: Router,
}

pub fn assemble_application_router() -> ApplicationAssembly {
    ApplicationAssembly {
        router: Router::new(),
    }
}
