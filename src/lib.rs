use once_cell::sync::OnceCell;

use x64dbg_bridge::{PauseDebugEvent, PluginHandle, PluginInit};

static PLUGIN: OnceCell<PluginHandle> = OnceCell::new();

#[no_mangle]
extern "C" fn CBPAUSEDEBUG(_ty: u32, _event: &mut PauseDebugEvent) {
    x64dbg_bridge::util::plugin_print(format!(
        "{:#?}",
        x64dbg_bridge::debug::register_dump().unwrap()
    ));
}

#[no_mangle]
extern "C" fn pluginit(plugin: &mut PluginInit) -> bool {
    plugin.initialize();
    plugin.name("ghidra");

    PLUGIN.set(plugin.handle()).is_ok()
}
