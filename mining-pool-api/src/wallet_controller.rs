use {crate::util::*, crate::wallet::*, actix_web::web::Json, actix_web::HttpResponse};

//List all wallet
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    /*
    TODO: Get all WalletDAO objects from db and convert them to Miners Objects
     */
    let wallet: Vec<Wallet> = vec![]; //empty for now
    ResponseType::Ok(wallet).get_response()
}

// Get a Wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    /*TODO:
    Get the WalletADOfrom the DB where id = requested id and convert it to Wallet Object
    */
    // Option when in the case we can have either absence or presence of a value
    let wallet: Option<Wallet> = None;

    //Returned response
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet not found.".to_string()))
            .get_response(),
    }
}

// Create New Miner

#[post("wallets")]
pub async fn create_wallet(wallet_request: Json<NewWalletRequest>) -> HttpResponse {
    /*
    TDOD: Create a new WalletDAO object from requested inputs and write to DB
     */
    let wallet: Vec<Wallet> = vec![];

    //Return response
    ResponseType::Created(wallet).get_response()
}
