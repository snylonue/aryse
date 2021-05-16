mod cli;
mod render;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use render::OpenImage;
use render::Playlist;

fn setup(mut commands: Commands, mut ev_open_img: EventWriter<OpenImage>) {
    let app = cli::app().get_matches();
    let images = app.values_of("image").unwrap().map(Into::into).collect();
    let playlist = Playlist::new(images);
    let first = playlist.current().to_owned();
    commands.insert_resource(playlist);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    ev_open_img.send(OpenImage(first));
}

fn keyboard(
    mut ev_key: EventReader<KeyboardInput>,
    mut ev_open_img: EventWriter<OpenImage>,
    mut playlist: ResMut<Playlist>,
) {
    for ev in ev_key.iter() {
        match ev.state {
            bevy::input::ElementState::Pressed if ev.key_code == Some(KeyCode::Right) => {
                let img = playlist.next();
                ev_open_img.send(OpenImage(img.to_owned()));
            }
            _ => {}
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(render::RenderPlugin)
        .add_startup_system(setup.system())
        .add_system(keyboard.system())
        .run();
}
