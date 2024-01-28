use dialoguer::{console::Term, theme::ColorfulTheme, Select};

pub(crate) fn select_image() -> Option<usize> {
    // List of available images
    let image_options = [
        "image1.jpg",
        "image1-1.jpg",
        "image2.jpg",
        "image2-2.jpg",
        "image3.jpg",
        "image4.jpg",
        "image5.jpg",
    ];

    // Create a select prompt
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an image:")
        .items(&image_options)
        .default(0) // Set the default selected item index
        .interact_on_opt(&Term::stderr())
        .unwrap_or_else(|e| {
            eprintln!("Error reading user input.");
            eprintln!("{}", e);
            std::process::exit(1);
        })
}

pub(crate) fn idx_to_image_path(index: usize) -> (&'static str, &'static str) {
    let image_paths = [
        ("../examples/picture1.jpg", "../examples/slika 1/"),
        ("../examples/picture1.jpg", "../examples/slika 1 - 1/"),
        ("../examples/picture2.jpg", "../examples/slika 2/"),
        ("../examples/picture2.jpg", "../examples/slika 2 - 1/"),
        ("../examples/picture3.jpg", "../examples/slika 3/"),
        ("../examples/picture4.jpg", "../examples/slika 4/"),
        ("../examples/picture5.jpg", "../examples/slika 5/"),
    ];

    (image_paths[index].0, image_paths[index].1)
}
