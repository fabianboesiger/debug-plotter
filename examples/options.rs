fn main() {
    for i in 0usize..1000usize {
        debug_plotter::plot!(
            i where
            caption = "Options",
            size = (400, 300),
            x_desc = "X Description",
            y_desc = "Y Description",
            path = "plots/Options.jpg"
        );
    }
}
