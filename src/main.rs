use redis;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::str;
use anyhow::Result;

const ROOT: &str = "CK";


fn get_alerting_policy(con: &mut redis::Connection, key: &str) -> Result<AlertingPolicy> {
    let ti: String = redis::cmd("JSON.GET").arg(ROOT).arg(format!(".AP.{}.title", key)).query(con)?;
    let me: String = redis::cmd("JSON.GET").arg(ROOT).arg(format!(".AP.{}.message", key)).query(con)?;
    let no: Vec<String> = redis::cmd("JSON.GET").arg(ROOT).arg(format!(".AP.{}.notification_channel", key)).query(con)?;

    Ok(AlertingPolicy {
        title: ti,
        message: me,
        notification_channel: no
    })
}

fn print_json(con: &mut redis::Connection, q: &str) -> Result<()> {
    let obj: String = redis::cmd("JSON.GET").arg(ROOT).arg(q).query(con)?;

    let gh: HashMap<String, AlertingPolicy> = serde_json::from_str(&format!("{}", obj))?;

    for (key, val) in gh {
        println!("{}: {}", key, val.title);
    };

    Ok(())
}

fn get_alerting_policy_json(con: &mut redis::Connection, key: &str) -> Result<()> {
    print_json(con, &format!(".AP.{}", key))
}

fn list_all_alearting_policy(con: &mut redis::Connection) -> Result<()> {
    print_json(con, ".AP")
}

#[derive(Serialize, Deserialize)]
struct AlertingPolicy {
    title: String,
    message: String,
    notification_channel: Vec<String>
}

fn create_alearting_policy(con: &mut redis::Connection, body: AlertingPolicy) -> redis::RedisResult<()> {
    let rstr: String = rand::thread_rng().sample_iter(&Alphanumeric).take(5).map(char::from).collect();
    let path: &str = &format!(".AP.{}", rstr);
    redis_set(con, path,  &serde_json::to_string(&body).unwrap())
}


fn redis_set(con: & mut redis::Connection, path: &str, body: &str) -> redis::RedisResult<()> {
    redis::cmd("JSON.SET").arg(ROOT).arg(path).arg(body).query(con)
}

fn create_base(con: &mut redis::Connection) -> redis::RedisResult<()> {
    redis_set(con, ".", "{}")?;
    redis_set(con, ".AP", "{}")?;
    redis_set(con, ".NC", "{}")?;
    Ok(())
}

fn main() -> redis::RedisResult<()> {

    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;

    // create_base(&mut con)?;

    let policy = AlertingPolicy {
        title: "Vedang's new policy1".to_owned(),
        message: "Hey there".to_owned(),
        notification_channel: vec!["email1".to_owned()]
    };

    // create_alearting_policy(&mut con, policy)?;
    // println!("{:?}", get_alerting_policy(&mut con, ".AP.gBxm3").unwrap().title);

    list_all_alearting_policy(&mut con).unwrap();
    // println!("{}", );


    Ok(())

}