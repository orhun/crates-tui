use ratatui::{layout::Position, prelude::*, widgets::*};

use crate::{app::Mode, command::Command, config};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SearchFilterPrompt {
    cursor_position: Option<Position>,
}

impl SearchFilterPrompt {
    pub fn cursor_position(&self) -> Option<Position> {
        self.cursor_position
    }
}

pub struct SearchFilterPromptWidget<'a> {
    mode: Mode,
    sort: crates_io_api::Sort,
    input: &'a tui_input::Input,
    vertical_margin: u16,
    horizontal_margin: u16,
}

impl<'a> SearchFilterPromptWidget<'a> {
    pub fn new(mode: Mode, sort: crates_io_api::Sort, input: &'a tui_input::Input) -> Self {
        Self {
            mode,
            sort,
            input,
            vertical_margin: 2,
            horizontal_margin: 2,
        }
    }

    fn horizontal_margin(&self) -> u16 {
        if self.mode.focused() {
            self.horizontal_margin
        } else {
            0
        }
    }

    fn vertical_margin(&self) -> u16 {
        if self.mode.focused() {
            self.vertical_margin
        } else {
            0
        }
    }

    fn input_block(&self) -> Block {
        let line = if self.mode.is_filter() {
            vec!["Filter: ".into(), "Enter".bold(), " to submit".into()]
        } else if self.mode.is_search() {
            vec!["Search: ".into(), "Enter".bold(), " to submit".into()]
        } else if self.mode.is_summary() {
            let help = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::SwitchMode(Mode::Help))
                .into_iter()
                .next()
                .unwrap_or_default();
            let open_in_browser = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::OpenCratesIOUrlInBrowser)
                .into_iter()
                .next()
                .unwrap_or_default();
            let search = config::get()
                .key_bindings
                .get_config_for_command(Mode::Common, Command::NextTab)
                .into_iter()
                .next()
                .unwrap_or_default();
            vec![
                open_in_browser.bold(),
                " to open in browser, ".into(),
                search.bold(),
                " to enter search, ".into(),
                help.bold(),
                " for help".into(),
            ]
        } else if self.mode.is_help() {
            vec!["ESC".bold(), " to return".into()]
        } else {
            let search = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::SwitchMode(Mode::Search))
                .into_iter()
                .next()
                .unwrap_or_default();
            let filter = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::SwitchMode(Mode::Filter))
                .into_iter()
                .next()
                .unwrap_or_default();
            let help = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::SwitchMode(Mode::Help))
                .into_iter()
                .next()
                .unwrap_or_default();
            vec![
                search.bold(),
                " to search, ".into(),
                filter.bold(),
                " to filter, ".into(),
                help.bold(),
                " for help".into(),
            ]
        };
        let input_block = Block::default()
            .borders(if self.mode.focused() {
                Borders::ALL
            } else {
                Borders::NONE
            })
            .title(
                block::Title::from(Line::from(line)).alignment(if self.mode.focused() {
                    Alignment::Left
                } else {
                    Alignment::Right
                }),
            )
            .fg(config::get().color.base05)
            .border_style(match self.mode {
                Mode::Search => Style::default().fg(config::get().color.base0a),
                Mode::Filter => Style::default().fg(config::get().color.base0b),
                _ => Style::default().fg(config::get().color.base06),
            });
        if self.mode.is_search() {
            let help = config::get()
                .key_bindings
                .get_config_for_command(self.mode, Command::SwitchMode(Mode::Help))
                .into_iter()
                .next()
                .unwrap_or_default();
            let toggle_sort = config::get()
                .key_bindings
                .get_config_for_command(
                    Mode::Search,
                    Command::ToggleSortBy {
                        reload: false,
                        forward: true,
                    },
                )
                .into_iter()
                .next()
                .unwrap_or_default();
            input_block
                .title(Line::from(vec![
                    toggle_sort.bold(),
                    " to toggle sort".into(),
                ]))
                .title_alignment(Alignment::Right)
                .title(
                    block::Title::from(Line::from(vec![help.bold(), " for help".into()]))
                        .position(block::Position::Bottom)
                        .alignment(Alignment::Right),
                )
        } else {
            input_block
        }
    }

    fn sort_by_info(&self) -> impl Widget {
        Paragraph::new(Line::from(vec![
            "Sort By: ".into(),
            format!("{:?}", self.sort.clone()).fg(config::get().color.base0d),
        ]))
        .right_aligned()
    }

    fn input_text(&self, width: usize) -> impl Widget + '_ {
        let scroll = self.input.cursor().saturating_sub(width.saturating_sub(4));
        let text = if self.mode.focused() {
            Line::from(vec![self.input.value().into()])
        } else if self.mode.is_summary() || self.mode.is_help() {
            Line::from(vec![])
        } else {
            Line::from(vec![
                self.input.value().into(),
                " (".into(),
                format!("{:?}", self.sort.clone()).fg(config::get().color.base0d),
                ")".into(),
            ])
        };
        Paragraph::new(text).scroll((0, scroll as u16))
    }

    fn update_cursor_state(&self, area: Rect, state: &mut SearchFilterPrompt) {
        let width = ((area.width as f64 * 0.75) as u16).saturating_sub(2);
        if self.mode.focused() {
            state.cursor_position = Some(Position::new(
                (area.x + self.horizontal_margin() + self.input.cursor() as u16).min(width),
                area.y + self.vertical_margin(),
            ));
        } else {
            state.cursor_position = None
        }
    }
}

impl StatefulWidget for SearchFilterPromptWidget<'_> {
    type State = SearchFilterPrompt;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.input_block().render(area, buf);
        let [input, meta] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Fill(0)]).areas(area);

        if self.mode.focused() {
            self.sort_by_info().render(
                meta.inner(&Margin {
                    horizontal: self.horizontal_margin(),
                    vertical: self.vertical_margin(),
                }),
                buf,
            );
        }
        self.input_text(input.width as usize).render(
            input.inner(&Margin {
                horizontal: self.horizontal_margin(),
                vertical: self.vertical_margin(),
            }),
            buf,
        );

        self.update_cursor_state(area, state);
    }
}
