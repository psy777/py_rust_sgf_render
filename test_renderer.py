from rust_sgf_renderer import render_sgf as rust_render_sgf

def render(sgf_content, output_path, **kwargs):
    theme = kwargs.get('theme', 'dark')
    kifu = kwargs.get('kifu', False)
    rust_render_sgf(sgf_content, output_path, theme=theme, kifu=kifu)

# Read the SGF content from the game file
with open("game_4.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()
    output_path = f"{file.name}_render.png"

# Render the SGF to PNG files with different themes
render(sgf_content, output_path)
