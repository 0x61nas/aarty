use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use super::Arguments;
use super::OutputMethod;

pub fn prepare_output(arguments: &Arguments) -> io::Result<BufWriter<Box<dyn Write>>> {
    match arguments.output_method {
        OutputMethod::File => {
            let output_file = Box::new(File::create(&arguments.output)?);
            Ok(BufWriter::with_capacity(1024, output_file))
        }
        OutputMethod::Stdout => {
            let output_wrap = Box::new(std::io::stdout().lock());
            Ok(BufWriter::with_capacity(1024, output_wrap))
        }
    }
}
