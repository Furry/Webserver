use rocket::Data;
use rocket::response::status;
use std::time::{SystemTime, UNIX_EPOCH};

#[post("/upload", data = "<data>")]
pub fn upload_image(data: Data) -> status::Accepted<String> {
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let ms = n.as_millis();
            let name = format!("{}{}{}", "./images/", &ms, ".png");
            data.stream_to_file(&name)
                .expect("There was an error writing to disk.");
            
                status::Accepted(
                    Some(
                        format!(
                            "
                            {{
                                \"status\": 1,
                                \"url\": \"http://localhost:8000/{}.png\"
                            }}
                            "
                        ,&ms)
                    )
                )

        }
        Err(_) => panic!("System Time before EPOCH?")
    }
}
