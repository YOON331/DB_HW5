use mysql::*;
use mysql::prelude::*;
use std::env;
use dotenv::dotenv;

#[derive(Debug)]
struct Book {
    bookid: i32,
    bookname: Option<String>,
    publisher: Option<String>,
    price: Option<i32>,
}

fn main() -> Result<()> {
    dotenv().ok();  // .env 파일 불러오기
    let database_url = env::var("DATABASE_URL").expect("not set the DATABAER_URL");

    let pool = Pool::new(database_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // 테이블에 데이터 삽입
    conn.query_drop(
        r"insert into Book(bookid, bookname, publisher, price)
        values (99, '테스트 book', '윤별', '99999')"
    )?;
    println!("Insert row in table");


    // table retriever (테이블 조회)
    println!("\nRetriever table");
    let result = conn.query_map(
        "SELECT * FROM Book",   // query 문
    |(bookid, bookname, publisher, price)|  
        Book{bookid, bookname, publisher, price},)?; // 보낸 쿼리에 대한 결과 mapping
    
    for r in result {
        println!("{} {} {} {}", r.bookid, r.bookname.unwrap_or("NULL".to_string()), r.publisher.unwrap_or("NULL".to_string()), r.price.unwrap_or_default());
        // bookname과 publisher, price는 null 값을 가질 수 있음 
        // rust에서 Int(i32)는 Null 값을 가질 수 없어서 0 으로 mapping
    }
    
    
    // 테이블에 데이터 삭제
    println!("\nDelete row in table");
    conn.query_drop(
        r"delete from Book where bookid='99'"
    )?;
    

    // table retriever (테이블 조회)
    let result = conn.query_map(
        "SELECT * FROM Book",   // query 문
    |(bookid, bookname, publisher, price)|  
        Book{bookid, bookname, publisher, price},)?; // 보낸 쿼리에 대한 결과 mapping
    
    for r in result {
        println!("{} {} {} {}", r.bookid, r.bookname.unwrap_or("NULL".to_string()), r.publisher.unwrap_or("NULL".to_string()), r.price.unwrap_or_default());
        // bookname과 publisher, price는 null 값을 가질 수 있음 
        // rust에서 Int(i32)는 Null 값을 가질 수 없어서 0 으로 mapping
    }

    Ok(())
}