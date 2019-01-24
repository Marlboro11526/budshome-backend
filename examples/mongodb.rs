use mongodb::{Bson, bson, doc};
use mongodb::{Client, CommandResult, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main() {
    let mut client = Client::connect("123.207.15.199", 27017, )
        .expect("Failed to initialize standalone client.");
    client.add_completion_hook(log_query_duration).unwrap();

    let db = client.db("lawelf");
    db.auth("lawelf", "lawelf.2018@cd")
        .expect("Failed to authorize database, check username/password.");

    let coll = db.collection("contract_analysis_result");

    let doc = doc! {
        "name": "花卉购销合同",
        "lack_items": [ "缺少争议解决条款", "缺少债权债务转让条款", "缺少合同解除条款" ],
        "risk_items": [ "担保物处理方式无效的风险", "连带担保的风险", "国家标准和行业标准不一致的风险" ],
    };
    println!("{}", doc);

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();
    println!("{:?}", item);

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc2)) => match doc2.get("_id") {
            Some(&Bson::ObjectId(ref _id)) => println!("{}", _id),
            _ => panic!("Expected name to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }

    let cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref name)) = item.get("name") {
                println!("name: {}", name);
            }
        }
    }
}

fn log_query_duration(_client: Client, command_result: &CommandResult) {
    match command_result {
        &CommandResult::Success { duration, .. } => {
            println!("Command took {} nanoseconds.", duration);
        },
        _ => println!("Failed to execute command."),
    }
}