use openssl::pkcs12::Pkcs12;
use openssl::pkcs12::ParsedPkcs12;
use std::fs::File;
use std::io::Read;

pub fn read_pkcs12(file_path: String, password: String) -> ParsedPkcs12 {
    let mut file: File = File::open(file_path).unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Pkcs12::from_der(&pkcs12).unwrap();
    pkcs12.parse(&password).unwrap()
}