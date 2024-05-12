#[derive(derive_more::Display)]

pub enum Template {
    #[display(fmt = "welcome")]
    Welcome,
}

#[derive(derive_more::Display)]
pub enum Banner {
    #[display(fmt = "space")]
    Space,

    #[display(fmt = "love")]
    Love,

    #[display(fmt = "purplewave")]
    PurpleWave,

    #[display(fmt = "mountain")]
    Mountain,

    #[display(fmt = "wave")]
    Wave,

    #[display(fmt = "sunset")]
    Sunset,

    #[display(fmt = "rainbow")]
    Rainbow,

    #[display(fmt = "swamp")]
    Swamp,

    #[display(fmt = "waifubot")]
    WaifuBot,
}

#[derive(derive_more::Display)]
pub enum Icons {
    #[display(fmt = "dragon")]
    Dragon,

    #[display(fmt = "dog")]
    Dog,

    #[display(fmt = "pikachu")]
    Pikachu,

    #[display(fmt = "cat")]
    Cat,

    #[display(fmt = "senko")]
    Senko,

    #[display(fmt = "neko")]
    Neko,

    #[display(fmt = "pepe")]
    Pepe,

    #[display(fmt = "shrek")]
    Shrek,

    #[display(fmt = "nyancat")]
    NyanCat,

    #[display(fmt = "chika")]
    Chika,
}

#[derive(derive_more::Display)]
pub enum Fonts {
    #[display(fmt = "DejaVu Serif")]
    DejaVuSerif,

    #[display(fmt = "URW Bookman")]
    URWBookman,

    #[display(fmt = "DejaVu Sans Mono")]
    DejaVuSansMono,

    #[display(fmt = "Argentum Sans")]
    ArgentumSans,

    #[display(fmt = "Argentum Sans Thin")]
    ArgentumSansThin,

    #[display(fmt = "Whitney Light")]
    WhitneyLight,

    #[display(fmt = "Whitney")]
    Whitney,

    #[display(fmt = "DejaVu Sans")]
    DejaVuSans,

    #[display(fmt = "DejaVu Sans Condensed")]
    DejaVuSansCondensed,

    #[display(fmt = "Argentum Sans Black")]
    ArgentumSansBlack,

    #[display(fmt = "Nimbus Mono PS")]
    NimbusMonoPS,

    #[display(fmt = "Starborn")]
    Starborn,

    #[display(fmt = "P052")]
    P052,

    #[display(fmt = "DejaVu Serif Condensed")]
    DejaVuSerifCondensed,

    #[display(fmt = "DejaVu Sans Light")]
    DejaVuSansLight,

    #[display(fmt = "Nimbus Sans")]
    NimbusSans,

    #[display(fmt = "Nimbus Sans Narrow")]
    NimbusSansNarrow,

    #[display(fmt = "C059")]
    C059,

    #[display(fmt = "KG Blank Space Solid")]
    KGBankSpaceSolid,

    #[display(fmt = "Z003")]
    Z003,

    #[display(fmt = "URW Gothic")]
    URWGothic,

    #[display(fmt = "Whitney Book")]
    WhitneyBook,

    #[display(fmt = "D050000L")]
    D050000L,

    #[display(fmt = "Argentum Sans ExtraLight")]
    ArgentumSansExtraLight,

    #[display(fmt = "Nimbus Roman")]
    NimbusRoman,

    #[display(fmt = "Segoe UI Emoji")]
    SegoeUIEmoji,

    #[display(fmt = "DejaVu Math TeX Gyre")]
    DejaVuMathTeXGyre,

    #[display(fmt = "Argentum Sans Medium")]
    ArgentumSansMedium,

    #[display(fmt = "Argentum Sans SemiBold")]
    ArgentumSansSemiBold,

    #[display(fmt = "Argentum Sans ExtraBold")]
    ArgentumSansExtraBold,

    #[display(fmt = "Daydream")]
    Daydream,

    #[display(fmt = "Rounded Mplus 1c")]
    RoundedMplus1c,

    #[display(fmt = "Rounded Mplus 1c Medium")]
    RoundedMplus1cMedium,

    #[display(fmt = "Future Earth")]
    FutureEarth,

    #[display(fmt = "Malgun Gothic")]
    MalgunGothic,

    #[display(fmt = "맑은 고딕")]
    MaGeunGothic, // "맑은 고딕"

    #[display(fmt = "Rounded Mplus 1c Light")]
    RoundedMplus1cLight,

    #[display(fmt = "Whitney Semibold")]
    WhitneySemibold,

    #[display(fmt = "Argentum Sans Light")]
    ArgentumSansLight,

    #[display(fmt = "Rounded Mplus 1c ExtraBold")]
    RoundedMplus1cExtraBold,

    #[display(fmt = "Standard Symbols PS")]
    StandardSymbolsPS,

    #[display(fmt = "Riffic Free")]
    RifficFree,

    #[display(fmt = "Riffic Free Medium")]
    RifficFreeMedium,

    #[display(fmt = "Rounded Mplus 1c Bold")]
    RoundedMplus1cBold,

    #[display(fmt = "Rounded Mplus 1c Black")]
    RoundedMplus1cBlack,

    #[display(fmt = "Rounded Mplus 1c Thin")]
    RoundedMplus1cThin,

    #[display(fmt = "Crang")]
    Crang,

    #[display(fmt = "KG Blank Space Sketch")]
    KGBankSpaceSketch,

    #[display(fmt = "! PEPSI !")]
    Pepsi,
}
