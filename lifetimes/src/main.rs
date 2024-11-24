// v should have at least one element
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

fn main() {
    println!("Borrows must outlive origin!");
    let s;
    // {
    let parabola=[9,4,1,0,1,4,9];
    // Borrowed value must outlive parabola
    s = smallest(&parabola);
    // }
    assert_eq!(*s, 0);

    println!("References must outlive structs!");
    struct S<'a> {
        r: &'a i32
    }

    let s;
    // {
    let x = 10;
    // Borrowed value does not life long enough! Needs to be in same scope.
    s = S { r: &x };
    // }
    assert_eq!(*s.r, 10);

    // struct T<'a> {
    struct T<'a, 'b> {
        x: &'a i32,
        y: &'b i32
    }
    let r;
    let yr;
    {
        let y = 20;
        {
            // &y borrowed value does not live long enough!
            // because y and x have the same lifetime!
            let s = T { x: &x, y: &y };
            r = s.x;
            yr = s.y;
        }
        println!("y: {} dies here.", yr);
    }
    println!("{}", r);

    // Lifetimes assumed based on definitions above.
    // fn sum_r_xy(r: &i32, s:T) -> i32 {
    //     r + s.x + s.y
    // }

    struct StringTable {
        elements: Vec<String>,
    }

    impl StringTable {
        // fn find_by_prefix<'a, 'b>(&'a &self, prefix: &'b str) -> Option<&'a String>
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0 .. self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    let table = StringTable {
        elements: vec![
            "aardvark".to_string(), 
            "anaerobic".to_string(), 
            "beta".to_string(), 
            "charis".to_string(), 
            "dracula".to_string(), 
            "energy".to_string()]
    };

    let answer = match table.find_by_prefix("an") {
        Some(value) => value,
        None => "None",
    };
    println!("{}", answer);
}
