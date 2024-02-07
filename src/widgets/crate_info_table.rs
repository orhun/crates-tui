use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

pub struct CrateInfoTableWidget {
    // FIXME don't abbreviate this
    crate_info: crates_io_api::CrateResponse,
}

impl CrateInfoTableWidget {
    pub fn new(crate_info: crates_io_api::CrateResponse) -> Self {
        Self { crate_info }
    }
}

impl Widget for CrateInfoTableWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ci = self.crate_info.clone();

        let created_at = ci
            .crate_data
            .created_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let updated_at = ci
            .crate_data
            .updated_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let mut rows = [
            ["Name", &ci.crate_data.name],
            ["Created At", &created_at],
            ["Updated At", &updated_at],
            ["Max Version", &ci.crate_data.max_version],
        ]
        .iter()
        .map(|row| {
            let cells = row.iter().map(|cell| Cell::from(*cell));
            Row::new(cells)
        })
        .collect_vec();

        if let Some(description) = self.crate_info.crate_data.description {
            // assume description is wrapped in 75%
            let desc = textwrap::wrap(&description, (area.width as f64 * 0.75) as usize)
                .iter()
                .map(|s| Line::from(s.to_string()))
                .collect_vec();
            let height = desc.len();
            rows.push(
                Row::new(vec![
                    Cell::from("Description"),
                    Cell::from(Text::from(desc)),
                ])
                .height(height as u16),
            );
        }
        if let Some(homepage) = self.crate_info.crate_data.homepage {
            rows.push(Row::new(vec![Cell::from("Homepage"), Cell::from(homepage)]));
        }
        if let Some(repository) = self.crate_info.crate_data.repository {
            rows.push(Row::new(vec![
                Cell::from("Repository"),
                Cell::from(repository),
            ]));
        }
        if let Some(recent_downloads) = self.crate_info.crate_data.recent_downloads {
            rows.push(Row::new(vec![
                Cell::from("Recent Downloads"),
                Cell::from(recent_downloads.to_string()),
            ]));
        }
        if let Some(max_stable_version) = self.crate_info.crate_data.max_stable_version {
            rows.push(Row::new(vec![
                Cell::from("Max Stable Version"),
                Cell::from(max_stable_version),
            ]));
        }

        let widths = [Constraint::Fill(1), Constraint::Fill(4)];
        let table_widget = Table::new(rows, widths).block(Block::default().borders(Borders::ALL));
        Widget::render(table_widget, area, buf);
    }
}
