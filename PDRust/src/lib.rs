use rocket::serde::Serialize;



#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseBody {
    Success(String),
    Error(String),
    PDF(Vec::<u8>),
    AuthToken(String),
}



pub struct PdfSession {
    pdf_session_type: PdfSessionType,

}

pub enum PdfSessionType {
    SinglePdf(Pdf),
    MultiplePdf(Vec::<Pdf>),
}


pub enum Pdf{
    Corrupt(PdfStore),
    Ready(PdfStore),
    PerformingAction(PdfStore)
}

impl Pdf {
    pub fn compress_pdf(&self) -> ResponseBody{
        if let Pdf::Ready(_) = &self {} else{
            return ResponseBody::Error("PDF is not ready for action".to_string());
        }

        let result = todo!("Compress PDF"); 
        match result {
            Ok(_) => ResponseBody::Success("PDF compressed successfully".to_string()),
            Err(_) => ResponseBody::Error("Failed to compress PDF".to_string()),
        }

    }
}

pub enum PdfStore {
    None,
    Memory(Vec::<u8>),
    File(String),
}

impl PdfStore {
    pub fn get_pdf(&self) -> Option<Vec::<u8>> {
        match self {
            PdfStore::None => None,
            PdfStore::Memory(pdf) => Some(pdf.clone()),
            PdfStore::File(path) => {
                let file = std::fs::File::open(path);
                match file {
                    Ok(mut file) => {
                        let mut buffer = Vec::<u8>::new();
                        match file.read_to_end(&mut buffer) {
                            Ok(_) => Some(buffer),
                            Err(_) => None,
                        }
                    }
                    Err(_) => None,
                }
            }
        }
    }
}

