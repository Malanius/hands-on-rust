use crate::prelude::*;

const ENTITY_RENDER_LAYER: usize = 1;
const ENTITY_RENDER_ORDER: usize = 5_000;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ENTITY_RENDER_LAYER);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(ENTITY_RENDER_ORDER).expect("Batch error");
}
