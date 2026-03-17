use crate::config::{RawDetection, RawIgnore, RawWordList};
use crate::tui::app::{AppState, DetectionCategory};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget};

pub fn get_words(detection: Option<&RawDetection>, category: DetectionCategory) -> Vec<&str> {
    let det = match detection {
        Some(d) => d,
        None => return vec![],
    };
    match category {
        DetectionCategory::RStems => det
            .r
            .as_ref()
            .and_then(|w| w.stems.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default(),
        DetectionCategory::RExact => det
            .r
            .as_ref()
            .and_then(|w| w.exact.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default(),
        DetectionCategory::Pg13Stems => det
            .pg13
            .as_ref()
            .and_then(|w| w.stems.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default(),
        DetectionCategory::Pg13Exact => det
            .pg13
            .as_ref()
            .and_then(|w| w.exact.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default(),
        DetectionCategory::FalsePositives => det
            .ignore
            .as_ref()
            .and_then(|i| i.false_positives.as_ref())
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default(),
    }
}

pub fn get_words_mut(
    detection: &mut Option<RawDetection>,
    category: DetectionCategory,
) -> &mut Vec<String> {
    let det = detection.get_or_insert_with(RawDetection::default);
    match category {
        DetectionCategory::RStems => {
            let wl = det.r.get_or_insert(RawWordList {
                stems: None,
                exact: None,
            });
            wl.stems.get_or_insert_with(Vec::new)
        }
        DetectionCategory::RExact => {
            let wl = det.r.get_or_insert(RawWordList {
                stems: None,
                exact: None,
            });
            wl.exact.get_or_insert_with(Vec::new)
        }
        DetectionCategory::Pg13Stems => {
            let wl = det.pg13.get_or_insert(RawWordList {
                stems: None,
                exact: None,
            });
            wl.stems.get_or_insert_with(Vec::new)
        }
        DetectionCategory::Pg13Exact => {
            let wl = det.pg13.get_or_insert(RawWordList {
                stems: None,
                exact: None,
            });
            wl.exact.get_or_insert_with(Vec::new)
        }
        DetectionCategory::FalsePositives => {
            let ig = det.ignore.get_or_insert(RawIgnore {
                false_positives: None,
            });
            ig.false_positives.get_or_insert_with(Vec::new)
        }
    }
}

fn category_color(cat: DetectionCategory) -> Color {
    match cat {
        DetectionCategory::RStems | DetectionCategory::RExact => Color::Red,
        DetectionCategory::Pg13Stems | DetectionCategory::Pg13Exact => Color::Yellow,
        DetectionCategory::FalsePositives => Color::Green,
    }
}

fn tag_bg(cat: DetectionCategory) -> Color {
    match cat {
        DetectionCategory::RStems | DetectionCategory::RExact => Color::Rgb(74, 34, 40),
        DetectionCategory::Pg13Stems | DetectionCategory::Pg13Exact => Color::Rgb(74, 61, 34),
        DetectionCategory::FalsePositives => Color::Rgb(45, 74, 34),
    }
}

pub fn render_detection(state: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Detection Rules ");
    let inner = block.inner(area);
    block.render(area, buf);

    if state.detection_state.editing {
        render_word_editor(state, inner, buf);
    } else {
        render_category_view(state, inner, buf);
    }
}

fn render_category_view(state: &AppState, area: Rect, buf: &mut Buffer) {
    let mut y = area.y;

    for (i, &cat) in DetectionCategory::ALL.iter().enumerate() {
        if y >= area.y + area.height {
            break;
        }

        let is_selected = i == state.detection_state.selected_category;
        let color = category_color(cat);
        let words = get_words(state.config.detection.as_ref(), cat);

        let label_style = if is_selected {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        let cursor = if is_selected { "▸ " } else { "  " };
        let label_line = Line::from(vec![
            Span::styled(cursor, label_style),
            Span::styled(cat.label(), label_style),
        ]);
        buf.set_line(area.x, y, &label_line, area.width);
        y += 1;

        if y >= area.y + area.height {
            break;
        }

        let bg = tag_bg(cat);
        let mut x = area.x + 2;
        for word in &words {
            let tag = format!(" {word} ");
            let len = tag.len() as u16;
            if x + len > area.x + area.width {
                y += 1;
                x = area.x + 2;
                if y >= area.y + area.height {
                    break;
                }
            }
            buf.set_string(x, y, &tag, Style::default().fg(color).bg(bg));
            x += len + 1;
        }

        if words.is_empty() {
            buf.set_string(
                area.x + 2,
                y,
                "(none)",
                Style::default().fg(Color::DarkGray),
            );
        }

        y += 2;
    }
}

fn render_word_editor(state: &AppState, area: Rect, buf: &mut Buffer) {
    let cat = DetectionCategory::ALL[state.detection_state.selected_category];
    let color = category_color(cat);
    let words = get_words(state.config.detection.as_ref(), cat);

    let title = format!(" {} — Esc=done  a=add  d=delete ", cat.label());
    let header = Line::styled(
        title,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    );
    buf.set_line(area.x, area.y, &header, area.width);

    let list_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height.saturating_sub(2),
    };

    let items: Vec<ListItem> = words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            let style = if i == state.detection_state.word_cursor {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let cursor = if i == state.detection_state.word_cursor {
                "▸ "
            } else {
                "  "
            };
            ListItem::new(Line::from(vec![
                Span::styled(cursor, style),
                Span::styled(*word, style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.detection_state.word_cursor));

    let list = List::new(items).highlight_style(Style::default().bg(Color::DarkGray));

    StatefulWidget::render(list, list_area, buf, &mut list_state);

    if state.detection_state.adding {
        let input_area = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };
        let input_line = Line::from(vec![
            Span::styled("Add: ", Style::default().fg(color)),
            Span::styled(
                &state.detection_state.text_input.text,
                Style::default().fg(Color::White),
            ),
            Span::styled("█", Style::default().fg(Color::White)),
        ]);
        buf.set_line(input_area.x, input_area.y, &input_line, input_area.width);
    }
}
