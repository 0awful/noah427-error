use godot::prelude::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct MapData {
    #[base]
    resource: Base<Resource>,
}

#[godot_api]
impl IResource for MapData {
    fn init(resource: Base<Resource>) -> Self {
        return Self { resource };
    }
} 
