use hyprland::dispatch::*;
use hyprland::event_listener::LayoutEvent;
use hyprland::shared::HResult;
use hyprland::{dispatch::Dispatch, event_listener::EventListener, shared::WorkspaceType};

fn main() -> HResult<()> {
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|id| println!("workspace changed to {id:#?}"));
    event_listener.add_workspace_change_handler(|id| {
        if id == WorkspaceType::Regular('9'.to_string()) {
            Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
                2,
            )))
            .unwrap();
        }
    });
    let keyboard_handler = |layout_data: LayoutEvent| {
        let noice = layout_data.0.splitn(2, ',').nth(1);
        println!("{}", noice.unwrap());
    };
    event_listener.add_keyboard_layout_change_handler(keyboard_handler);
    event_listener.start_listener()
}
