pub mod painter_db {
    use mongodb::bson::{doc, Bson};
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
    use mongodb::sync::Client;
    use std::env;
    use dotenv::dotenv;

    pub fn db_ping() -> mongodb::error::Result<()> {
        dotenv().ok();
        let mut client_options =
            ClientOptions::parse(env::var("DB_PATH").unwrap()).run().expect("option parsing failed");
        // Set the server_api field of the client_options object to set the version of the Stable API on the client
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Get a handle to the cluster
        let client = Client::with_options(client_options)?;
        // Ping the server to see if you can connect to the cluster
        client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .run().expect("Ping failed");
        let test_query: Option<Bson> = client.
            database("egui_art").
            collection("art").
            find_one(doc! {"author":"Joseph Joestar"})
            .run()
            .expect("Test failed");
        println!("{:#?}", test_query.unwrap());
        Ok(())
    }
}