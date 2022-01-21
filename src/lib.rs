use std::{fs};

use rvk::{methods::users, objects::user::User, APIClient, Params};

use mongodb::{Client, options::ClientOptions};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    api: String,
    params: Vec<_Params>,
    //db_url: String,
    //db_port: String,
    //db_user: String,
    //db_password: String,
    db_name: String,
    db_collection: String
}

#[derive(Debug, Serialize, Deserialize)]
struct _Params {
    key: String,
    value: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    // Main fields

    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    
    pub bdate: Option<String>,

    // connections
    pub skype: Option<String>,
    pub facebook: Option<String>,
    pub facebook_name: Option<String>,
    pub twitter: Option<String>,
    pub livejournal: Option<String>,
    pub instagram: Option<String>,

    pub domain: Option<String>,

    // education
    pub university: Option<i64>,
    pub university_name: Option<String>,
    pub faculty: Option<i64>,
    pub faculty_name: Option<String>,
    pub graduation: Option<i64>,

    pub sex: Option<i64>,
}

pub fn parse_config(args: &[String]) -> &str {
    &args[2]
}

pub async fn search(conf: &Conf) -> Vec<Target> {
    let api = APIClient::new(&conf.api);
    let mut params = Params::new();

    for p in &conf.params {
        let key = String::from(&p.key);
        let value = String::from(&p.value);

        params.insert(key.into(), value.into());
    }

    let resp = users::search::<serde_json::Value>(&api, params).await.expect("Failed parse");
    let items = resp["items"].clone();
    let result = serde_json::from_value::<Vec<User>>(items).expect("Error parsing from value");

    let mut target: Vec<Target> = vec![];

    for user in result {
        target.insert(0, Target {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            bdate: user.bdate,
            skype: user.skype,
            facebook: user.facebook,
            facebook_name: user.facebook_name,
            twitter: user.twitter,
            livejournal: user.livejournal,
            instagram: user.instagram,
            domain: user.domain,
            university: user.university,
            university_name: user.university_name,
            faculty: user.faculty,
            faculty_name: user.faculty_name,
            graduation: user.graduation,
            sex: user.sex,
        })
    }

    target
}

pub fn read_config(filename: String) -> Conf {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    serde_json::from_str::<Conf>(&contents).unwrap()
}

pub async fn mongodb_save(docs: Vec<Target>, conf: &Conf) {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("Failed connect");

    let client = Client::with_options(client_options).expect("Client error");
    let db = client.database(
        &conf.db_name);
    let collection = db.collection::<Target>(&conf.db_collection);
    collection.insert_many(docs, None).await.expect("Failed save");
}