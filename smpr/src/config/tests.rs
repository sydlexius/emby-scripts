use super::*;

#[test]
fn parse_full_toml() {
    let toml = r#"
[servers.home-emby]
url = "http://192.168.1.126:8096"
type = "emby"

[servers.home-emby.libraries.Music]
force_rating = "PG-13"

[servers.home-emby.libraries.Music.locations.classical]
force_rating = "G"

[servers.prod-jellyfin]
url = "http://prod.example.com:8096"
type = "jellyfin"

[servers.prod-jellyfin.libraries."Classical Music"]
force_rating = "G"

[detection.r]
stems = ["fuck", "shit"]
exact = ["blowjob"]

[detection.pg13]
stems = ["bitch"]
exact = ["hoe"]

[detection.ignore]
false_positives = ["cocktail", "hancock"]

[detection.g_genres]
genres = ["Classical", "Soundtrack"]

[general]
overwrite = false

[report]
output_path = "/tmp/report.csv"
"#;

    let raw = parse_toml(toml).expect("should parse full TOML");

    // Servers
    let servers = raw.servers.as_ref().unwrap();
    assert_eq!(servers.len(), 2);

    let emby = &servers["home-emby"];
    assert_eq!(emby.url.as_deref(), Some("http://192.168.1.126:8096"));
    assert_eq!(emby.server_type.as_deref(), Some("emby"));
    let libs = emby.libraries.as_ref().unwrap();
    assert_eq!(libs["Music"].force_rating.as_deref(), Some("PG-13"));
    let locs = libs["Music"].locations.as_ref().unwrap();
    assert_eq!(locs["classical"].force_rating.as_deref(), Some("G"));

    let jf = &servers["prod-jellyfin"];
    assert_eq!(jf.server_type.as_deref(), Some("jellyfin"));
    let jf_libs = jf.libraries.as_ref().unwrap();
    assert_eq!(jf_libs["Classical Music"].force_rating.as_deref(), Some("G"));

    // Detection
    let det = raw.detection.as_ref().unwrap();
    let r = det.r.as_ref().unwrap();
    assert_eq!(r.stems.as_deref().unwrap(), &["fuck", "shit"]);
    assert_eq!(r.exact.as_deref().unwrap(), &["blowjob"]);
    let pg13 = det.pg13.as_ref().unwrap();
    assert_eq!(pg13.stems.as_deref().unwrap(), &["bitch"]);
    assert_eq!(pg13.exact.as_deref().unwrap(), &["hoe"]);
    let ignore = det.ignore.as_ref().unwrap();
    assert_eq!(
        ignore.false_positives.as_deref().unwrap(),
        &["cocktail", "hancock"]
    );
    let genres = det.g_genres.as_ref().unwrap();
    assert_eq!(
        genres.genres.as_deref().unwrap(),
        &["Classical", "Soundtrack"]
    );

    // General
    assert_eq!(raw.general.as_ref().unwrap().overwrite, Some(false));

    // Report
    assert_eq!(
        raw.report.as_ref().unwrap().output_path.as_deref(),
        Some("/tmp/report.csv")
    );
}

#[test]
fn parse_minimal_toml() {
    let toml = r#"
[servers.myserver]
url = "http://localhost:8096"
"#;

    let raw = parse_toml(toml).expect("should parse minimal TOML");
    let servers = raw.servers.as_ref().unwrap();
    assert_eq!(servers.len(), 1);
    assert_eq!(
        servers["myserver"].url.as_deref(),
        Some("http://localhost:8096")
    );
    assert!(servers["myserver"].server_type.is_none());
    assert!(servers["myserver"].libraries.is_none());
    assert!(raw.detection.is_none());
    assert!(raw.general.is_none());
    assert!(raw.report.is_none());
}

#[test]
fn parse_empty_toml() {
    let raw = parse_toml("").expect("should parse empty TOML");
    assert!(raw.servers.is_none());
    assert!(raw.detection.is_none());
    assert!(raw.general.is_none());
    assert!(raw.report.is_none());
}

#[test]
fn parse_server_with_type_override() {
    let toml = r#"
[servers.jf]
url = "http://jf.local:8096"
type = "jellyfin"
"#;

    let raw = parse_toml(toml).expect("should parse");
    let servers = raw.servers.unwrap();
    assert_eq!(servers["jf"].server_type.as_deref(), Some("jellyfin"));
}

#[test]
fn parse_partial_detection_override() {
    let toml = r#"
[detection.r]
stems = ["custom"]
exact = ["custom_exact"]
"#;

    let raw = parse_toml(toml).expect("should parse partial detection");
    let det = raw.detection.as_ref().unwrap();
    assert!(det.r.is_some());
    assert_eq!(det.r.as_ref().unwrap().stems.as_deref().unwrap(), &["custom"]);
    assert_eq!(
        det.r.as_ref().unwrap().exact.as_deref().unwrap(),
        &["custom_exact"]
    );
    assert!(det.pg13.is_none());
    assert!(det.ignore.is_none());
    assert!(det.g_genres.is_none());
}

#[test]
fn parse_unknown_fields_ignored() {
    let toml = r#"
unknown_top_level = "should be ignored"
another_unknown = 42

[servers.test]
url = "http://localhost:8096"
extra_field = "ignored"

[detection]
unknown_nested = true

[some_unknown_section]
key = "value"
"#;

    let raw = parse_toml(toml).expect("unknown fields should be silently ignored");
    let servers = raw.servers.unwrap();
    assert_eq!(
        servers["test"].url.as_deref(),
        Some("http://localhost:8096")
    );
}
