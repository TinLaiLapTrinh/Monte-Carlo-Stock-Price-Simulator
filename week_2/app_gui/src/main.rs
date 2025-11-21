use core_sim::{gbm, plot, charts};
use plot::draw_paths;
use charts::draw_histogram;
use data_io::{loader, statistic, summary};
use slint::{ModelRc, SharedString, Image};
use std::path::Path;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    
    // === Load CSV callback ===
    {
        let ui_handle = ui_handle.clone();
        ui.on_request_load_csv(move || {
            if let Some(ui) = ui_handle.upgrade() {
                if let Ok(list) = loader::load_all_tickers("data.csv") {
                    let model: Vec<SharedString> = list.iter().map(|s| s.into()).collect();
                    ui.set_tickers(ModelRc::from(model.as_slice()));
                } else {
                    eprintln!("✗ Error loading CSV");
                }
                
                // match loader::load_all_tickers("data.csv") {
                //     Ok(list) => {
                //         let model: Vec<SharedString> = list
                //             .iter()
                //             .map(|s| SharedString::from(s.as_str()))
                //             .collect();

                //         ui.set_tickers(ModelRc::from(model.as_slice()));
                //         println!("✓ Loaded {} tickers", list.len());
                //     }
                //     Err(e) => eprintln!("✗ Error: {:?}", e),
                // }
            }
        });
    }

    // === Ticker selected callback ===
    {
        let ui_handle = ui_handle.clone();

        ui.on_ticker_selected(move |ticker| {
            println!("\n=== Processing: {} ===", ticker);
        
            let ticker = ticker.clone();
            let ui_handle_thread = ui_handle.clone();
        
            // Update UI immediately
            if let Some(ui) = ui_handle_thread.upgrade() {
                ui.set_selected_ticker(ticker.clone());
                ui.set_charts_visible(false);
            }
        
            std::thread::spawn(move || {
                // 1) Load CSV
                let records = match loader::load_csv_by_ticker("data.csv", &ticker) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("✗ CSV error: {:?}", e);
                        return;
                    }
                };
        
                let close_prices: Vec<f64> = records.iter().map(|r| r.close).collect();
                if close_prices.is_empty() {
                    eprintln!("✗ No data");
                    return;
                }
        
                // 2) Tính log-returns
                let returns = statistic::log_returns(&close_prices);
                let (mu, sigma) = statistic::mean_std(&returns);
                
                let initial_price = *close_prices.last().unwrap();
                let time_horizon = 1.0;
                let num_steps = 50;
                let num_simulations = 1000;
                let seed = 42;
        
                println!("  Initial Price: ${:.2}", initial_price);
                println!("  Drift (μ): {:.4}", mu);
                println!("  Volatility (σ): {:.4}", sigma);
        
                // 3) Monte Carlo GBM - Simulate paths
                let paths = gbm::simulate_gbm(
                    initial_price,
                    mu, 
                    sigma,
                    time_horizon,
                    num_steps,
                    num_simulations,
                    seed,
                );
        
                // 4) Lấy giá cuối + sample paths
                let terminal_prices: Vec<f64> = paths
                    .iter()
                    .map(|p| *p.last().unwrap())
                    .collect();
        
                let sample_paths: Vec<Vec<f64>> = paths
                    .iter()
                    .take(40)
                    .cloned()
                    .collect();
        
                // 5) Summary
                let sum = summary::summary(&terminal_prices);
        
                println!("  Mean terminal price: ${:.2}", sum.mean);
        
                // 6) Xuất biểu đồ ra file
                let paths_image_file = format!("paths_{}.png", ticker);
                let hist_image_file = format!("hist_{}.png", ticker);
        
                if let Err(e) = draw_paths(&paths_image_file, &sample_paths) {
                    eprintln!("✗ Error drawing paths: {:?}", e);
                    return;
                }
        
                if let Err(e) = draw_histogram(&hist_image_file, &terminal_prices) {
                    eprintln!("✗ Error drawing histogram: {:?}", e);
                    return;
                }
        
                println!("✓ Charts saved: {}, {}", paths_image_file, hist_image_file);
        
                // 7) Prepare data to move into event loop
                let mean = sum.mean as f32;
                let mu_f32 = mu as f32;
                let sigma_f32 = sigma as f32;
                let initial_f32 = initial_price as f32;
        
                // 8) Update UI with images and statistics
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_handle_thread.upgrade() {
                        // Load images INSIDE event loop (to avoid Send/Sync issues)
                        let paths_img = match Image::load_from_path(Path::new(&paths_image_file)) {
                            Ok(img) => img,
                            Err(e) => {
                                eprintln!("✗ Error loading paths image: {:?}", e);
                                return;
                            }
                        };
                        
                        let hist_img = match Image::load_from_path(Path::new(&hist_image_file)) {
                            Ok(img) => img,
                            Err(e) => {
                                eprintln!("✗ Error loading histogram image: {:?}", e);
                                return;
                            }
                        };
                        
                        // Set images
                        ui.set_paths_image(paths_img);
                        ui.set_histogram_image(hist_img);
                        
                        // Set statistics
                        ui.set_mean_price(mean);
                        ui.set_selected_ticker(ticker);
                        
                        // Set simulation parameters
                        ui.set_mu(mu_f32);
                        ui.set_sigma(sigma_f32);
                        ui.set_initial_price(initial_f32);
                        ui.set_num_simulations(num_simulations as i32);
                        ui.set_num_steps(num_steps as i32);
                        
                        ui.set_charts_visible(true);
        
                        println!("✓ UI updated with images and statistics");
                    }
                }).unwrap();
            });
        });
    }

    ui.run()?;
    Ok(())
}


// fn main() -> anyhow::Result<()> {
//     // A) Lấy danh sách tất cả tickers
//     let tickers = loader::load_all_tickers("data.csv")?;
//     println!("Tickers available: {:?}", tickers);

//     // B) Chọn 1 ticker người dùng muốn
//     let ticker = "AAA";

//     // C) Fetch đúng phiếu cho ticker đó
//     let records = loader::load_csv_by_ticker("data.csv", ticker)?;
//     let close_prices: Vec<f64> = records.iter().map(|r| r.close).collect();

//     // 2) Tính log-returns
//     let returns = statistic::log_returns(&close_prices);
//     let (mu, sigma) = statistic::mean_std(&returns);

//     // 3) Monte Carlo GBM
//     let paths = gbm::simulate_gbm(
//         *close_prices.last().unwrap(),
//         mu,
//         sigma,
//         1.0,
//         50,
//         1000,
//         42,
//     );

//     // 4) Lấy giá cuối + sample paths
//     let terminal_prices: Vec<f64> =
//         paths.iter().map(|p| *p.last().unwrap()).collect();

//     let sample_paths = paths.iter().take(20).cloned().collect::<Vec<_>>();

//     // 5) Summary
//     let sum = summary::summary(&terminal_prices);

//     // 6) Xuất biểu đồ
//     draw_paths("paths.png", &sample_paths)?;
//     draw_histogram("hist.png", &terminal_prices)?;

//     println!("Mean terminal price: {}", sum.mean);
//     Ok(())
// }
// // 