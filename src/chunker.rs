/// Chunker is used to read in the contents of an input file
/// and stores them in a buffer that has enough space allocated
/// to store the contents.
/// The file contents are processed in chunks of a given size
/// and in parallel to increase performance.
/// When changes to the contents are needed, the changes are made
/// directly to the bytes in the buffer so we don't need to
/// create any additional allocations.
use anyhow::{anyhow, Error, Ok};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use std::fs::{canonicalize, File};
use std::io::{self, Read, Write};
use std::os::unix::prelude::MetadataExt;

pub const CHUNK_SIZE: usize = 8196;

#[derive(Debug)]
pub struct Chunker {
    contents: Vec<u8>,
}

impl Chunker {
    /// Create a new Chunker object by reading input or a filepath from STDIN
    pub fn new() -> Result<Self, Error> {
        let mut buffer = String::new();

        // Read the first line from STDIN.
        // If it is a filepath, we'll want to read the contents from the file.
        // Otherwise, we assume that we've been given raw JSON from the user,
        // either through manual input or piped from a file.
        // For now, this cannot distinguish between a file that doesn't exist and raw JSON.
        let _bytes = io::stdin().read_line(&mut buffer)?;
        let resolved_path = canonicalize(buffer.trim());
        if resolved_path.is_ok() {
            let filepath = resolved_path
                .unwrap()
                .to_string_lossy().to_string();
            Chunker::from_file(&filepath)
        } else {
            let stdin = io::stdin().lock();
            // Buffer to read the contents into
            let mut contents: Vec<u8> = Vec::new();
            // Since we've already read the first line, we'll combine it with the
            // rest of the input to get all the input contents
            if let Err(e) = buffer.into_bytes().chain(stdin).read_to_end(&mut contents) {
                return Err(anyhow!("Error reading STDIN contents to buffer: {:?}", e));
            }

            Ok(Chunker { contents })
        }
    }

    /// Create a new Chunker object. Given a filepath, read and load the contents
    /// into a buffer.
    pub fn from_file(filepath: &str) -> Result<Self, Error> {
        let mut file =
            File::open(filepath).map_err(|e| anyhow!("Failed to open file {}: {}", filepath, e))?;
        let filesize = file
            .metadata()
            .map_err(|e| anyhow!("Failed to get metadata from file {}: {}", filepath, e))?
            .size();

        let mut chunker = Self {
            contents: Vec::with_capacity(filesize as usize),
        };

        file.read_to_end(&mut chunker.contents)
            .map_err(|e| anyhow!("Failed to read file contents: {}", e))?;

        Ok(chunker)
    }

    /// Process the contents by chunks and in parallel.
    /// Replaces each ';' with a ':'
    pub fn process(&mut self) -> Result<(), Error> {
        self.contents
            .par_rchunks_exact_mut(CHUNK_SIZE)
            .for_each(|chunk| {
                chunk.par_iter_mut().for_each(|byte| {
                    if *byte == b';' {
                        *byte = b':';
                    }
                });
            });

        // rayon's chunking with an exact chunk size will not process the last chunk
        // if the last chunk's size is not exactly the specified chunk size.
        // Therefore, an additional call needs to be made in order to process any
        // remaining contents
        let chunk = self
            .contents
            .par_rchunks_exact_mut(CHUNK_SIZE)
            .take_remainder();
        chunk.par_iter_mut().for_each(|byte| {
            if *byte == b';' {
                *byte = b':';
            }
        });

        Ok(())
    }

    /// Converts the content bytes to a string
    pub fn output(&self) -> Result<&str, Error> {
        std::str::from_utf8(&self.contents)
            .map_err(|e| anyhow!("Failed to convert buffer bytes to string: {}", e))
    }

    pub fn write_to_file(&self, filepath: &str) -> Result<(), Error> {
        let mut file =
            File::create(filepath).map_err(|e| anyhow!("Failed to create output file: {}", e))?;

        file.write_all(&self.contents)
            .map_err(|e| anyhow!("Failed to write contents to file: {}", e))
    }
}
