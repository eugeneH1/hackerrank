fn birthday_cake_candles(candles: &[i32]) -> i32 {
    candles.iter().max().copied().map_or(0, |tallest| {
        candles.iter().filter(|&&x| x == tallest).count() as i32
    })
}

fn main() {
    birthday_cake_candles(&[3, 2, 1, 3]);
}
