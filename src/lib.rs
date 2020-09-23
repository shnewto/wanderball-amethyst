//! wanderball, something not-pong built by following along with the amethyst pong tutorial

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
mod bundle;
mod config;
mod side;
mod spritesheet;
mod start;
mod systems;
mod wanderball;

use crate::audio::Music;
use crate::bundle::WanderballBundle;
use crate::config::WanderballConfig;
use crate::start::StartScreen;

pub fn run() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let wanderball_config = WanderballConfig::load(&config_path.join("wanderball.ron"));
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(WanderballBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        );

    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, StartScreen::default())?
        .with_resource(wanderball_config)
        .build(game_data)?;
    game.run();
    Ok(())
}
