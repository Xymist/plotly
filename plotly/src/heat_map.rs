//! Heat-map plot

use crate::common::{Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    z: Vec<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zhoverformat")]
    zhover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmax: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmid: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmin: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zsmooth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<Z> HeatMap<Z, f64, f64>
where Z: Serialize {
    pub fn new_z(z: Vec<Z>) -> Box<HeatMap<Z, f64, f64>> {
        Box::new(HeatMap {
            x: None,
            y: None,
            z,
            r#type: PlotType::HeatMap,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            zsmooth: None,
            connect_gaps: None,
            hover_label: None,
            hover_on_gaps: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        })
    }
}

impl<X, Y, Z> HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<HeatMap<Z, X, Y>> {
        Box::new(HeatMap {
            x: Some(x),
            y: Some(y),
            z,
            r#type: PlotType::HeatMap,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            zsmooth: None,
            connect_gaps: None,
            hover_label: None,
            hover_on_gaps: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<HeatMap<Z, X, Y>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<HeatMap<Z, X, Y>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<HeatMap<Z, X, Y>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<HeatMap<Z, X, Y>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<HeatMap<Z, X, Y>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<HeatMap<Z, X, Y>> {
        let text = private::owned_string_vector(text);
        self.text = Some(text);
        Box::new(self)
    }

    pub fn hover_text<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<HeatMap<Z, X, Y>> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<HeatMap<Z, X, Y>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<HeatMap<Z, X, Y>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<HeatMap<Z, X, Y>> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<HeatMap<Z, X, Y>> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<HeatMap<Z, X, Y>> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<HeatMap<Z, X, Y>> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<HeatMap<Z, X, Y>> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<HeatMap<Z, X, Y>> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn zauto(mut self, zauto: bool) -> Box<HeatMap<Z, X, Y>> {
        self.zauto = Some(zauto);
        Box::new(self)
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> Box<HeatMap<Z, X, Y>> {
        self.zhover_format = Some(zhover_format.to_owned());
        Box::new(self)
    }

    pub fn zmax(mut self, zmax: Z) -> Box<HeatMap<Z, X, Y>> {
        self.zmax = Some(zmax);
        Box::new(self)
    }

    pub fn zmid(mut self, zmid: Z) -> Box<HeatMap<Z, X, Y>> {
        self.zmid = Some(zmid);
        Box::new(self)
    }

    pub fn zmin(mut self, zmin: Z) -> Box<HeatMap<Z, X, Y>> {
        self.zmin = Some(zmin);
        Box::new(self)
    }

    pub fn zsmooth(mut self, zsmooth: &str) -> Box<HeatMap<Z, X, Y>> {
        self.zsmooth = Some(zsmooth.to_owned());
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<HeatMap<Z, X, Y>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<HeatMap<Z, X, Y>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> Box<HeatMap<Z, X, Y>> {
        self.hover_on_gaps = Some(hover_on_gaps);
        Box::new(self)
    }

    pub fn transpose(mut self, transpose: bool) -> Box<HeatMap<Z, X, Y>> {
        self.transpose = Some(transpose);
        Box::new(self)
    }

    pub fn x_calendar(mut self, calendar: Calendar) -> Box<HeatMap<Z, X, Y>> {
        self.x_calendar = Some(calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, calendar: Calendar) -> Box<HeatMap<Z, X, Y>> {
        self.y_calendar = Some(calendar);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
