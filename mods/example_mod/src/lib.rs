mod_macros::create_mod!("../../wit/module.wit");
mod_macros::create_data!();

use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    position: Position,
    health: i32,
    name: String,
}

pub struct Data {
    structure_manager: StructureManager,
    player_entity: RefCell<Option<GameEntity<PlayerData>>>,
}

impl GuestData for Data {
    fn new() -> Self {
        Data {
            structure_manager: StructureManager::new(),
            player_entity: RefCell::new(None),
        }
    }

    fn init(&self) {
        //let player_data = PlayerData {
        //    position: Position { x: 0.0, y: 0.0 },
        //    health: 100,
        //    name: "Player One".to_string(),
        //};
        //
        //let entity_id = self.structure_manager.generate_id("player");
        //let player_entity = GameEntity::new(&entity_id, "player", player_data);
        //
        //match player_entity.register(&self.structure_manager) {
        //    Ok(_) => log(&format!("Player entity registered with ID: {}", entity_id)),
        //    Err(e) => log(&format!("Failed to register player entity: {}", e)),
        //}
        //
        //*self.player_entity.borrow_mut() = Some(player_entity);
    }

    fn update(&self, delta: f32) {
        //let mut player_entity = self.player_entity.borrow_mut();
        //if let Some(ref mut player) = *player_entity {
        //    player.data.position.x += 1.0 * delta;
        //    player.data.position.y += 1.0 * delta;
        //}
    }

    fn shutdown(&self) {
        //let player_entity = self.player_entity.borrow();
        //if let Some(ref player) = *player_entity {
        //    if player.unregister(&self.structure_manager) {
        //        log("Player entity unregistered");
        //    } else {
        //        log("Failed to unregister player entity");
        //    }
        //}
    }
}
