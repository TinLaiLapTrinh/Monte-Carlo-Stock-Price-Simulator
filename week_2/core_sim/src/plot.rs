use plotters::prelude::*;

pub fn draw_paths(path: &str, paths: &[Vec<f64>]) -> anyhow::Result<()> {
    let root = BitMapBackend::new(path, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = paths.iter().flatten().cloned().fold(f64::MIN, f64::max);
    let min_y = paths.iter().flatten().cloned().fold(f64::MAX, f64::min);

    let days = paths[0].len();

    let mut chart = ChartBuilder::on(&root)
        .caption("Monte Carlo GBM Paths", ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..days, min_y..max_y)?;

    // ==============================
    // CONFIGURE MESH - CHỈ VẼ 2 TRỤC X VÀ Y
    // ==============================
    chart
        .configure_mesh()
        .disable_mesh()  // Tắt lưới
        .x_labels(10)
        .y_labels(10)
        .x_desc("Time Steps")
        .y_desc("Price ($)")
        .label_style(("sans-serif", 14))
        .axis_desc_style(("sans-serif", 16))
        .x_label_formatter(&|v| format!("t{}", v))
        .y_label_formatter(&|v| format!("{:.2}", v))
        .bold_line_style(BLACK.stroke_width(2))  // Làm đậm trục chính
        .light_line_style(TRANSPARENT)  // Ẩn các đường lưới phụ
        .draw()?;

    // ==============================
    // VẼ MŨI TÊN CHO CÁC TRỤC
    // ==============================
    let plotting_area = chart.plotting_area();
    
    // Lấy tọa độ pixel của vùng vẽ
    let coord_spec = plotting_area.get_pixel_range();
    let x_range = coord_spec.0;
    let y_range = coord_spec.1;
    
    let x_start = x_range.start;
    let x_end = x_range.end;
    let y_start = y_range.start;
    let y_end = y_range.end;
    
    let arrow_size = 8;

    // Vẽ mũi tên trục X (phía phải)
    root.draw(&PathElement::new(
        vec![
            (x_end - arrow_size, y_end - arrow_size),
            (x_end, y_end),
            (x_end - arrow_size, y_end + arrow_size),
        ],
        BLACK.filled(),
    ))?;

    // Vẽ mũi tên trục Y (phía trên)
    root.draw(&PathElement::new(
        vec![
            (x_start - arrow_size, y_start + arrow_size),
            (x_start, y_start),
            (x_start + arrow_size, y_start + arrow_size),
        ],
        BLACK.filled(),
    ))?;

    // ==============================
    // DRAW PATHS
    // ==============================
    for p in paths {
        let series: Vec<(usize, f64)> = 
            p.iter().enumerate().map(|(i, &v)| (i, v)).collect();

        chart.draw_series(LineSeries::new(series, &BLUE.mix(0.3)))?;
    }

    root.present()?;
    Ok(())
}