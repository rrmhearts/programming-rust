// use std::array;

fn update_2d_array(array: &mut [[u32; 3]; 3]) {
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            array[i][j] = 100;
        }
    }
}

fn update_2d_array_with_asmut<V: AsMut<[u32]>>(array: &mut [V]) {
    for i in 0..array.len() {
        for j in 0..array[i].as_mut().len() {
            array[i].as_mut()[j] = 200;
        }
    }
}

fn update_2d_array_const_generic<const N: usize>(mat: &mut [[u32; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            mat[i][j] = 300;
        }
    }
}

fn main() {
    let mut array_2d = [[0; 3]; 3];
    println!("Before: {:?}", array_2d);

    update_2d_array(&mut array_2d);
    println!("After: {:?}", array_2d);

    update_2d_array_with_asmut(&mut array_2d);
    println!("After AsMut: {:?}", array_2d);

    let mut array_2d_l = [[0; 10]; 10];
    update_2d_array_with_asmut(&mut array_2d_l);
    println!("After AsMut 10: {:?}", array_2d_l);

    update_2d_array_const_generic(&mut array_2d_l);
    println!("After const generic: {:?}", array_2d_l);
}
