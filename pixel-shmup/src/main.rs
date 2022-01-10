use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{AnchorMode, Camera2D, Sprite, SubTexture, Transform2D},
        engine::{EngineBuilder, EngineSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input, VirtualKeyCode},
        scene::{InputHandled, Scene, Transition},
    },
    ecs,
    error::BreakoutResult,
    math,
    shapes::rectangle::Rect,
};
use tiled::Tiled;

extern crate log;
extern crate pretty_env_logger;

struct Player;

struct MainState {
    map: Option<Tiled>,
}

impl MainState {
    fn new() -> Self {
        Self { map: None }
    }
}

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        let tiles_packed =
            _asset_manager.load_texture("assets/kenney_pixelshmup/Tilemap/tiles_packed.png")?;

        let ships =
            _asset_manager.load_texture("assets/kenney_pixelshmup/Tilemap/ships_packed.png")?;

        let map = tiled::Tiled::load_map(
            "assets/tiled/map.json",
            &[("tiles_packed.png", tiles_packed)],
        )?;
        map.spawn(_context)?;
        self.map = Some(map);

        let world = &mut _context.get_world();

        let mut transform = Transform2D::new();
        transform.set_pixel_snap(true);

        world.spawn((
            Player,
            Sprite {
                texture_id: Some(ships),
                sub_texture: Some(SubTexture::new(Rect::new(0.0, 0.0, 32.0, 32.0))),
                center_origin: true,
                ..Default::default()
            },
            Camera2D {
                // offset: math::ivec2(-200, -150),
                anchor_mode: AnchorMode::Center,
                ..Camera2D::keep_width(400)
            },
            transform,
        ));
        Ok(())
    }

    fn input(
        &mut self,
        _event: Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<InputHandled> {
        Ok(InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<Transition> {
        let mut direction = math::Vec2::ZERO;

        if _input.is_key_pressed(VirtualKeyCode::Up) {
            direction.y = -1.0;
        }
        if _input.is_key_pressed(VirtualKeyCode::Down) {
            direction.y = 1.0;
        }
        if _input.is_key_pressed(VirtualKeyCode::Left) {
            direction.x = -1.0;
        }
        if _input.is_key_pressed(VirtualKeyCode::Right) {
            direction.x = 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            println!("{}", direction.angle_between(math::vec2(0.0, 1.0)));
        }

        let world = &mut _context.get_world();

        for (_id, (transform, camera)) in
            &mut world.query::<ecs::With<Player, (&mut Transform2D, &mut Camera2D)>>()
        {
            transform.translate(direction * 100.0 * _dt);

            let mut position = transform.position();

            if position.x < 0.0 {
                position.x = 0.0;
            }
            if position.y < 0.0 {
                position.y = 0.0;
            }
            transform.set_position(position);

            transform.set_rotate(transform.rotate() + _dt);

            if transform.rotate() > std::f32::consts::TAU {
                transform.set_rotate(std::f32::consts::TAU - transform.rotate());
            }

            let camera_rect = camera.get_view_rect(&_engine.window_size(), &transform.position());

            // if camera_rect.x < 0.0 {
            //     camera.offset.x = -camera_rect.x as i32;
            // } else {
            //     camera.offset.x = 0;
            // }

            // if transform.position.y < 0.0 {
            //     camera.offset.y = -camera_rect.y as i32;
            // } else {
            //     camera.offset.y = 0;
            // }
        }

        self.map.as_ref().unwrap().update(_context);
        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Pixel Shmup")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
