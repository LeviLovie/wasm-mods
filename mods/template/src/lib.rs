use mod_macros::create_mod;

// This macro call will expand to create a mod with the given parameters
create_mod!(
    "TemplateMod",
    "Template Mod",
    "A template for creating new mods",
    "Mod Developer",
    "1.0.0"
);

// The macro has created a struct called TemplateModMod that implements ModInterface
// You can now add custom implementation details

//impl TemplateMod {
//    // Override the on_init method to provide custom initialization
//    pub fn on_init(&self, context: &common::ModContext) -> common::ModResult<()> {
//        println!(
//            "Template mod initialized with game version: {}",
//            context.game_version
//        );
//
//        // Your initialization code here
//
//        Ok(())
//    }
//
//    // Override the on_update method for regular updates
//    pub fn on_update(&self, delta_time: f32) -> common::ModResult<()> {
//        // Your update code here
//        // This will be called on each game tick/frame
//
//        Ok(())
//    }
//
//    // Override the on_shutdown method for cleanup
//    pub fn on_shutdown(&self) -> common::ModResult<()> {
//        println!("Template mod shutting down...");
//
//        // Your cleanup code here
//
//        Ok(())
//    }
//}
