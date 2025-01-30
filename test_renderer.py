from rust_sgf_renderer import render_sgf as _render_sgf

def render_sgf(sgf_content, output_path, **kwargs):
    """
    Render an SGF file to a PNG image.
    
    Args:
        sgf_content (str): The SGF file content to render
        output_path (str): The output PNG file path
        **kwargs: Optional arguments
            theme (str): "light", "dark", or "paper" (default: "paper")
            kifu (bool): Whether to show move numbers and keep all stones visible (default: False)
    """
    theme = kwargs.get('theme', 'dark')
    kifu = kwargs.get('kifu', False)
    _render_sgf(sgf_content, output_path, theme=theme, kifu=kifu)

# Read the SGF content from the game file
with open("game_1.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()

# Render the SGF to PNG files with different themes
render_sgf(sgf_content, "yunzi_dark.png", theme="dark")
