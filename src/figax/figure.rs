use crate::common::Env;
use crate::figax::axes;
use pyo3::prelude::*;

pub struct Figure<'p, T: pyo3::conversion::ToPyObject> {
    // Meant to represent matplotlib.figure.figure instance
    py: Python<'p>,
    plt: &'p PyModule,
    subplots: Subplots<'p, T>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Figure<'p, T> {
    pub fn new<'a: 'p>(env: &'a Env) -> Figure<'p, T> {
        // instantiate the figure object

        // start python instance
        let python = env.gil.python();

        // import matplotlib,pyplot
        let plot = python.import("matplotlib" /*".pyplot"*/).unwrap();

        // makes subplot
        let set_of_subplots = Subplots::initialise();
        Figure {
            py: python,
            plt: plot,
            subplots: set_of_subplots,
        }
    }

    pub fn add_subplot(mut self, new_axes: axes::Axes<'p, T>) -> Self {
        // adding subplots to figure and returning self
        self.subplots = self.subplots.add_subplot(new_axes);
        self
    }

    pub fn specify_grid_layout(&mut self, nrow: usize, ncol: usize) {
        // does what it says on the tin really
        self.subplots.specify_grid_layout(nrow, ncol);
    }

    pub fn show(self) {
        // function to go over all specified data and call the respective functions

        // call plt.figure() and associate the returned object with figure variable
        let figure = self
            .plt
            .call_method0("figure.Figure") // actaully calling plt.figure()
            .map_err(|e| {
                // reads pythons returned errors and prints them
                e.print_and_set_sys_last_vars(self.py);
            })
            .expect("Python Error"); // so the linter doesn't yell at me

        // to determine the layout if specified or not
        let layout = match self.subplots.grid_layout {
            Some(lay) => lay, // if specified then use specified layout
            None => (1, 1), // if not specified then set so that everything is super-imposed upon each other.
                            // rework so that it is more dynamic in determining an unspecified layout
        };

        for axis in &self.subplots.axes {
            // loop through each axis and plot accordingly

            // get plot type (i.e. plt.scatter(), plt.plot, etc. )
            let name = axis.identify();

            // gets arguements for
            let axis_kwargs = axis.get_kwargs(self.py);
            // position on the grid made up of grid layout and the index at which it is stored
            let position: (usize, usize, usize) = (layout.0, layout.1, axis.get_index().unwrap());

            // equivalent of calling figure.add_subplot(nrow, ncol, index)
            let ax = figure
                .call_method("add_subplot", position, Some(axis_kwargs)) // actually calling method
                .map_err(|e| {
                    // logging errors and printing
                    e.print_and_set_sys_last_vars(self.py);
                })
                .expect("Python Error"); // expects expect

            // extract the plotdata from Axis (probably can't have this in future due to inconsistent plotdata arguments and will need to combine ...)
            let plotdata = axis.get_plot_data().unwrap();

            // (... the following line together)  // convert plot data to &PyTuple so it can be passed to call_method function below
            let plot_args = plotdata.get_plotdata_pyargs(self.py);
            let plot_kwargs = plotdata.get_plotdata_pykwargs(self.py, self.plt);

            ax.call_method(name.as_str(), plot_args, Some(plot_kwargs)) // actually saying the plot type (i.e. plt.scatter(), plt.plot, etc. )
                .map_err(|e| {
                    // logging errors and printing
                    e.print_and_set_sys_last_vars(self.py);
                })
                .expect("Python Error");
        }

        self.plt // last actual call, equivalent to plt.show(), may need to change so it's fig.show() instead, where fig is instance of plt.figure()
            .call_method0("show") // calling method
            .map_err(|e| {
                // logging errors and printing
                e.print_and_set_sys_last_vars(self.py);
            })
            .expect("Python Error");
    }
}

struct Subplots<'p, T: pyo3::conversion::ToPyObject> {
    // container for Axes within a figure
    axes: Vec<axes::Axes<'p, T>>,
    grid_layout: Option<(usize, usize)>,
}

impl<'p, T: pyo3::conversion::ToPyObject> Subplots<'p, T> {
    pub fn initialise() -> Subplots<'p, T> {
        // function to make an empty Subplots object
        Subplots {
            axes: vec![],
            grid_layout: None,
        }
    }

    pub fn specify_grid_layout(&mut self, nrow: usize, ncol: usize) {
        // function to specify the layout of the figure, while mpl specifies it as a part of the axes this can lead to overlapping of the plots
        // can always change it belong to Axis and not SUbplots later
        self.grid_layout = Some((nrow, ncol));
    }

    #[allow(dead_code)]
    fn num_axes(&self) -> usize {
        // function to determine the number of plots in the figure
        self.axes.len()
    }

    pub fn add_subplot(mut self, new_axes: axes::Axes<'p, T>) -> Self {
        // function to add an axes to the subplot struct and in turn to figure
        self.axes.push(new_axes);
        self
    }
}
