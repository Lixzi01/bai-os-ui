use chrono::{Datelike, Local};
use slint::{ComponentHandle, Timer, TimerMode};
use std::time::Duration;

slint::include_modules!();

fn thai_date() -> String {
    const DAYS: [&str; 7] = [
        "จันทร์", "อังคาร", "พุธ", "พฤหัสบดี", "ศุกร์", "เสาร์", "อาทิตย์",
    ];
    const MONTHS: [&str; 12] = [
        "มกราคม", "กุมภาพันธ์", "มีนาคม", "เมษายน", "พฤษภาคม", "มิถุนายน",
        "กรกฎาคม", "สิงหาคม", "กันยายน", "ตุลาคม", "พฤศจิกายน", "ธันวาคม",
    ];
    let now = Local::now();
    let d = DAYS[now.weekday().num_days_from_monday() as usize];
    let m = MONTHS[now.month0() as usize];
    format!("{} {} {}", d, now.day(), m)
}

pub fn run() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let update = {
        let weak = ui.as_weak();
        move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_clock(Local::now().format("%H:%M").to_string().into());
                ui.set_date_text(thai_date().into());
            }
        }
    };
    update();

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), update);
    // บนเว็บ run() จบทันที ต้องกันไม่ให้ timer ถูกทำลาย
    std::mem::forget(timer);

    ui.run()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn wasm_start() {
    run().unwrap();
}
