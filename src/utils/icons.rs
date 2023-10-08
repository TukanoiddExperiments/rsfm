use macros::register_icons;

#[register_icons(
    egui_root = eframe::egui,
    crate_mod = egui_phosphor::regular,
    svgs = "../../assets/icons/phosphor/regular"
)]
enum PhosphorIcon {
    #[ri()]
    Folder,
    #[ri()]
    File,
    #[ri()]
    Link,
    #[ri()]
    SealWarning,
    #[ri()]
    Desktop,
    #[ri()]
    Palette,
}
