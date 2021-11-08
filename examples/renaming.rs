fn main() {
    for x in 0usize..20usize {
        debug_plotter::plot!(
            x => "x",
            (x as f32).sin() => "sin(x)";
            caption = "Renaming"
        );
    }
}
