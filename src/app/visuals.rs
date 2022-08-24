use egui::{
    style::{WidgetVisuals, Widgets},
    Stroke, Visuals,
};

use super::utils::animate_color;

/// Return default visuals for specific mode
fn default_visuals(dark: bool) -> Visuals {
    if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    }
}

/// Get visuals depending on dark mode setting with a smooth transition
pub fn get_visuals(ctx: &egui::Context, dark: bool) -> Visuals {
    Visuals {
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: animate_color(
                    ctx,
                    "vwnbf".to_string(),
                    default_visuals(dark).widgets.noninteractive.bg_fill,
                ),
                bg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwnbs".to_string(),
                        default_visuals(dark).widgets.noninteractive.bg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.noninteractive.bg_stroke
                },
                fg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwnfs".to_string(),
                        default_visuals(dark).widgets.noninteractive.fg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.noninteractive.fg_stroke
                },
                ..default_visuals(dark).widgets.noninteractive
            },
            inactive: WidgetVisuals {
                bg_fill: animate_color(
                    ctx,
                    "vwibf".to_string(),
                    default_visuals(dark).widgets.inactive.bg_fill,
                ),
                bg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwibs".to_string(),
                        default_visuals(dark).widgets.inactive.bg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.inactive.bg_stroke
                },
                fg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwifs".to_string(),
                        default_visuals(dark).widgets.inactive.fg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.inactive.fg_stroke
                },
                ..default_visuals(dark).widgets.inactive
            },
            hovered: WidgetVisuals {
                bg_fill: animate_color(
                    ctx,
                    "vwhbf".to_string(),
                    default_visuals(dark).widgets.hovered.bg_fill,
                ),
                bg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwhbs".to_string(),
                        default_visuals(dark).widgets.hovered.bg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.hovered.bg_stroke
                },
                fg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwhfs".to_string(),
                        default_visuals(dark).widgets.hovered.fg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.hovered.fg_stroke
                },
                ..default_visuals(dark).widgets.hovered
            },
            active: WidgetVisuals {
                bg_fill: animate_color(
                    ctx,
                    "vwabf".to_string(),
                    default_visuals(dark).widgets.active.bg_fill,
                ),
                bg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwabs".to_string(),
                        default_visuals(dark).widgets.active.bg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.active.bg_stroke
                },
                fg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwafs".to_string(),
                        default_visuals(dark).widgets.active.fg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.active.fg_stroke
                },
                ..default_visuals(dark).widgets.active
            },
            open: WidgetVisuals {
                bg_fill: animate_color(
                    ctx,
                    "vwobf".to_string(),
                    default_visuals(dark).widgets.open.bg_fill,
                ),
                bg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwobs".to_string(),
                        default_visuals(dark).widgets.open.bg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.open.bg_stroke
                },
                fg_stroke: Stroke {
                    color: animate_color(
                        ctx,
                        "vwofs".to_string(),
                        default_visuals(dark).widgets.open.fg_stroke.color,
                    ),
                    ..default_visuals(dark).widgets.open.fg_stroke
                },
                ..default_visuals(dark).widgets.open
            },
        },
        hyperlink_color: animate_color(
            ctx,
            "vh".to_string(),
            default_visuals(dark).hyperlink_color,
        ),
        faint_bg_color: animate_color(ctx, "vfb".to_string(), default_visuals(dark).faint_bg_color),
        extreme_bg_color: animate_color(
            ctx,
            "veb".to_string(),
            default_visuals(dark).extreme_bg_color,
        ),
        code_bg_color: animate_color(ctx, "vcb".to_string(), default_visuals(dark).code_bg_color),
        warn_fg_color: animate_color(ctx, "vwf".to_string(), default_visuals(dark).warn_fg_color),
        error_fg_color: animate_color(ctx, "vef".to_string(), default_visuals(dark).error_fg_color),
        ..default_visuals(dark)
    }
}
