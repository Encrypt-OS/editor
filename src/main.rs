pub mod components;
pub mod state;

use components::App;

fn main() {
    App::new().connect_events().execute();
}