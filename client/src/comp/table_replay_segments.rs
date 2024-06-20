use game::{
    tet::GameReplaySegment,
};

use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn TableReplaySegments(
    all_segments: Signal<Vec<GameReplaySegment>>,
    slider: RwSignal<f64>,
) -> impl IntoView {
    let make_table = move || {
        all_segments.with(|all_segments| {
            let total_row_count = all_segments.len();
            let page_size = total_row_count.min(34);
            let pointer = slider.get() as i32;
            let init_row = (pointer - (page_size / 2) as i32).max(0) as usize;

            let rows = all_segments
                .iter()
                .enumerate()
                .skip(init_row)
                .take(page_size)
                .map(|r| GameSegmentTableRow::new(r.0, r.1.clone(), pointer as usize))
                .collect::<Vec<_>>();

            view! { <TableContent rows/> }
        })
    };
    view! { <table id="table-replay-segments">{make_table}</table> }
}

#[derive(TableRow, Clone, Debug)]
#[table(impl_vec_data_provider)]
pub struct GameSegmentTableRow {
    pub _type: String,
    pub idx: String,
    pub action: String,
    pub since_last: String,
    pub selected: String,
}

impl GameSegmentTableRow {
    pub fn new(
        row_idx: usize,
        db_row: GameReplaySegment,
        current_slider: usize,
    ) -> Self {
        let selected = if row_idx == current_slider {
            "X".to_string()
        } else {
            "".to_string()
        };
        match db_row {
            GameReplaySegment::Init(_init) => Self {
                _type: "init".to_owned(),
                idx: "".to_owned(),
                action: "".to_owned(),
                since_last: "".to_owned(),
                selected,
            },
            GameReplaySegment::Update(_update) => Self {
                _type: "update".to_owned(),
                idx: _update.idx.to_string(),
                action: format!("{:?}", _update.event.action),
                since_last: "".to_string(),
                selected,
            },
            GameReplaySegment::GameOver => Self {
                _type: "game_over".to_owned(),
                idx: "".to_owned(),
                action: "".to_owned(),
                since_last: "".to_owned(),
                selected,
            },
        }
    }
}
