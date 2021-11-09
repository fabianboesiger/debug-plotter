fn main() {
    for x in 0usize..100usize {
        let x = x as f64 / 100.0 * std::f64::consts::PI * 2.0;
        let sin_x = x.sin();
        let cos_x = x.cos();

        debug_plotter::plot!((x, sin_x) as "sin(x)", (x, cos_x) as "cos(x)" where caption = "Trigonometry", x_desc = "x");
    }
}
