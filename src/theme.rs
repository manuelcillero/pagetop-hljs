use std::collections::HashMap;

use pagetop::prelude::*;

/// Supported themes.
///
/// Themes are identified as *PascalCase* enums and as *kebab-case* strings.
///
/// ```rust
/// use pagetop_hljs::HljsTheme;
///
/// assert_eq!(HljsTheme::AtelierPlateauLight.to_string(), "atelier-plateau-light".to_owned());
/// ```
#[derive(Eq, Hash, PartialEq)]
pub enum HljsTheme {
    A11yDark,
    A11yLight,
    Agate,
    AnOldHope,
    Androidstudio,
    ArduinoLight,
    Arta,
    Ascetic,
    AtelierCave,
    AtelierCaveLight,
    AtelierDune,
    AtelierDuneLight,
    AtelierEstuary,
    AtelierEstuaryLight,
    AtelierForest,
    AtelierForestLight,
    AtelierHeath,
    AtelierHeathLight,
    AtelierLakeside,
    AtelierLakesideLight,
    AtelierPlateau,
    AtelierPlateauLight,
    AtelierSavanna,
    AtelierSavannaLight,
    AtelierSeaside,
    AtelierSeasideLight,
    AtelierSulphurpool,
    AtelierSulphurpoolLight,
    AtomOneDark,
    AtomOneDarkReasonable,
    AtomOneLight,
    BrownPaper,
    CodepenEmbed,
    ColorBrewer,
    Darcula,
    Dark,
    Default,
    Devibeans,
    Docco,
    Dracula,
    Far,
    Foundation,
    Framer,
    Gigavolt,
    Github,
    Gml,
    Googlecode,
    GradientDark,
    GradientLight,
    Grayscale,
    GruvboxDarkHard,
    GruvboxLightHard,
    Hopscotch,
    Hybrid,
    Idea,
    IrBlack,
    KimbieDark,
    KimbieLight,
    Lightfair,
    Lioshi,
    Magula,
    MonoBlue,
    Monokai,
    MonokaiSublime,
    NightOwl,
    NnfxDark,
    NnfxLight,
    Obsidian,
    Ocean,
    Oceanicnext,
    PandaSyntaxDark,
    PandaSyntaxLight,
    Pojoaque,
    Purebasic,
    QtcreatorDark,
    QtcreatorLight,
    Railcasts,
    Rainbow,
    Routeros,
    SchoolBook,
    ShapesOfPurple,
    SolarizedDark,
    SolarizedLight,
    Srcery,
    StackoverflowDark,
    StackoverflowLight,
    Sunburst,
    TokioNightDark,
    TokioNightLight,
    Tomorrow,
    TomorrowNight,
    TomorrowNightBlue,
    TomorrowNightBright,
    Vs,
    Vs2015,
    Xcode,
    Xt256,
    Zenburn,
}

