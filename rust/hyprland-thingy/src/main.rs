use hyprland::dispatch::WorkspaceIdentifier;
use hyprland::event_listener::EventListener;
use hyprland::event_listener::LayoutEvent;
use hyprland::shared::HResult;
use hyprland::shared::WorkspaceType;

fn main() -> HResult<()> {
    let mut event_listener = EventListener::new();
    let keyboard_handler = |layout_data: LayoutEvent| {
        // split the MF in half then take the second half
        let layout_name = layout_data.0.splitn(2, ',').nth(1);
        println!("{}", layout_name.unwrap());
    };
    event_listener.add_keyboard_layout_change_handler(keyboard_handler);
    event_listener.start_listener()
}
