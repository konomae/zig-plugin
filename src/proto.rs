use crate::zig_dist::ZigDist;
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "Zig";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        minimum_proto_version: Some(Version::new(0, 47, 4)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::Linux => [HostArch::X86, HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X86, HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let mut version = input.context.version;
    if version.is_canary() {
        let response: ZigDist = fetch_json("https://ziglang.org/download/index.json")?;
        version = VersionSpec::parse(response.master.version)?;
    }

    let arch = match env.arch {
        HostArch::X86 => "x86",
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "aarch64",
        _ => unreachable!(),
    };

    let os = env.os;

    let prefix = match os {
        HostOS::Linux => format!("zig-linux-{arch}-{version}"),
        HostOS::MacOS => format!("zig-macos-{arch}-{version}"),
        HostOS::Windows => format!("zig-windows-{arch}-{version}"),
        _ => unreachable!(),
    };

    let filename = if os.is_windows() {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.xz")
    };

    let directory = match &version {
        VersionSpec::Semantic(v) if !v.build.is_empty() => "builds".to_string(),
        _ => format!("download/{version}"),
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        checksum_url: Some(format!(
            "https://ziglang.org/{directory}/{filename}.minisig"
        )),
        checksum_public_key: Some(
            "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into(),
        ),
        download_url: format!("https://ziglang.org/{directory}/{filename}"),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "zig".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("zig")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let response: ZigDist = fetch_json("https://ziglang.org/download/index.json")?;
    let mut versions: Vec<String> = response.versions.keys().map(|t| t.to_owned()).collect();
    versions.push(response.master.version.clone());

    let mut output = LoadVersionsOutput::from(versions)?;
    output.aliases.insert(
        "master".into(),
        UnresolvedVersionSpec::parse(&response.master.version)?,
    );

    Ok(Json(output))
}
