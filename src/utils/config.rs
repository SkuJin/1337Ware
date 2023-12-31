use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::{File, OpenOptions, read_dir, metadata, create_dir_all, remove_file}, sync::{Arc, Mutex}, path::PathBuf};
use directories::UserDirs;
use indexmap::IndexMap;
use lazy_static::lazy_static;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ProgramConfig {
    pub mod Package {
        pub const Name: &str = "1337Ware";
        pub const Description: &str = "OWNING OWNERS";
        pub const Executable: &str = "1337Ware.exe";
        pub const Version: &str = env!("CARGO_PKG_VERSION");
        pub const Authors: &str = &env!("CARGO_PKG_AUTHORS");
        pub const AskStart: bool = true;
    }

    pub mod Imgui {
        pub mod FontPaths {
            pub const Chinese: &str = "C:/Windows/Fonts/msyh.ttc";
            pub const Cryillic: &str = "C:/Windows/Fonts/calibri.ttf";
            pub const Arabic: &str = "C:/Windows/Fonts/calibri.ttf";
        }
    }

    pub mod Update {
        pub const Enabled: bool = true;
        pub const URL: &str = "";
        pub const HashURL: &str = "";
    }

    pub mod RPC {
        pub const Enabled: bool = true;
        pub const ClientID: u64 = 1179486796703334492;
        pub const ImageAsset: &str = "https://i.imgur.com/XFcgOk6.gif";
    }

    pub mod Links {
    }

    pub mod Keys {
        use glutin::event::VirtualKeyCode;
        use mki::Keyboard;

        pub const ToggleKey: VirtualKeyCode = VirtualKeyCode::Insert;
        pub const ToggleKeyMKI: Keyboard = Keyboard::Insert;

        pub const ExitKey: VirtualKeyCode = VirtualKeyCode::End;
        pub const ExitKeyMKI: Keyboard = Keyboard::Other(0x23);
    }

    pub mod TargetProcess {
        pub const Executable: &str = "cs2.exe";
        pub const MaxAttempts: u32 = 60;
        pub const UpdateOffsetsMaxAttempts: u32 = 60;
        pub const InitAddressesMaxAttempts: u32 = 60;

        pub mod Window {
            pub const Title: &str = "Counter-Strike 2";
            pub const Class: &str = "SDL_app";
        }
    }

    pub mod ThreadDelays {
        use std::time::Duration;

        pub const AttachTargetProcess: Duration = Duration::from_millis(1000);
        pub const UpdateOffsets: Duration = Duration::from_millis(1000);
        pub const InitAddresses: Duration = Duration::from_millis(1000);
        
        pub const UpdateConfigs: Duration = Duration::from_millis(50);
        pub const WindowTasks: Duration = Duration::from_millis(25);
        pub const IOTasks: Duration = Duration::from_millis(5);
        pub const RPC: Duration = Duration::from_millis(5);
    }

    pub mod CheatDelays {
        use std::time::Duration;

        pub const Aimbot: Duration = Duration::from_millis(10);
        pub const AimbotOffEntity: Duration = Duration::from_millis(500);
        pub const TriggerbotOffEntity: Duration = Duration::from_millis(500);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub esp: ESP,
    pub aimbot: Aimbot,
    pub triggerbot: Triggerbot,
    pub crosshair: Crosshair,
    pub radar: Radar,
    pub misc: Misc,
    pub window_positions: WindowPositions,
    pub style: Style,
    pub settings: Settings
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ESP {
    pub enabled: bool,
    pub outline: bool,
    pub thickness: f32,
    pub rounding: u32,
    pub box_enabled: bool,
    pub box_color: (u32, u32, u32, u32),
    pub box_mode: usize,
    pub box_target_enabled: bool,
    pub box_target_color: (u32, u32, u32, u32),
    pub filled_box_enabled: bool,
    pub filled_box_color_one: (u32, u32, u32, u32),
    pub filled_box_color_two: (u32, u32, u32, u32),
    pub filled_box_alpha: f32,
    pub skeleton_enabled: bool,
    pub skeleton_color: (u32, u32, u32, u32),
    pub head_enabled: bool,
    pub head_color: (u32, u32, u32, u32),
    pub head_mode: usize,
    pub eye_ray_enabled: bool,
    pub eye_ray_color: (u32, u32, u32, u32),
    pub health_bar_enabled: bool,
    pub health_bar_first_color: (u32, u32, u32, u32),
    pub health_bar_second_color: (u32, u32, u32, u32),
    pub health_bar_third_color: (u32, u32, u32, u32),
    pub armor_bar_enabled: bool,
    pub armor_bar_color: (u32, u32, u32, u32),
    pub bar_mode: usize,
    pub name_enabled: bool,
    pub name_color: (u32, u32, u32, u32),
    pub weapon_name_enabled: bool,
    pub weapon_name_color: (u32, u32, u32, u32),
    pub distance_enabled: bool,
    pub distance_color: (u32, u32, u32, u32),
    pub bomb_enabled: bool,
    pub bomb_color: (u32, u32, u32, u32),
    pub filled_bomb_enabled: bool,
    pub filled_bomb_color: (u32, u32, u32, u32),
    pub filled_bomb_alpha: f32,
    pub snap_line_enabled: bool,
    pub snap_line_color: (u32, u32, u32, u32),
    pub snap_line_mode: usize
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Aimbot {
    pub enabled: bool,
    pub key: usize,
    pub mode: usize,
    pub fov_circle_enabled: bool,
    pub fov_circle_color: (u32, u32, u32, u32),
    pub fov_circle_target_enabled: bool,
    pub fov_circle_target_color: (u32, u32, u32, u32),
    pub fov_circle_outline_enabled: bool,
    pub fov_circle_thickness: f32,
    pub only_visible: bool,
    pub only_grounded: bool,
    pub only_weapon: bool,
    pub bone: usize,
    pub fov: f32,
    pub smooth: f32,
    pub smooth_offset: f32,
    pub delay: u32,
    pub delay_offset: u32
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Triggerbot {
    pub enabled: bool,
    pub key: usize,
    pub mode: usize,
    pub tap_interval: u32,
    pub tap_interval_offset: u32,
    pub delay: u32,
    pub delay_offset: u32,
    pub always_activated: bool,
    pub only_weapon: bool
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Crosshair {
    pub enabled: bool,
    pub color: (u32, u32, u32, u32),
    pub target_enabled: bool,
    pub target_color: (u32, u32, u32, u32),
    pub outline_enabled: bool,
    pub dot_enabled: bool,
    pub dot_size: u32,
    pub circle_enabled: bool,
    pub circle_radius: u32,
    pub lines_enabled: bool,
    pub lines_width: u32,
    pub lines_height: u32,
    pub lines_space: u32,
    pub lines_thickness: u32,
    pub only_weapon: bool
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Radar {
    pub enabled: bool,
    pub color: (u32, u32, u32, u32),
    pub mode: usize,
    pub alpha: f32,
    pub outline: bool,
    pub crossline_enabled: bool,
    pub crossline_color: (u32, u32, u32, u32),
    pub point_size: f32,
    pub proportion: f32,
    pub range: f32
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Misc {
    pub enabled: bool,
    pub watermark_enabled: bool,
    pub watermark_color_one: (u32, u32, u32, u32),
    pub watermark_color_two: (u32, u32, u32, u32),
    pub cheat_list_enabled: bool,
    pub cheat_list_color_one: (u32, u32, u32, u32),
    pub cheat_list_color_two: (u32, u32, u32, u32),
    pub bomb_timer_enabled: bool,
    pub bomb_timer_color_disabled: (u32, u32, u32, u32),
    pub bomb_timer_color_enabled: (u32, u32, u32, u32),
    pub spectator_list_enabled: bool,
    pub spectator_list_color: (u32, u32, u32, u32),
    pub exclude_team: bool,
    pub show_on_spectate: bool,
    pub headshot_line_enabled: bool,
    pub headshot_line_color: (u32, u32, u32, u32)
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WindowPositions {
    pub menu: [f32; 2],
    pub watermark: [f32; 2],
    pub cheat_list: [f32; 2],
    pub bomb_timer: [f32; 2],
    pub spectator_list: [f32; 2]
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StyleColors {
    pub text: (u32, u32, u32, u32),
    pub text_disabled: (u32, u32, u32, u32),
    pub window_bg: (u32, u32, u32, u32),
    pub child_bg: (u32, u32, u32, u32),
    pub popup_bg: (u32, u32, u32, u32),
    pub border: (u32, u32, u32, u32),
    pub border_shadow: (u32, u32, u32, u32),
    pub frame_bg: (u32, u32, u32, u32),
    pub frame_bg_hovered: (u32, u32, u32, u32),
    pub frame_bg_active: (u32, u32, u32, u32),
    pub title_bg: (u32, u32, u32, u32),
    pub title_bg_collapsed: (u32, u32, u32, u32),
    pub title_bg_active: (u32, u32, u32, u32),
    pub text_selected_bg: (u32, u32, u32, u32),
    pub checkmark: (u32, u32, u32, u32),
    pub scrollbar_bg: (u32, u32, u32, u32),
    pub scrollbar_grab: (u32, u32, u32, u32),
    pub scrollbar_grab_hovered: (u32, u32, u32, u32),
    pub scrollbar_grab_active: (u32, u32, u32, u32),
    pub slider_grab: (u32, u32, u32, u32),
    pub slider_grab_active: (u32, u32, u32, u32),
    pub button: (u32, u32, u32, u32),
    pub button_hovered: (u32, u32, u32, u32),
    pub button_active: (u32, u32, u32, u32),
    pub tab: (u32, u32, u32, u32),
    pub tab_hovered: (u32, u32, u32, u32),
    pub tab_active: (u32, u32, u32, u32),
    pub separator: (u32, u32, u32, u32)
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Style {
    pub enabled: bool,
    pub alpha: f32,
    pub window_padding: [f32; 2],
    pub window_rounding: f32,
    pub window_border_size: f32,
    pub window_title_align: [f32; 2],
    pub frame_padding: [f32; 2],
    pub frame_rounding: f32,
    pub frame_border_size: f32,
    pub tab_rounding: f32,
    pub tab_border_size: f32,
    pub scrollbar_rounding: f32,
    pub scrollbar_size: f32,
    pub popup_rounding: f32,
    pub popup_border_size: f32,
    pub item_spacing: [f32; 2],
    pub item_inner_spacing: [f32; 2],
    pub indent_spacing: f32,
    pub grab_rounding: f32,
    pub colors: StyleColors
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub enabled: bool,
    pub bypass_capture: bool,
    pub discord_rpc_enabled: bool
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            esp: ESP {
                enabled: false,
                outline: false,
                thickness: 1.2,
                rounding: 0,
                box_enabled: false,
                box_color: (255, 255, 255, 255),
                box_mode: 0,
                box_target_enabled: false,
                box_target_color: (255, 0, 0, 255),
                filled_box_enabled: false,
                filled_box_color_one: (255, 255, 255, 255),
                filled_box_color_two: (255, 255, 255, 255),
                filled_box_alpha: 0.1,
                skeleton_enabled: false,
                skeleton_color: (255, 255, 255, 255),
                head_enabled: false,
                head_color: (255, 255, 255, 255),
                head_mode: 0,
                eye_ray_enabled: false,
                eye_ray_color: (255, 255, 255, 255),
                health_bar_enabled: false,
                health_bar_first_color: (0, 255, 0, 255),
                health_bar_second_color: (255, 255, 0, 255),
                health_bar_third_color: (255, 0, 0, 255),
                armor_bar_enabled: false,
                armor_bar_color: (255, 255, 255, 255),
                bar_mode: 1,
                name_enabled: false,
                name_color: (255, 255, 255, 255),
                weapon_name_enabled: false,
                weapon_name_color: (255, 255, 255, 255),
                distance_enabled: false,
                distance_color: (255, 255, 255, 255),
                bomb_enabled: false,
                bomb_color: (255, 0, 0, 255),
                filled_bomb_enabled: false,
                filled_bomb_color: (255, 0, 0, 255),
                filled_bomb_alpha: 0.1,
                snap_line_enabled: false,
                snap_line_color: (255, 255, 255, 255),
                snap_line_mode: 1
            },
            aimbot: Aimbot {
                enabled: false,
                key: 0,
                mode: 0,
                fov_circle_enabled: false,
                fov_circle_color: (255, 255, 255, 255),
                fov_circle_target_enabled: false,
                fov_circle_target_color: (255, 0, 0, 255),
                fov_circle_outline_enabled: false,
                fov_circle_thickness: 1.2,
                only_visible: false,
                only_grounded: false,
                only_weapon: false,
                bone: 1,
                fov: 5.0,
                smooth: 1.0,
                smooth_offset: 0.2,
                delay: 70,
                delay_offset: 15
            },
            triggerbot: Triggerbot {
                enabled:  false,
                key: 0,
                mode: 0,
                tap_interval: 120,
                tap_interval_offset: 15,
                delay: 70,
                delay_offset: 15,
                always_activated: false,
                only_weapon: false
            },
            crosshair: Crosshair {
                enabled: false,
                color: (255, 255, 255, 255),
                target_enabled: false,
                target_color: (255, 0, 0, 255),
                outline_enabled: false,
                dot_enabled: false,
                dot_size: 1,
                circle_enabled: false,
                circle_radius: 5,
                lines_enabled: false,
                lines_width: 9,
                lines_height: 9,
                lines_space: 7,
                lines_thickness: 1,
                only_weapon: false
            },
            radar: Radar {
                enabled: false,
                color: (255, 0, 0, 255),
                mode: 2,
                alpha: 0.0,
                outline: false,
                crossline_enabled: false,
                crossline_color: (255, 255, 255, 255),
                point_size: 1.0,
                proportion: 3100.0,
                range: 148.0
            },
            misc: Misc {
                enabled: false,
                watermark_enabled: false,
                watermark_color_one: (255, 255, 0, 255),
                watermark_color_two: (0, 255, 0, 255),
                cheat_list_enabled: false,
                cheat_list_color_one: (0, 255, 255, 255),
                cheat_list_color_two: (0, 255, 0, 255),
                bomb_timer_enabled: false,
                bomb_timer_color_disabled: (0, 255, 255, 255),
                bomb_timer_color_enabled: (255, 0, 0, 255),
                spectator_list_enabled: false,
                spectator_list_color: (0, 255, 255, 255),
                exclude_team: false,
                show_on_spectate: false,
                headshot_line_enabled: false,
                headshot_line_color: (255, 255, 255, 255)
            },
            style: Style {
                enabled: false,
                alpha: 1.0,
                window_padding: [7.5, 7.5],
                window_rounding: 5.0,
                window_border_size: 1.0,
                window_title_align: [0.5, 0.5],
                frame_padding: [2.5, 2.5],
                frame_rounding: 2.5,
                frame_border_size: 0.0,
                tab_rounding: 2.5,
                tab_border_size: 0.0,
                scrollbar_rounding: 2.5,
                scrollbar_size: 3.0,
                popup_rounding: 2.5,
                popup_border_size: 0.0,
                item_spacing: [7.5, 7.5],
                item_inner_spacing: [5.0, 5.0],
                indent_spacing: 2.5,
                grab_rounding: 2.5,
                colors: StyleColors {
                    text: (225, 225, 225, 255),
                    text_disabled: (200, 200, 200, 255),
                    window_bg: (25, 25, 25, 235),
                    child_bg: (25, 25, 25, 235),
                    popup_bg: (35, 35, 35, 235),
                    border: (51, 128, 245, 255),
                    border_shadow: (15, 15, 15, 255),
                    frame_bg: (51, 128, 245, 50),
                    frame_bg_hovered: (51, 128, 245, 100),
                    frame_bg_active: (51, 128, 245, 150),
                    title_bg: (51, 128, 245, 235),
                    title_bg_collapsed: (51, 128, 245, 235),
                    title_bg_active: (51, 128, 245, 235),
                    text_selected_bg: (51, 128, 245, 255),
                    checkmark: (51, 128, 245, 255),
                    scrollbar_bg: (25, 25, 25, 255),
                    scrollbar_grab: (45, 45, 45, 255),
                    scrollbar_grab_hovered: (45, 45, 45, 225),
                    scrollbar_grab_active: (45, 45, 45, 200),
                    slider_grab: (51, 128, 245, 255),
                    slider_grab_active: (51, 128, 245, 225),
                    button: (51, 128, 245, 255),
                    button_hovered: (51, 128, 245, 225),
                    button_active: (51, 128, 245, 200),
                    tab: (51, 128, 245, 255),
                    tab_hovered: (51, 128, 245, 225),
                    tab_active: (51, 128, 245, 200),
                    separator: (175, 175, 175, 125)
                }
            },
            settings: Settings {
                enabled: true,
                bypass_capture: false,
                discord_rpc_enabled: true
            },
            window_positions: WindowPositions {
                menu: [600.0, 150.0],
                watermark: [300.0, 5.0],
                cheat_list: [300.0, 58.0],
                bomb_timer: [30.0, 330.0],
                spectator_list: [475.0, 5.0]
            }
        };
    }
}

impl Config {
    pub fn save_config(&self, file_path: &str, update: bool) -> Result<(), &str> {
        let file = match OpenOptions::new().write(true).truncate(true).create(true).open(file_path) {
            Ok(file) => file,
            Err(_) => { return Err("CreateFile"); }
        };

        match serde_json::to_writer_pretty(file, &self) {
            Ok(_) => {},
            Err(_) => { return Err("WriteFile"); }
        };

        if update {
            update_configs();
        }

        return Ok(());
    }
}

lazy_static! {
    pub static ref CONFIG_EXTENSION: String = "config".to_string();
    pub static ref DEFAULT_CONFIG: String = format!("Default.{}", CONFIG_EXTENSION.clone());
    pub static ref CONFIG_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
    pub static ref CONFIGS: Arc<Mutex<IndexMap<String, Option<Config>>>> = Arc::new(Mutex::new(IndexMap::new()));
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

pub fn get_directory_dir(name: &str) -> Option<String> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            let config_dir = document_dir.join(name);

            if let Some(config_path) = config_dir.to_str() {
                return Some(config_path.to_owned());
            };
        };
    };

    return None;
}

pub fn update_configs() -> Option<String> {
    let config_dir = CONFIG_DIR.lock().unwrap().clone();
    let directory_pathbuf = PathBuf::from(&*config_dir);

    if !metadata(&directory_pathbuf).is_ok() {
        match create_dir_all(&directory_pathbuf) {
            Ok(_) => {},
            Err(_) => { return Some("CreateDirectory".to_string()); }
        };
    };
    
    let mut configs = IndexMap::new();
    let paths = match read_dir(directory_pathbuf.clone()) {
        Ok(paths) => paths,
        Err(_) => { return Some("DirectoryPath".to_string()); }
    };

    let default_config_name = &*DEFAULT_CONFIG;
    let mut update_default_config = false;

    for path in paths {
        if let Ok(entry) = path {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(&format!(".{}", *CONFIG_EXTENSION)) {
                    if let Some(config_path) = directory_pathbuf.join(file_name).to_str() {
                        match load_config(config_path) {
                            Ok(config) => {
                                let (config_index, _) = configs.insert_full(file_name.to_string(), Some(config));
                                
                                if file_name == &*DEFAULT_CONFIG {
                                    configs.move_index(config_index, 0);
                                }
                            },
                            Err(_) => {
                                configs.insert_full(file_name.to_string(), None);
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(default_config) = configs.get(default_config_name) {
        if !default_config.is_some() {
            update_default_config = true;
        }
    } else {
        update_default_config = true;
    }

    if update_default_config {
        if let Some(default_config_path) = directory_pathbuf.join(default_config_name).to_str() {
            match CONFIG.lock().unwrap().clone().save_config(default_config_path, false) {
                Err(_) => { return Some("SaveDefaultConfig".to_string()); },
                Ok(_) => {}
            };
        }
    }

    *CONFIGS.lock().unwrap() = configs;
    return None;
}

pub fn setup_config() -> Option<String> {
    let directory_path = match get_directory_dir(ProgramConfig::Package::Name) {
        Some(path) => path,
        None => { return Some("FindDirectory".to_string()); }
    };

    *CONFIG_DIR.lock().unwrap() = directory_path.to_string();

    match update_configs() {
        Some(str) => { return Some(str); },
        None => {}
    };

    if let Some(default_config) = (*CONFIGS.lock().unwrap()).get(&*DEFAULT_CONFIG) {
        if let Some(default_config) = default_config {
            *CONFIG.lock().unwrap() = *default_config;
        }
    }

    return None;
}

pub fn merge_config(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_config(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn load_config(file_path: &str) -> Result<Config, &str> {
    let file = match File::open(file_path) {
        Ok(path) => path,
        Err(_) => { return Err("FilePath"); }
    };
    
    let config: Config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(_) => {
            let file = match File::open(file_path) {
                Ok(path) => path,
                Err(_) => { return Err("FilePath"); }
            };
            
            let old_config: Value = match serde_json::from_reader(file) {
                Ok(value) => value,
                Err(_) => { return Err("ParseFile"); }
            };

            let mut config = match serde_json::to_value(Config::default()) {
                Ok(value) => value,
                Err(_) => { return Err("ParseValue"); }
            };

            merge_config(&mut config, &old_config);

            let new_config: Config = match serde_json::from_value(config) {
                Ok(config) => config,
                Err(_) => { return Err("ParseConfig"); }
            };

            match new_config.save_config(file_path, false) {
                Err(_) => { return Err("SaveConfig"); },
                Ok(_) => {}
            };

            new_config
        }
    };

    return Ok(config);
}

pub fn delete_config(file_path: &str) -> Result<bool, &str> {
    if let Err(_) = remove_file(file_path) {
        return Err("DeleteFile");
    };

    return Ok(true);
}