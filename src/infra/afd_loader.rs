// infra/afd_loader.rs
use std::{fs};
use encoding_rs::WINDOWS_1252; // ou ISO_8859_1, se preferir

use std::path::PathBuf;

pub fn decode_from_win1252_to_utf8(path: PathBuf)-> Option<String>{
    match fs::read(&path){
        Ok(bytes) => {
            let (content, _, _) = WINDOWS_1252.decode(&bytes);
            Some(content.into_owned())

        }
        Err(e)=>{
            eprint!("Erro ao ler o arquivo:{}", e);
            None
        }

    }
}
