use std::collections::HashMap;

use std::path::PathBuf;
use bio::io::fasta;

use clap::Parser;
use serde_json;



#[derive(Parser)]
struct Cli {
    /// Path to the input file
    #[arg(long, short)]
    input_file_path: PathBuf,

    /// Where to write the output FASTA file
    #[arg(long, short)]
    output_file_path: PathBuf,

    /// Where to write the mapping of new name to old name
    #[arg(long, short)]
    name_mapping_output_path: PathBuf,

    /// What to prepend to the new sequence names
    #[arg(long, short='p')]
    seq_name_prefix: String,

    /// Strip the gaps before collapsing
    #[arg(long, short)]
    strip_gaps: bool
}


fn main() {
    let cli = Cli::parse();
    simple_logger::SimpleLogger::new().env().init().unwrap();



    let reader = fasta::Reader::from_file(&cli.input_file_path).expect("IO Error.");
    let mut unique_sequences: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    log::info!("Reading input file {:?}", &cli.input_file_path);
    for result in reader.records(){

        let record = result.expect("Issue getting the record from the result.");

        let record_id = record.id();
        let mut record_seq = record.seq().to_vec();

        if cli.strip_gaps{
            record_seq.retain(|&val| val != 45); // 45 is the value of -
        }

        unique_sequences
            .entry(record_seq)
            .and_modify(| seq_name_vec| seq_name_vec.push(record_id.to_owned()))
            .or_insert(vec![record_id.to_owned()]);


    }

    let mut counter = 0;
    let mut writer = fasta::Writer::to_file(&cli.output_file_path).expect("Could not open output file");


    let mut name_mapping: HashMap<String, &Vec<String>> = HashMap::with_capacity(unique_sequences.len());

    log::info!("Writing unique sequences to file {:?}", &cli.output_file_path);
    for (sequence, sequence_names) in &unique_sequences{
        // This will generate a sequence with a unique int for each collapsed seq, and a count
        // for the sequences that make up this collapsed one
        let seq_name = format!("{}_{:0>4}_{:0>4}", &cli.seq_name_prefix, counter, sequence_names.len());

        writer.write(&seq_name, None, sequence).expect("Error writing record");
        counter += 1;
        name_mapping.insert(seq_name.clone(), sequence_names);

    }

    log::info!("Writing name mapping to {:?}", &cli.name_mapping_output_path);
    std::fs::write(
        cli.name_mapping_output_path,
        serde_json::to_string(&name_mapping).expect("Error serializing the name map.")
    ).expect("Error with writing the name map to the disk.");

    log::info!("Done!");
}
