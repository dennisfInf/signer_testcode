use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub(crate) struct SignatureRequest{
   pub(crate) message:String,
}
#[derive(Serialize,Deserialize)]
pub(crate) struct SignatureStruct{
    pub(crate) message:String,
    pub(crate) signature:Vec<u8>
}
 



