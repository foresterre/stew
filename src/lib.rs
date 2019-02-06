use std::path::Path;

use clap::{App, Arg, ArgMatches};
use image;

use crate::config::{
    Config, FormatEncodingSettings, JPEGEncodingSettings, PNMEncodingSettings, SelectedLicenses,
};
use crate::processor::conversion::ConversionProcessor;
use crate::processor::encoding_format::EncodingFormatDecider;
use crate::processor::image_operations::ImageOperationsProcessor;
use crate::processor::license_display::LicenseDisplayProcessor;
use crate::processor::{ProcessMutWithConfig, ProcessWithConfig};

mod config;
mod operations;
mod processor;

pub fn get_app_skeleton(name: &str) -> App<'static, 'static> {
    App::new(name)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Martijn Gribnau <garm@ilumeo.com>")
        .about("Stew is a set of image transformation tools, adapted from `sic`. This is stew-{}")
        .arg(Arg::with_name("forced_output_format")
            .short("f")
            .long("output-format")
            .value_name("FORMAT")
            .help("Force the output image format to use FORMAT, regardless of the (if any) extension of the given output file path. \
                Output formats (FORMAT values) supported: BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM and PAM. ")
            .takes_value(true))
        .arg(Arg::with_name("license")
            .long("license")
            .help("Displays the license of this piece of software (`sic`).")
            .takes_value(false))
        .arg(Arg::with_name("dep_licenses")
            .long("dep-licenses")
            .help("Displays the licenses of the dependencies on which this software relies.")
            .takes_value(false))
        .arg(Arg::with_name("jpeg_encoding_quality")
            .long("jpeg-encoding-quality")
            .help("Set the jpeg quality to QUALITY. Valid values are natural numbers from 1 up to and including 100. Will only be used when the output format is determined to be jpeg.")
            .value_name("QUALITY")
            .takes_value(true))
        .arg(Arg::with_name("pnm_encoding_ascii")
            .long("pnm-encoding-ascii")
            .help("Use ascii based encoding when using a PNM image output format (pbm, pgm or ppm). Doesn't apply to 'pam' (PNM Arbitrary Map)."))
        .arg(Arg::with_name("disable_automatic_color_type_adjustment")
            .long("disable-automatic-color-type-adjustment")
            .help("Some image output formats do not support the color type of the image buffer prior to encoding. By default sic tries to adjust the color type. If this flag is provided, sic will not try to adjust the color type."))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless_one(&["license", "dep_licenses", "user_manual"])
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the desired output file")
            .value_name("OUTPUT_FILE")
            .required_unless_one(&["license", "dep_licenses", "user_manual"])
            .index(2))
}

// Here any option should not panic when invalid.
// Previously, it was allowed to panic within Config, but this is no longer the case.
pub fn get_default_config(matches: &ArgMatches) -> Result<Config, String> {
    let res = Config {
        licenses: match (
            matches.is_present("license"),
            matches.is_present("dep_licenses"),
        ) {
            (true, true) => vec![
                SelectedLicenses::ThisSoftware,
                SelectedLicenses::Dependencies,
            ],
            (true, _) => vec![SelectedLicenses::ThisSoftware],
            (_, true) => vec![SelectedLicenses::Dependencies],
            _ => vec![],
        },

        forced_output_format: matches.value_of("forced_output_format").map(String::from),

        disable_automatic_color_type_adjustment: matches
            .is_present("disable_automatic_color_type_adjustment"),

        encoding_settings: FormatEncodingSettings {
            // 3 possibilities:
            //   - present + i (1 ... 100)
            //   - present + i !(1 ... 100)
            //   - not present (take default)
            jpeg_settings: JPEGEncodingSettings::new_result((
                matches.is_present("jpeg_encoding_quality"),
                matches.value_of("jpeg_encoding_quality"),
            ))?,
            pnm_settings: PNMEncodingSettings::new(matches.is_present("pnm_encoding_ascii")),
        },

        output: matches.value_of("output_file").map(|v| v.into()),
    };

    Ok(res)
}

/// The run function runs the sic application, taking the matches found by Clap.
/// This function is separated from the main() function so that it can be used more easily in test cases.
/// This function consumes the matches provided.
pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let options = get_default_config(&matches)?;

    let license_display_processor = LicenseDisplayProcessor::new();
    license_display_processor.process(&options);

    let input = matches
        .value_of("input_file")
        .ok_or_else(|| String::from("An INPUT was expected, but none was given."))
        .map(|input_str| Path::new(input_str));

    // open image, -> DynamicImage
    let mut buffer = input.and_then(|path| image::open(path).map_err(|err| err.to_string()))?;

    // perform image operations
    let mut image_operations_processor = ImageOperationsProcessor::new(&mut buffer);
    image_operations_processor.process_mut(&options)?;

    let output_format_processor = EncodingFormatDecider::new();
    let output_format = output_format_processor.process(&options);

    let conversion_processor = ConversionProcessor::new(&buffer, output_format?);
    conversion_processor.process(&options)
}
