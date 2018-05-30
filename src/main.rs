extern crate walkdir;

use std::env;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let current_dir = env::current_dir().expect("an access to a current directory");
    println!("{:?}", current_dir);
    for entry in WalkDir::new(current_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_xcode_file)
    {
        println!("{}", entry.path().display());
    }
}

fn is_xcode_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        ext == "xcodeproj" || ext == "xcworkspace"
    } else {
        false
    }
}

// TODO: handle paths under .xcodeproj:
// /Users/zummenix/projects/Backgrounder/Backgrounder.xcodeproj
// /Users/zummenix/projects/Backgrounder/Backgrounder.xcodeproj/project.xcworkspace
// /Users/zummenix/projects/Backgrounder/Pods/Pods.xcodeproj
// /Users/zummenix/projects/Backgrounder/Backgrounder.xcworkspace

// TODO: filter Pods (smart)
// /Users/zummenix/projects/Backgrounder/Backgrounder.xcodeproj
// /Users/zummenix/projects/Backgrounder/Pods/Pods.xcodeproj
// /Users/zummenix/projects/Backgrounder/Backgrounder.xcworkspace

// TODO: filter note_modules (smart)
// /Users/zummenix/projects/RNApp/node_modules/react-native/Libraries/Sample/Sample.xcodeproj
// /Users/zummenix/projects/RNApp/node_modules/react-native/Libraries/LinkingIOS/RCTLinking.xcodeproj
