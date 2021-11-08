#[macro_export]
macro_rules! plot {
    ($($name:expr;)? $($variable:ident),*) => {
        #[cfg(debug_assertions)]
        #[cfg(feature = "debug")]
        {
            use once_cell::unsync::Lazy;
            use std::cell::RefCell;
            use $crate::Plottable;

            thread_local! {
                static PLOT: Lazy<RefCell<$crate::Plot>> = Lazy::new(|| {
                    let mut name: Option<String> = None;
                    $(
                        name = Some(format!("{}", $name));
                    )?
                    RefCell::new($crate::Plot::new([$(stringify!($variable)),*], (file!(), line!()), name))
                });
            }
            PLOT.with(|plot| {
                plot.borrow_mut().insert([$($variable.to_plot_type()),*]);
            })
        }
    };
}

#[cfg(feature = "debug")]
pub use debug::*;

#[cfg(feature = "debug")]
mod debug {
    use num_traits::cast::ToPrimitive;
    use plotters::prelude::*;

    pub type PlotType = f64;

    pub trait Plottable {
        fn to_plot_type(self) -> PlotType;
    }

    impl<T> Plottable for T
    where
        T: ToPrimitive,
    {
        fn to_plot_type(self) -> PlotType {
            self.to_f64().unwrap()
        }
    }

    type Location = (&'static str, u32);
    type Name = &'static str;

    pub struct Plot {
        values: Vec<Vec<PlotType>>,
        names: Vec<Name>,
        location: Location,
        name: Option<String>,
    }

    impl Plot {
        pub fn new<const N: usize>(
            names: [&'static str; N],
            location: Location,
            name: Option<String>,
        ) -> Plot {
            Plot {
                values: vec![Vec::new(); N],
                names: names.to_vec(),
                location,
                name,
            }
        }

        pub fn insert<const N: usize>(&mut self, values: [PlotType; N]) {
            for (i, &value) in values.iter().enumerate() {
                self.values[i].push(value.to_plot_type())
            }
        }

        fn x_min(&self) -> usize {
            0
        }

        fn x_max(&self) -> usize {
            self.values
                .iter()
                .map(|values| values.len() - 1)
                .max()
                .unwrap_or(0)
        }

        fn y_min(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter())
                .flatten()
                .fold(PlotType::MAX, |acc, &val| if val < acc { val } else { acc })
        }

        fn y_max(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter())
                .flatten()
                .fold(PlotType::MIN, |acc, &val| if val > acc { val } else { acc })
        }

        fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
            let default_caption = &format!("{}:{}", self.location.0, self.location.1);
            let caption = self.name.as_ref().unwrap_or(default_caption);
            let path = format!(".plots/{}.png", caption.replace("/", "-").replace(" ", "_"));
            let path = std::path::Path::new(&path);
            println!("Saving plot \"{}\" to {:?}", caption, path);
            std::fs::create_dir_all(&path.parent().unwrap()).unwrap();

            let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption(caption, ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(self.x_min()..self.x_max(), self.y_min()..self.y_max())?;

            chart.configure_mesh().draw()?;

            for (i, (&name, values)) in self.names.iter().zip(self.values.iter()).enumerate() {
                let color = HSLColor(i as f64 / self.names.len() as f64, 1.0, 0.5);

                chart
                    .draw_series(LineSeries::new(
                        values.iter().map(|&v| v).enumerate(),
                        &color,
                    ))?
                    .label(name)
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
            }

            chart
                .configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw()?;

            Ok(())
        }
    }

    impl Drop for Plot {
        fn drop(&mut self) {
            self.plot().unwrap()
        }
    }
}
