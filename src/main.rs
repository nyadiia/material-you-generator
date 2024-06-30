#![recursion_limit = "256"]

use material_colors::{
    dynamic_color::DynamicScheme,
    image::{FilterType, ImageReader},
    palette,
    scheme::Scheme,
    theme::{Theme, ThemeBuilder},
};

use clap::{Parser, ValueEnum};
use serde_json::{json, Value};

/// Generate a material you theme from an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image to generate the theme from
    #[arg(value_parser = check_file)]
    image: String,

    // /// Light or dark
    // #[arg(short, long, value_enum, default_value = "both")]
    // polarity: Polarity,

    // /// Contrast level
    // #[arg(short, long, value_enum)]
    // contrast: Option<Vec<Contrast>>,
    /// Color variant.
    /// Set of themes supported by Dynamic Color
    #[arg(short, long, value_enum, default_value = "tonal-spot")]
    variant: Variant, // the crate doesn't use #[derive(ValueEnum)] so it's not possible to use it here

    /// Color match.
    /// Forces the use of the exact color provided in the image
    #[arg(short, long, default_value = "false")]
    exact: bool,

    /// Output file
    #[arg(short, long, default_value = "theme.json")]
    output: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Polarity {
    Light,
    Dark,
    Both,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Contrast {
    Standard,
    Medium,
    High,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Variant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Content,
    Rainbow,
    FruitSalad,
}

fn check_file(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("File not found: {}", path))
    }
}

fn scheme_to_json(scheme: &Scheme) -> Value {
    json!({
        "primary": scheme.primary.to_hex_with_pound(),
        "onPrimary": scheme.on_primary.to_hex_with_pound(),
        "primaryContainer": scheme.primary_container.to_hex_with_pound(),
        "onPrimaryContainer": scheme.on_primary_container.to_hex_with_pound(),
        "inversePrimary": scheme.inverse_primary.to_hex_with_pound(),
        "primaryFixed": scheme.primary_fixed.to_hex_with_pound(),
        "primaryFixedDim": scheme.primary_fixed_dim.to_hex_with_pound(),
        "onPrimaryFixed": scheme.on_primary_fixed.to_hex_with_pound(),
        "onPrimaryFixedVariant": scheme.on_primary_fixed_variant.to_hex_with_pound(),
        "secondary": scheme.secondary.to_hex_with_pound(),
        "onSecondary": scheme.on_secondary.to_hex_with_pound(),
        "secondaryContainer": scheme.secondary_container.to_hex_with_pound(),
        "onSecondaryContainer": scheme.on_secondary_container.to_hex_with_pound(),
        "secondaryFixed": scheme.secondary_fixed.to_hex_with_pound(),
        "secondaryFixedDim": scheme.secondary_fixed_dim.to_hex_with_pound(),
        "onSecondaryFixed": scheme.on_secondary_fixed.to_hex_with_pound(),
        "onSecondaryFixedVariant": scheme.on_secondary_fixed_variant.to_hex_with_pound(),
        "tertiary": scheme.tertiary.to_hex_with_pound(),
        "onTertiary": scheme.on_tertiary.to_hex_with_pound(),
        "tertiaryContainer": scheme.tertiary_container.to_hex_with_pound(),
        "onTertiaryContainer": scheme.on_tertiary_container.to_hex_with_pound(),
        "tertiaryFixed": scheme.tertiary_fixed.to_hex_with_pound(),
        "tertiaryFixedDim": scheme.tertiary_fixed_dim.to_hex_with_pound(),
        "onTertiaryFixed": scheme.on_tertiary_fixed.to_hex_with_pound(),
        "onTertiaryFixedVariant": scheme.on_tertiary_fixed_variant.to_hex_with_pound(),
        "error": scheme.error.to_hex_with_pound(),
        "onError": scheme.on_error.to_hex_with_pound(),
        "errorContainer": scheme.error_container.to_hex_with_pound(),
        "onErrorContainer": scheme.on_error_container.to_hex_with_pound(),
        "surfaceDim": scheme.surface_dim.to_hex_with_pound(),
        "surface": scheme.surface.to_hex_with_pound(),
        "surfaceBright": scheme.surface_bright.to_hex_with_pound(),
        "surfaceContainerLowest": scheme.surface_container_lowest.to_hex_with_pound(),
        "surfaceContainerLow": scheme.surface_container_low.to_hex_with_pound(),
        "surfaceContainer": scheme.surface_container.to_hex_with_pound(),
        "surfaceContainerHigh": scheme.surface_container_high.to_hex_with_pound(),
        "surfaceContainerHighest": scheme.surface_container_highest.to_hex_with_pound(),
        "onSurface": scheme.on_surface.to_hex_with_pound(),
        "onSurfaceVariant": scheme.on_surface_variant.to_hex_with_pound(),
        "outline": scheme.outline.to_hex_with_pound(),
        "outlineVariant": scheme.outline_variant.to_hex_with_pound(),
        "inverseSurface": scheme.inverse_surface.to_hex_with_pound(),
        "inverseOnSurface": scheme.inverse_on_surface.to_hex_with_pound(),
        "surfaceVariant": scheme.surface_variant.to_hex_with_pound(),
        "background": scheme.background.to_hex_with_pound(),
        "onBackground": scheme.on_background.to_hex_with_pound(),
        "shadow": scheme.shadow.to_hex_with_pound(),
        "scrim": scheme.scrim.to_hex_with_pound(),
    })
}

