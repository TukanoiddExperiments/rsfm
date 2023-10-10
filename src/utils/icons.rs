use macros::register_icons;

#[register_icons(
    egui_root = eframe::egui,
    crate_mod = egui_phosphor::regular,
    svgs = "../../assets/icons/phosphor/regular"
)]
enum PhosphorIcon {
    #[ri()]
    ArrowLeft,
    #[ri()]
    ArrowRight,
    #[ri()]
    ArrowClockwise,

    #[ri()]
    Desktop,
    #[ri()]
    DotsNine,
    #[ri()]
    Download,

    #[ri()]
    Folder,
    #[ri()]
    File,

    #[ri()]
    House,

    #[ri()]
    Link,

    #[ri()]
    Palette,

    #[ri()]
    SealWarning,
}
