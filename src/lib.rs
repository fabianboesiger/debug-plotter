//! This crate provides a convenient macro to quickly plot variables.

#[deny(unsafe_code)]

/// This macro is used to quickly generate plots for a list of variables.
/// All types that implement `num_traits::cast::ToPrimitive` can be plotted.
///
/// The macro takes a list of variables.
///
/// ```rust
/// debug_plotter::plot(a, b, c);
/// ```
///
/// Optionally, you can provide a caption and other options for the plot
/// after a semicolon.
/// If no name is provided, the name defaults to the file
/// and the line number of where the macro was called.
///
/// ```rust
/// debug_plotter::plot(a, b, c; caption = "My Caption");
/// ```
///
/// The following table lists all available options.
///
/// |Identifier|Example Value|Description|
/// |---|---|---|
/// |`caption`|`"caption"`|Sets the caption of the plot.|
/// |`size`|`(400, 300)`|Sets the size of the resulting image.|
/// |`x_desc`|`"x description"`|Sets the description of the x axis.|
/// |`y_desc`|`"y description"`|Sets the description of the y axis.|
///
/// When running in release mode, no plots are generated.
/// To disable compilation of dependencies in release mode,
/// pass `--no-default-features` to this crate.
#[macro_export]
macro_rules! plot {
    ( $($variable:expr $( => $name:literal )? ),* $(,)? $( ; $($key:ident = $value:expr),* $(,)? )?) => {
        #[cfg(debug_assertions)]
        #[cfg(feature = "debug")]
        {
            use std::cell::RefCell;
            use $crate::{PLOTS, Plottable, Plot, Options, Location};

            let values = [$($variable.to_plot_type()),*];

            let location = Location {
                file: file!(),
                line: line!(),
                column: column!(),
            };

            PLOTS.with(|plots| {
                let mut map = plots
                    .plots
                    .borrow_mut();
                
                let entry = map
                    .entry(location.clone())
                    .or_insert_with(|| {
                        let names = [$({
                            let name = stringify!($variable);
                            $(
                                let name = $name;
                            )?
                            name
                        }),*];

                        let options = Options {
                            $($(
                                $key: Some($value.into()),
                            )*)?
                            ..Default::default()
                        };

                        Plot::new(names, location, options)
                    });

                entry.insert([$($variable.to_plot_type()),*]);
            });
        }
    };
}

#[cfg(feature = "debug")]
pub use debug::*;

#[cfg(feature = "debug")]
mod debug {
    use num_traits::cast::ToPrimitive;
    use plotters::prelude::*;
    use std::{cell::RefCell, collections::HashMap, fmt};

    thread_local! {
        pub static PLOTS: Plots = Plots::new();
    }

    pub struct Plots {
        pub plots: RefCell<HashMap<Location, Plot>>,
    }

    // The plots are generated as soon as `Drop` is called.
    impl Drop for Plots {
        fn drop(&mut self) {
            for (_, plot) in self.plots.borrow().iter() {
                plot.plot().unwrap();
            }
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub struct Location {
        pub file: &'static str,
        pub line: u32,
        pub column: u32,
    }

    impl fmt::Display for Location {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}:{}:{}", self.file, self.line, self.column)
        }
    }

    impl Plots {
        fn new() -> Self {
            Plots {
                plots: RefCell::new(HashMap::new()),
            }
        }
    }

    type PlotType = f64;

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

    type Name = &'static str;

    pub struct Plot {
        values: Vec<Vec<PlotType>>,
        names: Vec<Name>,
        location: Location,
        options: Options,
    }

    impl Plot {
        pub fn new<const N: usize>(
            names: [&'static str; N],
            location: Location,
            options: Options,
        ) -> Plot {
            Plot {
                values: vec![Vec::new(); N],
                names: names.to_vec(),
                location,
                options,
            }
        }

        // Insert new values into the plot.
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

        // Generates and saves the plot as PNG.
        fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
            let default_caption = &format!("{}", self.location);
            let caption = self.options.caption.as_ref().unwrap_or(default_caption);
            let path = format!("plots/{}.png", caption.replace("/", "-").replace(" ", "_"));
            let path = std::path::Path::new(&path);
            println!("Saving plot \"{}\" to {:?}", caption, path);
            std::fs::create_dir_all(&path.parent().unwrap()).unwrap();

            let root = BitMapBackend::new(&path, self.options.size.unwrap_or((640, 480)))
                .into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption(caption, 30)
                .margin(30)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d(self.x_min()..self.x_max(), self.y_min()..self.y_max())?;

            let mut mesh = chart.configure_mesh();
            if let Some(x_desc) = &self.options.x_desc {
                mesh.x_desc(x_desc);
            }
            if let Some(y_desc) = &self.options.y_desc {
                mesh.y_desc(y_desc);
            }
            mesh.draw()?;

            for (i, (&name, values)) in self.names.iter().zip(self.values.iter()).enumerate() {
                let color = HSLColor(i as f64 / self.names.len() as f64, 1.0, 0.5);

                chart
                    .draw_series(LineSeries::new(
                        values.iter().copied().enumerate(),
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

    #[derive(Default)]
    pub struct Options {
        pub caption: Option<String>,
        pub size: Option<(u32, u32)>,
        pub x_desc: Option<String>,
        pub y_desc: Option<String>,
    }
}
