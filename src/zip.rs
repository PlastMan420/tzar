// ZIP file spec: https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
/*
 4.3.6 Overall .ZIP file format:

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

mod zip
{
	// Zip version of the followed specification = 3.6.9
	const version: 			u16 = 369;

	// compression Methods
	const deflate64:		u8 = 9;
	const zstd: 			u8 = 93;
	const store: 			u8 = 0;

	// Required
	// 28+ Bytes
	struct LocalFileHeader
	{
		const signature: 	u32 = 0x04034b50,
		version: 			u16 = version,
		compression: 		u16,
		lastModifiedTime: 	u16,
		lastModifiedDate: 	u16,
		crc32: 				u32,
		compressedSize: 	u32,
		uncompressedSize: 	u32,
		fileNamelength: 	u16,
		extraFieldLength: 	u16,

		fileName: 			String,
		extra: 				String,

	}

	struct EncryptionHeader{}

	struct FileData
	{
		/*
			Zero-byte files, directories, and other file types that 
			contain no content MUST NOT include file data.
		*/
		file: 				Vec<u8>,
		// const compressable: bool = true;
		// const encryptable: bool = true,
	}
	// 20+ Bytes
	struct DataDescriptor
	{
		// 4.3.9.1 This descriptor MUST exist if bit 3 of the general
		// purpose bit flag is set
		// It is byte aligned and immediately follows the last byte of compressed data.
		// This descriptor SHOULD be used only when it was not possible to
		// seek in the output .ZIP file, e.g., when the output .ZIP file
		// was standard output or a non-seekable device.  For ZIP64(tm) format
		// archives, the compressed and uncompressed sizes are 8 bytes each.

		crc32: 				u32,
		compressedSize: 	u64,
		uncompressedSize: 	u64,
	}

	struct DecryptionHeader
	{
		/*
			4.3.10.1 The Archive Decryption Header is introduced in version 6.2
			of the ZIP format specification.  This record exists in support
			of the Central Directory Encryption Feature implemented as part of 
			the Strong Encryption Specification as described in this document.
			When the Central Directory Structure is encrypted, this decryption
			header MUST precede the encrypted data segment.  
		*/
	}
	// 8+ Bytes
	struct ExtraRecord
	{
		const   extraSignature:     u32 = 0x08064b50,
				extraFieldLength:   u32,
				extraFieldData:     String,
	}

	// 52+ Bytes
	struct CentralDirectoryHeader
	{
		/*
			[central directory header 1]
		.
		.
		. 
			[central directory header n]
			[digital signature]
		*/

		const centralHeaderSignature: 	u32 = 0x02014b50,
		creatorVersion: 				u16,
		extractorVersion: 				u16,
		flag: 							u16,
		compressionMethod: 				u16,
		lastModifiedTime: 				u16,
		lastmodifiedDate: 				u16,
		crc32: 							u32,
		compressedSize: 				u64,
		uncompressedSize: 				u64,
		filenameLength: 				u16,
		extraFieldLength: 				u16,
		commentLength: 					u16,
		diskNumberStart: 				u16,
		internalFileAttributes: 		u16,
		externalFileAttributes: 		u16,
		relativeOffset: 				u32,

		fileName: 						String,
		extraField: 					String,
		fileComment: 					String,

	}

	// 6+ Bytes
	struct DigitalSignature
	{
		const headerSiggnature: 		u32 = 0x05054b50,
		dataSize: 						u16,
		SignatureData: 					String,
	}

	//
	struct Zip64EOCDRecordV1
	{
		const zip64EOCRSignature: 		u32 = 0x06064b50,
		let zip64EOCRSize: 				u64 = 44 // + sizeof( zip64exDataSect.toInt() ),
		creatorVersion: 				u16,
		extractorVersion: 				u16,
		thisDisk: 						u32,
		cDirStartDisk: 					u32,
		cDirDiskEntries: 				u64,
		cDirTotalEntries: 				u64,
		cDirSize: 						u64,
		startOffset: 					u64,
		zip64exDataSect: 				String,
	}

	struct Zip64EOCDRecordV2
	{

	}

	struct Zip64EOCDLocator
	{
		const signature: 				u32 = 0x07064b50,
		startDisk: 						u32,
		relativeOffset: 				u64,
		totalDisks: 					u32,
	}

	// Required
	struct EOCDRecord
	{
		const signature: 				u32 = 0x06054b50,
		thisdisk: 						u16,
		cDirStartDisk: 					u16,
		diskEntries: 					u16,
		cDirEntries: 					u16.
		cDirSize: 						u32,
		cDirStartOffset: 				u32,
		zipCommentSize:					u16,
		zipComment: 					String,
	}

	// A collection of ziplogs forms a Zip tape. it's the collection of stored files excluding the central directory
	struct ziplog
	{
		lfh:	zip::LocalFileHeader,
		eh:		zip::EncryptionHeader,
		file:	Vec<u8>,
		cdh: 	zip::CentralDirectoryHeader,

	}

	// The zip file: it's the zip file itself. with the files INCLUDING the central directory
	struct zipfile
	{
		 ziptape: 	Vec<ziplog>,
		      ds: 	zip::DigitalSignature,
		z64eocdr: 	zip::Zip64EOCDRecordV1,
		z64eocdl: 	zip::Zip64EOCDLocator,
		   eocdr: 	zip::EOCDRecord,
	}

}
