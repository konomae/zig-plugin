use proto_pdk_test_utils::*;

generate_resolve_versions_tests!("zig-test", {
    "0.10" => "0.10.1",
    "0.11.0" => "0.11.0",
    "0" => "0.11.0",
});

#[test]
fn loads_versions_from_dist_url() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test");

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test");

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[test]
fn sets_master_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test");

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.aliases.contains_key("latest"));
    assert!(!output.aliases.get("master").unwrap().build.is_empty());
}
