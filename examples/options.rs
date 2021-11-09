fn main() {
    for i in 0usize..1000usize {
        debug_plotter::plot!(
            i where
            caption = "Options",
            size = (400, 300),
            path = "plots/Options.jpg",
            x_desc = "X Description",
            y_desc = "Y Description",
            x_range = 0f64..500f64,
            y_range = 0f64..500f64
        );
    }
}
