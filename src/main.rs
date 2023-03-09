fn main() {
    println!("{:?}", get_col_val(1));
    println!("{:?}", get_col_val(2));
}

fn get_col_val(col: i32 ) -> Option<String> {
    if col == 1 {
        Some("Jeff".to_string())
    } else {
        None
    }
}