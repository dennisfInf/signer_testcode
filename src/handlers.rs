use actix_web::{HttpResponse, web::{Data, self}, Error};
use ring::signature::{ KeyPair, self};

use crate::{app_state::AppState, models::{SignatureRequest, SignatureStruct}};

pub(crate) async fn handle_sign_certificate(
    data: Data<AppState>,
    msg: web::Json<SignatureRequest>,
) -> Result<HttpResponse, Error>  {
    let msg = msg.into_inner().message;
    let sig = data.key_pair.sign(msg.as_bytes());
    let sig_bytes = sig.as_ref().to_vec();
    Ok(HttpResponse::Ok().json(&SignatureStruct{message:msg,signature:sig_bytes}))
}

pub(crate) async fn handle_verify_certificate(
    data: Data<AppState>,
    sig_struct: web::Json<SignatureStruct>,
) -> Result<HttpResponse, Error> {
    let sig_struct = sig_struct.into_inner();
    let peer_public_key_bytes = data.key_pair.public_key().as_ref();
    let peer_public_key =
    signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
    peer_public_key.verify(sig_struct.message.as_bytes(), &sig_struct.signature).unwrap();
    Ok(HttpResponse::Ok().finish())
}