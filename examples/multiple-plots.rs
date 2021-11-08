use std::cell::RefCell;

use plotters::prelude::*;

fn main() {
    {
        thread_local! {
            static TEST: RefCell<Test> = RefCell::new(Test());
        }
        TEST.with(|_| {});
    }
    {
        thread_local! {
            static TEST: RefCell<Test> = RefCell::new(Test());
        }
        TEST.with(|_| {});
    }
    
}

struct Test();

impl Drop for Test {
    fn drop(&mut self) {
        let root = BitMapBackend::new("test.png", (100, 100))
                .into_drawing_area();

        let mut chart = ChartBuilder::on(&root)
            .caption("Test", 30)
            .build_cartesian_2d(0..0, 0..0)
            .unwrap();

        chart
            .configure_series_labels()
            .draw()
            .unwrap();
    }
}
