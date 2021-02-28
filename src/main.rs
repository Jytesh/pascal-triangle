fn main() {
    println!("Enter the number of lines!");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();
        if input.trim() == "--quit" {
            break
        };
        let input: usize = input.trim().parse().expect("EXPECTED INTEGER got something else!!");
        pascal(input);
    }
    println!("Thank you for using Pascal Generator!")
}

fn pascal(input: usize) {
    assert!(input < 20);
    assert!(input > 1);
    let mut lines : Vec<Vec<usize>> = Vec::with_capacity(input);
    lines.push(vec![1]);
    lines.push(vec![1,1]);
    for i in 2..input{
        let size = i + 1;
        let mut insert = Vec::with_capacity(size);
        insert.push(1);
        for j in 1..size-1 {
            insert.push(lines[i-1][j-1] + lines[i-1][j]);
        }
        insert.insert(size-1,1);
        lines.push(insert);
    }
    for i in 0..lines.len() {
        print!("{}", " ".repeat(lines.len() - i-1));
        for el in &lines[i] {
            print!("{} ",el);
        }
        print!("\n");
    }

}