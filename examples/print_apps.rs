extern crate ns_bindings;

use ns_bindings::nsarray::NSArrayRef;
use ns_bindings::nsstring::NSStringRef;
use ns_bindings::{
    INSRunningApplication, INSWorkspace, NSRunningApplication, NSWorkspace,
    NSWorkspace_NSWorkspaceRunningApplications, INSURL,
};

#[derive(Debug)]
pub struct MacOSApplication {
    pub name: Option<String>,
    pub bundle_identifier: Option<String>,
    pub process_identifier: i32,
    pub bundle_path: Option<String>,
}

fn main() {
    let apps = running_applications();
    for app in apps {
        println!("{:?}", app);
    }
}
pub fn running_applications() -> Vec<MacOSApplication> {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let apps: NSArrayRef<NSRunningApplication> = workspace.runningApplications().into();
        apps.into_iter()
            .map(|app| {
                let application = NSRunningApplication(*app as *mut _);

                let name = NSStringRef::new(application.localizedName().0);
                let bundle_id = NSStringRef::new(application.bundleIdentifier().0);
                let bundle_path = NSStringRef::new(application.bundleURL().path().0);

                MacOSApplication {
                    name: name.as_str().map(|s| s.to_string()),
                    bundle_identifier: bundle_id.as_str().map(|s| s.to_string()),
                    bundle_path: bundle_path.as_str().map(|s| s.to_string()),
                    process_identifier: application.processIdentifier(),
                }
            })
            .collect()
    }
}
