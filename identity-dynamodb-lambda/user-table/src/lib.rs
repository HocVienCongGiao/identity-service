use lambda_runtime::{Context, Error};
use serde_json::{json, Value};

pub async fn func(event: Value, _: Context) -> Result<Value, Error> {
    // let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("welcome to dynamodb processor!!!!");
    println!("Event payload is {:?}", event);

    println!("{}", (json!({ "message": format!("Hello, {:?}!", event) })));

    // Get item by hash key
    // upsert to cognito
    Ok(json!({ "message": format!("Hello, {:?}!", event) }))
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Map, Value};

    use crate::func;

    #[tokio::test]
    async fn create_user_success() {
        let mut records: Map<String, Value> = Default::default();
        records.insert("Records".parse().unwrap(), Value::Array(vec![]));

        let value = Value::Object(
            records
        );
        func(value, Default::default()).await;
        // let v = json!({ "a": ["an", "array"], "b": { "an": "object" } });
        // let records = v["a"].as_array().unwrap();
        // 
        // // The length of `["an", "array"]` is 2 elements.
        // assert_eq!(v["a"].as_array().unwrap().len(), 2);
        // 
        // // The object `{"an": "object"}` is not an array.
        // assert_eq!(v["b"].as_array(), None);

        // Object(
        //     {
        //         "Records": Array(
        //             [
        //                 Object(
        //         {
        //             "awsRegion": String(
        //             "ap-southeast-1"),
        //             "dynamodb": Object(
        //             {
        //                 "ApproximateCreationDateTime": Number(
        //                 1627141532.0),
        //                 "Keys": Object(
        //                 {
        //                     "HashKey": Object(
        //                     {
        //                         "S": String(
        //                         "11905088586532604268")
        //                     })
        //                 }),
        //                 "SequenceNumber": String(
        //                 "14562700000000010320701664"),
        //                 "SizeBytes": Number(
        //                 27),
        //                 "StreamViewType": String(
        //                 "KEYS_ONLY")
        //             }),
        //             "eventID": String(
        //             "6ad7a20462f36feb8632ccbd9e517a00"),
        //             "eventName": String(
        //             "INSERT"),
        //             "eventSource": String(
        //             "aws:dynamodb"),
        //             "eventSourceARN": String(
        //             "arn:aws:dynamodb:ap-southeast-1:891616054205:table/dev-sg_UserTable/stream/2021-07-21T10:43:28.424"),
        //             "eventVersion": String(
        //             "1.1")
        //         }
        //         )
        //         ]
        //         )
        //     }
        // )
    }
}