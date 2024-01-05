use godot::engine::Engine;
use godot::prelude::Node3D;
use godot::prelude::*;

use crate::map::MapData;

#[derive(GodotClass)]
#[class(base=Node3D, tool)]
struct MapEditor {
    #[export]
    map_data: Option<Gd<MapData>>,
    #[base]
    node: Base<Node3D>,
}

#[godot_api]
impl INode3D for MapEditor {
    fn init(node: Base<Node3D>) -> Self {
        return Self {
            node,
            map_data: Some(MapData::new_gd()),
        };
    }

    fn process(&mut self, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            return;
        }
    }
}
