#[derive(Debug, rustler::NifStruct)]
#[module = "DenoRider.Error"]
pub struct Error {
    pub message: Option<std::string::String>,
    pub name: rustler::Atom,
}
