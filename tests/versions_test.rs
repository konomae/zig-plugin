use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("zig-test", {
    "0.10" => "0.10.1",
    "0.11.0" => "0.11.0",
    "0" => "0.11.0",
});

#[test]
fn loads_versions_from_dist_url() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("zig-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("zig-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}
