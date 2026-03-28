use hdim_render::{View, render};
use image::GenericImageView;
use std::path::PathBuf;

fn run_snapshot_test_for_area_size(area_size: u32) {
    // 1. Setup Path
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/WindowsXP.png");

    // 2. Load the image
    let image =
        image::open(&path).unwrap_or_else(|_| panic!("Could not find test image at {:?}", path));

    // 3. Define the View to replicate old `area_size` behavior
    let (image_width, image_height) = image.dimensions();
    let view = View {
        source_x: 0,
        source_y: 0,
        source_width: image_width,
        source_height: image_height,
        target_width: (image_width as f32 / area_size as f32).ceil() as u32,
        target_height: (image_height as f32 / (area_size as f32 * 2.0)).ceil() as u32,
    };

    // 4. Run System Under Test
    let output = render(&image, &view).expect("Rendering failed");
    println!("{}", output);

    // 5. Verify with Snapshot
    let snapshot_name = format!("render_real_image_snapshot_size_{}", area_size);
    insta::assert_snapshot!(snapshot_name, output);
}

#[test]
fn test_render_real_image_snapshot_size_2() {
    run_snapshot_test_for_area_size(2);
}

#[test]
fn test_render_real_image_snapshot_size_4() {
    run_snapshot_test_for_area_size(4);
}

#[test]
fn test_render_real_image_snapshot_size_8() {
    run_snapshot_test_for_area_size(8);
}
