use adblock::lists::{FilterSet, ParseOptions};
use adblock::resources::{MimeType, PermissionMask, Resource, ResourceType};
use adblock::Engine;
use std::sync::{Arc, Mutex};

pub fn create_adblock_engine() -> Arc<Mutex<Engine>> {
    let all_filters = [
        include_str!("../assets/filters/annoyances-cookies.txt"),
        include_str!("../assets/filters/annoyances-others.txt"),
        include_str!("../assets/filters/annoyances.txt"),
        include_str!("../assets/filters/badlists.txt"),
        include_str!("../assets/filters/badware.txt"),
        include_str!("../assets/filters/filters-2025.txt"),
        include_str!("../assets/filters/filters-general.txt"),
        include_str!("../assets/filters/filters-mobile.txt"),
        include_str!("../assets/filters/filters.txt"),
        include_str!("../assets/filters/lan-block.txt"),
        include_str!("../assets/filters/legacy.txt"),
        include_str!("../assets/filters/privacy.txt"),
        include_str!("../assets/filters/quick-fixes.txt"),
        include_str!("../assets/filters/resource-abuse.txt"),
        include_str!("../assets/filters/ubo-link-shorteners.txt"),
        include_str!("../assets/filters/ubol-filters.txt"),
        include_str!("../assets/filters/youtube_and_google.txt"),
    ];

    let mut filter_set = FilterSet::default();

    for filter in all_filters {
        filter_set.add_filters(filter.lines(), ParseOptions::default());
    }

    let mut engine = Engine::from_filter_set(filter_set, true);

    engine.use_resources([get_scriptlet_resource()]);

    Arc::new(Mutex::new(engine))
}

fn get_scriptlet_resource() -> Resource {
    let scriptlet_js = include_str!("../assets/scriptlets.js");

    Resource {
        name: "scriptlets.js".to_string(),
        aliases: vec!["scriptlets.js".to_string()],
        kind: ResourceType::Mime(MimeType::ApplicationJavascript),
        content: scriptlet_js.to_string(),
        dependencies: vec![],
        permission: PermissionMask::default(),
    }
}
