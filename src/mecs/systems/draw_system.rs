use graphics::DrawResources;
use mecs::{
    BodyComponent, DataComponent, DrawComponent, Entity, GlutinDeviceEvent, GlutinWindowEvent,
    StaticWorld, System, World,
};
use std::time::Duration;
use tools::{Camera, CameraController, Vec2, Vec3, Vec4};

pub struct DrawSystem {
    pub camera_entity: Entity,
    pub resources: DrawResources,
}

impl System for DrawSystem {
    fn update(&mut self, delta_time: Duration, world: &mut StaticWorld) {
        let camera = world
            .component_mut::<DataComponent<Camera>>(self.camera_entity)
            .unwrap();
        camera.data.update(delta_time);
    }

    fn render(&mut self, world: &mut StaticWorld) {
        let mut entities = vec![];
        let camera = world
            .component_mut::<DataComponent<Camera>>(self.camera_entity)
            .unwrap();
        let view_matrix = camera.data.view();

        for (e, c) in world.entities_with_component::<DrawComponent>() {
            let mut center = Vec3::origin();
            if let Some(body) = world.component::<BodyComponent>(e) {
                center = body.center;
            }

            let model_view_mat = view_matrix * c.model_matrix;
            let center_in_view = model_view_mat * Vec4::from_vec3(center, 1.0);
            let mut depth = -center_in_view.z;
            if !depth.is_normal() {
                depth = 0.0;
            }

            entities.push((e, depth, c.render_seq.has_transparent()));
        }

        entities.sort_by(|(_, depth_a, transp_a), (_, depth_b, transp_b)| {
            if transp_a != transp_b {
                transp_a.cmp(transp_b)
            } else {
                depth_a.partial_cmp(depth_b).unwrap()
            }
        });

        for (e, _, _) in entities {
            let c = world.component::<DrawComponent>(e).unwrap();
            self.resources.model_matrix = c.model_matrix;
            let camera = world
                .component::<DataComponent<Camera>>(self.camera_entity)
                .unwrap();
            camera
                .data
                .render_to_screen(&c.render_seq, &mut self.resources);
        }
    }

    fn window_event(&mut self, event: &GlutinWindowEvent, world: &mut StaticWorld) {
        let camera = world
            .component_mut::<DataComponent<Camera>>(self.camera_entity)
            .unwrap();
        camera.data.on_window_event(event);

        if let GlutinWindowEvent::Resized(s) = event {
            self.resources.window_info.size = Vec2::new(s.width as f32, s.height as f32);
        }
    }

    fn device_event(&mut self, event: &GlutinDeviceEvent, world: &mut StaticWorld) {
        let camera = world
            .component_mut::<DataComponent<Camera>>(self.camera_entity)
            .unwrap();
        camera.data.on_device_event(event);
    }
}

impl DrawSystem {
    pub fn new<C>(world: &mut World, controller: C) -> DrawSystem
    where
        C: CameraController,
    {
        let draw_res = DrawResources::new(world.window_info().unwrap()).unwrap();

        let camera_entity = world.as_static_mut().entity();
        world.as_static_mut().add_component(
            camera_entity,
            DataComponent::<Camera> {
                data: Camera::new(controller),
            },
        );

        DrawSystem {
            camera_entity,
            resources: draw_res,
        }
    }
}
