macro_rules! debug_resource {
    ($app:expr, $resource:ty) => {{
        $app.add_plugins(
            bevy_inspector_egui::quick::ResourceInspectorPlugin::<$resource>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
    }};
}
pub(crate) use debug_resource;
