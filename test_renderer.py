from rust_sgf_renderer import render_sgf

# Read the SGF content from an sgf file called game_4.sgf
with open("game_4.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()

# Render the SGF to a PNG file called output.png
render_sgf(sgf_content, "output.png")