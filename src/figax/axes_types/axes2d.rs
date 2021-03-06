use crate::addition_objs::colormap::Colormap;
use crate::addition_objs::markerstyle::MarkerStyle;
use crate::figax::plots::*;
use crate::plots::*;
use numpy::Element;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct Axes2D<'py, T: pyo3::conversion::ToPyObject + Element> {
    plot_data: Option<PlotData<'py, T>>,
    title: Option<String>, // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
    xlabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
    ylabel: Option<String>, //https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
    plot_index: Option<usize>,
}

impl<'py, T: pyo3::conversion::ToPyObject + Element> Axes2D<'py, T> {
    pub fn empty() -> Axes2D<'py, T> {
        // creates empty instance of Axes
        Axes2D::<T> {
            plot_data: None,
            title: None,
            xlabel: None,
            ylabel: None,
            plot_index: None,
        }
    }

    pub fn get_kwargs(&self, py: Python<'py>) -> &PyDict {
        let new_dict = PyDict::new(py);
        // get methods not suitable as error's occur when Err variant is returned
        match &self.title {
            Some(title) => new_dict.set_item("title", title),
            None => new_dict.set_item("title", py.None()),
        }
        .map_err(|e| {
            // logging errors and printing
            e.print_and_set_sys_last_vars(py);
        })
        .expect("error in getting title");

        match &self.xlabel {
            Some(xlabel) => new_dict.set_item("xlabel", xlabel),
            None => new_dict.set_item("xlabel", py.None()),
        }
        .map_err(|e| {
            // logging errors and printing
            e.print_and_set_sys_last_vars(py);
        })
        .expect("error in getting xlabel");

        match &self.ylabel {
            Some(ylabel) => new_dict.set_item("ylabel", ylabel),
            None => new_dict.set_item("ylabel", py.None()),
        }
        .map_err(|e| {
            // logging errors and printing
            e.print_and_set_sys_last_vars(py);
        })
        .expect("Error in getting ylabel");
        new_dict
    }

    pub fn set_index(&mut self, index: usize) {
        // specifies the index of this instance of the axis
        self.plot_index = Some(index);
    }

    pub fn get_index(&self) -> Result<usize, &'static str> {
        // gets the index previously specified or errors
        match &self.plot_index {
            Some(ind) => Ok(*ind),
            None => Err("No index set, try: Axis.set_index(index: usize)"),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_title.html#matplotlib-axes-axes-set-title
        // sets title
        self.title = Some(title.to_owned());
    }

    pub fn get_title(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_title.html#matplotlib-axes-axes-get-title
        // gets title or errors
        match &self.title {
            Some(get_title) => Ok(get_title.to_string()),
            None => Err("No title set, try: Axis.set_title(title: &str)"),
        }
    }

    pub fn set_xlabel(&mut self, xlabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html#matplotlib.axes.Axes.set_xlabel
        // sets xlabel of the data
        self.xlabel = Some(xlabel.to_owned());
    }

    pub fn get_xlabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        // gets the xlabel if specified, errors if not
        match &self.xlabel {
            Some(get_xlabel) => Ok(get_xlabel.to_string()),
            None => Err("No xlabel set, try: Axis.set_xlabel(xlabel: &str)"),
        }
    }

    pub fn set_ylabel(&mut self, ylabel: &str) {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html#matplotlib.axes.Axes.set_ylabel
        // sets the ylabel of the data
        self.ylabel = Some(ylabel.to_owned());
    }

    pub fn get_ylabel(&self) -> Result<String, &'static str> {
        // https://matplotlib.org/3.2.2/api/_as_gen/matplotlib.axes.Axes.get_xlabel.html#matplotlib.axes.Axes.get_xlabel
        // gets the ylabel if specified, errors if not
        match &self.ylabel {
            Some(get_ylabel) => Ok(get_ylabel.to_string()),
            None => Err("No ylabel set, try: Axis.set_ylabel(ylabel: &str)"),
        }
    }

    pub fn get_plot_data(&self) -> Result<&PlotData<'py, T>, &'static str> {
        // gets the plotdata if specified else errors
        match &self.plot_data {
            Some(plot_data) => Ok(plot_data),
            None => Err("No plot data set, try calling desired plot type on Axis object"),
        }
    }

    pub fn scatter(mut self, x: &'py [T], y: &'py [T]) -> Self {
        // sets the plotdata to scatter plot and makes instance of scatterplot
        let scatter_plot: PlotData<'py, T> = PlotData::Scatter(Scatter::new(x, y));
        self.plot_data = Some(scatter_plot);
        self
    }

    pub fn plot(mut self, x: &'py [T], y: &'py [T]) -> Self {
        let plot: PlotData<'py, T> = PlotData::Plot(Plot::new(x, y));
        self.plot_data = Some(plot);
        self
    }

    pub fn set_xdata(&mut self, x_data: &'py [T]) {
        // sets the xdata of the plot type
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_xdata(x_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn set_ydata(&mut self, y_data: &'py [T]) {
        // sets ydata of the plot type
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_ydata(y_data),
            _ => println!("Not implimented yet."),
        }
    }

    pub fn set_marker(&mut self, marker: MarkerStyle) {
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_marker(marker),
            _ => println!("Not implimented yet"),
        }
    }

    pub fn set_cmap(&mut self, name: String, n: Option<usize>) {
        let quant = match n {
            Some(uval) => uval,
            None => 256 as usize,
        };
        let colormap = Colormap::new(name, Some(quant));
        match &mut self.plot_data {
            Some(PlotData::Scatter(scatter_plot)) => scatter_plot.set_cmap(colormap),
            _ => println!("Not implimented yet"),
        }
    }
    pub fn identify(&self) -> String {
        // getst the type of plot and returns name of method call
        match &self.plot_data {
            Some(PlotData::Scatter(_scatter_plot)) => "scatter".to_owned(),
            Some(PlotData::PlotSurface(_surface_plot)) => "plot_surface".to_owned(),
            Some(PlotData::Plot(_plot)) => "plot".to_owned(),
            None => "No known plot specified".to_owned(), // this will completely mess up the call
        }
    }
}
