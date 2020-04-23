use actix_web::{HttpServer, App};
use rjson::{index, edb, rdb, ndb, sdb, ddb, xdb};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
            .service(index)
            .service(edb)
            .service(rdb)
            .service(ndb)
            .service(sdb)
            .service(ddb)
            .service(xdb)
        )
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
