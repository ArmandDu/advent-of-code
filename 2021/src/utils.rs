pub fn neighbors(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    [
        (y > 0).then(|| (x, y - 1)),
        (x > 0).then(|| (x - 1, y)),
        (x + 1 < width).then(|| (x + 1, y)),
        (y + 1 < height).then(|| (x, y + 1)),
    ]
    .iter()
    .filter_map(|&c| c)
    .collect()
}
