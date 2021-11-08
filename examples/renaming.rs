fn main() {
    for x in 0usize..20usize {
        let sin_x = (x as f32).sin();
        debug_plotter::plot!(
            x as "x",
            sin_x as "sin(x)" where
            caption = "Renaming"
        );
    }
}
