use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::{Value,json};
use std::fs;

#[get("/")]
pub async fn index() -> impl Responder {
    let inp = fs::read_to_string("static/index.html").unwrap();

    HttpResponse::Ok().body(inp)
}

fn cdb(key: &str, path:&str) -> impl Responder {
    if key.len() != 30 {
        return HttpResponse::BadRequest().body("Improperly sized key");
    }

    if path != "" {
        let k = match fs::read_to_string(format!("db/{}.json", key)){
            Err(_) => return HttpResponse::NotFound().body("Could not find file"),
            Ok(e) => e,
        };

        let mut d: Value = serde_json::from_str(&k).unwrap();
        let mut x: &mut Value = &mut d;

        let p: Vec<&str> = path.split("/").collect();
        
        for k in p.iter() {
            x = match x.get_mut(&k){
                None => return HttpResponse::BadRequest().body("Field does not exist in JSON"),
                Some(c) => c,
            } 
        }   

        x.take();

        let file = match fs::File::create(format!("db/{}.json", key)) {
            Ok(n) => n,
            Err(_) => return HttpResponse::InternalServerError().body("Could not create file"),
        };

        match serde_json::to_writer(file, &d.as_object()) {
            Ok(_) => 0,
            Err(_) => return HttpResponse::InternalServerError().body("Could not write to file"),
        };
        
        return HttpResponse::Ok().body(format!("Success"))
    }

    match fs::remove_file(format!("db/{}.json", key)) {
        Err(_) => HttpResponse::InternalServerError().body("Unable to delete file."),
        Ok(_) => HttpResponse::Ok().body("Success!"),
    }
} 

fn gdb(key: &str, path: &str) -> impl Responder {
    if key.len() != 30 {
        return HttpResponse::BadRequest().body("Improperly sized key");
    }
    
    let k = match fs::read_to_string(format!("db/{}.json", key)){
        Err(_) => return HttpResponse::NotFound().body("Could not find file"),
        Ok(e) => e,
    };

    let mut i: &Value = &serde_json::from_str(&k).unwrap();

    if path != "" {
        let p: Vec<&str> = path.split("/").collect();
        
        for k in p.iter() {
            i = match i.get(&k){
                None => return HttpResponse::BadRequest().body("Field does not exist in JSON"),
                Some(c) => c,
            } 
        }
    } 

    match i.as_object() {
        None => HttpResponse::BadRequest().body("Field does not exist in JSON"),
        Some(c) => HttpResponse::Ok().json(c),
    }
}

fn wdb(key: &str, path: &str, info: web::Json<Value>) -> impl Responder {
    if key.len() != 30 {
        return HttpResponse::BadRequest().body("Improperly sized key, {}");
    }

    let mut d:Value = json!(info.as_object().unwrap());
    if path != ""{
        let p: Vec<&str> = path.split("/").collect();
        for k in p.iter().rev() {
            d = json!({k[..]: d.as_object().unwrap()});
        }
    } 

    let file = match fs::File::create(format!("db/{}.json", key)) {
        Ok(n) => n,
        Err(_) => return HttpResponse::InternalServerError().body("Could not create file"),
    };

    match serde_json::to_writer(file, &d.as_object()) {
        Ok(_) => 0,
        Err(_) => return HttpResponse::InternalServerError().body("Could not write to file"),
    };
    
    HttpResponse::Ok().body(format!("Success"))

}

#[post("/api/{key}/{path:.*}")]
pub async fn edb(rdata: web::Path<(String, String)>, info: web::Json<Value>) -> impl Responder {
    wdb(&rdata.0, &rdata.1, info)
}

#[post("/api/{key}")]
pub async fn ndb(rdata: web::Path<(String,)>, info: web::Json<Value>) -> impl Responder {
    wdb(&rdata.0, "", info)
}

#[get("/api/{key}/{path:.*}")]
pub async fn rdb(rdata: web::Path<(String, String)>) -> impl Responder {
    gdb(&rdata.0, &rdata.1)
}

#[get("/api/{key}")]
pub async fn sdb(rdata: web::Path<(String,)>) -> impl Responder {
    gdb(&rdata.0, "")
}

#[delete("/api/{key}")]
pub async fn ddb(rdata: web::Path<(String,)>) -> impl Responder {
    cdb(&rdata.0, "")
}

#[delete("/api/{key}/{path:.*}")]
pub async fn xdb(rdata: web::Path<(String, String)>) -> impl Responder {
    cdb(&rdata.0, &rdata.1)
}
