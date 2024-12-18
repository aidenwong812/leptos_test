use charming::{
    component::*,
    element::*,
    element::{
        Emphasis, ItemStyle, Label, LabelLine, LabelPosition, LineStyle, LineStyleType, Sort,
        Tooltip, Trigger,
    },
    datatype::DataPointItem,
    series::*,
    Chart, WasmRenderer,
    df
};
use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to_body;
#[component]
fn App() -> impl IntoView {
    let (mon, set_mon) = signal(150);
    let (tue, set_tue) = signal(230);
    let (wed, set_wed) = signal(224);
    let (thur, set_thur) = signal(218);
    let (fri, set_fri) = signal(135);
    let (sat, set_sat) = signal(147);
    let (sun, set_sun) = signal(260);
    let (typenum, set_typenum) = signal(0);
    let action = create_action(move |_input: &()| async move {
        let chart = match typenum.get() {
            0 => {
                Chart::new()
                    .title(Title::new().text("Demo: Leptos + Charming"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(vec![
                        mon.get(),
                        tue.get(),
                        wed.get(),
                        thur.get(),
                        fri.get(),
                        sat.get(),
                        sun.get()
                    ]))
            }
            1 => {
                // Simple Bar Chart
                Chart::new()
                    .title(Title::new().text("Simple Bar Chart"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Bar::new().data(vec![
                        mon.get(),
                        tue.get(),
                        wed.get(),
                        thur.get(),
                        fri.get(),
                        sat.get(),
                        sun.get(),
                    ]))
            }
            2 => {
                // Bar Chart with Custom Style for a Data Point
                Chart::new()
                    .title(Title::new().text("Bar Chart with Highlighted Point"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Bar::new().data(vec![
                        mon.get().into(),
                        DataPointItem::new(tue.get()).item_style(ItemStyle::new().color("#a90000")),
                        wed.get().into(),
                        thur.get().into(),
                        fri.get().into(),
                        sat.get().into(),
                        sun.get().into(),
                    ]))
            }
            3 => {
                // Bar Chart with Background Style
                Chart::new()
                    .title(Title::new().text("Bar Chart with Background"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(
                        Bar::new()
                            .show_background(true)
                            .background_style(BackgroundStyle::new().color("rgba(180, 180, 180, 0.2)"))
                            .data(vec![
                                mon.get(),
                                tue.get(),
                                wed.get(),
                                thur.get(),
                                fri.get(),
                                sat.get(),
                                sun.get(),
                            ]),
                    )
            }
            4 => {Chart::new()
            .legend(Legend::new().top("bottom"))
            .series(
                Pie::new()
                    .name("Nightingale Chart")
                    .rose_type(PieRoseType::Radius)
                    .radius(vec!["50", "250"])
                    .center(vec!["50%", "50%"])
                    .item_style(ItemStyle::new().border_radius(8))
                    .data(vec![
                        (mon.get(), "Monday"),
                        (tue.get(), "Tuesday"),
                        (wed.get(), "Wednesday"),
                        (thur.get(), "Thursday"),
                        (fri.get(), "Friday"),
                        (sat.get(), "Saturday"),
                        (sun.get(), "Sunday"),
                    ])
            )} // Default empty chart
            _ => {
                Chart::new()
                .title(Title::new().text("Funnel"))
                .tooltip(
                    Tooltip::new()
                        .trigger(Trigger::Item)
                        .formatter("{a} <br/>{b} : {c}%"),
                )
                .toolbox(
                    Toolbox::new().feature(
                        Feature::new()
                            .data_view(DataView::new().read_only(false))
                            .restore(Restore::new())
                            .save_as_image(SaveAsImage::new()),
                    ),
                )
                .legend(Legend::new().data(vec!["Show", "Click", "Visit", "Inquiry", "Order"]))
                .series(
                    Funnel::new()
                        .name("Funnel")
                        .left("10%")
                        .top(60)
                        .bottom(60)
                        .width("80%")
                        .min(0)
                        .max(100)
                        .min_size("0%")
                        .max_size("100%")
                        .sort(Sort::Descending)
                        .gap(2)
                        .label(Label::new().show(true).position(LabelPosition::Inside))
                        .label_line(
                            LabelLine::new()
                                .length(10)
                                .line_style(LineStyle::new().width(1).type_(LineStyleType::Solid)),
                        )
                        .item_style(ItemStyle::new().border_color("#fff").border_width(1))
                        .emphasis(Emphasis::new().label(Label::new().font_size(20)))
                        .data(df![
                            (mon.get(), "Monday"),
                            (tue.get(), "Tuesday"),
                            (wed.get(), "Wednesday"),
                            (thur.get(), "Thursday"),
                            (fri.get(), "Friday"),
                            (sat.get(), "Saturday"),
                            (sun.get(), "Sunday"),
                        ]),
                )
            }
        };

        let renderer = WasmRenderer::new(1000, 700);
        renderer.render("chart",&chart).unwrap();
    });
        
    view! { 
        <div class="metric">
            <div class="chart-field">
                <div  id="chart"></div>
                <div class="setting-field">
                    <div class="dataset-field">
                        <input type="number"
                        on:input:target=move |ev| {
                            set_mon.set(ev.target().value().parse().unwrap());
                        }
                        prop:value=mon
                        class="num-input"
                        />
                        <p>"mon is: " {mon}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_tue.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=tue
                            class="num-input"
                        />
                        <p>"tue is: " {tue}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_wed.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=wed
                            class="num-input"
                        />
                        <p>"wed is: " {wed}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_thur.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=thur
                            class="num-input"
                        />
                        <p>"thur is: " {thur}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_fri.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=fri
                            class="num-input"
                        />
                        <p>"fri is: " {fri}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_sat.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=sat
                            class="num-input"
                        />
                        <p>"sat is: " {sat}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_sun.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=sun
                            class="num-input"
                        />
                        <p>"sun is: " {sun}</p>
                    </div>
                    <select
                        class="type-setting"
                        on:change:target=move |ev| {
                        set_typenum.set(ev.target().value().parse().unwrap());
                        }
                        prop:value=move || typenum.get().to_string()
                    >
                        <option value="0">"Line"</option>
                        <option value="1">"Bar1"</option>
                        <option value="2">"Bar2"</option>
                        <option value="3">"Bar3"</option>
                        <option value="4">"Piechart"</option>
                        <option value="5">"Funnel"</option>
                    </select>
                </div>
            </div>
            <button on:click=move |_| {action.dispatch(());} class="draw-btn">"Show chart"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}