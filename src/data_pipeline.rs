// This exercise demonstrates a simple DATA PROCESSING PIPELINE in Rust
// Concepts used:
// - Vectors and tuples
// - Ownership and borrowing
// - Filtering data
// - Transforming data
// - Aggregation (average)
// - Iteration and basic logic

pub fn run_data_pipeline() {

  // ----------------------------------------------------
  // STEP 0: Input dataset (sensor name, temperature list)
  // ----------------------------------------------------

  // Each tuple contains:
  // - &str  -> sensor name
  // - Vec<f64> -> temperature readings in Celsius
  let sensor_data: Vec<(&str, Vec<f64>)> = vec![
      ("sensor_1", vec![22.5, 23.0, 22.8, -60.0, 23.3]),
      ("sensor_2", vec![18.0, 19.5, 18.7, 20.0, 19.2]),
      ("sensor_3", vec![25.0, 24.8, 25.2, 25.1, 24.9]),
  ];

  // This will store (sensor_name, average_temperature_fahrenheit)
  let mut sensor_averages: Vec<(&str, f64)> = Vec::new();

  // ----------------------------------------------------
  // STEP 1–3: Process data for each sensor
  // ----------------------------------------------------

  for (sensor_name, readings) in &sensor_data {

      // -----------------------------
      // STEP 1: Filter invalid readings
      // -----------------------------
      // Valid temperature range: -50°C to 60°C

      let valid_readings: Vec<f64> = readings
    .iter()
    .filter_map(|&temp| {
        if temp >= -50.0 && temp <= 60.0 {
            Some(temp)
        } else {
            None
        }
    })
    .collect();

      // If no valid readings exist, skip this sensor
      if valid_readings.is_empty() {
          continue;
      }

      // -----------------------------
      // STEP 2: Convert Celsius → Fahrenheit
      // Formula: F = C * 9/5 + 32
      // -----------------------------

      let fahrenheit_readings: Vec<f64> = valid_readings
          .iter()
          .map(|&c| c * 9.0 / 5.0 + 32.0)
          .collect();

      // -----------------------------
      // STEP 3: Calculate average
      // -----------------------------

      let sum: f64 = fahrenheit_readings.iter().sum();
      let count = fahrenheit_readings.len() as f64;
      let average = sum / count;

      // Store the result
      sensor_averages.push((sensor_name, average));
  }

  // ----------------------------------------------------
  // STEP 4: Identify sensor with highest average
  // ----------------------------------------------------

  let mut highest_sensor = "";
  let mut highest_average = f64::MIN;

  for (sensor_name, avg_temp) in &sensor_averages {
      if *avg_temp > highest_average {
          highest_average = *avg_temp;
          highest_sensor = sensor_name;
      }
  }

  // ----------------------------------------------------
  // FINAL OUTPUT
  // ----------------------------------------------------

  println!("Average temperature (Fahrenheit) per sensor:");
  for (sensor_name, avg_temp) in &sensor_averages {
      println!("{} → {:.2}°F", sensor_name, avg_temp);
  }

  println!(
      "\nSensor with highest average temperature: {} ({:.2}°F)",
      highest_sensor, highest_average
  );
}