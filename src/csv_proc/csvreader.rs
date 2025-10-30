use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    str::FromStr,
};

type Result<T> = std::result::Result<T, CsvError>;

#[derive(Debug, Default)]
pub struct CsvLineLen {
    pub line_num: usize,
    pub num_entries: usize,
}

#[derive(Debug)]
pub enum CsvError {
    FileNonExists,
    CouldNotOpenFile(std::io::Error),
    CouldNotParseLine(Box<dyn std::error::Error>),
    FileIsEmpty,
    CouldNotParseValue(String),
    LineTooShort(CsvLineLen),
    LineTooLong(CsvLineLen),
}

#[derive(Debug, Default)]
pub struct CsvData<T: Copy + Default + FromStr> {
    pub header: Vec<String>,
    pub data: Vec<Vec<T>>,
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for CsvError {
    fn from(value: std::io::Error) -> Self {
        Self::CouldNotOpenFile(value)
    }
}

impl Error for CsvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn read_csv<T: Copy + Default + FromStr>(filename: &str) -> Result<CsvData<T>> {
    let lines = read_to_line(filename)?;
    // Err 4 File was empty
    if lines.is_empty() {
        return Err(CsvError::FileIsEmpty);
    }

    let header: Vec<String> = lines[0].split(",").map(|s| s.into()).collect();
    let mut data: Vec<Vec<T>> = Vec::with_capacity(lines.len() - 1);

    for i in 1..lines.len() {
        let entries: Vec<Result<T>> = lines[i]
            .split(",")
            .map(|e| {
                let res = e.parse::<T>();
                res.map_err(|_| CsvError::CouldNotParseLine(e.into()))
            })
            // Err 5 Cold not parse
            .collect();
        //Err 6  Line was too Short

        let entries: Vec<T> = entries.into_iter().collect::<Result<_>>()?;
        if entries.len() == header.len() {
            // Err 7 (hidden) line was too long
            data.push(entries);
        } else if entries.len() < header.len() {
            return Err(CsvError::LineTooShort(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        } else {
            return Err(CsvError::LineTooLong(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        }
    }
    Ok(CsvData { header, data })
}

fn read_to_line(filename: &str) -> Result<Vec<String>> {
    let path = std::path::Path::new(filename);
    // err:1 file could not exist
    if !path.exists() {
        return Err(CsvError::FileNonExists);
    }
    let file = OpenOptions::new().read(true).open(path)?;
    let lines = BufReader::new(file).lines();
    // Err 3 line con not be paresed
    lines
        .into_iter()
        .map(|line| line.map_err(|err| CsvError::CouldNotParseLine(Box::new(err))))
        .collect()
}
