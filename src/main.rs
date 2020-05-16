mod wl_protocol_idle;

use wayland_client::Display;
use self::wl_protocol_idle::idle_protocol::org_kde_kwin_idle::OrgKdeKwinIdle;

fn main() {
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    //    let globals = GlobalManager::new(&attached_display);

    let registry = attached_display.get_registry();
    registry.bind::<OrgKdeKwinIdle>(1, 1);

    // roundtrip to retrieve the globals list
    event_queue.sync_roundtrip(&mut (), |_, _, _| unreachable!());

}
