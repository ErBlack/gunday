use crate::components::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, WORLD_WIDTH};
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerControl {
    pub enabled: bool,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct WinMusic(pub bool);

#[cfg(target_arch = "wasm32")]
pub mod browser_events {
    use js_sys::{Date, Object, Reflect};
    use wasm_bindgen::prelude::JsValue;
    use wasm_bindgen::prelude::*;
    use web_sys::{CustomEvent, CustomEventInit, window};
    fn encrypt(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut result = String::with_capacity(bytes.len() + bytes.len() / 4);

        for (index, &byte) in bytes.iter().enumerate() {
            let adjusted = byte as u32 + 16 + index as u32;
            let ch = char::from_u32(adjusted).unwrap_or('�');
            result.push(ch);

            if index != bytes.len() - 1 && (index + 1) % 4 == 0 {
                result.push('-');
            }
        }

        result
    }

    fn encrypted_timestamp() -> String {
        let millis = Date::now() as u64;
        let trimmed = millis / 10;
        encrypt(&trimmed.to_string())
    }
    #[wasm_bindgen]
    pub fn send_game_result(win: bool) {
        if let Some(w) = window() {
            let init = {
                let tmp = CustomEventInit::new();

                let _ = tmp.set_bubbles(true);
                tmp
            };

            let obj = Object::new();
            let _ = Reflect::set(&obj, &JsValue::from_str("win"), &JsValue::from_bool(win));
            if win {
                let code = encrypted_timestamp();
                let _ = Reflect::set(&obj, &JsValue::from_str("code"), &JsValue::from_str(&code));
            }
            let detail: JsValue = obj.into();
            let _ = init.set_detail(&detail);
            if let Ok(event) = CustomEvent::new_with_event_init_dict("gunday-result", &init) {
                let _ = w.dispatch_event(event.as_ref());
            }
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    let spawn_x = SCREEN_WIDTH / 4.0;
    let half_screen_width = SCREEN_WIDTH / 2.0;
    let world_left_bound = half_screen_width;
    let world_right_bound = WORLD_WIDTH - half_screen_width;
    let initial_camera_x = spawn_x.clamp(world_left_bound, world_right_bound);

    commands.insert_resource(PlayerControl::default());
    commands.insert_resource(WinMusic::default());
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        CameraState {
            current_x: initial_camera_x,
            max_reached_x: initial_camera_x,
            lock_position: None,
        },
        Transform::from_translation(Vec3::new(initial_camera_x, 0.0, 0.0)),
        Visibility::Visible,
        InheritedVisibility::default(),
    ));
}

pub fn setup_layer_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    geometry_storage: Res<LayerGeometryStorage>,
) {
    for geometry in &geometry_storage.objects {
        let center_x = geometry.bottom_left.x + (geometry.width / 2.0);
        let center_y = geometry.bottom_left.y + (geometry.height / 2.0);
        let bevy_center_y = center_y - (SCREEN_HEIGHT / 2.0);

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(geometry.width, geometry.height))),
            Transform::from_translation(Vec3::new(center_x, bevy_center_y, 1.0)),
            LayerGeometry::new_rectangle(
                geometry.bottom_left.x,
                geometry.bottom_left.y,
                geometry.width,
                geometry.height,
            ),
            Solid,
        ));
    }
}
