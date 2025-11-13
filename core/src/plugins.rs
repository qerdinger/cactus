use std::path::{Path, PathBuf};
use anyhow::Context;
use libloading::{Library, Symbol};
use plugin_api::{Plugin, EntryFn, ENTRY_SYMBOL};

pub struct LoadedPlugin {
    pub _lib: Library,
    pub plugin: Box<dyn Plugin>,
}

pub fn load_from_dir(dir: &Path) -> anyhow::Result<Vec<LoadedPlugin>> {
    let mut out = vec![];
    for entry in std::fs::read_dir(dir).context("reading plugins dir")? {
        let path = entry?.path();
        if is_dylib(&path) {
            unsafe {
                let lib = Library::new(&path).with_context(|| format!("load {:?}", path))?;
                let entry: Symbol<EntryFn> = lib.get(ENTRY_SYMBOL)?;
                let plugin = entry();
                out.push(LoadedPlugin { _lib: lib, plugin });
            }
        }
    }
    Ok(out)
}

fn is_dylib(p: &PathBuf) -> bool {
    let s = p.extension().and_then(|e| e.to_str()).unwrap_or_default();
    #[cfg(target_os="macos")] { s == "dylib" }
    #[cfg(target_os="linux")] { s == "so" }
    #[cfg(target_os="windows")] { s == "dll" }
}
