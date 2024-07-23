use std::io::{self, BufRead, Write};

use crate::{context::Context, substitution::{EnvSubstitutor, FileSubstitutor, Substitutor, VariableSubstitutor}};

pub struct StreamProcessor {
    context: Context
}

impl StreamProcessor {
    pub fn new(context: Context) -> Self {
        Self {
            context
        }
    }

    pub fn process_stream<R: BufRead, W: Write>(&self, stream: &mut R, out_stream: &mut W) -> io::Result<()> {

        loop {
            let mut buffer: String = String::new();

            if let Ok(n_bytes) = stream.read_line(&mut buffer) {
                if n_bytes == 0 {
                    break;
                }

                buffer = VariableSubstitutor::substitute(&self.context, buffer);
                buffer = EnvSubstitutor::substitute(&self.context, buffer);
                buffer = FileSubstitutor::substitute(&self.context, buffer);
                out_stream.write_all(buffer.as_bytes());
            }
            else {
                println!("Finished parsing stdin");
                break;
            }
        }

        Ok(())
    }
}