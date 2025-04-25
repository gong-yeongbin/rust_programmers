use std::io::{self, BufRead};
use std::process::exit;

fn is_alphabetic(str: &str) -> bool {
    let text = str.clone().trim().replace(" ", "");
    for c in text.chars() {
        if !c.is_alphabetic() { return false; }
    }
    true
}

fn is_alphanumeric(str: &str) -> bool {
    let text = str.clone().trim().replace(" ", "");
    for c in text.chars() {
        if c.is_alphanumeric() { return false; }
    }
    true
}

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
    // println!("##### 문자열 출력하기 #####");
    //
    // loop {
    //     println!("입력 #1");
    //
    //     let mut str = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut str)
    //         .expect("reading console input error.");
    //
    //     str = str.replace(" ", "").trim().to_string();
    //
    //     if str.len() >= 1 && str.len() <= 100 {
    //         println!("출력 #1");
    //         println!("{str}");
    //         break;
    //     } else {
    //         println!("문자열 길이는 1 보다 크고 100 보다 작아야 합니다.");
    //     }
    // }

    /*
    a와 b 출력하기
    문제 설명
        - 정수 a와 b가 주어집니다. 각 수를 입력받아 입출력 예와 같은 형식으로 출력하는 코드를 작성해 보세요.
    제한사항
        - -100,000 ≤ a, b ≤ 100,000
    입출력 예
        입력 #1
            4 5
        출력 #1
            a = 4
            b = 5
    */
    // println!("##### a와 b 출력하기 #####");
    // let mut arg = String::new();
    //
    // println!("입력 #1 ");
    // io::stdin().read_line(&mut arg).expect("");
    //
    // let mut input = arg.trim().split_whitespace();
    // let a = input.next().unwrap().parse::<i32>().unwrap();
    // let b = input.next().unwrap().parse::<i32>().unwrap();
    //
    // if a <= -100000 || b >= 100000 {
    //     println!("-100,000 ≤ a, b ≤ 100,000 error!")
    // } else {
    //     println!("출력 #1 ");
    //     println!("a = {a}");
    //     println!("b = {b}");
    // }

    /*
    문자열 반복해서 출력하기
    문제 설명
        - 문자열 str과 정수 n이 주어집니다.
        - str이 n번 반복된 문자열을 만들어 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ str의 길이 ≤ 10
        - 1 ≤ n ≤ 5
    입출력 예
        입력 #1
            string 5
        출력 #1
            stringstringstringstringstring
    */
    // println!("##### 문자열 반복해서 출력하기 #####");
    // let mut args = String::new();
    //
    // loop {
    //     println!("입력 #1");
    //     io::stdin().read_line(&mut args).unwrap();
    //     let input: Vec<&str> = args.split_whitespace().collect();
    //
    //     let arg1 = input[0];
    //     let arg2 = input[1].parse::<i32>().unwrap_or(1);
    //
    //     if arg1.len() < 1 || arg1.len() > 10 { continue; }
    //     if arg2 < 1 || arg2 > 5 { continue; }
    //
    //     println!("출력 #1");
    //     let mut iter = 0;
    //     loop {
    //         if iter == arg2 { break; } else { print!("{arg1}"); }
    //         iter += 1;
    //     }
    //     break;
    // }

    /*
    대소문자 바꿔서 출력하기
    문제 설명
        영어 알파벳으로 이루어진 문자열 str이 주어집니다. 각 알파벳을 대문자는 소문자로 소문자는 대문자로 변환해서 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ str의 길이 ≤ 20
        - str은 알파벳으로 이루어진 문자열입니다.
    입출력 예
    입력 #1
        aBcDeFg
    출력 #1
        AbCdEfG
    */
    // println!("##### 대소문자 바꿔서 출력하기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #1");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // if !is_alphabetic(&args) {
    //     println!("알파벳 문자가 아닙니다.");
    //     exit(0);
    // }
    //
    // println!("출력 #1");
    // for c in args.chars() {
    //     if c.is_uppercase() { print!("{}", c.to_lowercase()) }
    //     if c.is_lowercase() { print!("{}", c.to_uppercase()) }
    // }

    /*
    특수문자 출력하기
    문제 설명
        다음과 같이 출력하도록 코드를 작성해 주세요.
    출력 예시
        !@#$%^&*(\'"<>?:;
    */
    // println!("##### 특수문자 출력하기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #1");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // if !is_alphanumeric(&args) {
    //     println!("특수문자가 아닙니다.");
    //     exit(0);
    // }
    // println!("출력 #1");
    // println!("{}", args);

    /*
    덧셈식 출력하기
    문제 설명
        두 정수 a, b가 주어질 때 다음과 같은 형태의 계산식을 출력하는 코드를 작성해 보세요.
        a + b = c
    제한사항
        - 1 ≤ a, b ≤ 100
    입출력 예
    입력 #1
        4 5
    출력 #1
        4 + 5 = 9
    */
    println!("##### 덧셈식 출력하기 #####");
    let mut args = String::new();

    println!("입력 #1");
    io::stdin().read_line(&mut args).unwrap();

    let input: Vec<&str> = args.trim().split_whitespace().collect();
    let a = input[0].parse::<i32>().unwrap_or(0);
    let b = input[1].parse::<i32>().unwrap_or(0);

    if a < 0 || b > 100 {
        println!("1 ≤ a, b ≤ 100 error!");
        exit(0)
    }

    println!("출력 #1");
    println!("{} + {} = {}", a, b, a + b);



    /*
문자열 붙여서 출력하기
문제 설명
두 개의 문자열 str1, str2가 공백으로 구분되어 입력으로 주어집니다.
입출력 예와 같이 str1과 str2을 이어서 출력하는 코드를 작성해 보세요.

제한사항
1 ≤ str1, str2의 길이 ≤ 10
입출력 예
입력 #1

apple pen
출력 #1

applepen
입력 #2

Hello World!
출력 #2

HelloWorld!
    */

    /*
    문자열 돌리기
    문제 설명
문자열 str이 주어집니다.
문자열을 시계방향으로 90도 돌려서 아래 입출력 예와 같이 출력하는 코드를 작성해 보세요.

제한사항
1 ≤ str의 길이 ≤ 10

입출력 예
입력 #1

abcde
출력 #1

a
b
c
d
e
*/
}

































