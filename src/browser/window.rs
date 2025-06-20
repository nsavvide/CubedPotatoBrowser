use crate::browser::view::PWindowDelegate;
use cef::{window_create_top_level, BrowserView, Window};

pub fn create_main_window(browser_view: BrowserView) -> Option<Window> {
    let mut delegate = PWindowDelegate::new(browser_view);
    window_create_top_level(Some(&mut delegate))
}
