use std::time::Duration;
use std::thread;
use rand::Rng;

const N: usize = 20;

fn gen_random_cell() -> [[u8;N];N] {
    let mut rng = rand::thread_rng();
    let mut cell:[[u8;N];N] = [[0;N];N];
    for i in 0..N {
        for j in 0..N{
            cell[i][j] = rng.gen();
            cell[i][j] %= 2;
        }
    }
    return cell;
}
fn print_cell(cell:&[[u8;N];N]) {
    print!("{}[2J", 27 as char);
    for i in cell.iter() {
        for j in i.iter() {
            print!("{}", if *j==1 {"■"}else{"□"});
        }
        print!("\n");
    }
}

fn cell_update(_cell:&[[u8;N];N]) -> [[u8;N];N] {
    let mut rng = rand::thread_rng();
    let mut cell:[[u8;N];N] = [[0;N];N];
    for i in 0..N {
        for j in 0..N{
            //count 
            let mut count = 0;
            let i_p = if i+1 == N { 0 } else { i + 1 };
            let j_p = if j+1 == N { 0 } else { j + 1 };
            let i_m = if i   == 0 {N-1} else { i - 1 };
            let j_m = if j   == 0 {N-1} else { j - 1 };

            count += _cell[i][j_p] + _cell[i_p][j];
            count += _cell[i][j_m] + _cell[i_m][j];
            count += _cell[i_p][j_p] + _cell[i_m][j_m];
            count += _cell[i_p][j_m] + _cell[i_m][j_p];
            // update
            cell[i][j] = if count == 3 {1} else {0} ;
            if _cell[i][j] == 1 && count == 2{
                cell[i][j] = 1;
            } else {
                let r:u16 = rng.gen();
                cell[i][j] = if r < 16 {1} else {cell[i][j]};
            }
        }
    }
    
    return cell;
}

fn main() {
    let mut a = gen_random_cell();
    loop{
        a = cell_update(&a);
        print_cell(&a);
        thread::sleep(Duration::from_millis(500));
    }

}
