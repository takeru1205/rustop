use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
    terminal, Result,
};
use std::io::Write;

// data is vec![used, free]
pub fn display_pie_chart(
    stdout: &mut impl Write,
    y: &mut u16,
    data: &mut Vec<usize>,
    index: u16,
) -> Result<u16> {
    let (width, _) = terminal::size().unwrap();
    let half_width = (width - crate::EDGE) / 2;

    if data.iter().sum::<usize>() < 1 {
        *data = vec![0, 1];
    }

    let normalized_data = normalize_to_100(&data);
    let labels = vec!["X", "O"];

    let max_index = normalized_data
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0);

    let start_angle_offset = 270.0 - ((max_index as f32) * 3.6);

    let x_offset = (crate::EDGE + index * half_width) as f32;
    let y_offset = y.clone();
    // Draw pie chart
    for (i, (value, label)) in normalized_data.iter().zip(labels.iter()).enumerate() {
        let angle = 360.0 * (*value / 100.0);
        let start_angle = if i == 0 {
            start_angle_offset
        } else {
            start_angle_offset + normalized_data[..i].iter().sum::<f32>() * 3.6
        };

        let mut colors = vec![Color::Green, Color::DarkGrey];
        if normalized_data[0] < 40. {
            colors[0] = Color::Green;
        } else if normalized_data[0] < 80. {
            colors[0] = Color::Yellow;
        } else {
            colors[0] = Color::Red;
        }

        for a in (start_angle as usize)..((start_angle + angle) as usize) {
            let rad = (a as f32).to_radians();
            for r in 0..crate::RADIUS {
                let x_coord = r as f32 * rad.cos() + 20.0 + x_offset;
                let y_coord = (r as f32 / 2.0) * rad.sin() + 5. + y_offset as f32;
                queue!(
                    stdout,
                    cursor::MoveTo(x_coord as u16, y_coord as u16),
                    SetForegroundColor(colors[i]),
                    Print(label)
                )
                .unwrap();
            }
        }
    }

    Ok(*y)
}

fn calculate_sum(numbers: &[usize]) -> f32 {
    let sum: usize = numbers.iter().sum();
    sum as f32
}

fn normalize_to_100(values: &[usize]) -> Vec<f32> {
    // sum elements
    let sum: f32 = calculate_sum(values);

    // normalize and set sum as 100
    values.iter().map(|&x| x as f32 / sum * 100.0).collect()
}
