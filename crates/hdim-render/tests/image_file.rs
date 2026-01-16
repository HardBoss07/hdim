use hdim_render::Renderer;
use std::path::PathBuf;

#[test]
fn test_render_real_image_snapshot_size_2() {
    // 1. Setup Path
    // Adjust the path to where you store your test images
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/WindowsXP.png");

    // 2. Load the image
    // Note: This will fail the test if the image isn't found
    let img = image::open(&path)
        .expect(&format!("Could not find test image at {:?}", path));

    // 3. Initialize Renderer
    // Using area_size 4 to downsample a large image for terminal display
    let renderer = Renderer::new(2);

    // 4. Run System Under Test
    let output = renderer.render(&img).expect("Rendering failed");
    println!("{}", output);
    
    // 5. Verify with Snapshot
    // Note: If the image is large, the snapshot file will be quite big.
    insta::assert_snapshot!(output);
}

#[test]
fn test_render_real_image_snapshot_size_4() {
    // 1. Setup Path
    // Adjust the path to where you store your test images
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/WindowsXP.png");

    // 2. Load the image
    // Note: This will fail the test if the image isn't found
    let img = image::open(&path)
        .expect(&format!("Could not find test image at {:?}", path));

    // 3. Initialize Renderer
    // Using area_size 4 to downsample a large image for terminal display
    let renderer = Renderer::new(4);

    // 4. Run System Under Test
    let output = renderer.render(&img).expect("Rendering failed");
    println!("{}", output);
    
    // 5. Verify with Snapshot
    // Note: If the image is large, the snapshot file will be quite big.
    insta::assert_snapshot!(output);
}

#[test]
fn test_render_real_image_snapshot_size_8() {
    // 1. Setup Path
    // Adjust the path to where you store your test images
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/WindowsXP.png");

    // 2. Load the image
    // Note: This will fail the test if the image isn't found
    let img = image::open(&path)
        .expect(&format!("Could not find test image at {:?}", path));

    // 3. Initialize Renderer
    // Using area_size 4 to downsample a large image for terminal display
    let renderer = Renderer::new(8);

    // 4. Run System Under Test
    let output = renderer.render(&img).expect("Rendering failed");
    println!("{}", output);
    
    // 5. Verify with Snapshot
    // Note: If the image is large, the snapshot file will be quite big.
    insta::assert_snapshot!(output);
}