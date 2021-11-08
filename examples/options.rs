fn main() {
    for i in 0usize..1000usize {
        debug_plotter::plot!(
            i;
            caption = "Options Testing",
            size = (400, 300),
            x_desc = "X Description",
            y_desc = "Y Description",
        );
    }
}
