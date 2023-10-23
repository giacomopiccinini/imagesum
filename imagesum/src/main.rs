use std::collections::HashSet;
use std::env;
use image::io::Reader as ImageReader;
use walkdir::WalkDir;
use std::path::Path;

fn main() {
    // Get the target directory from command line arguments
    let args: Vec<String> = env::args().collect();

    // If no target directory is specified, print usage and exit
    if args.len() < 2 {
        println!("Usage: {} <target_directory>", args[0]);
        return;
    }

    // Get the target
    let target = Path::new(&args[1]);

    // Init the image counter
    let mut counter = 0;

    // Init the set of unique dimensions
    let mut dimensions: HashSet<(u32, u32)> = HashSet::new();

    // Case where the target is a directory
    if target.is_dir() {

        // Iterate over the files in the directory
        for path in WalkDir::new(&target).into_iter().filter_map(|e| e.ok()){

            // Get the path
            let path = path.path();

            if path.is_dir() {
                continue;
            }

            // If the path has an extension
            if let Some(ext) = path.extension() {
                // If the extension is an image extension
                if ["jpg", "jpeg", "png", "JPG", "bmp", "gif", "tiff"].contains(&ext.to_str().unwrap()) {

                    // Increment the image counter
                    counter += 1;

                    // Try to read the dimensions of the image
                    match ImageReader::open(&path) {
                        Ok(reader) => {
                            if let Ok((width, height)) = reader.into_dimensions() {
                                dimensions.insert((height, width));
                            }
                        },
                        Err(e) => {
                            println!("Failed to read dimensions of {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    }
    else {
        if ["jpg", "jpeg", "png", "JPG", "bmp", "gif", "tiff"].contains(&target.extension().expect("Error with extension").to_str().unwrap()) {

            // Increment the image counter
            counter += 1;

            // Try to read the dimensions of the image
            match ImageReader::open(&target) {
                Ok(reader) => {
                    if let Ok((width, height)) = reader.into_dimensions() {
                        dimensions.insert((height, width));
                    }
                },
                Err(e) => {
                    println!("Failed to read dimensions of {:?}: {}", target, e);
                }
            }
        }
    }
    // Print the results
    println!("Total images: {}", counter);
    println!("Unique (height, width) pairs: {:?}", dimensions);
}
