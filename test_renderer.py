from rust_sgf_renderer import render_sgf as rust_render_sgf

def render(sgf_content, output_path, **kwargs):
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
    rust_render_sgf(sgf_content, output_path, theme=theme, kifu=kifu)

# Read the SGF content from the game file
with open("game_4.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()
    output_path = f"{file.name}_render.png"

# Render the SGF to PNG files with different themes
render(sgf_content, output_path, theme="dark", kifu=True)
