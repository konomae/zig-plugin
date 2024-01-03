use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "Zig";

#[plugin_fn]
pub fn register_tool(_: ()) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}
