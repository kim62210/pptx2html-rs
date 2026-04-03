use quick_xml::Reader;
use quick_xml::events::Event;

use super::xml_utils;
use crate::error::PptxResult;
use crate::model::{ChartGrouping, ChartMarkerSpec, ChartSeries, ChartSpec, ChartType};

#[derive(Default)]
struct SeriesBuilder {
    name: Option<String>,
    categories: Vec<String>,
    values: Vec<f64>,
    marker: Option<ChartMarkerSpec>,
}

pub fn parse_chart(xml: &str) -> PptxResult<Option<ChartSpec>> {
    let mut reader = Reader::from_str(xml);
    let mut in_bar_chart = false;
    let mut in_line_chart = false;
    let mut in_pie_chart = false;
    let mut chart_type = ChartType::Column;
    let mut grouping = ChartGrouping::Clustered;
    let mut gap_width = None;
    let mut overlap = None;
    let mut current_series: Option<SeriesBuilder> = None;
    let mut series = Vec::new();
    let mut in_tx = false;
    let mut in_cat = false;
    let mut in_val = false;
    let mut in_pt = false;
    let mut in_v = false;
    let mut in_marker = false;

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
                    "pieChart" => {
                        in_pie_chart = true;
                        chart_type = ChartType::Pie;
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
                    "grouping" if in_bar_chart || in_line_chart => {
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
                    "ser" if in_bar_chart || in_line_chart || in_pie_chart => {
                        current_series = Some(SeriesBuilder::default())
                    }
                    "marker" if current_series.is_some() && in_line_chart => in_marker = true,
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
                    } else if in_pt && in_cat {
                        series_builder.categories.push(text);
                    } else if in_pt && in_val && let Ok(value) = text.parse::<f64>() {
                        series_builder.values.push(value);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                match local.as_str() {
                    "v" => in_v = false,
                    "pt" => in_pt = false,
                    "tx" => in_tx = false,
                    "cat" => in_cat = false,
                    "val" => in_val = false,
                    "ser" => {
                        if let Some(series_builder) = current_series.take()
                            && !series_builder.categories.is_empty()
                            && !series_builder.values.is_empty()
                        {
                            series.push(ChartSeries {
                                name: series_builder.name,
                                categories: series_builder.categories,
                                values: series_builder.values,
                                marker: series_builder.marker,
                            });
                        }
                    }
                    "marker" => in_marker = false,
                    "barChart" | "bar3DChart" => in_bar_chart = false,
                    "lineChart" | "line3DChart" => in_line_chart = false,
                    "pieChart" => in_pie_chart = false,
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
            series,
        }))
    }
}
