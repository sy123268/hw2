struct Buffer<T> {
    data: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Copy> Buffer<T> {
    fn sum(&self) -> T {
        let mut total = self.data[0];
        for i in 1..self.data.len() {
            total = total + self.data[i];
        }
        total
    }
}

fn compare_string(x: &str, y: &str) -> bool {
    let x: Vec<char> = x.chars().collect();
    let y: Vec<char> = y.chars().collect();
    let len = if x.len() < y.len() {
        x.len()
    } else {
        y.len()
    };
    for i in 0..len {
        if x[i] < y[i] {
            return true;
        } else if x[i] > y[i] {
            return false;
        }
    }
    true
}

fn main() {
    let buffer = Buffer { data: vec![1, 2, 3, 4, 5] };
    let sum = buffer.sum();
    println!("Sum: {}", sum);

    let x = "aabb";
    let y = "aacc";
    let result = compare_string(x, y);

    if result {
        println!("x > y");
    } else {
        println!("x <= y");
    }

    let one: Vec<char> = vec!['a', 'b', 'c', 'd', 'e']; 
    let two: Vec<char> = one
        .iter()
        .map(|&c| {
            if c == 'e' {
                'f'
            } else {
                (c as u8 + 1) as char
            }
        })
        .collect();
    println!("{:?}", two);

}