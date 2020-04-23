use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Explore DICOM
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Parses a single file and prints the DICOM elements to stdout.
    Dump {
        #[structopt(short, long)]
        /// Process the dataset as a stream.
        ///
        /// If specified, instead of reading the entire dataset into memory will print elements as
        /// they are parsed.
        stream: bool,

        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Opens a DICOM dataset in a TUI for browsing and editing.
    Edit {
        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Scans a folder recursively for DICOM datasets and prints results of found DICOM.
    Scan {
        /// The folder to recursively scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Index a directory of unstructured DICOM files.
    ///
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Index {
        #[structopt(short, long)]
        /// The mongo URI to insert dicom records into.
        mongo: String,

        /// The folder to scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Archives DICOM datasets from a source folder into a destination folder.
    ///
    /// The source folder is assumed to be unstructured whereas the DICOM datasets will be copied
    /// into the destination folder in a consistent structure:
    ///   - One series per folder
    ///   - Each DICOM file will be named in the format `[SOP_UID].dcm`
    Archive {
        /// The source folder of DICOM datasets to process.
        source: PathBuf,

        /// The destination folder to archive datasets into.
        destination: PathBuf,
    },
}