enum LoaderName {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}
struct LoaderInfo {
    name: LoaderName,
    version: Option<String>,
}
struct ModInfo {
    name: String,
    url: String,
    version: Option<String>,
}
struct MCXConfig {
    loader_info: LoaderInfo,
    mods: Vec<ModInfo>,
}
