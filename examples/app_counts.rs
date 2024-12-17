extern crate ns_bindings;

use std::os::raw::c_uint;

use ns_bindings::{
    INSArray, INSWorkspace, NSArray, NSRunningApplication, NSWorkspace,
    NSWorkspace_NSWorkspaceRunningApplications,
};

fn main() {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();

        let apps: &NSArray = &workspace.runningApplications();

        let count: c_uint = INSArray::<NSRunningApplication>::count(apps) as c_uint;

        println!("Running applications count: {}", count);
    }
}
