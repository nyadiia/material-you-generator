use material_colors::{
    dynamic_color::DynamicScheme,
    image::{FilterType, ImageReader},
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

    /// Light or dark
    #[arg(short, long, value_enum)]
    polarity: Option<Polarity>,

    /// Contrast level
    #[arg(short, long, value_enum)]
    contrast: Option<Vec<Contrast>>,

    /// Color variant
    /// Set of themes supported by Dynamic Color.
    #[arg(short, long, value_enum, default_value = "neutral")]
    variant: Variant, // the crate doesn't use #[derive(ValueEnum)] so it's not possible to use it here

    /// Output file
    #[arg(short, long, default_value = "theme.json")]
    output: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Polarity {
    Light,
    Dark,
    All
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Contrast {
    Standard,
    Medium,
    High,
    All
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Variant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad
}

fn check_file(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("File not found: {}", path))
    }
}

fn scheme_to_json_value(scheme: &DynamicScheme) -> Value {
    json!({
        "primary": scheme.primary().to_hex(),
        "on_primary": scheme.on_primary().to_hex(),
        "primary_container": scheme.primary_container().to_hex(),
        "on_primary_container": scheme.on_primary_container().to_hex(),
        "secondary": scheme.secondary().to_hex(),
        "on_secondary": scheme.on_secondary().to_hex(),
        "secondary_container": scheme.secondary_container().to_hex(),
        "on_secondary_container": scheme.on_secondary_container().to_hex(),
        "tertiary": scheme.tertiary().to_hex(),
        "on_tertiary": scheme.on_tertiary().to_hex(),
        "tertiary_container": scheme.tertiary_container().to_hex(),
        "on_tertiary_container": scheme.on_tertiary_container().to_hex(),
        "error": scheme.error().to_hex(),
        "on_error": scheme.on_error().to_hex(),
        "error_container": scheme.error_container().to_hex(),
        "on_error_container": scheme.on_error_container().to_hex(),
        "background": scheme.background().to_hex(),
        "on_background": scheme.on_background().to_hex(),
        "surface": scheme.surface().to_hex(),
        "on_surface": scheme.on_surface().to_hex(),
        "surface_variant": scheme.surface_variant().to_hex(),
        "on_surface_variant": scheme.on_surface_variant().to_hex(),
        "outline": scheme.outline().to_hex(),
        "outline_variant": scheme.outline_variant().to_hex(),
        "shadow": scheme.shadow().to_hex(),
        "scrim": scheme.scrim().to_hex(),
        "inverse_surface": scheme.inverse_surface().to_hex(),
        "inverse_on_surface": scheme.inverse_on_surface().to_hex(),
        "inverse_primary": scheme.inverse_primary().to_hex()
    })
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    panic!("Not implemented yet");
    // if no image is provided, clap will show the help message so unwrap is fine here
    let mut image = ImageReader::open(args.image).unwrap();

    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    // However, if you don't like the results, you can always try other FilterType values.
    image.resize(128, 128, FilterType::Lanczos3);
    let primary_color = ImageReader::extract_color(&image);
    let variant_real = match args.variant {
        Variant::Monochrome => material_colors::dynamic_color::Variant::Monochrome,
        Variant::Neutral => material_colors::dynamic_color::Variant::Neutral,
        Variant::TonalSpot => material_colors::dynamic_color::Variant::TonalSpot,
        Variant::Vibrant => material_colors::dynamic_color::Variant::Vibrant,
        Variant::Expressive => material_colors::dynamic_color::Variant::Expressive,
        Variant::Fidelity => material_colors::dynamic_color::Variant::Fidelity,
        Variant::Content => material_colors::dynamic_color::Variant::Content,
        Variant::Rainbow => material_colors::dynamic_color::Variant::Rainbow,
        Variant::FruitSalad => material_colors::dynamic_color::Variant::FruitSalad,
    };
    
    // let contrast_level = match args.contrast.into_iter() {
    //     Contrast::Standard => 0.0,
    //     Contrast::Medium => 0.5,
    //     Contrast::High => 1.0,
    //     Contrast::All => todo!(),
    // };

    // let scheme = DynamicScheme::by_variant(
    //     primary_color,
    //     &variant_real,
    //     is_dark,
    //     Some(contrast_level),
    // );

    // let serailized_scheme = serde_json::to_string_pretty(&scheme_to_json_value(&scheme)).unwrap();
// 
    std::fs::write(args.output, serailized_scheme).expect("Unable to write file");
}
