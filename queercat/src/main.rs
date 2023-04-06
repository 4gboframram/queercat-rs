#![deny(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

#[allow(clippy::wildcard_imports)] // the functions are automatically generated and do not affect readability at all
use queercat_lib::{color::Color, flag::*, Ansi, Bits24, QueerCat, QueerCatFrequency};

use clap::{Args, Parser, ValueEnum};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::PathBuf;

/// Concatenate FILE(s), or standard input, to standard output.
/// With no FILE, or when FILE is -, read standard input.
//
// Report all queercat bugs to <https://github.com/4gboframram/queercat-rs/issues>
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    flag: FlagArg,

    #[arg(required = false)]
    /// The files to read
    files: Vec<PathBuf>,

    /// Whether to use 24 bit RGB color. This may be slower and incompatible with some terminals, but it looks amazing
    #[arg(required = false, short = 'b', long = "24bit")]
    bits24: bool,

    /// Offset of the start of the flag
    #[arg(short, long, default_value_t = 0.0)]
    offset: f32,

    #[command(flatten)]
    frequency: Frequency,
}

#[derive(Args, Clone, PartialEq, PartialOrd)]
#[group(required = false)]
pub struct CustomFlag {
    /// Stripes for the flag entered as hexadecimal numbers
    #[arg(short, long)]
    #[arg(value_parser = clap_num::maybe_hex::<u32>)]
    stripes: Vec<u32>,

    /// Ansi codes for the flag written as decimal numbers
    #[arg(short, long, conflicts_with = "stripes")]
    ansi_codes: Vec<u8>,

    #[arg(long)]
    #[arg(default_value_t = 4.0)]
    factor: f32,
}

#[derive(Args, Clone, PartialEq, PartialOrd)]
#[group(required = false, multiple = false)]
pub struct FlagArg {
    /// The builtin flag to use if a custom flag pattern is not specified
    #[arg(value_enum, default_value_t = FlagChoice::Rainbow, conflicts_with="stripes", conflicts_with="ansi_codes")]
    #[arg(short, long)]
    flag: FlagChoice,

    #[command(flatten)]
    custom: Option<CustomFlag>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, ValueEnum)]
pub enum FlagChoice {
    Rainbow,
    #[value(alias("trans"), alias("tra"))]
    Transgender,
    #[value(alias("enby"), alias("nb"))]
    NonBinary,
    #[value(alias("lesbiab"), alias("lesb"), alias("debian"))]
    Lesbian,
    Gay,
    #[value(alias("pan"))]
    Pansexual,
    #[value(alias("bi"))]
    Bisexual,

    GenderFluid,
    #[value(alias("ace"), alias("garlic-bread"), alias("invaded-denmark"))]
    Asexual,
    Unlabeled,
    #[value(alias("aro"))]
    Aromantic,
    Aroace,
}

#[derive(Args, Clone, PartialEq, PartialOrd)]
#[group(required = false)]
struct Frequency {
    /// Horizontal rainbow frequency
    #[arg(default_value_t = 0.1)]
    #[arg(short = 'z', long)]
    horizontal_frequency: f32,

    /// Vertical rainbow frequency
    #[arg(default_value_t = 0.23)]
    #[arg(short, long)]
    vertical_frequency: f32,
}

fn get_file(path: &PathBuf) -> Result<Box<dyn Read>> {
    if path == std::path::Path::new("-") {
        Ok(Box::new(std::io::stdin().lock()))
    } else {
        Ok(Box::new(BufReader::new(File::open(path)?)))
    }
}

fn make_24bit<W: Write>(
    writer: W,
    flag: Flag<'_>,
    freq: QueerCatFrequency,
    offset: f32
) -> QueerCat<'_, W, Bits24> {
    let c = Bits24::new(freq).with_offset(offset);
    QueerCat::new(c, writer, flag)
}

fn make_ansi<W: Write>(
    writer: W,
    flag: Flag<'_>,
    freq: QueerCatFrequency,
    offset: f32
) -> QueerCat<'_, W, Ansi> {
    #[allow(clippy::cast_possible_truncation)]
    let c = Ansi::new(flag.ansi_colors.len() as u32, freq).with_offset(offset);
    QueerCat::new(c, writer, flag)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut stripe_colors = Vec::new();
    let mut ansi_colors = Vec::new();
    let (bits24, flag) = if let Some(custom) = cli.flag.custom {
        let ansi_colors_arr = custom.ansi_codes;
        let stripes = custom.stripes;
        stripe_colors.extend(stripes.iter().map(|x| Color::from_hex(*x)));
        ansi_colors.extend(ansi_colors_arr);
        let method = ColorMethod::Stripes;
        (
            ansi_colors.is_empty(),
            Flag {
                name: "",
                ansi_colors: &ansi_colors,
                stripe_colors: &stripe_colors,
                color_method: method,
                factor: queercat_lib::Extended::from_num(custom.factor),
            },
        )
    } else {
        use FlagChoice::*;
        (
            cli.bits24,
            match cli.flag.flag {
                Rainbow => rainbow(),
                Transgender => transgender(),
                NonBinary => nonbinary(),
                Lesbian => lesbian(),
                Gay => gay(),
                Pansexual => pansexual(),
                Bisexual => bisexual(),
                GenderFluid => gender_fluid(),
                Asexual => asexual(),
                Unlabeled => unlabeled(),
                Aromantic => aromantic(),
                Aroace => aroace(),
            },
        )
    };

    // If stdin is piped, then we probably want to observe it in real time, so we don't buffer it
    let writer: Box<dyn Write> = if atty::is(atty::Stream::Stdin) {
        Box::new(BufWriter::new(std::io::stdout().lock()))
    } else {
        Box::new(std::io::stdout().lock())
    };

    let freq = cli.frequency;
    let freq = QueerCatFrequency::Custom(freq.vertical_frequency, freq.horizontal_frequency);
    let offset = cli.offset;

    if cli.files.is_empty() {
        let stdin = std::io::stdin().lock();
        if bits24 {
            make_24bit(writer, flag, freq, offset).cat(stdin)
        } else {
            make_ansi(writer, flag, freq, offset).cat(stdin)
        }
    } else {
        
        use multi_reader::MultiReader;
        let mut readers = Vec::with_capacity(cli.files.len());
        // we can't use ? in iter.map()
        for file in cli.files {
            let file = get_file(&file)?;
            readers.push(file);
        }
        let reader = MultiReader::new(readers.drain(..));
        if bits24 {
            make_24bit(writer, flag, freq, offset).cat(reader)
        } else {
            make_ansi(writer, flag, freq, offset).cat(reader)
        }
    }
}
