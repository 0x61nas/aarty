use aarty::{Config, TextImage, COLORS, REVERSE};

#[test]
fn test_snapshot_basic() {
    let config = Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into());
    let ti = TextImage::new(
        vec![
            vec![0, 128, 255],
            vec![128, 255, 0],
        ],
        2,
        3,
    );
    let result = ti.to_text(&config);
    insta::assert_snapshot!(result);
}

#[test]
fn test_snapshot_colors() {
    let config = Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into())
        .with_flags(COLORS);
    let ti = TextImage::new(vec![vec![255, 0, 0]], 1, 1);
    let result = ti.to_text(&config);
    insta::assert_snapshot!(result);
}

#[test]
fn test_snapshot_reverse_colors() {
    let config = Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into())
        .with_flags(COLORS | REVERSE);
    let ti = TextImage::new(vec![vec![255, 0, 0]], 1, 1);
    let result = ti.to_text(&config);
    insta::assert_snapshot!(result);
}
