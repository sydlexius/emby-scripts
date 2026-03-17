use super::widgets::force_tree::build_tree;
use crate::config::*;
use std::collections::BTreeMap;

#[test]
fn build_tree_empty_config() {
    let config = RawConfig::default();
    let nodes = build_tree(&config);
    assert!(nodes.is_empty());
}

#[test]
fn build_tree_server_with_libraries_and_locations() {
    let mut locations = BTreeMap::new();
    locations.insert(
        "Classical".to_string(),
        RawLocationConfig {
            force_rating: Some("G".to_string()),
        },
    );

    let mut libraries = BTreeMap::new();
    libraries.insert(
        "Music".to_string(),
        RawLibraryConfig {
            force_rating: None,
            locations: Some(locations),
        },
    );

    let mut servers = BTreeMap::new();
    servers.insert(
        "home-emby".to_string(),
        RawServerConfig {
            url: Some("http://localhost:8096".to_string()),
            server_type: Some("emby".to_string()),
            libraries: Some(libraries),
        },
    );

    let config = RawConfig {
        servers: Some(servers),
        ..Default::default()
    };

    let nodes = build_tree(&config);
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].depth, 0);
    assert_eq!(nodes[0].label, "home-emby");
    assert_eq!(nodes[1].depth, 1);
    assert_eq!(nodes[1].label, "Music");
    assert!(nodes[1].is_library);
    assert_eq!(nodes[2].depth, 2);
    assert_eq!(nodes[2].label, "Classical");
    assert_eq!(nodes[2].force_rating, Some("G".to_string()));
}

#[test]
fn build_tree_multiple_servers() {
    let mut servers = BTreeMap::new();
    servers.insert(
        "server-a".to_string(),
        RawServerConfig {
            url: None,
            server_type: None,
            libraries: None,
        },
    );
    servers.insert(
        "server-b".to_string(),
        RawServerConfig {
            url: None,
            server_type: None,
            libraries: None,
        },
    );

    let config = RawConfig {
        servers: Some(servers),
        ..Default::default()
    };
    let nodes = build_tree(&config);
    assert_eq!(nodes.len(), 2);
}
