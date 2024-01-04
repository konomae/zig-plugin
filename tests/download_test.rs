use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_download_install_tests!("zig-test", "0.11.0");

mod canary {
    use super::*;

    generate_download_install_tests!("zig-test", "canary");
}

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-linux-aarch64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-linux-aarch64-0.11.0.tar.xz.minisig"
                    .into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-linux-aarch64-0.11.0.tar.xz".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-linux-aarch64-0.11.0.tar.xz"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-linux-x86_64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-linux-x86_64-0.11.0.tar.xz".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x86() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X86,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-linux-x86-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-linux-x86-0.11.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-linux-x86-0.11.0.tar.xz".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-linux-x86-0.11.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-macos-aarch64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-macos-aarch64-0.11.0.tar.xz.minisig"
                    .into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-macos-aarch64-0.11.0.tar.xz".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-macos-aarch64-0.11.0.tar.xz"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-macos-x86_64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-macos-x86_64-0.11.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-macos-x86_64-0.11.0.tar.xz".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-macos-x86_64-0.11.0.tar.xz"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-windows-aarch64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-windows-aarch64-0.11.0.zip.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-windows-aarch64-0.11.0.zip".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-windows-aarch64-0.11.0.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-windows-x86_64-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-windows-x86_64-0.11.0.zip.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-windows-x86_64-0.11.0.zip".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-windows-x86_64-0.11.0.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x86() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X86,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("zig-windows-x86-0.11.0".into()),
            checksum_url: Some(
                "https://ziglang.org/download/0.11.0/zig-windows-x86-0.11.0.zip.minisig".into()
            ),
            checksum_public_key: Some(
                "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into()
            ),
            download_name: Some("zig-windows-x86-0.11.0.zip".into()),
            download_url: "https://ziglang.org/download/0.11.0/zig-windows-x86-0.11.0.zip".into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.11.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("zig".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zig-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.11.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("zig.exe".into())
    );
}
