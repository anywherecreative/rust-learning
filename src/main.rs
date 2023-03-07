fn main() {
    let str = String::from("first, last, email, phone\njeff,manning,555-555-1234,test@example.com");

    for line in str.split("\n") {
        for col in line.split(",") {
            print!("{}, ", col.trim())
        }
        println!("");
    }
}