use crate::prelude::*;

#[derive(Clone, Copy)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Component)]
struct TrafficLight {
    color: TrafficLightColor,
    window_has_focus: bool,
}

impl TrafficLight {
    fn new(color: TrafficLightColor, window_has_focus: bool) -> Self {
        Self {
            color,
            window_has_focus,
        }
    }

    fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        let theme = theme(cx);

        let fill = match (self.window_has_focus, self.color) {
            (true, TrafficLightColor::Red) => theme.mac_os_traffic_light_red,
            (true, TrafficLightColor::Yellow) => theme.mac_os_traffic_light_yellow,
            (true, TrafficLightColor::Green) => theme.mac_os_traffic_light_green,
            (false, _) => theme.filled_element,
        };

        div().w_3().h_3().rounded_full().bg(fill)
    }
}

#[derive(Component)]
pub struct TrafficLights {
    window_has_focus: bool,
}

impl TrafficLights {
    pub fn new() -> Self {
        Self {
            window_has_focus: true,
        }
    }

    pub fn window_has_focus(mut self, window_has_focus: bool) -> Self {
        self.window_has_focus = window_has_focus;
        self
    }

    fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(TrafficLight::new(
                TrafficLightColor::Red,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Yellow,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Green,
                self.window_has_focus,
            ))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Component)]
    pub struct TrafficLightsStory;

    impl TrafficLightsStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, TrafficLights>(cx))
                .child(Story::label(cx, "Default"))
                .child(TrafficLights::new())
                .child(Story::label(cx, "Unfocused"))
                .child(TrafficLights::new().window_has_focus(false))
        }
    }
}
