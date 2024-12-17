extern crate ns_bindings;

use ns_bindings::{INSRunningApplication, INSWorkspace, NSWorkspace_NSDeprecated};

fn main() {
    let active_app = ns_bindings::NSWorkspace::sharedWorkspace();

    // match active_app {
    //     Some(app) => {
    //         let name = app.localized_name();
    //         println!("Active Application: {}", name);
    //     }
    //     None => {
    //         println!("No active application found.");
    //     }
    // }
}
