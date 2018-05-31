extern crate walkdir;

use std::path::{Path, PathBuf};
use walkdir::DirEntry;

pub fn is_xcode_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        ext == "xcodeproj" || ext == "xcworkspace"
    } else {
        false
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FileKind {
    Project,
    Workspace,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileModel {
    pub path: PathBuf,
    pub kind: FileKind,
}

pub fn files(root: &Path) -> Vec<FileModel> {
    Vec::new()
}

pub trait FilePathProvider {
    fn iter(&self) -> Box<Iterator<Item = &FilePath>>;
}

pub trait FilePath {
    fn path(&self) -> &Path;
}

impl FilePath for DirEntry {
    fn path(&self) -> &Path {
        self.path()
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
