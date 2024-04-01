use std::thread;
use std::time::Duration;

#[allow(non_snake_case)]
fn main() {
    let mut A: f32 = 0.0;
    let mut B: f32 = 0.0;
    let mut z = vec![0.0; 7040];
    let mut b = vec![' '; 1760];

    print!("\x1b[2J");

    loop {
        z.fill(0.0);
        b.fill(' ');

        let mut j: f32 = 0.0;
        while j < 6.28 {
            
            let mut i: f32 = 0.0;
            while i < 6.28 {
                let c: f32 = i.sin();
                let d: f32 = j.cos();
                let e: f32 = A.sin();
                let f: f32 = j.sin();
                let g: f32 = A.cos();

                let h: f32 = d + 2.0;
                let D: f32 = 1.0 / (c * h * e + f * g + 5.0);

                let l: f32 = i.cos();
                let m: f32 = B.cos();
                let n: f32 = B.sin();

                let t: f32 = c * h * g - f * e;
                let x: i32 = (40.0 + 30.0 * D * (l * h * m - t * n)) as i32;
                let y: i32 = (12.0 + 15.0 * D * (l * h * n + t * m)) as i32;
                let o: usize = (x + 80 * y) as usize;
                let N: i32 = (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as i32;

                if y < 22 && y >= 0 && x >= 0 && x < 80 && D > z[o] {
                    z[o] = D;
                    let idx: usize = if N > 0 { N as usize } else { 0 };
                    b[o] = ".,-~:;=!*#$@".chars().nth(idx).unwrap_or_default();
                }

                i += 0.02;
            }

            j += 0.07;
        }

        print!("\x1b[H");

        for k in 0..1761 {
            print!("{}", if k % 80 != 0 { b[k] } else { '\n' });
            A += 0.00004;
            B += 0.00002;
        }

        thread::sleep(Duration::from_millis(30));
    }
}

