use plotters::prelude::*;

pub fn draw_histogram(path: &str, data: &[f64]) -> anyhow::Result<()> {
    let root = BitMapBackend::new(path, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);

    let bins = 30;
    let step = (max - min) / bins as f64;

    let mut freq = vec![0; bins];
    for &v in data {
        let mut idx = ((v - min) / step) as usize;
        if idx >= bins {
            idx = bins - 1;
        }
        freq[idx] += 1;
    }

    let max_freq = *freq.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram of Terminal Prices", ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min..max, 0..max_freq)?;

    chart
        .configure_mesh()
        .disable_mesh() // Tắt lưới
        .x_labels(10)
        .y_labels(10)
        .x_desc("Portfolio")
        .y_desc("Frequency")
        .label_style(("sans-serif", 14))
        .axis_desc_style(("sans-serif", 16))
        .x_label_formatter(&|v| format!("t{}", v))
        .y_label_formatter(&|v| format!("{:.2}", v))
        .bold_line_style(BLACK.stroke_width(2)) // Làm đậm trục chính
        .light_line_style(TRANSPARENT) // Ẩn các đường lưới phụ
        .draw()?;

    chart.draw_series(freq.iter().enumerate().map(|(i, &count)| {
        let x0 = min + i as f64 * step;
        let x1 = x0 + step;
        Rectangle::new([(x0, 0), (x1, count)], BLUE.mix(0.5).filled())
    }))?;

    root.present()?;
    Ok(())
}
