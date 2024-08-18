use proto_pdk_test_utils::*;

generate_resolve_versions_tests!("zig-test", {
    "0.10" => "0.10.1",
    "0.11.0" => "0.11.0",
    "0" => "0.13.0",
});

#[tokio::test(flavor = "multi_thread")]
async fn loads_versions_from_dist_url() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test").await;

    let output = plugin.load_versions(LoadVersionsInput::default()).await;

    assert!(!output.versions.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn sets_latest_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test").await;

    let output = plugin.load_versions(LoadVersionsInput::default()).await;

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[tokio::test(flavor = "multi_thread")]
async fn sets_master_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zig-test").await;

    let output = plugin.load_versions(LoadVersionsInput::default()).await;

    assert!(output.aliases.contains_key("latest"));
    assert!(!output
        .aliases
        .get("master")
        .unwrap()
        .to_resolved_spec()
        .as_version()
        .unwrap()
        .build
        .is_empty());
}
