use quick_xml::Reader;
use quick_xml::events::Event;

use super::xml_utils;
use crate::error::PptxResult;
use crate::model::{
    ChartDataLabelSettings, ChartGrouping, ChartMarkerSpec, ChartSeries, ChartSpec, ChartType,
};

#[derive(Default)]
struct SeriesBuilder {
    name: Option<String>,
    categories: Vec<String>,
    x_values: Vec<f64>,
    values: Vec<f64>,
    marker: Option<ChartMarkerSpec>,
}

pub fn parse_chart(xml: &str) -> PptxResult<Option<ChartSpec>> {
    let mut reader = Reader::from_str(xml);
    let mut in_bar_chart = false;
    let mut in_line_chart = false;
    let mut in_scatter_chart = false;
    let mut in_area_chart = false;
    let mut in_pie_chart = false;
    let mut in_doughnut_chart = false;
    let mut chart_type = ChartType::Column;
    let mut grouping = ChartGrouping::Clustered;
    let mut gap_width = None;
    let mut overlap = None;
    let mut hole_size = None;
    let mut current_series: Option<SeriesBuilder> = None;
    let mut series = Vec::new();
    let mut category_axis_title = String::new();
    let mut value_axis_title = String::new();
    let mut data_labels = ChartDataLabelSettings::default();
    let mut saw_dlbls = false;
    let mut in_tx = false;
    let mut in_cat = false;
    let mut in_val = false;
    let mut in_x_val = false;
    let mut in_y_val = false;
    let mut in_pt = false;
    let mut in_v = false;
    let mut in_marker = false;
    let mut in_cat_ax = false;
    let mut in_val_ax = false;
    let mut in_title = false;
    let mut in_title_text = false;
    let mut in_dlbls = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                match local.as_str() {
                    "barChart" | "bar3DChart" => in_bar_chart = true,
                    "lineChart" | "line3DChart" => {
                        in_line_chart = true;
                        chart_type = ChartType::Line;
                    }
                    "scatterChart" => {
                        in_scatter_chart = true;
                        chart_type = ChartType::Scatter;
                        grouping = ChartGrouping::Standard;
                    }
                    "areaChart" => {
                        in_area_chart = true;
                        chart_type = ChartType::Area;
                        grouping = ChartGrouping::Standard;
                    }
                    "pieChart" => {
                        in_pie_chart = true;
                        chart_type = ChartType::Pie;
                    }
                    "doughnutChart" => {
                        in_doughnut_chart = true;
                        chart_type = ChartType::Doughnut;
                    }
                    "barDir" if in_bar_chart => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            chart_type = if val == "bar" {
                                ChartType::Bar
                            } else {
                                ChartType::Column
                            };
                        }
                    }
                    "grouping" if in_bar_chart || in_line_chart || in_area_chart => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            grouping = match val.as_str() {
                                "stacked" => ChartGrouping::Stacked,
                                "percentStacked" => ChartGrouping::PercentStacked,
                                "standard" => ChartGrouping::Standard,
                                _ => ChartGrouping::Clustered,
                            };
                        }
                    }
                    "gapWidth" if in_bar_chart => {
                        gap_width = xml_utils::attr_str(e, "val")
                            .and_then(|val| val.parse::<i32>().ok())
                            .map(|val| val.clamp(0, 500));
                    }
                    "overlap" if in_bar_chart => {
                        overlap = xml_utils::attr_str(e, "val")
                            .and_then(|val| val.parse::<i32>().ok())
                            .map(|val| val.clamp(-100, 100));
                    }
                    "holeSize" if in_doughnut_chart => {
                        hole_size = xml_utils::attr_str(e, "val")
                            .and_then(|val| val.parse::<i32>().ok())
                            .map(|val| val.clamp(10, 90));
                    }
                    "dLbls" if in_pie_chart || in_doughnut_chart => {
                        in_dlbls = true;
                        saw_dlbls = true;
                    }
                    "showVal" if in_dlbls => {
                        data_labels.show_value = xml_utils::attr_str(e, "val")
                            .map(|val| val != "0")
                            .unwrap_or(true);
                    }
                    "showCatName" if in_dlbls => {
                        data_labels.show_category_name = xml_utils::attr_str(e, "val")
                            .map(|val| val != "0")
                            .unwrap_or(true);
                    }
                    "catAx" => in_cat_ax = true,
                    "valAx" => in_val_ax = true,
                    "title" if in_cat_ax || in_val_ax => in_title = true,
                    "t" if in_title => in_title_text = true,
                    "ser" if in_bar_chart || in_line_chart || in_scatter_chart || in_area_chart || in_pie_chart || in_doughnut_chart => {
                        current_series = Some(SeriesBuilder::default())
                    }
                    "marker" if current_series.is_some() && (in_line_chart || in_scatter_chart) => in_marker = true,
                    "symbol" if current_series.is_some() && in_marker => {
                        if let Some(symbol) = xml_utils::attr_str(e, "val")
                            && let Some(series_builder) = current_series.as_mut()
                        {
                            let marker = series_builder
                                .marker
                                .get_or_insert_with(ChartMarkerSpec::default);
                            marker.symbol = Some(symbol);
                        }
                    }
                    "size" if current_series.is_some() && in_marker => {
                        if let Some(size) = xml_utils::attr_str(e, "val")
                            .and_then(|val| val.parse::<i32>().ok())
                            .map(|val| val.clamp(2, 72))
                            && let Some(series_builder) = current_series.as_mut()
                        {
                            let marker = series_builder
                                .marker
                                .get_or_insert_with(ChartMarkerSpec::default);
                            marker.size = Some(size);
                        }
                    }
                    "tx" if current_series.is_some() => in_tx = true,
                    "cat" if current_series.is_some() => in_cat = true,
                    "val" if current_series.is_some() => in_val = true,
                    "xVal" if current_series.is_some() && in_scatter_chart => in_x_val = true,
                    "yVal" if current_series.is_some() && in_scatter_chart => in_y_val = true,
                    "pt" if current_series.is_some() => in_pt = true,
                    "v" if current_series.is_some() => in_v = true,
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) if current_series.is_some() && in_v => {
                let text = e.unescape().unwrap_or_default().to_string();
                if let Some(series_builder) = current_series.as_mut() {
                    if in_tx && !in_cat && !in_val {
                        if !text.trim().is_empty() {
                            series_builder.name = Some(text);
                        }
                    } else if in_pt && in_x_val && let Ok(value) = text.parse::<f64>() {
                        series_builder.x_values.push(value);
                    } else if in_pt && in_y_val && let Ok(value) = text.parse::<f64>() {
                        series_builder.values.push(value);
                    } else if in_pt && in_cat {
                        series_builder.categories.push(text);
                    } else if in_pt && in_val && let Ok(value) = text.parse::<f64>() {
                        series_builder.values.push(value);
                    }
                }
            }
            Ok(Event::Text(ref e)) if in_title_text => {
                let text = e.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
                    if in_cat_ax {
                        category_axis_title.push_str(&text);
                    } else if in_val_ax {
                        value_axis_title.push_str(&text);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                match local.as_str() {
                    "v" => in_v = false,
                    "t" => in_title_text = false,
                    "pt" => in_pt = false,
                    "tx" => in_tx = false,
                    "cat" => in_cat = false,
                    "val" => in_val = false,
                    "xVal" => in_x_val = false,
                    "yVal" => in_y_val = false,
                    "title" => in_title = false,
                    "dLbls" => in_dlbls = false,
                    "catAx" => in_cat_ax = false,
                    "valAx" => in_val_ax = false,
                    "ser" => {
                        if let Some(series_builder) = current_series.take()
                            && ((!series_builder.categories.is_empty() && !series_builder.values.is_empty())
                                || (!series_builder.x_values.is_empty() && !series_builder.values.is_empty()))
                        {
                            series.push(ChartSeries {
                                name: series_builder.name,
                                categories: series_builder.categories,
                                x_values: series_builder.x_values,
                                values: series_builder.values,
                                marker: series_builder.marker,
                            });
                        }
                    }
                    "marker" => in_marker = false,
                    "barChart" | "bar3DChart" => in_bar_chart = false,
                    "lineChart" | "line3DChart" => in_line_chart = false,
                    "scatterChart" => in_scatter_chart = false,
                    "areaChart" => in_area_chart = false,
                    "pieChart" => in_pie_chart = false,
                    "doughnutChart" => in_doughnut_chart = false,
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(crate::error::PptxError::Xml(e)),
            _ => {}
        }
    }

    if series.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ChartSpec {
            chart_type,
            grouping,
            gap_width,
            overlap,
            hole_size,
            category_axis_title: (!category_axis_title.trim().is_empty())
                .then_some(category_axis_title),
            value_axis_title: (!value_axis_title.trim().is_empty()).then_some(value_axis_title),
            data_labels: saw_dlbls.then_some(data_labels),
            series,
        }))
    }
}
