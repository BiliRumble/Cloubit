use slint::quit_event_loop;
use tray_item::{IconSource, TrayItem};

use crate::error::AppError;

pub fn init() -> Result<(), AppError> {
    let logo_raw = image::load_from_memory(include_bytes!("../../resources/logo.png"))?.to_rgba8(); // 强转rgba8似乎导致了颜色失真

    let logo = IconSource::Data {
        height: logo_raw.height() as i32,
        width: logo_raw.width() as i32,
        data: logo_raw.into_vec(),
    };

    let mut tray = TrayItem::new("Cloubit", logo)?;

    // 临时方案
    tray.add_menu_item("显示", || println!("Open clicked"))?;
    tray.add_menu_item("退出", || {
        let _ = quit_event_loop();
    })?;

    #[cfg(target_os = "macos")]
    {
        inner.add_quit_item("Quit");
        inner.display();
    }

    Ok(())
}
