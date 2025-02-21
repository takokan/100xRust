fn main() {
    let name = String::from("hello sir");
    let ans = first_word(name);
    println!("ans is {}", ans);

    let new = "hello";
}

fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }

    return ans;
}