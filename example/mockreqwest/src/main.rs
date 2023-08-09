
use futures_util::future::FutureExt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Customer {
    firstname: String,
    lastname: String,
}

#[tokio::main]
async fn main()  {
    let handler = axumtest::oneshot().await;

    print!("Running get customer...");
    let result = std::panic::AssertUnwindSafe(get_user()).catch_unwind().await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    let customer = Customer{firstname: "Mary".to_string(), lastname: "Joe".to_string()};

    print!("Running change Customer...");
    let result = std::panic::AssertUnwindSafe(changecustomer(customer)).catch_unwind().await;
    match result {
        Ok(_) => {
            println!("✓");
        }
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    let _ = handler.sender.send(1);


} 
    
async fn get_user() {
    // given
    let client = reqwest::Client::new();
    let expectedRes = Customer{firstname: "foo".to_string(), lastname: "bar".to_string()};

    // when
    let res = client
        .get("http://localhost:3000/customer")
        .send()
        .await
        .unwrap()
        .json::<Customer>()
        .await;

    // then
    assert_eq!(res.unwrap(), expectedRes);
}

async fn changecustomer(customer: Customer) {
    // given
    let client = reqwest::Client::new();

    // when
    let res = client
        .post("http://localhost:3000/changeservice")
        .json(&customer)
        .send()
        .await
        .unwrap();

    // then
        assert_eq!(res.status(), 200);
        // let res = res
        // .json::<String>()
        // .await;
    

    // println!("{}",res.unwrap());

    // assert_eq!(res.unwrap(), "Firstname: Mary, Lastname: Joe".to_string());
}
