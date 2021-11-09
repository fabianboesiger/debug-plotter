fn main() {
    let mut i = 0;
    loop {
        let x = i as f64 / 100.0 * std::f64::consts::PI * 2.0;
        let sin_x = x.sin();
        let cos_x = x.cos();

        debug_plotter::plot!(
            (x, sin_x) as "sin(x)",
            (x, cos_x) as "cos(x)"
            where
            caption = "Live Trigonometry",
            x_desc = "x",
            values = 100usize,
            live = true,
            size = (1080, 720)
        );

        i += 1;
    }
}
