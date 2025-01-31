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
            move_number (int, optional): If provided, renders the board state after this move number.
                                       If not provided, renders the final board state.
    """
    theme = kwargs.get('theme', 'dark')
    kifu = kwargs.get('kifu', False)
    move_number = kwargs.get('move_number', None)
    rust_render_sgf(sgf_content, output_path, theme=theme, kifu=kifu, move_number=move_number)

# Example usage
if __name__ == "__main__":
    # Read the SGF content from the game file
    with open("game_2.sgf", "r", encoding="utf-8") as file:
        sgf_content = file.read()
        base_path = file.name.rsplit('.', 1)[0]

    # Render different views of the game
    # 1. Full game with move numbers (kifu)
    render(sgf_content, f"{base_path}_kifu.png", theme='dark', kifu=True)
    
    # 2. Final board state without move numbers
    render(sgf_content, f"{base_path}_final.png", theme='dark')
    
    # 3. Board state after move 20
    render(sgf_content, f"{base_path}_move20.png", theme='dark', move_number=20)
    
    # 4. Board state after move 20 with move numbers
    render(sgf_content, f"{base_path}_move20_kifu.png", theme='dark', kifu=True, move_number=20)
