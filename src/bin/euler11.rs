//! * `W` is a grid width
//! * `H` is a grid height
//! * `j` is a column index

fn horizontal_products<const W: usize, const H: usize>(
    rows: &[[u8; W]; H],
    len: usize,
) -> impl Iterator<Item = u32> + '_ {
    rows.iter().flat_map(move |row| {
        row.windows(len).map(|win| {
            let window_values = win.iter().map(|&byte| byte as u32);
            window_values.product()
        })
    })
}

fn vertical_products<const W: usize, const H: usize>(
    rows: &[[u8; W]; H],
    len: usize,
) -> impl Iterator<Item = u32> + '_ {
    (0..W).flat_map(move |j| {
        (0..=(H - len)).map(move |i| {
            let window_rows = &rows[i..(i + len)];
            let window_values = window_rows.iter().map(|row| row[j] as u32);
            window_values.product()
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_GRID: [[u8; 4]; 4] = [
        [26, 38, 40, 67],
        [95, 63, 94, 39],
        [97, 17, 78, 78],
        [20, 45, 35, 14],
    ];

    #[test]
    fn test_horizontal_products() {
        assert_eq!(horizontal_products(&TEST_GRID, 4).max(), Some(21941010));
    }

    #[test]
    fn test_vertical_products() {
        assert_eq!(vertical_products(&TEST_GRID, 4).max(), Some(10264800));
    }
}

fn main() {
    //println!("{}", euler());

    const TEST_GRID: [[u32; 4]; 4] = [
        [26, 38, 40, 67],
        [95, 63, 94, 39],
        [97, 17, 78, 78],
        [20, 45, 35, 14],
    ];

    dbg!(26 * 95 * 97 * 20);
    dbg!(38 * 63 * 17 * 45);
    dbg!(40 * 94 * 78 * 35);
    dbg!(67 * 39 * 78 * 14);
}
