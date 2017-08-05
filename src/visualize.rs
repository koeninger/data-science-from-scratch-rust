use rustplotlib::{Axes2D, Backend, Figure, FillBetween, Line2D, Scatter};
use rustplotlib::backend::Matplotlib;

use std::f64::consts::PI;

fn save_figure(fig: Figure, file: &str) {
    let mut mpl = Matplotlib::new().unwrap();
    mpl.set_style("ggplot").unwrap();

    fig.apply(&mut mpl).unwrap();

    mpl.savefig(file).unwrap();
    mpl.wait().unwrap();
}

fn make_readme_figure<'a>(x: &'a [f64], y1: &'a [f64], y2: &'a [f64]) -> Figure<'a> {

    let ax1 = Axes2D::new()
        .add(Scatter::new(r"$y_1 = \sin(x)$").data(x, y1).marker("o"))
        .add(
            Line2D::new(r"$y_2 = \cos(x)$")
                .data(x, y2)
                .color("red")
                .marker("x")
                .linestyle("--")
                .linewidth(1.0),
        )
        .xlabel("Time [sec]")
        .ylabel("Distance [mm]")
        .legend("lower right")
        .xlim(0.0, 8.0)
        .ylim(-2.0, 2.0);

    let ax2 = Axes2D::new()
        .add(FillBetween::new().data(x, y1, y2).interpolate(true))
        .xlim(0.0, 8.0)
        .ylim(-1.5, 1.5);

    Figure::new().subplots(2, 1, vec![Some(ax1), Some(ax2)])
}

fn readme() {
    let x: Vec<f64> = (0..40)
        .into_iter()
        .map(|i| (i as f64) * 0.08 * PI)
        .collect();
    let y1: Vec<f64> = x.iter().map(|x| x.sin()).collect();
    let y2: Vec<f64> = x.iter().map(|x| x.cos()).collect();

    let fig = make_readme_figure(&x, &y1, &y2);

    save_figure(fig, "/tmp/simple.png");
}

fn line() {
    let variance: Vec<f64> = vec![1., 2., 4., 8., 16., 32., 64., 128., 256.];
    let bias_squared: Vec<f64> = vec![256., 128., 64., 32., 16., 8., 4., 2., 1.];
    let total_error: Vec<f64> = variance
        .iter()
        .zip(bias_squared.iter())
        .map(|(x, y)| x + y)
        .collect();
    let xs: Vec<f64> = (0..variance.len()).map(|x| x as f64).collect();
    let ax = Axes2D::new()
        .add(
            Line2D::new(r"$variance$")
                .data(&xs, &variance)
                .color("green")
                .linestyle("--"),
        )
        .add(
            Line2D::new(r"$bias^2$")
                .data(&xs, &bias_squared)
                .color("red")
                .linestyle("-."),
        )
        .add(
            Line2D::new(r"$total error$")
                .data(&xs, &total_error)
                .color("blue")
                .linestyle(":"),
        )
        .legend("upper center")
        .xlabel("model complexity");

    let fig = Figure::new().subplots(1, 1, vec![Some(ax)]);
    save_figure(fig, "/tmp/line.png");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme() {
        readme();
    }
    #[test]
    fn test_line() {
        line();
    }
}
