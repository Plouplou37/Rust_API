use {crate::miner::*, crate::util::*, actix_web::web::Json, actix_web::HttpResponse};

//List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    /*
    TODO: Get all MinerDAO objects from db and convert them to Miners Objects
     */
    let miners: Vec<Miner> = vec![]; //empty for now
    ResponseType::Ok(miners).get_response()
}

// Get a Miner
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    /*TODO:
    Get the MinerADOfrom the DB where id = requested id and convert it to Miner Object
    */
    // Option when in the case we can have either absence or presnece of a value
    let miner: Option<Miner> = None;

    //Returned response
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Miner not found.".to_string()))
            .get_response(),
    }
}

// Create New Miner

#[post("wallet/{id}/miners")]
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    /*
    TDOD: Create a new MinerDAO object from requested inputs and write to DB
     */
    let miner: Vec<Miner> = vec![];

    //Return response
    ResponseType::Created(miner).get_response()
}
