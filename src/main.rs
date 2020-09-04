extern crate clap;
use clap::{App, load_yaml};

mod zip;
mod archiver;

fn main()
 {
    /////////////////////// User Input /////////////////////////////////////////////////////////
    
    // zipper -i <file(s)> -c <algo> -s <size> -e <algo> <key>

    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    let docompress:   	bool = true;
    let  doencrypt:    	bool = false;
    let    dosplit:     bool = false;

    // Determine compression algorithm
    if let Some(compress) = m.value_of("compress") {
        match compress {
            "store" => {
                let docompress = false;
				println!("Creating without compressing")
				},
            "zstd" => {
				println!("Compressing using ZSTD")
			},
			"deflate" => {
				println!("Compressing using DEFLATE64")
			}
            _ => {
				println!("Compressing using ZSTD")
			},
        }
    }

    // File input
    // values_of() instead of value_of() for mltiple inputs
    let files: Vec<_> = m.values_of("input").unwrap().collect();
    println!("{}", files[0]);
    println!("{}", files[1]);
    
    // Encrypting
	if let Some(encrypt) = m.value_of("encrypt") {
        let doencrypt = true;
    }
    // Splitting
    if let Some(split) = m.value_of("split") {
        let dosplit = true;
    }
    
    let Some(output) = m.value_of("output");


    /////////////////////// User Input /////////////////////////////////////////////////////////
    let ziparchive = zip::zipfile
    {

    }
    // Loop over the files[] user input argument and save each file into a Vec<u8>.
    // each would be pushed into a larger Vec<Vec<u8>> which would be the zip::ziptape struct
    for i in files.iter()
    {
        ziparchive.ziptape.push(archiver::archive(i): Vec<u8>);
    }

 }