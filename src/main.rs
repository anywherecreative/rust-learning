fn main() {
    for a in 0..5 {
        let val = get_col_val(a);
        if val.is_some()  {
            println!("{} {}",a,val.unwrap())
        } else {
            println!("{}",a)
        }
    }
}

fn get_col_val(col: i32 ) -> Option<String> {
    if col == 1 {
        Some("Jeff".to_string())
    } else {
        None
    }
}