from dotenv import load_dotenv
from src import create_app
import os

load_dotenv()

app = create_app()

if __name__ == "__main__":
    debug = os.environ.get("DEBUG", "False") == "True"
    if debug:
        app.run(debug=os.environ.get("DEBUG", "False") == "True", host=os.environ.get("HOST", "localhost"), port=os.environ.get("PORT", "5000"))
    else: 
        from waitress import serve 
        serve(app, host=os.environ.get("HOST", "localhost"), port=os.environ.get("PORT", "5000"))
