import os
from rust_sgf_renderer import render_sgf as rust_render_sgf

def render(sgf_path, **kwargs):
    """
    Render an SGF file to a PNG image with a dynamically generated output filename.

    Args:
        sgf_path (str): The file path of the SGF file to extract the base filename.
        **kwargs: Optional arguments
            theme (str): "light", "dark", or "paper" (default: "dark").
            kifu (bool): Whether to show move numbers and keep all stones visible (default: False).
            move (int, optional): If provided, renders the board state after this move number.
                                         If not provided, renders the final board state.
    """
    theme = kwargs.get("theme", "dark")
    kifu = kwargs.get("kifu", False)
    move = (kwargs["move"] + 1) if "move" in kwargs else None

    # Extract filename from sgf_path
    filename = os.path.splitext(os.path.basename(sgf_path))[0]

    # Determine output filename based on rendering options
    output_path = f"{filename}_{theme}"
    if kifu:
        output_path += "_kifu"
    if move is not None:
        output_path += f"_move{move-1}"

    output_path += ".png"

    # Read SGF content
    with open(sgf_path, "r", encoding="utf-8") as file:
        sgf_content = file.read()

    # Render SGF using Rust library
    rust_render_sgf(sgf_content, output_path, theme=theme, kifu=kifu, move_number=move)

# Example usage
if __name__ == "__main__":
    sgf_file_path = "game_4.sgf"
    render(sgf_file_path, move=78, kifu=True)
