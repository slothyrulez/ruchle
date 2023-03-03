import contextlib
from importlib.resources import files as resource_files
from string import ascii_lowercase

from rich.console import Console
from rich.theme import Theme
from ruchle_rust import (
    Lang,
    get_config,
    get_random_word,
    get_words,
    refresh_page,
    show_guesses,
    game_over,
    guess_word,
)

console = Console(width=40, theme=Theme({"warning": "red on yellow"}))

NUM_LETTERS, NUM_GUESSES = get_config()


def get_dictionaries_path() -> str:
    resource = resource_files("data")
    if resource._paths:
        return resource._paths[0].as_posix()
    return ""


def main():
    words = get_words(get_dictionaries_path(), Lang.Es)
    word = get_random_word(words)
    guesses = ["_" * NUM_LETTERS] * NUM_GUESSES

    with contextlib.suppress(KeyboardInterrupt):
        for idx in range(NUM_GUESSES):
            refresh_page(console, f"Guess {idx + 1}")
            show_guesses(console, guesses, word, ascii_lowercase)

            guesses[idx] = guess_word(console, guesses[:idx], words)

            if guesses[idx] == word:
                break

    game_over(console, guesses, word, ascii_lowercase, win=word in guesses)


if __name__ == "__main__":
    main()
