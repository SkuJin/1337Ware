use std::sync::{Arc, Mutex};
use imgui::Ui;
use mint::Vector4;
use lazy_static::lazy_static;
use crate::{utils::config::{CONFIG, Config}, ui::{main::WINDOWS_ACTIVE, functions::color_u32_to_f32}};

lazy_static! {
    pub static ref CHEAT_LIST_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn render_cheat_list(ui: &mut Ui, config: Config, pawn: bool, aimbot_toggled: bool, triggerbot_toggled: bool) {

}