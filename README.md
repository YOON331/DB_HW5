# DB_HW5
- Rust 를 이용한 CentOS MySQL 서버의 madang 데이터베이스에 접근하여 데이터 삽입, 삭제, 검색 기능을 수행하는 프로그램입니다.  <br/><br/>
- 구조체 이용
  ~~~
  struct Book {
    bookid: i32,
    bookname: Option<String>,
    publisher: Option<String>,
    price: Option<i32>,
  }
  ~~~
  <img width="367" alt="image" src="https://github.com/user-attachments/assets/db362899-c2a0-4b77-bf49-396f6f0ef85d">   
  
  - madang 데이터베이스의 Book 테이블은 bookid를 제외한 나머지가 Null 값을 가질 수 있기 때문에 구조체에 Option 타입 사용   <br/><br/>
- 기능
  - 데이터 삽입 및 삭제 = `query_drop` 메서드를 사용하여 입력한 쿼리문 실행 데이터 삽입 및 삭제
  - 데이터 조회
    - `query_map` 메서드를 사용하여 쿼리결과를 Vec 타입으로 반환.
    - 반환된 vector를 순회하며 해당하는 구조체 필드에 매핑
    - Null 값을 갖는 필드 처리
      - String은 "Null"로 표시하고 int는 Null 값을 0 으로 대체함.

- 실행결과
  
![image](https://github.com/user-attachments/assets/413b6181-b6ca-422a-a977-f7136014d0dc)
