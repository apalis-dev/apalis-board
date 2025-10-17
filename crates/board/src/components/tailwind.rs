use leptos_struct_table::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "group/table-row relative w-full outline-none after:absolute after:bottom-0 after:left-3 after:right-0 after:h-px after:bg-grid-dimmed rounded-t-lg",
            template_classes
        )
    }

    fn thead_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "text-text-dimmed",
            _ => "text-apple-500",
        };

        format!(
            "px-3 py-2.5 pb-3 align-middle text-2sm font-medium {sort_class} bg-charcoal-850 border-b border-grid-bright {template_classes}"
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, _row_index: usize, _selected: bool, template_classes: &str) -> String {
        format!(
            "{} {}",
            "border-b border-grid-dimmed transition-colors hover:bg-mint-50 dark:hover:bg-charcoal-900 focus-custom cursor-pointer",
             template_classes
        )
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("{} {}", "px-5 py-2 bg-charcoal-850", prop_class)
    }

    fn loading_cell_inner(&self, row_index: usize, _col_index: usize, prop_class: &str) -> String {
        let width = match row_index % 4 {
            0 => "w-[calc(85%-2.5rem)]",
            1 => "w-[calc(90%-2.5rem)]",
            2 => "w-[calc(75%-2.5rem)]",
            _ => "w-[calc(60%-2.5rem)]",
        };
        format!(
            "animate-pulse h-2 bg-charcoal-400 rounded-full dark:bg-charcoal-700 inline-block align-middle {width} {prop_class}"
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!(
            "px-5 py-2 text-2sm font-sans text-charcoal-200 border-b border-grid-dimmed {template_classes}"
        )
    }
}
