use std::cmp::Ordering;
use std::fmt::format;
use std::io::{self, BufRead};
use std::process::exit;
use std::slice::SliceIndex;
use std::str::SplitWhitespace;

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

fn is_validation_mystring(str: &str) -> Result<(), String> {
    let len = str.len();
    if len < 1 {
        return Err(format!("최소 {}자 이상 입력해야 합니다. 현재: {}", 1, len));
    } else if len > 1000 { return Err(format!("최대 {}자 이하여야 합니다. 현재: {}", 1000, len)); } else { Ok(()) }
}

fn is_validation_overwrite_string(str1: &str, str2: &str) -> Result<(), String> {
    let str1_len = str1.len();
    let str2_len = str2.len();
    if str2_len < 1 {
        return Err(format!("최소 {}자 이상 입력해야 합니다. 현재: {}", 1, str1_len));
    } else if str1_len < str2_len {
        return Err(format!("최대 {}자 이하여야 합니다. 현재: {}", 1000, str1_len));
    } else { Ok(()) }
}

fn is_validation_s(n1: usize, n2: usize) -> Result<(), String> {
    if n1 < 0 {
        return Err(format!("최소 {}이상 입력해야 합니다. 현재: {}", 0, n1));
    } else if n1 > n2 { return Err(format!("최대 {} 이하여야 합니다. 현재: {}", n2, n1)); } else { Ok(()) }
}

fn print_mix_str(str1: &str, str2: &str) {
    if str1.len() != str2.len() { panic!("The two characters are not the same length.") }

    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    for i in 0..chars1.len() {
        print!("{}{}", chars1[i], chars2[i]);
    }
}

fn is_mix_str_verification(str: &str) -> Result<(), String> {
    for s in str.chars() {
        if !s.is_lowercase() {
            return Err(format!("Is not lowercase"));
        }
        if !s.is_alphabetic() {
            return Err(format!("Is not alphabet"));
        }
    }

    if str.len() <= 0 {
        return Err(format!("Minimum length must be 1."));
    }
    if str.len() > 10 {
        return Err(format!("Maximum length must be 10."));
    }

    Ok(())
}

// fn solution(str: Vec<&str>) -> String {
//     let mut result = String::new();
//
//     for s in str {
//         result.push_str(s)
//     }
//
//     result
// }

