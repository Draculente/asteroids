from .user_ressource import user
from .games_ressource import games
from .items_ressource import items
from flask_swagger_ui import get_swaggerui_blueprint
import os

def register_blueprints(app, global_prefix):
    app.register_blueprint(user, url_prefix=f"{global_prefix}/user")
    app.register_blueprint(games, url_prefix=f"{global_prefix}/games")
    app.register_blueprint(items, url_prefix=f"{global_prefix}/items")
    # Swagger UI route
    SWAGGER_URL = f'{global_prefix}/swagger-ui'
    API_URL = f'{global_prefix}/swagger'
    swaggerui_blueprint = get_swaggerui_blueprint(
        SWAGGER_URL,
        API_URL,
       config={
           'app_name': "Asteroids API | Dokumentation"
       }
    )
    app.register_blueprint(swaggerui_blueprint, url_prefix=SWAGGER_URL)