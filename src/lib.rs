mod common;
mod figax;
mod pyplot;

// re-exports
pub use common::*;
pub use figax::*;
pub use pyplot::*;

/*
enum Marker {
    // acceptable arguments for fmt marker refer to api in PyPlot struct
    Point,
    Pixel,
    Circle,
    TriangleDown,
    TriangleUp,
    TriangleLeft,
    TriangelRight,
    TriDown,
    TriUp,
    TriLeft,
    TriRight,
    Square,
    Pentagon,
    Star,
    HexagonOne,
    HexagonTwo,
    Plus,
    X,
    Diamond,
    ThinDiamond,
    Vline,
    Hline,
}

impl Marker {
    fn get(&self) -> String {
        // getter method to assign the correct char for parsing later to pass to python
        let ans: String = match self {
            Self::Point => '.'.to_string(),
            Self::Pixel => ",".to_string(),
            Self::Circle => "o".to_string(),
            Self::TriangleDown => 'v'.to_string(),
            Self::TriangleUp => '^'.to_string(),
            Self::TriangleLeft => "<".to_string(),
            Self::TriangelRight => ">".to_string(),
            Self::TriDown => '1'.to_string(),
            Self::TriUp => '2'.to_string(),
            Self::TriLeft => "3".to_string(),
            Self::TriRight => "4".to_string(),
            Self::Square => 's'.to_string(),
            Self::Pentagon => 'p'.to_string(),
            Self::Star => "*".to_string(),
            Self::HexagonOne => "h".to_string(),
            Self::HexagonTwo => 'H'.to_string(),
            Self::Plus => '+'.to_string(),
            Self::X => "x".to_string(),
            Self::Diamond => "D".to_string(),
            Self::ThinDiamond => 'd'.to_string(),
            Self::Vline => '|'.to_string(),
            Self::Hline => "_".to_string(),
        };
        ans
    }

    fn set(fmt: String) -> Option<Marker> {
        // setter method for use in LineFMT
        let ans = match fmt.as_str() {
            "." => Some(Marker::Point),
            "," => Some(Marker::Pixel),
            "o" => Some(Marker::Circle),
            "v" => Some(Marker::TriangleDown),
            "^" => Some(Marker::TriangleUp),
            "<" => Some(Marker::TriangleLeft),
            ">" => Some(Marker::TriangelRight),
            "1" => Some(Marker::TriDown),
            "2" => Some(Marker::TriUp),
            "3" => Some(Marker::TriLeft),
            "4" => Some(Marker::TriRight),
            "s" => Some(Marker::Square),
            "p" => Some(Marker::Pentagon),
            "*" => Some(Marker::Star),
            "h" => Some(Marker::HexagonOne),
            "H" => Some(Marker::HexagonTwo),
            "+" => Some(Marker::Plus),
            "x" => Some(Marker::X),
            "D" => Some(Marker::Diamond),
            "d" => Some(Marker::ThinDiamond),
            "|" => Some(Marker::Vline),
            "_" => Some(Marker::Hline),
            _ => {
                println!("Marker provided not found. Default marker will be used");
                None
            },
        };
        ans
    }
}



enum LineStyle {
    // acceptable arguments for fmt line refer to api in PyPlot struct
    SolidLine,
    DashedLine,
    DashDotLine,
    DottedLine
}

impl LineStyle {
    fn get(&self) -> String {
        // getter method to assign the correct char for parsing later to pass to python
        let ans: String = match self {
            Self::SolidLine => '-'.to_string(),
            Self::DashedLine => "--".to_string(),
            Self::DashDotLine => "-.".to_string(),
            Self::DottedLine => ':'.to_string(),
        };
        ans
    }

    fn set(fmt: String) -> Option<LineStyle> {
        // setter method for use in LineFMT
        let ans = match fmt.as_str() {
            "-" => Some(LineStyle::SolidLine),
            "--" => Some(LineStyle::DashedLine),
            "-." => Some(LineStyle::DashDotLine),
            ":" => Some(LineStyle::DottedLine),
            _ => {
                println!("Line Style provided not found. Default line style will be used");
                None
            },
        };
        ans
    }
}

enum Colors {
    // acceptable arguments for fmt color refer to api in PyPlot struct
    B,
    G,
    R,
    C,
    M,
    Y,
    K,
    W,
}

impl Colors {
    fn get(&self) -> String {
        // getter method to assign the correct char for parsing later to pass to python
        let ans: String = match self {
            Self::B => 'b'.to_string(),
            Self::G => 'g'.to_string(),
            Self::R => 'r'.to_string(),
            Self::C => 'c'.to_string(),
            Self::M => 'm'.to_string(),
            Self::Y => 'y'.to_string(),
            Self::K => 'k'.to_string(),
            Self::W => 'w'.to_string(),
        };
        ans
    }


    fn set(fmt: String) -> Option<Colors> {
        // setter method for use in LineFMT
        let ans = match fmt.as_str() {
            "b" => Some(Colors::B),
            "g" => Some(Colors::G),
            "r" => Some(Colors::R),
            "c" => Some(Colors::C),
            "m" => Some(Colors::M),
            "y" => Some(Colors::Y),
            "k" => Some(Colors::K),
            "w" => Some(Colors::W),
            _ => {
                println!("Colour provided not found. Default colour will be used");
                None
            },
        };
        ans
    }
}

struct LineFMT{
    marker: Option<Marker>,
    linestyle: Option<LineStyle>,
    colors: Option<Colors>,
}

impl LineFMT {
    fn parse_fmt(to_parse: String) -> LineFMT {
        let length = to_parse.len();
        let mut mark: Option<Marker> = None;
        let mut linesty: Option<LineStyle> = None;
        let mut colour: Option<Colors> = None;
        // implement parsing
        LineFMT {
            marker:mark,
            linestyle:linesty,
            colors:colour,
        }
    }

    fn to_arg(&self) -> String {
        let col = match &self.colors {
            Some(i) => i.get(),
            None => ' '.to_string(),
        };
        let line = match &self.linestyle {
            Some(i) => i.get(),
            None => ' '.to_string(),
        };
        let mark = match &self.marker {
            Some(i) => i.get(),
            None => ' '.to_string(),
        };
        format!("{}{}{}", mark, line, col).chars().filter(|c| !c.is_whitespace()).collect()
    }
}
*/

