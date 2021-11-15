# Debug Plotter

This crate provides a convenient macro to quickly plot variables.

## Documentation

For more information on how to use this crate, please take a look at the [documentation](https://docs.rs/debug_plotter/0.2.0/debug_plotter/) or the [examples](https://github.com/fabianboesiger/debug-plotter/tree/main/examples).

## Usage
### An Example

In this example, we quickly want to plot the variables `a`, `b`, and `c`.
Optionally, we can name the plot. Plots are saved as a PNG image in the
`plots` directory that is created in the working directory.

```rust
fn main() {
    for a in 0usize..10usize {
        let b = (a as f32 / 2.0).sin() * 10.0;
        let c = 5 - (a as i32);

        debug_plotter::plot!(a, b, c where caption = "My Plot");
    }
}
```

The example above generates a plot named "My Plot" and
saves it to 'plots/My_Plot.png`.

![Basic PLot](plots/My_Plot.png)

### Additional Use Cases

The macro takes a list of variables. By default, the value of the variable
is mapped to the y axis, and the x axis shows the iteration number.

```rust
debug_plotter::plot!(a, b, c);
```

It is possible to pass a tuple if you want the x axis to be another value
instead of the iteration number.

```rust
debug_plotter::plot!((x, a), (x, b), (x, c));
```

It is also possible to rename variables in the legend using the keyword `as`.

```rust
debug_plotter::plot!(a as "Alice", b as "Bob", c as "Charlie");
```

It is possible to provide additional options for the plot
after a `where` keyword.

```rust
debug_plotter::plot!(a, b, c where caption = "My Caption");
```

The following table lists all available options.

|Identifier|Example Value|Description|
|---|---|---|
|`caption`|`"caption"`|Sets the caption of the plot.|
|`size`|`(400, 300)`|Sets the size of the resulting image or window.|
|`x_desc`|`"x description"`|Sets the description of the x axis.|
|`y_desc`|`"y description"`|Sets the description of the y axis.|
|`path`|`"/plots/my_plot.jpg"`|Defines where the plot is saved.|
|`x_range`|`0f64..100f64`|Defines start and end of the x axis.|
|`y_range`|`0f64..100f64`|Defines start and end of the y axis.|
|`values`|`1000usize`|Defines the maximal number of values that are stored. If the macro is called more times than this number, the oldest value is dropped.|
|`live`|`true`|Enables live mode which opens the plot in a window with live updates.|

### Features Overview

The following table lists all available features.

|Feature|Description|
|---|---|
|`debug`|Enabled by default, disable to avoid compiling the dependencies in release mode.|
|`plot-release`|Disabled by default, enable to start plotting in release mode.|
|`live`|Enabled by default, enables live debugging by opening a window when passing `live = true` to the `plot!` macro.|

## Debug and Release Mode

The `plot!` macro generates plots in debug and release mode.
If you want to avoid generating plots in release mode, use `debug_plot!` instead.