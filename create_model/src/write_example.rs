use std::{fs::File, io::Write};

pub fn write(mut file: File, name: String, type_name: String) {
    let data = format!(
        r#"
#include<slowmokit/methods/{type_name}/{name}.h>

int main()
{{
    
    // add example usage here
    return 0;
}}
"#
    );
    file.write_all(data.as_bytes())
        .expect("Error writing example file. Aborted");

    println!("Example file added!")
}
