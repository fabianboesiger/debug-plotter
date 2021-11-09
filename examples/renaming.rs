fn main() {
    for a in 0usize..10usize {
        let b = (a as f32 / 2.0).sin() * 10.0;
        let c = 5 - (a as i32);

        debug_plotter::plot!(a as "Alice", b as "Bob", c as "Charlie" where caption = "Renaming");
    }
}
