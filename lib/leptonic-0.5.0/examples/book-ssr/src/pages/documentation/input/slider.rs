use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
#[allow(clippy::too_many_lines)]
#[allow(clippy::similar_names)]
pub fn PageSlider() -> impl IntoView {
    let (value1, set_value1) = create_signal(6.0);
    let (value2, set_value2) = create_signal(4.2);
    let (value3, set_value3) = create_signal(-3.0);
    let (value4, set_value4) = create_signal(0.5);
    let (value5, set_value5) = create_signal(0.5);
    let (range_a, set_range_a) = create_signal(0.5);
    let (range_b, set_range_b) = create_signal(0.75);
    let (range_a_step, set_range_a_step) = create_signal(2.0);
    let (range_b_step, set_range_b_step) = create_signal(4.0);

    view! {
        <H1>"Slider"</H1>

        <P>"Allow users to adjust a value within a specified range by sliding a handle."</P>

        <P>
            "All sliders require the "<Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>" and "<Code inline=true>"step"</Code>" properties, specifying the range of values the slider provides. "
            "Using smaller step values results in ever so slightly smoother sliders until they can be considered \"continuous\". "
            "You may exclude the "<Code inline=true>"step"</Code>" prop altogether to let the sliders use its full "<Code inline=true>"f64"</Code>" precision."
        </P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(6.0);
                view! {
                    <Slider min=0.0 max=1.0 step=0.0001
                        value=value set_value=set_value
                        value_display=move |v| format!("{v:.4}") />
                }
            "#)}
        </Code>

        <P>
            "The slider always operates with "<Code inline=true>"f64"</Code>" values and may suffer from typical IEEE-math rounding problems. "
            "We use the "<Code inline=true>"value_display"</Code>" property to specify how a selected value should be rendered."
        </P>

        <Slider min=0.0 max=1.0 step=0.0001
            value=value4 set_value=set_value4
            value_display=move |v| format!("{v:.4}") />

        <H2>"Volume slider"</H2>

        <P>"Continuous sliders are perfect when the exact value selected is of no particular interest to your user. For example, when operating a volume slider."</P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(0.5);
                view! {
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Zero>
                        <Icon icon=icondata::BsVolumeDownFill style="font-size: 2.5em;"/>
                        <Slider min=0.0 max=1.0
                            value=value set_value=set_value
                            value_display=move |v| format!("{:.0}%", v * 100.0)
                            style="width: 10em"/>
                        <Icon icon=icondata::BsVolumeUpFill style="font-size: 2.5em; margin-left: 0.25em;"/>
                    </Stack>
                }
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Zero>
            <Icon icon=icondata::BsVolumeDownFill style="font-size: 2.5em;"/>
            <Slider min=0.0 max=1.0 value=value5 set_value=set_value5 style="width: 10em"
                value_display=move |v| format!("{:.0}%", v * 100.0)/>
            <Icon icon=icondata::BsVolumeUpFill style="font-size: 2.5em; margin-left: 0.25em;"/>
        </Stack>

        <H2>"Marks"</H2>

        <P>
            "Small step values result in lesser selectable values, as only values starting from min and increased by multiples of step are selectable. "
            "To help visualize the selectable values of the slider, marks can be automatically generated."
        </P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(6.0);
                view! {
                    <Slider min=1.0 max=10.0 step=1.0
                        value=value set_value=set_value
                        marks=SliderMarks::Automatic { create_names: false }
                        value_display=move |v| format!("{v:.0}")/>
                }
            "#)}
        </Code>

        <Slider min=1.0 max=10.0 step=1.0
            value=value1 set_value=set_value1
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.0}")/>

        <P>
            "Note that marks are only helpful when dealing with sliders having a limited number of selectable values, meaning ones with small ranges and a high stepping value. "
            "Automatic mark generation is currently limited to creating 20 evenly spaced marks so that continuous sliders will not create thousands of them."
        </P>

        <P>
            "You can also specify custom marks! Custom marks will be validated. "
            "If the specified value is outside the sliders [min..max] range or the percentage is outside the [0..1] range, the mark will be excluded and a warning will be logged to the console."
        </P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(6.0);
                view! {
                    <Slider min=1.0 max=10.0 step=1.0
                        value=value set_value=set_value
                        marks=SliderMarks::Custom {
                            marks: vec![
                                SliderMark {
                                    value: SliderMarkValue::Value(5.5),
                                    name: Some("5.5".into())
                                },
                                SliderMark {
                                    value: SliderMarkValue::Value(7.0),
                                    name: Some("7".into())
                                },
                                SliderMark {
                                    value: SliderMarkValue::Percentage(0.888),
                                    name: Some("88%".into())
                                },
                                SliderMark {
                                    value: SliderMarkValue::Value(20.0),
                                    name: Some("this mark will not show up".into())
                                }
                            ]
                        }
                        value_display=move |v| format!("{v:.0}")/>
                }
            "#)}
        </Code>

        <Slider min=1.0 max=10.0 step=1.0
            value=value1 set_value=set_value1
            marks=SliderMarks::Custom {
                marks: vec![
                    SliderMark {
                        value: SliderMarkValue::Value(5.5),
                        name: Some("5.5".into())
                    },
                    SliderMark {
                        value: SliderMarkValue::Value(7.0),
                        name: Some("7".into())
                    },
                    SliderMark {
                        value: SliderMarkValue::Percentage(0.888),
                        name: Some("88%".into())
                    }
                ]
            }
            value_display=move |v| format!("{v:.0}")/>

        <H2>"Arbitrary ranges"</H2>

        <P>"Sliders can use any combination of min, max and step values."</P>

        <Slider value=value2 set_value=set_value2 min=2.0 max=8.0 step=0.4
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.1}")/>

        <P>"You can also use a positive value for the "<Code inline=true>"min"</Code>" prop, and a negative value for the "<Code inline=true>"max"</Code>" prop, resulting in a reversed axis."</P>

        <Slider value=value3 set_value=set_value3 min=9.0 max=-9.0 step=1.0
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.0}")/>

        <H2>"Range sliders"</H2>

        <P>
            "A range of values can be selected using the "<Code inline=true>"RangeSlider"</Code>" component. "
            "The component requires two values and in return provides a slider with two control knobs, allowing you to select a range of values. "
            "One knob can be dragged over the other, letting them switch places."
        </P>

        <Code>
            {indoc!(r#"
                let (value_a, set_value_a) = create_signal(0.5);
                let (value_b, set_value_b) = create_signal(0.75);
                view! {
                    <RangeSlider
                        value_a=range_a
                        value_b=range_b
                        set_value_a=set_value_a
                        set_value_b=set_value_b
                        min=0.0
                        max=1.0
                        popover=SliderPopover::Always
                        value_display=move |v| format!("{v:.4}")
                    />
                }
            "#)}
        </Code>

        <RangeSlider
            value_a=range_a
            value_b=range_b
            set_value_a=set_range_a
            set_value_b=set_range_b
            min=0.0
            max=1.0
            popover=SliderPopover::Always
            value_display=move |v| format!("{v:.4}")
        />

        <P>"Range sliders can also use marks, just like the normal slider."</P>

        <RangeSlider
            value_a=range_a_step
            value_b=range_b_step
            set_value_a=set_range_a_step
            set_value_b=set_range_b_step
            min=1.0
            max=5.0
            step=1.0
            marks=SliderMarks::Automatic { create_names: true }
            value_display=move |v| format!("{v:.0}")
        />

        <H2>"Keyboard input"</H2>

        <P>
            "Slider knobs are keyboard-interactable and can be cycled through using the "<Code inline=true>"Tab"</Code>" key. "
            "Manipulation of slider knobs using the error keys will come in a future update."
        </P>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --slider-margin
                --slider-bar-height
                --slider-bar-background-color
                --slider-bar-background-image
                --slider-range-height
                --slider-range-background-color
                --slider-range-background-image
                --slider-knob-size
                --slider-knob-border-width
                --slider-knob-border-color
                --slider-knob-border-style
                --slider-knob-background-color
                --slider-knob-halo-size
                --slider-knob-halo-size-while-dragged
                --slider-knob-halo-opacity
                --slider-knob-halo-background-color
                --slider-knob-transition-speed
                --slider-knob-box-shadow
                --slider-mark-size
                --slider-mark-color
                --slider-mark-color-in-range
                --slider-mark-title-color
                --slider-mark-title-color-in-range
            ")}
        </Code>
    }
}
