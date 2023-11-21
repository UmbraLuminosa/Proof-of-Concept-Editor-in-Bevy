use bevy::{prelude::*, scene::serde::SceneDeserializer};
use bevy_utils::HashMap;
use egui::util::undoer::Undoer;
use serde::de::DeserializeSeed;

pub struct UndoPlugin;

impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UndoResource>()
            .add_systems(PreUpdate, feed_undo_state)
            .add_systems(Update, undo_on_keypress)
            .add_systems(Update, redo_on_keypress);
    }
}

#[derive(Resource)]
struct UndoResource {
    pub undoer: Undoer<String>,
    pub redoer: Undoer<String>,
}

impl Default for UndoResource {
    fn default() -> Self {
        Self {
            undoer: Undoer::<String>::default(),
            redoer: Undoer::<String>::default(),
        }
    }
}

fn extract_undo_scene(world: &mut World) -> DynamicScene {
    DynamicSceneBuilder::from_world(world)
        .deny_all_resources()
        .allow::<Transform>()
        .extract_entities(world.iter_entities().map(|entity| entity.id()))
        .extract_resources()
        .build()
}

fn feed_undo_state(world: &mut World) {
    let scene = extract_undo_scene(world);
    let time = world.resource::<Time<Real>>().clone();
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let mut undo_resource = world.resource_mut::<UndoResource>();
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();

    undo_resource
        .undoer
        .feed_state(time.elapsed().as_secs_f64(), &serialized_scene);
}

fn undo_on_keypress(world: &mut World) {
    let keys = world.resource::<Input<KeyCode>>().clone();

    if !(keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keys.just_pressed(KeyCode::Z))
    {
        return;
    }

    let scene = extract_undo_scene(world);
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();
    let undo_resource = world.resource_mut::<UndoResource>().into_inner();
    let undoer = &mut undo_resource.undoer;
    let redoer = &mut undo_resource.redoer;

    let Some(old_state) = undoer.undo(&serialized_scene) else {
        return;
    };

    redoer.add_undo(&serialized_scene);

    let mut deserializer = ron::de::Deserializer::from_str(&old_state).unwrap();
    let derser_type_reg = type_registry.0.clone();

    let scene_deserializer = SceneDeserializer {
        type_registry: &derser_type_reg.read(),
    };

    let result = scene_deserializer.deserialize(&mut deserializer).unwrap();
    let mut entity_map: HashMap<Entity, Entity> = HashMap::new();
    for e in world.iter_entities() {
        entity_map.entry(e.id()).or_insert(e.id());
    }

    if let Err(e) = result.write_to_world_with(world, &mut entity_map, &type_registry) {
        println!("Error updating world: {}", e);
    }
}

fn redo_on_keypress(world: &mut World) {
    let keys = world.resource::<Input<KeyCode>>().clone();

    if !(keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keys.just_pressed(KeyCode::R))
    {
        return;
    }

    let scene = extract_undo_scene(world);
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();
    let undo_resource = world.resource_mut::<UndoResource>().into_inner();
    let redoer = &mut undo_resource.redoer;

    let Some(old_state) = redoer.undo(&serialized_scene) else {
        return;
    };

    let mut deserializer = ron::de::Deserializer::from_str(&old_state).unwrap();
    let derser_type_reg = type_registry.0.clone();

    let scene_deserializer = SceneDeserializer {
        type_registry: &derser_type_reg.read(),
    };

    let result = scene_deserializer.deserialize(&mut deserializer).unwrap();
    let mut entity_map: HashMap<Entity, Entity> = HashMap::new();
    for e in world.iter_entities() {
        entity_map.entry(e.id()).or_insert(e.id());
    }

    if let Err(e) = result.write_to_world_with(world, &mut entity_map, &type_registry) {
        println!("Error updating world: {}", e);
    }
}