fn tonal_palette_to_json(tonal_palette: &palette::TonalPalette) -> Value {
    const TONES: [i32; 18] = [
        0, 5, 10, 15, 20, 25, 30, 35, 40, 50, 60, 70, 80, 90, 95, 98, 99, 100,
    ];
    let tones_json: Value = TONES
        .iter()
        .map(|&tone| {
            let color = tonal_palette.tone(tone);
            let color_hex = color.to_hex_with_pound();
            let color_name = tone.to_string();
            (color_name, color_hex)
        })
        .collect();
    return json!(tones_json);
}

fn theme_to_json(theme: &Theme, schemes: [&Scheme; 4]) -> Value {
    json!({
        "schemes": {
            "light": scheme_to_json(&theme.schemes.light),
            "dark": scheme_to_json(&theme.schemes.dark),
            "light-medium-contrast": scheme_to_json(&schemes[0]),
            "light-high-contrast": scheme_to_json(&schemes[1]),
            "dark-medium-contrast": scheme_to_json(&schemes[2]),
            "dark-high-contrast": scheme_to_json(&schemes[3])
        },
        "palettes": {
            "primary": tonal_palette_to_json(&theme.palettes.primary),
            "secondary": tonal_palette_to_json(&theme.palettes.secondary),
            "tertiary": tonal_palette_to_json(&theme.palettes.tertiary),
            "error": tonal_palette_to_json(&theme.palettes.error),
            "neutral": tonal_palette_to_json(&theme.palettes.neutral),
            "neutral_variant": tonal_palette_to_json(&theme.palettes.neutral_variant)
        }
    })
}

fn main() {
    let args = Args::parse();

    // if no image is provided, clap will show the help message so unwrap is fine here
    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    let mut image = ImageReader::open(args.image).unwrap();
    let image = image.resize(128, 128, FilterType::Lanczos3);
    let primary_color = ImageReader::extract_color(image);

    let variant = match args.variant {
        Variant::Monochrome => material_colors::dynamic_color::Variant::Monochrome,
        Variant::Neutral => material_colors::dynamic_color::Variant::Neutral,
        Variant::TonalSpot => material_colors::dynamic_color::Variant::TonalSpot,
        Variant::Vibrant => material_colors::dynamic_color::Variant::Vibrant,
        Variant::Expressive => material_colors::dynamic_color::Variant::Expressive,
        Variant::Content => material_colors::dynamic_color::Variant::Content,
        Variant::Rainbow => material_colors::dynamic_color::Variant::Rainbow,
        Variant::FruitSalad => material_colors::dynamic_color::Variant::FruitSalad,
    };
     
    let dyn_scheme_light_med = DynamicScheme::by_variant(primary_color, &variant, false, Some(0.5));
    let dyn_scheme_light_high = DynamicScheme::by_variant(primary_color, &variant, false, Some(1.0));
    let dyn_scheme_dark_med = DynamicScheme::by_variant(primary_color, &variant, true, Some(0.5));
    let dyn_scheme_dark_high = DynamicScheme::by_variant(primary_color, &variant, true, Some(1.0));

    let scheme_light_med = Scheme::from(dyn_scheme_light_med);
    let scheme_light_high = Scheme::from(dyn_scheme_light_high);
    let scheme_dark_med = Scheme::from(dyn_scheme_dark_med);
    let scheme_dark_high = Scheme::from(dyn_scheme_dark_high);
    
    let theme = ThemeBuilder::with_source(primary_color.clone())
        .variant(variant)
        .color_match(args.exact)
        .build();
    
    let serialized_theme = serde_json::to_string_pretty(&theme_to_json(&theme, [&scheme_light_med, &scheme_light_high, &scheme_dark_med, &scheme_dark_high])).unwrap();
    std::fs::write(args.output, serialized_theme).expect("Unable to write file");
}
