//! This crate provides a convenient macro to quickly plot variables.
//!
//! When running in release mode, no plots are generated.
//! To disable compilation of dependencies in release mode,
//! pass `--no-default-features` to this crate.

#[deny(unsafe_code)]

/// This macro is used to quickly generate plots for a list of variables.
/// All types that implement `num_traits::cast::ToPrimitive` can be plotted.
///
/// The macro takes a list of variables. By default, the value of the variable
/// is mapped to the y axis, and the x axis shows the iteration number.
///
/// ```rust
/// debug_plotter::plot!(a, b, c);
/// ```
/// 
/// It is possible to pass a tuple if you want the x axis to be another value
/// instead of the iteration number.
/// 
/// ```rust
/// debug_plotter::plot!((x, a), (x, b), (x, c));
/// ```
///
/// It is also possible to rename variables in the legend using the keyword `as`.
///
/// ```rust
/// debug_plotter::plot!(a as "Alice", b as "Bob", c as "Charlie");
/// ```
///
/// It is possible to provide additional options for the plot
/// after a `where` keyword.
///
/// ```rust
/// debug_plotter::plot!(a, b, c where caption = "My Caption");
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
/// |`path`|`"/plots/my_plot.jpg"`|Defines where the plot is saved.|
/// |`x_range`|`0f64..100f64`|Defines start and end of the x axis.|
/// |`y_range`|`0f64..100f64`|Defines start and end of the y axis.|
#[macro_export]
macro_rules! plot {
    (
        $( $( $variable:ident )? $( ( $x:ident, $y:ident ) )? $( as $name:literal )? ),* $(,)?
        $( where $($key:ident = $value:expr),* $(,)? )?
    ) => {
        #[cfg(any(debug_assertions, feature = "plot-release"))]
        #[cfg(feature = "debug")]
        {
            use std::cell::RefCell;
            use $crate::{PLOTS, Plottable, Plot, Options, Location};


            let location = Location {
                file: file!(),
                line: line!(),
                column: column!(),
            };

            PLOTS.with(|plots| {
                let mut map = plots
                    .plots
                    .borrow_mut();

                let plot = map
                    .entry(location.clone())
                    .or_insert_with(|| {
                        let names = [$({
                            $(
                                let name = stringify!($variable);
                            )?
                            $(
                                let name = stringify!($y);
                            )?
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

                let iteration = plot.iteration();

                plot.insert([$({
                    $(
                        let value = (iteration.to_plot_type(), $variable.to_plot_type());
                    )?
                    $(
                        let value = ($x.to_plot_type(), $y.to_plot_type());
                    )?
                    value
                }),*]);
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
    use std::{cell::RefCell, collections::HashMap, fmt, ops::Range};

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
        values: Vec<Vec<(PlotType, PlotType)>>,
        names: Vec<Name>,
        location: Location,
        options: Options,
        iteration: u64,
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
                iteration: 0,
            }
        }

        // Get current number of iteration.
        pub fn iteration(&self) -> u64 {
            self.iteration
        }

        // Insert new values into the plot.
        pub fn insert<const N: usize>(&mut self, values: [(PlotType, PlotType); N]) {
            for (i, &value) in values.iter().enumerate() {
                self.values[i].push(value)
            }
            self.iteration += 1;
        }

        fn x_min(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter().map(|(x, _)| x))
                .flatten()
                .fold(PlotType::MAX, |acc, &val| if val < acc { val } else { acc })
        }

        fn x_max(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter().map(|(x, _)| x))
                .flatten()
                .fold(PlotType::MIN, |acc, &val| if val > acc { val } else { acc })
        }

        fn y_min(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter().map(|(_, y)| y))
                .flatten()
                .fold(PlotType::MAX, |acc, &val| if val < acc { val } else { acc })
        }

        fn y_max(&self) -> PlotType {
            self.values
                .iter()
                .map(|values| values.iter().map(|(_, y)| y))
                .flatten()
                .fold(PlotType::MIN, |acc, &val| if val > acc { val } else { acc })
        }

        // Generates and saves the plot.
        fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
            let default_caption = &format!("{}", self.location);
            let caption = self.options.caption.as_ref().unwrap_or(default_caption);
            let default_path =
                &format!("plots/{}.png", caption.replace("/", "-").replace(" ", "_"));
            let path = self.options.path.as_ref().unwrap_or(default_path);
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
                .build_cartesian_2d(
                    self.options.x_range.clone().unwrap_or(self.x_min()..self.x_max()), 
                    self.options.y_range.clone().unwrap_or(self.y_min()..self.y_max()),
                )?;

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
                    .draw_series(LineSeries::new(values.iter().copied(), &color))?
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
        pub path: Option<String>,
        pub x_range: Option<Range<PlotType>>,
        pub y_range: Option<Range<PlotType>>,
    }
}
