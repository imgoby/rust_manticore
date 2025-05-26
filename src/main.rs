use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
    remark: Option<String>,
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root@192.168.0.211:9306/Manticore";
    // # Opts::try_from(url)?;
    // # let url = get_opts();
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS payment (
            customer_id int ,
            amount int ,
            account_name string,
            remark text
        )engine='columnar'")?;

    // let payments = vec![
    //     Payment { customer_id: 1, amount: 2, account_name: None },
    //     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    //     Payment { customer_id: 5, amount: 6, account_name: None },
    //     Payment { customer_id: 7, amount: 8, account_name: None },
    //     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    // ];

    conn.query_drop("INSERT INTO payment (customer_id, amount, account_name,remark) VALUES (1,1,'tom','hello tom'), (2,2,'jack','helo jack')")?;

    // Now let's insert payments to the database
    // conn.exec_batch(
    //     r"INSERT INTO payment (customer_id, amount, account_name)
    //       VALUES (:customer_id, :amount, :account_name)",
    //     payments.iter().map(|p| params! {
    //         "customer_id" => p.customer_id,
    //         "amount" => p.amount,
    //         "account_name" => &p.account_name,
    //     })
    // )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name,remark from payment",
            |(customer_id, amount, account_name,remark)| {
                Payment { customer_id, amount, account_name,remark }
            },
        )?;

    for item in selected_payments{
        println!("{:?}",item);
    }

    let val: Option<u64> = conn.query_first("SELECT count(*) from payment")?;
    match val{
        Some(v)=>{
            println!("{}",v);
        },
        None=>{
            println!("None");
        }
    }

    //https://www.cnblogs.com/jaciots/p/14761611.html
    //方式1：流式查询  数据逐行读取，数据不会存储在内存中
    conn.query_iter("Select id,account_name,amount from payment").unwrap()
    .for_each(|row|{
        let r:(i64,String,i32)=from_row(row.unwrap());
        println!("id={},name={},age={}",r.0,r.1,r.2);
    });

    // //方式2：将数据集取出存储在Vec中
    let res:Vec<(i64,String,i32)>=conn.query("Select id,account_name,amount from payment").unwrap();
    for r in res{
        println!("id={},name={},age={}",r.0,r.1,r.2);
    }


    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    // assert_eq!(payments, selected_payments);
    // println!("Yay!");

    Ok(())
}