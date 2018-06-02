#[macro_use(expect)]
#[cfg(test)]
extern crate expectest;
extern crate walkdir;

use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn files(root: &Path) -> Vec<PathBuf> {
    let iter = WalkDir::new(root).into_iter().filter_map(|e| e.ok());
    files_internal(root, iter)
}

fn files_internal<I, F>(root: &Path, files_iter: I) -> Vec<PathBuf>
where
    I: Iterator<Item = F>,
    F: FilePath,
{
    files_iter
        .filter_map(|file_path| {
            let path = file_path.path();
            if is_xcodeproj(path) {
                return Some(path.to_owned());
            }
            if is_xcworkspace(path) {
                // Skip workspaces under xcodeproj, example:
                // /Backgrounder/Backgrounder.xcodeproj
                // /Backgrounder/Backgrounder.xcodeproj/project.xcworkspace
                // /Backgrounder/Backgrounder.xcworkspace
                if path.parent().map(is_xcodeproj).unwrap_or(false) {
                    return None;
                } else {
                    return Some(path.to_owned());
                }
            }
            None
        })
        .collect()
}

pub fn is_xcodeproj(path: &Path) -> bool {
    has_extension(path, "xcodeproj")
}

pub fn is_xcworkspace(path: &Path) -> bool {
    has_extension(path, "xcworkspace")
}

fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .map(|ext| ext == extension)
        .unwrap_or(false)
}

trait FilePath {
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

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;

    impl FilePath for PathBuf {
        fn path(&self) -> &Path {
            self.as_path()
        }
    }

    #[test]
    fn filters_out_project_and_workspace_files() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/examples"),
            PathBuf::from("/projects/my/some.txt"),
            PathBuf::from("/projects/my/App.xcodeproj"),
            PathBuf::from("/projects/my/App.xcodeproj/project.xcworkspace"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        let result = vec![
            PathBuf::from("/projects/my/App.xcodeproj"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        expect!(files_internal(&root, input.into_iter())).to(be_equal_to(result));
    }
}
