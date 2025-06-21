use std::{mem, path::Path};

use eyre::OptionExt;
use tracing::instrument;
use windows::{
    core::{s, HSTRING},
    Win32::{
        Foundation::FreeLibrary,
        System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    },
};

use crate::LauncherResult;

#[instrument(skip_all, err)]
pub fn require_steam<P: AsRef<Path>>(_game_binary: P) -> LauncherResult<()> {
    Ok(())
}
