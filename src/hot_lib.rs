#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
pub mod library_bridge {
    pub use lib::helpers::{LibState, ReloadFlags};
    pub use lib::program::{Program, ProgramError};

    // Specify which program we want to run here.
    // This should also be specified in lib/lib.rs
    pub use lib::demo::DemoProgram as CurrentProgram;

    // Specific hot reload helpers.
    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    // a monotonically increasing counter (starting with 0) that counts library reloads
    #[lib_version]
    pub fn version() -> usize {}
}

#[cfg(not(feature = "reload"))]
pub mod library_bridge {
    pub use lib::helpers::{LibState, ReloadFlags};
    pub use lib::program::{Program, ProgramError};

    // Specify which program we want to run here.
    // This should also be specified in lib/lib.rs
    pub use lib::demo::DemoProgram as CurrentProgram;

    // Include lib file directly since it is not done via the hot-reload module.
    pub use lib::*;
}