/*
impl<T: std::clone::Clone> PyPlot<T> {
    pub fn new(y_points: &[T], label: String) -> PyPlot<T> {
        let mut vec = Vec::new();
        vec.extend_from_slice((y_points).clone());
        let mut plots = HashMap::new();
        plots.insert(label, vec);
        // vec.extend_from_slice((y_points).clone());
        let mut commands = HashMap::new();

        PyPlot {
            y: plots,
            x: None,
            format: LineFMT {
                marker: None,
                linestyle: None,
                colors: None,
            },
            kwargs: commands,
        }
    }

    pub fn add_axis(&mut self, x_axis: &[T]) {
        let mut vec = Vec::new();
        vec.extend_from_slice((x_axis).clone());
        self.x = Some(vec);
    }

    pub fn fmt(&mut self, linespec: String) {
        self.format = LineFMT::parse_fmt(linespec);
    }



    pub fn plot(&self) -> PyResult<()>{
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut container: Vec<Vec<T>> = Vec::new();
        for val in self.y.values() {
            container.push(val.to_vec());
        }
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        let plot = plt.call1("plot", (vec![0,1,2,3,4,5,6,7,8,9,],));
        let show = plt.call0("show");
        Ok(())
    }

}
*/

// pub fn plot2d(x: Vec<f64>, y: Vec<(Vec<f64>, &str, Option<PyDict>)>) -> PyResult<()> {
//     let gil = Python::acquire_gil();
//     let py = gil.python();
//     let mpl = PyModule::import(py, "matplotlib.pyplot")?;

//     for i in y.iter() {
//         let (vec, col, sd) = i;
//         let args1 = (x[..].to_owned(), vec[..].to_owned(), *col,);

//         mpl.call("plot",args1, sd.as_ref()).map_err(|e| {
//             e.print_and_set_sys_last_vars(py);
//         }).expect("Python Error");
//     }

//     mpl.call0("show").map_err(|e| {
//         e.print_and_set_sys_last_vars(py);
//     }).expect("Python Error");

//     Ok(())
// }

// macro_rules! kwargs {

//     ($($kw:expr => $arg:expr),*) => {
//         let mut hm = HashMap::new();
//         $(hm.insert($kw, $arg);)*
//         hm
//     };
// }

// macro_rules! to_kwargs {
//     ($py:expr, $hm:expr) => {
//         let dict = vec![].to_py_dict($py);

//     };
// }
