/*
	  [local file header 1]
      [encryption header 1]
      [file data 1]
      [data descriptor 1]
      . 
      .
      .
      [local file header n]
      [encryption header n]
      [file data n]
      [data descriptor n]

      [archive decryption header] 
      [archive extra data record] 

      [central directory header 1]
      .
      .
      .
      [central directory header n]
      [digital signature] 
      
      [zip64 end of central directory record]
      [zip64 end of central directory locator] 
      [end of central directory record]

 */

use std::fs::File;
use std::io::Read;


#[path = "./zip.rs"]
mod zip;

mod archiver
{
    struct FileMetaData
    {
        lastModifiedTime:   u16,
        lastMosifiedDate:   u16,
        crc32:              u32,
        fileName:           String,
        fileNameLength:     u16,
        fileComment:        String,
    }
    // Store each file into a Vec<u8> and extract some metadata
	fn archive(input: String) -> zip::ziplog {
        let mut file = File::open(input)?;
    
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
    
        return Ok(data);
    }
}

