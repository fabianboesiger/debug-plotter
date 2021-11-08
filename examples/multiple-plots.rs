fn main() {
    for i in 0usize..1000usize {
        debug_plotter::plot!(i; caption = "Plot 1");
        debug_plotter::plot!(i; caption = "Plot 2");
    }
}
