extern crate getopts;
extern crate image;

use getopts::Options;
use image::DynamicImage::ImageRgba8;

use std::default::Default;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Read};

pub mod css;
mod css_test;
pub mod cssom;
mod cssom_test;
pub mod dom;
mod dom_test;
pub mod html;
mod html_test;
pub mod layout;
pub mod painting;
pub mod pdf;
pub mod style;
pub mod style_test;

fn main() {
    // Parse command-line options:
    let args: Vec<String> = env::args().skip(1).collect();
    let opts = parse_args();
    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error parsing command line options: {}", e);
            std::process::exit(1);
        }
    };

    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    // Choose a format:
    let png = match &str_arg("f", "png")[..] {
        "png" => true,
        "pdf" => false,
        x => {
            eprintln!("Unknown output format: {}", x);
            std::process::exit(1);
        }
    };

    // Read input files:
    let html = read_source(&str_arg("h", "examples/test.html")).unwrap_or_else(|err| {
        eprintln!("Error reading HTML file: {}", err);
        String::new()
    });
    let css = read_source(&str_arg("c", "examples/test.css")).unwrap_or_else(|err| {
        eprintln!("Error reading CSS file: {}", err);
        String::new()
    });

    // Since we don't have an actual window, hard-code the "viewport" size.
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    // Parsing and rendering:
    /* html parsing  */
    let root_node = html::parse(html);
    dom::pretty_print(&root_node, 2);
    /* css parsing  */
    let stylesheet = css::parse(css);
    /* styled tree */
    let style_root = style::style_tree(&root_node, &stylesheet);
    /* layout tree */
    let layout_root = layout::layout_tree(&style_root, viewport);

    // Create the output file:
    let filename = str_arg("o", if png { "output.png" } else { "output.pdf" });
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    // Write to the file:
    let ok = if png {
        let canvas = painting::paint(&layout_root, viewport.content);
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
            let color = canvas.pixels[(y * w + x) as usize];
            image::Rgba([color.r, color.g, color.b, color.a])
        });
        ImageRgba8(img).save(filename.clone()).is_ok()
    } else {
        pdf::render(&layout_root, viewport.content, &mut file).is_ok()
    };

    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}

fn parse_args() -> Options {
    let mut opts = Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");
    opts
}

fn read_source(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
