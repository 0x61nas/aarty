<p align="center">
    <img alt="Showcase" src="./assests/showcase.png" width="250">
</p>

## **aarty**
mini freamwork to render images in the terminals/ttys.

[![crates.io](https://img.shields.io/crates/v/aarty.svg)](https://crates.io/crates/aarty)
[![docs.rs](https://docs.rs/aarty/badge.svg)](https://docs.rs/aarty)
[![downloads](https://img.shields.io/crates/d/aarty.svg)](https://crates.io/crates/aarty)
[![license](https://img.shields.io/crates/l/aarty.svg)](https://github.com/0x61nas/aarty/blob/aurora/LICENSE)

## Examples
```rust
let cfg = Config {
    sympols: vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into(),
    background: None,
    flags: 0,
};
let image = image::open("mylove.jpg").unwrap();
let (w, h) = image.dimensions();

let mut out = BufWriter::with_capacity(cfg.calc_buf_size(w, h), io::stdout().lock());

convert_image_to_ascii(&cfg, &image, &mut out).expect("IO error");
```
Enable the foreground colors
```rust
let cfg = Config {
    sympols: vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into(),
    background: None,
    flags: COLORS,
};
// ...
```
Reverse them with the background color
```rust
let cfg = Config {
    sympols: Sympols::empty(),
    background: Some((232, 209, 204).into()),
    flags: COLORS | REVERSE,
};
// ...
```
If you wanna build a rebresentesion in memory so you can modify it or use it multiple times, then you may found that implement [`FragmentWriter`]
for such a structher is useful.
```rust
struct TerminalFrame {
    fragments: Vec<(char, ANSIColor)>,
   cfg: Config,
}

impl FragmentWriter for TerminalFrame {
    fn background(&mut self, _: &ANSIColor) -> Result<bool, Box<dyn std::error::Error>> {
        // Nah, I don't care, I have my configs :p
        //  but pretent like if you care so it will skip the swap operation.
        Ok(true)
    }

    fn write_fragment(&mut self, info: FragmentInfo) -> Result<(), Box<dyn std::error::Error>> {
        self.fragments.push((info.sym, info.fg));
        Ok(())
    }

    fn write_colored_fragment(
        &mut self,
        info: FragmentInfo,
        _: Option<&ANSIColor>,
        _: Option<&ANSIColor>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_fragment(info)
    }

    fn write_bytes(&mut self, _bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Just ignore them
        Ok(())
    }
}

// So you can use it as a buffer
let cfg = Config {
    sympols: vec!['.', ',', '0', '1', '2', '3', '4', '5', '6', '8'].into(),
    background: None,
    flags: 0,
};
let image = image::open("mylove.jpg").unwrap();
let (w, h) = image.dimensions();
let mut frame = TerminalFrame {
    fragments: Vec::with_capacity(w * h),
    cfg: cfg.clone(),
};
aarty::convert_image_to_ascii(&cfg, &image, &mut frame).expect("Write error");

## Dependencies graph

![deps graph](./_deps.png)

> Generated with [cargo-depgraph](https://crates.io/crates/cargo-depgraph)
