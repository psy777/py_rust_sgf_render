from rust_sgf_renderer import render_sgf

def render(sgf_content, **kwargs):
    defaults = {
        "output_path": "output.png",
        "theme": "paper",
        "board_width": 19,
        "board_height": 19,
    }
    # Merge defaults with user overrides
    final = {**defaults, **kwargs}

    render_sgf(
        sgf_content,
        final["output_path"],
        final["theme"],
        final["board_width"],
        final["board_height"],
    )

# Read the SGF content from an sgf file called game_4.sgf
with open("game_4.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()

render(sgf_content)