pub(crate) static HLJS_THEMES: LazyStatic<HashMap<HljsTheme, &'static str>> =
    LazyStatic::new(|| {
        use HljsTheme::*;
        kv![
            A11yDark                => "a11y-dark",
            A11yLight               => "a11y-light",
            Agate                   => "agate",
            AnOldHope               => "an-old-hope",
            Androidstudio           => "androidstudio",
            ArduinoLight            => "arduino-light",
            Arta                    => "arta",
            Ascetic                 => "ascetic",
            AtelierCave             => "atelier-cave",                 // base16
            AtelierCaveLight        => "atelier-cave-light",           // base16
            AtelierDune             => "atelier-dune",                 // base16
            AtelierDuneLight        => "atelier-dune-light",           // base16
            AtelierEstuary          => "atelier-estuary",              // base16
            AtelierEstuaryLight     => "atelier-estuary-light",        // base16
            AtelierForest           => "atelier-forest",               // base16
            AtelierForestLight      => "atelier-forest-light",         // base16
            AtelierHeath            => "atelier-heath",                // base16
            AtelierHeathLight       => "atelier-heath-light",          // base16
            AtelierLakeside         => "atelier-lakeside",             // base16
            AtelierLakesideLight    => "atelier-lakeside-light",       // base16
            AtelierPlateau          => "atelier-plateau",              // base16
            AtelierPlateauLight     => "atelier-plateau-light",        // base16
            AtelierSavanna          => "atelier-savanna",              // base16
            AtelierSavannaLight     => "atelier-savanna-light",        // base16
            AtelierSeaside          => "atelier-seaside",              // base16
            AtelierSeasideLight     => "atelier-seaside-light",        // base16
            AtelierSulphurpool      => "atelier-sulphurpool",          // base16
            AtelierSulphurpoolLight => "atelier-sulphurpool-light",    // base16
            AtomOneDark             => "atom-one-dark",
            AtomOneDarkReasonable   => "atom-one-dark-reasonable",
            AtomOneLight            => "atom-one-light",
            BrownPaper              => "brown-paper",
            CodepenEmbed            => "codepen-embed",
            ColorBrewer             => "color-brewer",
            Darcula                 => "darcula",                      // base16
            Dark                    => "dark",
            Default                 => "default",
            Devibeans               => "devibeans",
            Docco                   => "docco",
            Dracula                 => "dracula",                      // base16
            Far                     => "far",
            Foundation              => "foundation",
            Framer                  => "framer",                       // base16
            Gigavolt                => "gigavolt",                     // base16
            Github                  => "github",
            Gml                     => "gml",
            Googlecode              => "googlecode",
            GradientDark            => "gradient-dark",
            GradientLight           => "gradient-light",
            Grayscale               => "grayscale",
            GruvboxDarkHard         => "gruvbox-dark-hard",            // base16
            GruvboxLightHard        => "gruvbox-light-hard",           // base16
            Hopscotch               => "hopscotch",                    // base16
            Hybrid                  => "hybrid",
            Idea                    => "idea",
            IrBlack                 => "ir-black",
            KimbieDark              => "kimbie-dark",
            KimbieLight             => "kimbie-light",
            Lightfair               => "lightfair",
            Lioshi                  => "lioshi",
            Magula                  => "magula",
            MonoBlue                => "mono-blue",
            Monokai                 => "monokai",
            MonokaiSublime          => "monokai-sublime",
            NightOwl                => "night-owl",
            NnfxDark                => "nnfx-dark",
            NnfxLight               => "nnfx-light",
            Obsidian                => "obsidian",
            Ocean                   => "ocean",                        // base16
            Oceanicnext             => "oceanicnext",                  // base16
            PandaSyntaxDark         => "panda-syntax-dark",
            PandaSyntaxLight        => "panda-syntax-light",
            Pojoaque                => "pojoaque",
            Purebasic               => "purebasic",
            QtcreatorDark           => "qtcreator-dark",
            QtcreatorLight          => "qtcreator-light",
            Railcasts               => "railcasts",                    // base16
            Rainbow                 => "rainbow",
            Routeros                => "routeros",
            SchoolBook              => "school-book",
            ShapesOfPurple          => "shapes-of-purple",
            SolarizedDark           => "solarized-dark",               // base16
            SolarizedLight          => "solarized-light",              // base16
            Srcery                  => "srcery",
            StackoverflowDark       => "stackoverflow-dark",
            StackoverflowLight      => "stackoverflow-light",
            Sunburst                => "sunburst",
            TokioNightDark          => "tokio-night-dark",
            TokioNightLight         => "tokio-night-light",
            Tomorrow                => "tomorrow",                     // base16
            TomorrowNight           => "tomorrow-night",               // base16
            TomorrowNightBlue       => "tomorrow-night-blue",
            TomorrowNightBright     => "tomorrow-night-bright",
            Vs                      => "vs",
            Vs2015                  => "vs2015",
            Xcode                   => "xcode",
            Xt256                   => "xt256",
            Zenburn                 => "zenburn",                      // base16
        ]
    });

impl ToString for HljsTheme {
    fn to_string(&self) -> String {
        HLJS_THEMES.get(self).unwrap().to_string()
    }
}

impl HljsTheme {
    pub fn to_url(theme: &str) -> String {
        concat_string!("/hljs/css/", theme, ".min.css")
    }
}
