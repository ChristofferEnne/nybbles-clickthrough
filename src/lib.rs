use godot::prelude::*;
use godot::classes::DisplayServer;
use godot::classes::display_server::HandleType;

struct ClickthroughExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ClickthroughExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ClickThroughManager {
    base: Base<Node>,
}

#[godot_api]
impl INode for ClickThroughManager {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl ClickThroughManager {
    /// Toggles the OS-level clickthrough state of the Godot window.
    #[func]
    fn set_clickthrough(&mut self, enabled: bool) {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::Foundation::HWND;
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE,
                WS_EX_LAYERED, WS_EX_TRANSPARENT
            };

            let mut ds = DisplayServer::singleton();

            let handle_int = ds.window_get_native_handle(HandleType::WINDOW_HANDLE);

            // `window_get_native_handle()` already returns a native integer handle on Windows.
            let hwnd = handle_int as isize as HWND;

            unsafe {
                let mut ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

                if enabled {
                    // Apply both layered and transparent styles to allow clickthrough.
                    ex_style |= (WS_EX_LAYERED | WS_EX_TRANSPARENT) as isize;
                } else {
                    // Remove only the transparent style.
                    // We leave WS_EX_LAYERED alone as Godot may be using it for visual transparency.
                    ex_style &= !(WS_EX_TRANSPARENT as isize);
                }

                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style);
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            godot_print!("Native window clickthrough flags are only implemented for Windows OS.");
        }
    }
}