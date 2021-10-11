use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    println!("Hello, world!");
    let (mut doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
            .. Default::default()
    }));
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let left_offset = 20.0;
    let top_offset = 20.0;

    let text = "Bohdan Tsap CV";
    let text2 = "Personal Info";

    let font = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
    let font2 = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

    // text, font size, x from left edge, y from bottom edge, font
    current_layer.use_text(text, 48.0, Mm(200.0), Mm(200.0), &font);

    // For more complex layout of text, you can use functions
    // defined on the PdfLayerReference
    // Make sure to wrap your commands
    // in a `begin_text_section()` and `end_text_section()` wrapper
    current_layer.begin_text_section();

        // setup the general fonts.
        // see the docs for these functions for details
        current_layer.set_font(&font2, 33.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(10.0));
        current_layer.set_line_height(33.0);
        current_layer.set_word_spacing(3000.0);
        current_layer.set_character_spacing(10.0);
        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

        // write two lines (one line break)
        current_layer.write_text(text.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text(text2.clone(), &font2);
        current_layer.add_line_break();

        // write one line, but write text2 in superscript
        current_layer.write_text(text.clone(), &font2);
        current_layer.set_line_offset(10.0);
        current_layer.write_text(text2.clone(), &font2);

    current_layer.end_text_section();
    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}