// fn solution(input: Vec<&str>) -> String {
//     let mut result = String::new();
//     let my_string = input[0];
//     let k = input[1].parse::<usize>().unwrap();
//
//     for i in 0..k {
//         result.push_str(my_string);
//     }
//     result
// }
// fn solution(a: &str, b: &str) -> String {
//     let ab_string = format!("{}{}", a, b);
//     let ba_string = format!("{}{}", b, a);
//     let ab_i32 = ab_string.parse::<i32>().unwrap();
//     let ba_i32 = ba_string.parse::<i32>().unwrap();
//
//     if ab_i32 > ba_i32 {
//         ab_string
//     } else {
//         ba_string
//     }
// }
// fn solution(a: &i32, b: &i32) -> i32 {
//     let concat_ab = format!("{a}{b}").parse::<i32>().unwrap();
//     let calculate_ab = 2 * a * b;
//
//     if concat_ab > calculate_ab {
//         concat_ab
//     } else {
//         calculate_ab
//     }
// }
// fn solution(num: &i32, n: &i32) -> i32 {
//     let result = num % n;
//
//     if result == 0 {
//         1
//     } else {
//         0
//     }
// }
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
    // println!("##### 덧셈식 출력하기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #1");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // let input: Vec<&str> = args.trim().split_whitespace().collect();
    // let a = input[0].parse::<i32>().unwrap_or(0);
    // let b = input[1].parse::<i32>().unwrap_or(0);
    //
    // if a < 0 || b > 100 {
    //     println!("1 ≤ a, b ≤ 100 error!");
    //     exit(0)
    // }
    //
    // println!("출력 #1");
    // println!("{} + {} = {}", a, b, a + b);

    /*
    문자열 붙여서 출력하기
    문제 설명
        두 개의 문자열 str1, str2가 공백으로 구분되어 입력으로 주어집니다.
        입출력 예와 같이 str1과 str2을 이어서 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ str1, str2의 길이 ≤ 10
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
    // println!("##### 문자열 붙여서 출력하기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // let input: Vec<&str> = args.trim().split_whitespace().collect();
    // let str1 = input[0];
    // let str2 = input[1];
    //
    // if str1.len() < 1 || str2.len() > 10 {
    //     println!("- 1 ≤ str1, str2의 길이 ≤ 10 error!");
    //     exit(0);
    // }
    //
    // let mut result = str1.clone().to_string();
    // result.push_str(str2);
    //
    // println!("출력 #");
    // println!("{}", result);

    /*
    문자열 돌리기
    문제 설명
        문자열 str이 주어집니다.
        문자열을 시계방향으로 90도 돌려서 아래 입출력 예와 같이 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ str의 길이 ≤ 10
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
    // println!("##### 문자열 돌리기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // let len = args.trim().len();
    // if len < 1 || len > 10 {
    //     println!("- 1 ≤ str의 길이 ≤ 10 error!");
    //     exit(0);
    // }
    //
    // println!("출력 #");
    // for c in args.chars() {
    //     println!("{c}");
    // }

    /*
    홀짝 구분하기
    문제 설명
        자연수 n이 입력으로 주어졌을 때 만약 n이 짝수이면 "n is even"을, 홀수이면 "n is odd"를 출력하는 코드를 작성해 보세요.
    제한사항
        - 1 ≤ n ≤ 1,000
    입출력 예
    입력 #1
        100
    출력 #1
        100 is even
    입력 #2
        1
    출력 #2
        1 is odd
    */
    // println!("##### 홀짝 구분하기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // let n = match args.trim().parse::<i32>() {
    //     Ok(n) => n,
    //     Err(_) => {
    //         println!("숫자가 아닙니다.");
    //         exit(0)
    //     }
    // };
    //
    // println!("출력 #");
    // if n % 2 == 0 {
    //     println!("{} is even", n);
    // } else {
    //     println!("{} is odd", n);
    // }

    /*
    문자열 겹쳐쓰기
    문제 설명
        문자열 my_string, overwrite_string과 정수 s가 주어집니다. 문자열 my_string의 인덱스 s부터 overwrite_string의 길이만큼을 문자열 overwrite_string으로 바꾼 문자열을 return 하는 solution 함수를 작성해 주세요.
    제한사항
        - my_string와 overwrite_string은 숫자와 알파벳으로 이루어져 있습니다.
        - 1 ≤ overwrite_string의 길이 ≤ my_string의 길이 ≤ 1,000
        - 0 ≤ s ≤ my_string의 길이 - overwrite_string의 길이
    입출력 예
        my_string           overwrite_string    s	result
        "He11oWor1d"	    "lloWorl"	        2	"HelloWorld"
        "Program29b8UYP"	"merS123"	        7	"ProgrammerS123"
    입출력 예 설명
    입출력 예 #1
        예제 1번의 my_string에서 인덱스 2부터 overwrite_string의 길이만큼에 해당하는 부분은 "11oWor1"이고 이를 "lloWorl"로 바꾼 "HelloWorld"를 return 합니다.
    입출력 예 #2
        예제 2번의 my_string에서 인덱스 7부터 overwrite_string의 길이만큼에 해당하는 부분은 "29b8UYP"이고 이를 "merS123"로 바꾼 "ProgrammerS123"를 return 합니다.
    */
    // println!("##### 문자열 겹쳐쓰기 #####");
    // let mut args = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut args).unwrap();
    //
    // let buffer: Vec<&str> = args.split_whitespace().collect();
    // let arg1 = buffer[0];
    // let arg2 = buffer[1];
    // let arg3 = buffer[2].parse::<usize>().unwrap();
    //
    // let mut my_string = match is_validation_mystring(arg1) {
    //     Ok(()) => arg1.to_string(),
    //     Err(err) => exit(0)
    // };
    // let overwrite_string = match is_validation_overwrite_string(arg1, arg2) {
    //     Ok(()) => arg2,
    //     Err(err) => exit(0)
    // };
    // let s = match is_validation_s(buffer[2].parse::<usize>().unwrap(), arg1.len() - arg2.len()) {
    //     Ok(()) => arg3,
    //     Err(err) => exit(0)
    // };
    //
    // my_string.replace_range(s..s + overwrite_string.len(), overwrite_string);
    // println!("출력 #");
    // println!("{}", my_string)

    /*
    문자열 섞기
    문제 설명
        길이가 같은 두 문자열 str1과 str2가 주어집니다.
        두 문자열의 각 문자가 앞에서부터 서로 번갈아가면서 한 번씩 등장하는 문자열을 만들어 return 하는 solution 함수를 완성해 주세요.
    제한사항
        - 1 ≤ str1의 길이 = str2의 길이 ≤ 10
        - str1과 str2는 알파벳 소문자로 이루어진 문자열입니다.
    입출력 예
        str1	str2	result
        "aaaaa"	"bbbbb"	"ababababab"
    */
    // println!("##### 문자열 섞기 #####");
    // let mut buffer = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut buffer).unwrap();
    //
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let str1 = match is_mix_str_verification(input[0]) {
    //     Ok(()) => input[0],
    //     Err(err) => panic!("{}", err)
    // };
    // let str2 = match is_mix_str_verification(input[1]) {
    //     Ok(()) => input[1],
    //     Err(err) => panic!("{}", err)
    // };
    //
    // println!("출력 #");
    // print_mix_str(str1, str2);

    /*
    문자 리스트를 문자열로 변환하기
    문제 설명
        문자들이 담겨있는 배열 arr가 주어집니다. arr의 원소들을 순서대로 이어 붙인 문자열을 return 하는 solution함수를 작성해 주세요.
    제한사항
        - 1 ≤ arr의 길이 ≤ 200
        - arr의 원소는 전부 알파벳 소문자로 이루어진 길이가 1인 문자열입니다.
    입출력 예
        arr             result
        ["a","b","c"]	"abc"
    */
    // println!("##### 문자 리스트를 문자열로 변환하기 #####");
    //
    // println!("입력 #");
    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer).unwrap();
    //
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let result = solution(input);
    // println!("출력 #");
    // println!("{}", result);

    /*
    문자열 곱하기
    문제 설명
        문자열 my_string과 정수 k가 주어질 때, my_string을 k번 반복한 문자열을 return 하는 solution 함수를 작성해 주세요.
    제한사항
        - 1 ≤ my_string의 길이 ≤ 100
        - my_string은 영소문자로만 이루어져 있습니다.
        - 1 ≤ k ≤ 100
    입출력 예
        my_string	k	result
        "string"	3	"stringstringstring"
        "love"	    10	"lovelovelovelovelovelovelovelovelovelove"
    입출력 예 설명
    입출력 예 #1
        예제 1번의 my_string은 "string"이고 이를 3번 반복한 문자열은 "stringstringstring"이므로 이를 return 합니다.
    입출력 예 #2
        예제 2번의 my_string은 "love"이고 이를 10번 반복한 문자열은 "lovelovelovelovelovelovelovelovelovelove"이므로 이를 return 합니다.
    */
    // println!("##### 문자열 곱하기 #####");
    // println!("입력 #");
    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer).unwrap();
    //
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let result = solution(input);
    // println!("출력 #");
    // println!("{result}");

    /*
    더 크게 합치기
    문제 설명
        연산 ⊕는 두 정수에 대한 연산으로 두 정수를 붙여서 쓴 값을 반환합니다. 예를 들면 다음과 같습니다.
        - 12 ⊕ 3 = 123
        - 3 ⊕ 12 = 312
        양의 정수 a와 b가 주어졌을 때, a ⊕ b와 b ⊕ a 중 더 큰 값을 return 하는 solution 함수를 완성해 주세요.
        단, a ⊕ b와 b ⊕ a가 같다면 a ⊕ b를 return 합니다.
    제한사항
        1 ≤ a, b < 10,000
    입출력 예
        a	b	result
        9	91	991
        89	8	898
    입출력 예 설명
    입출력 예 #1
        a ⊕ b = 991 이고, b ⊕ a = 919 입니다. 둘 중 더 큰 값은 991 이므로 991을 return 합니다.
    입출력 예 #2
        a ⊕ b = 898 이고, b ⊕ a = 889 입니다. 둘 중 더 큰 값은 898 이므로 898을 return 합니다.
    */
    // println!("##### 더 크게 합치기 #####");
    // let mut buffer = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut buffer).unwrap();
    //
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let a = input[0];
    // let b = input[1];
    //
    // let result = solution(&a, &b);
    // println!("출력 #");
    // println!("{result}");

    /*
    두 수의 연산값 비교하기
    문제 설명
    연산 ⊕는 두 정수에 대한 연산으로 두 정수를 붙여서 쓴 값을 반환합니다. 예를 들면 다음과 같습니다.
        12 ⊕ 3 = 123
        3 ⊕ 12 = 312
    양의 정수 a와 b가 주어졌을 때, a ⊕ b와 2 * a * b 중 더 큰 값을 return하는 solution 함수를 완성해 주세요.
    단, a ⊕ b와 2 * a * b가 같으면 a ⊕ b를 return 합니다.
    제한사항
        1 ≤ a, b < 10,000
    입출력 예
        a	b	result
        2	91	364
        91	2	912
    입출력 예 설명
    입출력 예 #1
        a ⊕ b = 291 이고, 2 * a * b = 364 입니다. 둘 중 더 큰 값은 364 이므로 364를 return 합니다.
    입출력 예 #2
        a ⊕ b = 912 이고, 2 * a * b = 364 입니다. 둘 중 더 큰 값은 912 이므로 912를 return 합니다.
    */
    // println!("##### 두 수의 연산값 비교하기 #####");
    // let mut buffer = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut buffer).unwrap();
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let a = input[0].parse::<i32>().unwrap();
    // let b = input[1].parse::<i32>().unwrap();
    //
    // let result = solution(&a, &b);
    // println!("결과 #");
    // println!("{result}");

    /*
    n의 배수
    문제 설명
        정수 num과 n이 매개 변수로 주어질 때, num이 n의 배수이면 1을 return n의 배수가 아니라면 0을 return하도록 solution 함수를 완성해주세요.
    제한사항
        2 ≤ num ≤ 100
        2 ≤ n ≤ 9
    입출력 예
        num	n	result
        98	2	1
        34	3	0
    입출력 예 설명
    입출력 예 #1
        98은 2의 배수이므로 1을 return합니다.
    입출력 예 #2
        32는 3의 배수가 아니므로 0을 return합니다.
    */
    // println!("##### n의 배수 #####");
    // let mut buffer = String::new();
    //
    // println!("입력 #");
    // io::stdin().read_line(&mut buffer).unwrap();
    // let input: Vec<&str> = buffer.split_whitespace().collect();
    // let num = input[0].parse::<i32>().unwrap();
    // let n = input[1].parse::<i32>().unwrap();
    //
    // let result = solution(&num, &n);
    // println!("출력 #");
    // println!("{result}");

    /*
    공배수
    문제 설명
        정수 number와 n, m이 주어집니다. number가 n의 배수이면서 m의 배수이면 1을 아니라면 0을 return하도록 solution 함수를 완성해주세요.
    제한사항
        10 ≤ number ≤ 100
        2 ≤ n, m < 10
    입출력 예
        number	n	m	result
            60	2	3	    1
            55	10	5	    0
    입출력 예 설명
    입출력 예 #1
        60은 2의 배수이면서 3의 배수이기 때문에 1을 return합니다.
    입출력 예 #2
        55는 5의 배수이지만 10의 배수가 아니기 때문에 0을 return합니다.
    */
    println!("##### 공배수 #####");
    let mut buffer = String::new();

    println!("입력 #");
    io::stdin().read_line(&mut buffer).unwrap();
    let input: Vec<&str> = buffer.split_whitespace().collect();

    let number: i32 = input[0].parse::<i32>().unwrap();
    let n: i32 = input[0].parse::<i32>().unwrap();
    let m: i32 = input[0].parse::<i32>().unwrap();

    let result = solution(&number, &n, &m);
    println!("출력 #");
    println!("{result}");
}

fn solution(number: &i32, n: &i32, m: &i32) -> i32 {
    if number % n == 0 && number % m == 0 {
        1
    } else {
        0
    }
}






























