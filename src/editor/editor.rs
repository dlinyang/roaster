use titanium::*;
use titanium::application::*;
use titanium::base::camera::Camera;
use titanium::base::color::*;
use titanium::event::*;
use titanium::renderer::*;
use titanium::backend::glium::*;
use titanium::backend::glium::renderer::GLRenderer;
use titanium::gui::layout::Layout;
use rmu::raw::ID4F;

pub struct Editor {
    pub config: Config,
    pub event_access_speed: f32,
}

impl Editor {
    pub fn new() -> Self {
        let mut config = Config::create("roaster".to_string(), 1240.0, 720.0);
        config.world.background = GREY31;
        Editor {
            config,
            event_access_speed: 60.0,
        }
    }

    pub fn run(&mut self) {
        let mut application:GLApplication = Application::new(self.config.clone());

        let mut font_builder = FontBuilder::new();
        font_builder.add("fira_code".into(), "font/FiraCode-Regular.ttf".into());
        let mut layout = Layout::new(self.config.window.size);
        use super::widget::menu::TopMenuBuilder;
        layout.add_widget(TopMenuBuilder::new("top".into(), 20.0, GREY11, "fira_code".into(), WHITE).to_widget());
        layout.build();

        use std::time::Duration;
        use std::thread::sleep;
 
        let sleep_time = Duration::from_millis( (1000.0 / self.event_access_speed) as u64);

        let cube = model::generate::cube(1.0);

        let mut camera = Camera::new([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0,0.0,1.0]);
        let mut transform = base::transform::Transform::new();
        transform.traslation = [0.0, 2.0, 0.0];
        let mut scene_render_data = SceneRenderData::new();
        scene_render_data.load_mesh("cube".into(), &cube, ID4F);
        scene_render_data.load_mesh("cube1".into(), &cube, transform.transform().into());

        let parallel = Light::parallel_light([0.4,0.5,0.2], [1.0,1.0,0.0]);
        let point = Light::point_light([0.5,0.6,0.2], [2.0,1.0,1.0]);
        let spot = Light::spot_light([1.0, 0.4, 0.6],[2.0, 0.0, 0.0], [-2.0, 0.0, 0.0], 0.3f32, 0.1);

        let mut event_system = application.get_event_system();

        let mut renderer = application.get_renderer();

        renderer.font_builder = font_builder;

        renderer.update_scene(&mut scene_render_data);
        renderer.update_canvas(&mut layout.canvas);
        renderer.update_camera(&camera);
        renderer.update_light(&String::from("parallel"), &parallel);
        renderer.update_light(&String::from("point"), &point);
        renderer.update_light(&String::from("spot"), &spot);
        renderer.clear();
        renderer.render_scene();
        renderer.render_canvas();
        renderer.swap_buffer();

        use super::signal::*;

        let mut drag = Drag::new(MouseButton::Middle);

        let mut q_w = KeyCombination::new();
        q_w.keys = vec![Key::Q,Key::W];

        loop {
            let mut flag = false;
            let event = event_system.get_event();

            if event == Event::Exit {
                break;
            }

            q_w.match_event(event);

            if q_w.matched() {
                break;
            }

            drag.match_event(event);

            if drag.state == DragState::DragEnd || drag.state == DragState::Dragging {
                let m = drag.get_move();
                let x = m[0] / self.config.window.size.width;
                let y = m[1] / self.config.window.size.height;

                camera.rotation_with_look_at(x, y, 0.0);

                renderer.update_camera(&camera);

                flag = true;

                if drag.state == DragState::DragEnd {
                    drag.state = DragState::NoDrag;
                }
            }

            match event { 
                Event::KeyEvent{state: ButtonState::Press, key} => match key {
                    Key::Up => {
                        camera.translation(0.1, 0.0, 0.0);
                        renderer.update_camera(&camera);
                        flag = true;
                    },
                    Key::Down => {
                        camera.translation(-0.1, 0.0, 0.0);
                        renderer.update_camera(&camera);
                        flag = true;
                    },
                    _ => (),
                },
                _ => ()
            }

            if flag {
                renderer.clear();
                renderer.render_scene();
                renderer.render_canvas();
                renderer.swap_buffer();
            }


            std::thread::sleep(sleep_time);
        }
    }
}