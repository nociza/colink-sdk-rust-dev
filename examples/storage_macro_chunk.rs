use colink::{decode_jwt_without_validation, CoLink};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let addr = &args[0];
    let jwt = &args[1];
    let payload = "hello".repeat(100);
    let msg = if args.len() > 2 { &args[2] } else { &payload };
    let user_id = decode_jwt_without_validation(jwt).unwrap().user_id;
    println!("user_id: {}", user_id);

    let cl = CoLink::new(addr, jwt);
    let key_name = "storage_macro_demo_3:$chunk";

    // create
    println!("Creating entry...");
    let response = cl.create_entry(key_name, msg.clone().as_bytes()).await?;
    println!("created entry at key name: {}", response);

    // read
    println!("Reading entry...");
    let data = cl.read_entry(key_name).await?;
    println!("{}", String::from_utf8_lossy(&data));

    // update
    println!("Updating entry...");
    let new_msg = "bye".repeat(100);
    let response = cl.update_entry(key_name, new_msg.as_bytes()).await?;
    println!("updated entry at key name: {}", response);

    // read again to verify
    println!("Reading entry again...");
    let data = cl.read_entry(key_name).await?;
    println!("{}", String::from_utf8_lossy(&data));

    // delete
    println!("Deleting entry...");
    cl.delete_entry(key_name).await?;

    Ok(())
}
