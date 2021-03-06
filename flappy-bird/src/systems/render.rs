use ggez::{Context, GameResult};
use mint::Point2;
use specs::{prelude::ComponentEvent, BitSet, Entity, Join, ReaderId, World, WorldExt};
use specs_guided_join::GuidedJoin;

use crate::*;
#[derive(Debug)]
pub struct RenderSystem {
    // These keep track of where you left off in the event channel.
    reader_id: ReaderId<ComponentEvent>,
    sorted_entities: Vec<Entity>,
    dirty_sort: bool,
    show_fps: bool,
    inserted: BitSet,
    deleted: BitSet,
}

impl RenderSystem {
    pub fn new(world: &mut World) -> Self {
        let mut render_components = world.write_storage::<components::Render>();
        Self {
            reader_id: render_components.register_reader(),
            sorted_entities: Vec::new(),
            dirty_sort: true,
            show_fps: true,
            inserted: BitSet::new(),
            deleted: BitSet::new(),
        }
    }

    fn sort_entities(&mut self, world: &World) {
        self.inserted.clear();
        self.deleted.clear();

        let render_entities = world.write_storage::<components::Render>();
        for event in render_entities.channel().read(&mut self.reader_id) {
            match event {
                ComponentEvent::Modified(_) => {
                    self.dirty_sort = true;
                }
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                    self.dirty_sort = true;
                }
                ComponentEvent::Removed(id) => {
                    self.deleted.add(*id);
                }
            };
        }

        for e in &self.deleted {
            self.sorted_entities.retain(|r| r.id() != e);
        }

        for (e, _) in (&world.entities(), &self.inserted).join() {
            self.sorted_entities.push(e);
        }
        if self.dirty_sort {
            let pos_storage = &world.read_storage::<components::Position>();
            self.sorted_entities.sort_by(|a, b| {
                let pos_a = pos_storage.get(*a).unwrap();
                let pos_b = pos_storage.get(*b).unwrap();
                pos_a.z.cmp(&pos_b.z)
            });
            self.dirty_sort = false;
        }
    }

    pub fn run(&mut self, ctx: &mut Context, world: &World) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(40, 45, 52));
        self.sort_entities(world);

        for (render, pos, size, rotation, image, text) in (
            &world.read_storage::<components::Render>(),
            &world.read_storage::<components::Position>(),
            (&world.read_storage::<components::Size>()).maybe(),
            (&world.read_storage::<components::Rotation>()).maybe(),
            (&world.read_storage::<components::Image>()).maybe(),
            (&world.read_storage::<components::Text>()).maybe(),
        )
            .guided_join(&self.sorted_entities)
        {
            if !render.visible {
                continue;
            }

            if let Some(image) = image {
                let mut draw_params =
                    graphics::DrawParam::new().dest(Point2 { x: pos.x, y: pos.y });

                if let Some(rotation) = rotation {
                    draw_params = graphics::DrawParam::new()
                        .offset(Point2 {
                            x: rotation.x,
                            y: rotation.y,
                        })
                        .rotation(rotation.deg)
                        .dest(Point2 { x: pos.x, y: pos.y });
                }

                graphics::draw(ctx, &image.image, draw_params)?;
            } else if let Some(text) = text {
                let font = *world
                    .read_resource::<Fonts>()
                    .get(&text.font)
                    .ok_or_else(|| {
                        ggez::GameError::FontError("Unable to load text font".to_string())
                    })?;

                let t = graphics::Text::new((text.text.as_str(), font, text.font_size));

                let dest = match text.align {
                    Alignment::Left => mint::Point2 { x: pos.x, y: pos.y },
                    Alignment::Right => mint::Point2 {
                        x: match size {
                            Some(size) => size.w - t.dimensions(ctx).0 as f32,
                            None => VIRTUAL_WIDTH - t.dimensions(ctx).0 as f32,
                        },
                        y: pos.y,
                    },
                    Alignment::Centered => mint::Point2 {
                        x: match size {
                            Some(size) => (size.w / 2.0) - (t.dimensions(ctx).0 / 2) as f32,
                            None => (VIRTUAL_WIDTH / 2.0) - (t.dimensions(ctx).0 / 2) as f32,
                        },
                        y: pos.y,
                    },
                };

                graphics::queue_text(ctx, &t, dest, Some(text.color));
            }
        }

        if self.show_fps {
            self.draw_fps(ctx, &world);
        }

        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Nearest,
        )?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn draw_fps(&mut self, ctx: &mut Context, world: &World) {
        let fonts = &world.read_resource::<Fonts>();

        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new((
            format!("FPS: {:.1}", fps),
            *fonts.get(&FontType::Retro).unwrap(),
            8.0,
        ));

        graphics::queue_text(
            ctx,
            &fps_display,
            mint::Point2 { x: 10.0, y: 10.0 },
            Some(graphics::Color::from_rgb(0, 255, 0)),
        );
    }
}
