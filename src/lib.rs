use log::error;

use pyo3::prelude::*;
use rand::prelude::*;

use pyo3::types::PyDict;
use std::fs;
use std::io::Write;
use std::path::Path;

const NUM_LETTERS: usize = 5;
const NUM_GUESSES: usize = 6;
const BASE_FILENAME: &'static str = "words";

#[pyclass]
#[derive(Clone)]
enum Lang {
  En,
  Es,
}

impl Lang {
  fn as_str(&self) -> &'static str {
    match self {
      Lang::En => "en",
      Lang::Es => "es",
    }
  }
}

#[pyfunction]
fn get_words(folder_path: String, lang: Lang) -> Vec<String> {
  let file_name = format!("{}_{}.txt", BASE_FILENAME, lang.as_str());
  let full_path = Path::new(&folder_path).join(file_name);
  let result = fs::read_to_string(full_path);

  match result {
    Ok(r) => return r.lines().map(|l| l.into()).collect(),
    Err(e) => {
      error!("{}", e);
      return Vec::new();
    }
  }
}

#[pyfunction]
fn get_random_word(words_list: Vec<&str>) -> Option<&str> {
  let mut rng = thread_rng();
  return words_list.choose(&mut rng).copied();
}

#[pyfunction]
fn get_config() -> (usize, usize) {
  return (NUM_LETTERS, NUM_GUESSES);
}

#[pyfunction]
fn game_over(
  console: Py<PyAny>,
  guesses: Vec<String>,
  word: String,
  ascii_letters: String,
  win: Option<bool>,
) -> PyResult<()> {
  Python::with_gil(|py| -> PyResult<()> {
    let is_win = win.unwrap_or(false);
    refresh_page(console.clone(), "Game Over".to_string())?;
    show_guesses(console.clone(), guesses, word.clone(), ascii_letters)?;
    let message = match is_win {
      true => format!("\n[bold white on green]Correct, the word is {}[/]", word),
      false => format!("\n[bold white on red]Sorry, the word was {}[/]", word),
    };
    console.call_method1(py, "print", (message,))?;
    Ok(())
  })
}

#[pyfunction]
fn refresh_page(console: Py<PyAny>, headline: String) -> PyResult<()> {
  Python::with_gil(|py| -> PyResult<()> {
    console.call_method0(py, "clear")?;
    let args = (format!(
      "[bold blue]:leafy_green: {} :leafy_green:[/]\n",
      headline
    ),);
    console.call_method1(py, "rule", args)?;
    Ok(())
  })
}

#[pyfunction]
fn show_guesses(
  console: Py<PyAny>,
  guesses: Vec<String>,
  word: String,
  ascii_letters: String,
) -> PyResult<()> {
  Python::with_gil(|py| -> PyResult<()> {
    let letter_status = PyDict::new(py);
    for letter in ascii_letters.chars() {
      letter_status.set_item(letter, letter)?;
    }
    for guess in guesses.iter() {
      let mut styled_guess: Vec<String> = Vec::new();
      for (letter, correct) in guess.chars().zip(word.chars()) {
        let style = if correct == letter {
          "bold white on green"
        } else if word.contains(letter) {
          "bold white on yellow"
        } else if ascii_letters.contains(letter) {
          "white on #666666"
        } else {
          "dim"
        };
        styled_guess.push(format!("[{}]{}[/]", style, letter));
        if letter != '_' {
          letter_status.set_item(letter, format!("[{}]{}[/]", style, letter))?;
        }
      }
      let args = (styled_guess.join(""),);
      let kwargs = PyDict::new(py);
      kwargs.set_item("justify", "center")?;
      console.call_method(py, "print", args, Some(kwargs))?;
    }
    let args = (letter_status.values().extract::<Vec<String>>()?.join(""),);
    let kwargs = PyDict::new(py);
    kwargs.set_item("justify", "center")?;
    console.call_method(py, "print", args, Some(kwargs))?;
    Ok(())
  })
}

#[pyfunction]
fn guess_word(console: Py<PyAny>, guesses: Vec<String>, words: Vec<String>) -> PyResult<String> {
  let mut guess = String::new();
  print!("\nGuess word: ");
  std::io::stdout().flush()?;
  _ = std::io::stdin().read_line(&mut guess)?;
  guess = guess
    .trim_end_matches('\n')
    .to_string()
    .to_ascii_lowercase();

  if guesses.contains(&guess) {
    Python::with_gil(|py| -> PyResult<()> {
      let args = (format!("You have already used {}", guess),);
      let kwargs = PyDict::new(py);
      kwargs.set_item("style", "warning")?;
      console.call_method(py, "print", args, Some(kwargs))?;
      Ok(())
    })?;
    return guess_word(console, guesses, words);
  }

  if guess.len() != NUM_LETTERS {
    Python::with_gil(|py| -> PyResult<()> {
      let args = (format!("Your guess must be {} letters", NUM_LETTERS),);
      let kwargs = PyDict::new(py);
      kwargs.set_item("style", "warning")?;
      console.call_method(py, "print", args, Some(kwargs))?;
      Ok(())
    })?;
    return guess_word(console, guesses, words);
  }

  if !words.contains(&guess) {
    Python::with_gil(|py| -> PyResult<()> {
      let args = (format!("Your guess must be {} letters", NUM_LETTERS),);
      let kwargs = PyDict::new(py);
      kwargs.set_item("style", "warning")?;
      console.call_method(py, "print", args, Some(kwargs))?;
      Ok(())
    })?;
    return guess_word(console, guesses, words);
  }

  Ok(guess)
}

/// A Python module implemented in Rust.
#[pymodule]
fn ruchle_rust(_py: Python, m: &PyModule) -> PyResult<()> {
  pyo3_log::init();

  m.add_function(wrap_pyfunction!(get_random_word, m)?)?;
  m.add_function(wrap_pyfunction!(get_words, m)?)?;
  m.add_function(wrap_pyfunction!(get_config, m)?)?;
  m.add_function(wrap_pyfunction!(game_over, m)?)?;
  m.add_function(wrap_pyfunction!(refresh_page, m)?)?;
  m.add_function(wrap_pyfunction!(show_guesses, m)?)?;
  m.add_function(wrap_pyfunction!(guess_word, m)?)?;
  m.add_class::<Lang>()?;
  Ok(())
}
