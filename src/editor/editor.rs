use titanium::*;
use titanium::base::color::*;
use titanium::event::*;
use titanium::renderer::*;
use titanium::backend::glium::*;

pub struct Editor {
    pub application: Application,
}

impl Editor {
    pub fn new() -> Self {
        let mut application = Application::create("roaster".to_string(), 1240.0, 720.0);
        application.world.background = GREY31;
        Editor {
            application,
        }
    }

    pub fn run(&self) {
        let mut backend = self.application.init();
        
        backend.font_builder.add("Firacode-Regular".to_string(),"font/Firacode-Regular.ttf".to_string());

        use titanium::user_interface::layout::*;
        use titanium::scene::object::Object;
        use titanium::scene::camera::Camera;

        use base::Size;
        use super::widget::*;

        let mut layout = Layout::new(self.application.window.size);
        layout.font = Some("Firacode-Regular".to_string());
        let mut head = TopBarBuilder::new(20.0, GREY11, WHITE);
        head.list = vec!["File".into(),"Edit".into(), "Info".into()];
        let mut info_bar = InfoBarBuilder::new(20.0, GREY11, WHITE);
        layout.add_widget(head.to_widget());
        layout.add_widget(info_bar.to_widget());
        let canvas = layout.build();

        let cube = model::generate::cube("cube".to_string(),1.0, scene::material::SimpleMaterial([1.0,1.0,1.0]));
        let mut scene = scene::Scene::new("Main".to_string());
        scene.insert_object(cube);

        let camera = Camera::new([2.0,-1.0,1.0], [0.0, 0.0, 0.0], [0.0,1.0,0.0]);

        let render_data = RenderData {
            scene: Some(scene),
            canvas: Some(canvas),
        };

        backend.load(&render_data);

        backend.update_camera(&camera);

        backend.set_render_edge("cube".to_string(),true);

        backend.render_scene();

        backend.render_canvas();

        backend.render();

        loop {
            let event = backend.get_event();
            match event {
                Event::Exit => break,
                Event::CursorEvent{x,y} => {
                    if let Some(name) = layout.event(x, y) {

                    }
                }
                _ => (),
            }
        }

    }
}