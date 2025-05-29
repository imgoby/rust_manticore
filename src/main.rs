use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
    remark: Option<String>,
    post_date: i64
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
            remark text,
            post_date timestamp
        )engine='columnar'")?;

    // let payments = vec![
    //     Payment { customer_id: 1, amount: 2, account_name: None },
    //     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    //     Payment { customer_id: 5, amount: 6, account_name: None },
    //     Payment { customer_id: 7, amount: 8, account_name: None },
    //     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    // ];

    //设置时区
    // SET GLOBAL timezone = 'Asia/Shanghai';

    let local: DateTime<Local> = Local::now();
    let local_str = local.format("%Y-%m-%dT%H:%M:%s").to_string();
    println!("local_str:{}",local_str);

    let timestamp: i64 = local.timestamp();

    //%Y-%m-%d'T'%H:%M:%S%Z
    //2025-05-27'T'11:42:32+08:00

    //%Y-%m-%dT%H:%M:%E*S%Z 不支持

    //%Y-%m-%dT%H:%M:%s
    //2025-05-27T11:46:1748317579

    let utc_now: DateTime<Utc> = Utc::now();
    // let utc_str = utc_now.format("%Y-%m-%d %H:%M:%S %z");
    // let utc_str = utc_now.format("%Y-%m-%dT%H:%M:%s");
    let utc_str = utc_now.format("%Y-%m-%d'T'%H:%M:%S%Z");
    println!("utc_str:{}",utc_str);

    let sql=format!("INSERT INTO payment (customer_id, amount, account_name,remark,post_date) VALUES (1,1,'tom','hello tom',{}), (2,2,'jack','helo jack',{})",timestamp,timestamp);
    conn.query_drop(sql)?;

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
            "SELECT customer_id, amount, account_name,remark,post_date from payment",
            |(customer_id, amount, account_name,remark,post_date)| {
                Payment { customer_id, amount, account_name,remark,post_date }
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
    conn.query_iter("Select id,account_name,amount,post_date from payment").unwrap()
    .for_each(|row|{
        let r:(i64,String,i32,u32)=from_row(row.unwrap());
        println!("id={},name={},age={},post_date={}",r.0,r.1,r.2,r.3);
    });

    // //方式2：将数据集取出存储在Vec中
    let res:Vec<(i64,String,i32)>=conn.query("Select id,account_name,amount from payment").unwrap();
    for r in res{
        println!("id={},name={},age={}",r.0,r.1,r.2);
    }


    let row: Row = conn.query_first("select 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")?.unwrap();
    for i in 0..row.len() {
        // println!("{:?}",row[i]);
        println!("{}",from_value::<i32>(row[i].clone()))

        // let x mysql_common::value::Value=row[i]
        // print_type_of(&row[i]);
    }



    let res:Vec<Row>=conn.query("Select id,account_name,amount,post_date from payment").unwrap();
    for row in res{
        println!("{}",from_value::<i64>(row[0].clone()));
        println!("{}",from_value::<String>(row[1].clone()));
        println!("{}",from_value::<i32>(row[2].clone()));
        println!("{}",from_value::<i64>(row[3].clone()));

        let timestamp: i64 =from_value::<i64>(row[3].clone());
        let local_str=timestamp_to_localtime_string(timestamp,"%Y-%m-%d %H:%M:%S");
        println!("local_str:{}",local_str);

        let timestamp2=localtime_string_to_timestamp(&local_str,"%Y-%m-%d %H:%M:%S");
        println!("timestamp2:{}",timestamp2);
    }



    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    // assert_eq!(payments, selected_payments);
    // println!("Yay!");

    Ok(())
}

pub fn next_day_timestamp(day:i64) ->i64{
    return day+24*60*60
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


pub fn localdate_string_to_timestamp(date_string: &str,fmt:&str) ->i64{
    let datetime_string=format!("{} 00:00:00",date_string);
    let datetime_fmt=format!("{} %H:%M:%S",fmt);
    return localtime_string_to_timestamp(datetime_string.as_str(),datetime_fmt.as_str())
}



//时间戳格式化本地时间
fn timestamp_to_localtime_string(timestamp: i64,fmt:&str) ->String{
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let local_now: DateTime<Local> = datetime.with_timezone(&Local);

    let local_string = local_now.format(fmt);
    return format!("{}",local_string)
}

//本地时间转时间戳
fn localtime_string_to_timestamp(date_string: &str,fmt:&str) ->i64{
    match NaiveDateTime::parse_from_str(date_string, fmt) {
        Ok(naive_date_time) => {
            let parsed = Local.from_local_datetime(&naive_date_time).single();
            match parsed {
                Some(parsed) => {
                    parsed.timestamp()
                }
                None => 0,
            }
        },
        Err(e) => {
            println!("Error parsing datetime: {}", e);
            return 0;
        }
    }
}
