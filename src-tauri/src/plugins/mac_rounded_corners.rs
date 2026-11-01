#[cfg(target_os = "macos")]
use tauri::{Context, Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSWindow, NSWindowButton};

/// Offsets from the position centered in the app's 56-point navbar.
#[cfg(target_os = "macos")]
pub struct TrafficLightsConfig {
    /// Offset in logical points (positive = right, negative = left).
    pub offset_x: f64,
    /// Offset in logical points (positive = down, negative = up).
    pub offset_y: f64,
}

#[cfg(target_os = "macos")]
impl Default for TrafficLightsConfig {
    fn default() -> Self {
        // Edit these offsets to move the controls at startup and after native layout changes.
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Configure the macOS window before creation so Tao owns and restores its layout.
#[cfg(target_os = "macos")]
pub fn configure_context<R: Runtime>(mut context: Context<R>) -> Context<R> {
    let config = TrafficLightsConfig::default();
    let window = context
        .config_mut()
        .app
        .windows
        .iter_mut()
        .find(|window| window.label == "main")
        .expect("main window configuration is missing");

    window.decorations = true;
    window.title_bar_style = tauri::TitleBarStyle::Overlay;
    window.hidden_title = true;
    // Apply the native styling in setup before the window's first visible frame.
    window.visible = false;
    window.traffic_light_position = Some(tauri::utils::config::LogicalPosition {
        x: 20.0 + config.offset_x,
        // Tao's inset positions the title-bar container, including its native padding.
        y: 30.0 + config.offset_y,
    });

    context
}

/// Applies the initial style synchronously from Tauri's main-thread setup hook.
#[cfg(target_os = "macos")]
pub fn initialize_main_window<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    let ns_window = window.ns_window()?;
    // SAFETY: Setup runs on the main thread, and the window owns this live NSWindow.
    let ns_window = unsafe { &*ns_window.cast::<NSWindow>() };
    ns_window.setHasShadow(true);
    ns_window.setOpaque(false);

    if let Some(content_view) = ns_window.contentView() {
        content_view.setWantsLayer(true);

        if let Some(layer) = content_view.layer() {
            layer.setCornerRadius(12.0);
            layer.setMasksToBounds(true);
        }
    }

    if let Some(zoom_button) = ns_window.standardWindowButton(NSWindowButton::ZoomButton) {
        zoom_button.setHidden(true);
    }

    Ok(())
}
