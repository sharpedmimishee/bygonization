use bygonizer;
use gpui::*;
use gpui_component::*;
use gpui_component::button::*;
use gpui_component::menu::*;
use gpui_component::Size;

pub struct Menu;

impl Render for Menu {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
        .v_flex()
        .gap_2()
        .child(
            ButtonGroup::new("Menu")
            .child(
                Button::new("File")
                    .primary()
                    .label("File")
                    .bg(rgb(0x2e2e2e))
                    .on_click(|_, _, _| println!("Clicked!")),
            )
            .child(
                Button::new("Layout")
                    .primary()
                    .label("Layout")
                    .bg(rgb(0x2e2e2e))
                    .on_click(|_, _, _| println!("Clicked!")),
            )
            .child(
                Button::new("Settings")
                    .primary()
                    .label("Settings")
                    .bg(rgb(0x2e2e2e))
                    .on_click(|_, _, _| println!("Clicked!")),
            )
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions { title: Some(SharedString::new_static("Bygonization")), ..Default::default()}),

                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| Menu);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view.into(), window, cx))
            },
        ).unwrap();
    })
}
