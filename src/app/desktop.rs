use crate::app::app_type::AppType;

#[derive(Clone, PartialEq)]
pub struct App {
    pub id: &'static str,
    pub name: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct WindowState {
    pub app_type: AppType,
    pub id: u32,
    pub title: &'static str,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_dragging: bool,
    pub drag_offset: (i32, i32),
    pub is_open: bool, 
    pub z_index: i32,
}

pub const WINDOW_COORD_X: i32 = 200;
pub const WINDOW_COORD_Y: i32 = 200;
pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_HEIGHT: i32 = 400;
