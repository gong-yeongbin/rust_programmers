use std::io::{self, BufRead};

fn main() {
    /*
    문자열 출력하기
    문제 설명
        - 문자열 str이 주어질 때, str을 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ str의 길이 ≤ 1,000,000
        - str에는 공백이 없으며, 첫째 줄에 한 줄로만 주어집니다.
    입출력 예
        입력 #1
            HelloWorld!
        출력 #1
            HelloWorld!
    */
    println!("##### 문자열 출력하기 #####");

    loop {
        println!("입력 #1");

        let mut str = String::new();

        io::stdin()
            .read_line(&mut str)
            .expect("reading console input error.");

        str = str.replace(" ", "").trim().to_string();

        if str.len() >= 1 && str.len() <= 100 {
            println!("출력 #1");
            println!("{str}");
            break;
        } else {
            println!("문자열 길이는 1 보다 크고 100 보다 작아야 합니다.");
        }
    }
}
