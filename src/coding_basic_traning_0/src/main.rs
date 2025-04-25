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
    println!("##### 문자열 반복해서 출력하기 #####");
    let mut args = String::new();

    loop {
        println!("입력 #1");
        io::stdin().read_line(&mut args).unwrap();
        let input: Vec<&str> = args.split_whitespace().collect();

        let arg1 = input[0];
        let arg2 = input[1].parse::<i32>().unwrap_or(1);

        if arg1.len() < 1 || arg1.len() > 10 { continue; }
        if arg2 < 1 || arg2 > 5 { continue; }

        println!("출력 #1");
        let mut iter = 0;
        loop {
            if iter == arg2 { break; } else { print!("{arg1}"); }
            iter += 1;
        }
        break;
    }
}

































