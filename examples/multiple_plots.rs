fn main() {
    for a in 0usize..1000usize {
        let b = a * 2;
        debug_plotter::plot!(a where caption = "Plot 1");
        debug_plotter::plot!(b where caption = "Plot 2");
    }
}
