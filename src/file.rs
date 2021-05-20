use std::fs::OpenOptions;
use std::io::Write;

pub struct FileUtil {
    file: std::fs::File,
}

impl FileUtil {
    pub fn new(path: &String) -> FileUtil {
        let f = match OpenOptions::new().append(true).create(true).open(path) {
            Ok(file) => file,
            Err(error) => {
                panic!("Error when opening file : {}, message : {:?}", path, error)
            }
        };
        FileUtil { file: f }
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all(data)?;
        Ok(())
    }
}
