pub mod painter_db {
    use mongodb::bson::{doc, Document};
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
    use mongodb::sync::Client;
    use std::env;
    use dotenv::dotenv;
    use crate::drawn_shape::drawn_shape_mod::DrawingShapes;

    fn set_client() -> mongodb::error::Result<Client> {
        dotenv().ok();
        let mut client_options =
            ClientOptions::parse(env::var("DB_PATH").unwrap()).run().expect("option parsing failed");
        // Set the server_api field of the client_options object to set the version of the Stable API on the client
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Get a handle to the cluster
        Client::with_options(client_options)
    }

    pub fn db_ping() -> mongodb::error::Result<()> {
        let client = set_client()?;
        // Ping the server to see if you can connect to the cluster
        client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .run().expect("Ping failed");
        Ok(())
    }
    pub fn db_query(id: String, author: String, date: String) -> Vec<DrawingShapes> {
        let client = set_client().unwrap();
        client
            .database("egui_art")
            .collection::<DrawingShapes>("art")
            .find(doc! {
                "$and": fill_query(id,author,date)
            })
            .run()
            .expect("Query paniced")
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
    }
    fn fill_query(id: String, author: String, date: String) -> Vec<Document> {
        let mut and_vec = Vec::new();
        if !id.is_empty() { and_vec.push(doc! {"_id": id}) };
        if !author.is_empty() { and_vec.push(doc! {"author": author}) };
        if !date.is_empty() { and_vec.push(doc! {"date": date}) };
        and_vec
    }
}