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
    Print {
        #[structopt(short, long)]
        /// Process the dataset as a stream instead of loading fully into memory.
        ///
        /// If printing a DICOM dataset which fails to fully parse and this argument is not
        /// specified then no elements will be printed to stdout, only an error. Specifying this
        /// argument will result in all elements which succeed in parsing to be printed to stdout
        /// until the error is encountered.
        stream: bool,

        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Opens a DICOM dataset in a TUI for browsing and editing.
    Edit {
        /// The file to process as a DICOM dataset.
        file: PathBuf,
    },
    /// Recursively parses a folder of DICOM datasets and prints results of parsing.
    ///
    /// This is primarily useful for locating DICOM files which fail to parse.
    Parse {
        /// The folder to recursively scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Manage a database index of DICOM on disk.
    ///
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Index {
        #[structopt(short, long)]
        /// The db URI of the index.
        db: String,

        #[structopt(subcommand)]
        /// Index sub-command
        cmd: IndexCommand,
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

#[derive(StructOpt, Debug)]
pub enum IndexCommand {
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Scan {
        /// The folder to scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Verify records in the database reference valid files on-disk.
    Verify {},
}
