struct Q<const CAP: usize, T> {
    head: usize,
    tail: usize,
    q: [T; CAP],
}

fn main() {
    let mut q: Q<64, u32> = q::new();
}
