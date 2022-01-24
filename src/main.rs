use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    println!("Hello, world!");
    let (mut doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(320.0), Mm(590.0), "Layer 1");
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
            .. Default::default()
    }));
    let current_layer = doc.get_page(page1).get_layer(layer1);

    //Data defined here
    let name = "Bohdan Tsap";
    let position = "Developer";


    let skills_label = "SKILLS";
    let skills_data1 = " • Team work";
    let skills_data2 = " • Collaboration";
    let skills_data3 = " • Proactiveness";


    // Fonts
    let font = doc.add_external_font(File::open("src/assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
    let font2 = doc.add_external_font(File::open("src/assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

    // text, font size, x from left edge, y from bottom edge, font
    //current_layer.use_text(text, 48.0, Mm(200.0), Mm(200.0), &font);

    // For more complex layout of text, you can use functions
    // defined on the PdfLayerReference
    // Make sure to wrap your commands
    // in a `begin_text_section()` and `end_text_section()` wrapper
    current_layer.begin_text_section();

        // setup the general fonts.
        // see the docs for these functions for details
        current_layer.set_font(&font2, 33.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(550.0));
        current_layer.set_line_height(33.0);
        current_layer.set_word_spacing(3000.0);
        current_layer.set_character_spacing(10.0);
        //current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

        // write two lines (one line break)

        current_layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
        current_layer.write_text(name.clone(), &font2);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();
        current_layer.write_text(position.clone(), &font2);
        current_layer.add_line_break();

        current_layer.write_text("cv.bohdaq.name", &font2);
        current_layer.add_line_break();


        current_layer.add_line_break();

        


        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);

        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        let personal_info_label = "INFO";
        current_layer.write_text(personal_info_label.clone(), &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();

        let personal_info_data = "Highly competent sofware engineer capable of designing";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();

        let personal_info_data = "solutions from 'ground up'. Deep understanding of";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();


        let personal_info_data = "underlying technical stack alongside with knowledge";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();

        let personal_info_data = "of the basic computer science. More than 8 years of";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();

        let personal_info_data = "software engineering experience including frontend";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();

        let personal_info_data = "and backend.";
        current_layer.write_text(personal_info_data.clone(), &font);
        current_layer.add_line_break();
        current_layer.add_line_break();

        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        
        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        current_layer.write_text(skills_label, &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();
        current_layer.write_text(skills_data1, &font);
        current_layer.add_line_break();
        current_layer.write_text(skills_data2, &font);
        current_layer.add_line_break();
        current_layer.write_text(skills_data3, &font);
        current_layer.add_line_break();


        current_layer.add_line_break();


        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        let contact_label = "CONTACT";
        current_layer.write_text(contact_label, &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);

        let contact_cell = "Cell: 38 063 03 86 173";
        let contact_email = "Email: bohdan.tsap@tutanota.com";
        let contact_location = "Lviv, Ukraine";

        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.add_line_break();
        current_layer.write_text(contact_cell, &font);
        current_layer.add_line_break();
        current_layer.write_text(contact_email, &font);
        current_layer.add_line_break();
        current_layer.write_text(contact_location, &font);
        current_layer.add_line_break();

        current_layer.add_line_break();

        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        let employment_history_label = "EMPLOYMENT HISTORY";
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_history_label, &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();


        let employment_item_label = String::from("CorporateServices.com (2016-2021)");
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_item_label, &font);
        current_layer.add_line_break();

        let employment_item_description = "Worked as a frontend lead in a small team on a product.";
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = "Product purpose is to provide ability of a company";
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = "incorporation in Singapore.";
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "Core technologies: Polymer, lit-element, Firebase,";
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "Google Cloud, Spring, Git, Jenkins";
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();


        current_layer.add_line_break();
        let employment_item_label = String::from("ThinkTank.net (2014-2016)");
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_item_label, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("Worked as a backend enginner in a dedicated team on a");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();


        let employment_item_description = "product. Product purpose is to provide ability of a ";
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = "collaboration and brainstorming for teams.";
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();


        let employment_item_core_technologies = "Core technologies: GWT, Spring, PostgreSQL,";
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();


        let employment_item_core_technologies = "HTTP Long Pooling, Objective C, Git";
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();


        current_layer.add_line_break();
        let employment_item_label = String::from("Pics.io 2014");
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_item_label, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("Worked as a frontend enginner on a startup.
Startup");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("purpose is to provide in-browser photo editing");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("capabilities like in Lightroom.");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "Core technologies: JavaScript, Git";
        current_layer.write_text(employment_item_core_technologies, &font);

        current_layer.add_line_break();




        current_layer.add_line_break();
        let employment_item_label = String::from("WDS.co (2012-2014)");
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_item_label, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("Worked as a Java developer. Took part in development");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("projects related to mobile device managment and");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_description = String::from("provisioning.");
        current_layer.write_text(employment_item_description, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "Core technologies: Java, Play Framework,"; 
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "OMA Device Management, Ruby on Rails,"; 
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();

        let employment_item_core_technologies = "MySQL, Git"; 
        current_layer.write_text(employment_item_core_technologies, &font);
        current_layer.add_line_break();


        current_layer.add_line_break();


        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        let employment_history_label = "TECHNICAL SKILLS";
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(employment_history_label, &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();


        current_layer.write_text("A vision of how things are working on, from a physical", &font);
        current_layer.add_line_break();  


        current_layer.write_text("layer up to application layer. Significant expertise in", &font);
        current_layer.add_line_break();

        current_layer.write_text("building, deploying and maintaining software. Hands on", &font);
        current_layer.add_line_break();

        current_layer.write_text("experience with setting up environments, releases and", &font);
        current_layer.add_line_break();

        current_layer.write_text("production support.", &font);
        current_layer.add_line_break();

        
        current_layer.add_line_break();


        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        let educational_history_label = "EDUCATIONAL HISTORY";
        current_layer.set_font(&font2, 22.0);
        current_layer.set_line_height(22.0);
        current_layer.set_word_spacing(2000.0);
        current_layer.set_character_spacing(5.0);
        current_layer.write_text(educational_history_label, &font);
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        current_layer.add_line_break();


        let educational_history_item = "Lviv Polytechnic National University";
        current_layer.write_text(educational_history_item, &font);
        current_layer.add_line_break();

        let educational_history_item = "Bachelor of Computer Science (2010-2014)";

        current_layer.write_text(educational_history_item, &font);
        current_layer.add_line_break();


        current_layer.add_line_break();


        let educational_history_item = "Lviv Polytechnic National University";
        current_layer.write_text(educational_history_item, &font);
        current_layer.add_line_break();

        let educational_history_item = "Master of Computer Science (2014-2015)";

        current_layer.write_text(educational_history_item, &font);
        current_layer.add_line_break();

        current_layer.add_line_break();
        current_layer.add_line_break();

        let educational_history_label = "Source code of this CV is available at:";
        current_layer.set_font(&font2, 14.0);
        current_layer.set_line_height(14.0);
        current_layer.set_word_spacing(1000.0);
        current_layer.set_character_spacing(3.0);
        current_layer.write_text(educational_history_label, &font);
        current_layer.add_line_break();
        current_layer.write_text("https://github.com/bohdaq/rust-cv-pdf/", &font);
        current_layer.add_line_break();



        // write one line, but write text2 in superscript
        //current_layer.write_text(text.clone(), &font2);
        //current_layer.set_line_offset(10.0);
        //current_layer.write_text(text2.clone(), &font2);

    current_layer.end_text_section();
    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}
