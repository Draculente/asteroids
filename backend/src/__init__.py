from flask import Flask, send_from_directory
from flask_sqlalchemy import SQLAlchemy
from sqlalchemy_utils import database_exists, create_database
from flask import request, jsonify
from json import JSONEncoder
from flask_jwt_extended import JWTManager
from datetime import timedelta
import os
import json
from flask.json.provider import JSONProvider
from flask_cors import CORS
from flask_swagger import swagger

db = SQLAlchemy()


class CustomJSONProvider(JSONProvider):

    def dumps(self, obj, **kwargs):
        return json.dumps(obj, **kwargs, cls=JSON_Improved)

    def loads(self, s, **kwargs):
        return json.loads(s, **kwargs)

class JSON_Improved(JSONEncoder):
    def default(self, obj):
        from .models import Base
        if isinstance(obj, Base):
            return obj.as_dict()
        return JSONEncoder.default(self, obj)


def create_app():
    app = Flask(__name__)
    app.config["SECRET_KEY"] = os.environ.get("SECRET_KEY")
    app.config["SQLALCHEMY_DATABASE_URI"] = os.environ.get("DATABASE_URI")
    app.config["JWT_ACCESS_TOKEN_EXPIRES"] = timedelta(weeks=25)
    app.json= CustomJSONProvider(app)
    CORS(app)

    global_prefix = os.environ.get("GLOBAL_PREFIX", "/api/v1")
    

    register_flask_errorhandler(app)

    jwt = JWTManager(app)
    db.init_app(app)

    from .models import User, Item, Game, ItemLevel

    with app.app_context():
        create_database_if_not_exist(app)

    @jwt.user_identity_loader
    def user_identity_lookup(user):
        return user.id

    @jwt.user_lookup_loader
    def user_lookup_callback(_jwt_header, jwt_data):
        identity = jwt_data["sub"]
        return User.query.filter_by(id=identity).one_or_none()
    
    register_jwt_error_callbacks(jwt)

    from .routes import register_blueprints
    register_blueprints(app, global_prefix)
    
    @app.before_request
    def check_for_content_type_header():
        if request.method == "POST" and not "Content-Type" in request.headers:
            return jsonify({"error": "Content-Type header missing"}), 400
        
    swag = swagger(app)
    swag["info"]["version"] = "1.0"
    swag["info"]["title"] = "Asteroids API"
        
    @app.route(f"{global_prefix}/swagger")
    def spec():
        return send_from_directory(os.path.join(app.root_path, 'doc'), 'openapi.yml', as_attachment=False)
    
    @app.route(f"{global_prefix}/healthz")
    def healthz():
        return jsonify({"status": "ok"}), 200
        

    return app


def create_database_if_not_exist(app):
    if not database_exists(app.config["SQLALCHEMY_DATABASE_URI"]):
        print("Creating database")
        create_database(app.config["SQLALCHEMY_DATABASE_URI"])
    db.create_all()

def register_jwt_error_callbacks(jwt):
    @jwt.unauthorized_loader
    def unauthorized_callback(callback):
        return jsonify({"error": "Unauthorized"}), 401
    
    @jwt.invalid_token_loader
    def invalid_token_callback(callback):
        return jsonify({"error": "Invalid token"}), 422
    
    @jwt.expired_token_loader
    def expired_token_callback(jwt_header, jwt_data):
        return jsonify({"error": "Expired token"}), 401
    
    @jwt.needs_fresh_token_loader
    def needs_fresh_token_callback(callback):
        return jsonify({"error": "Fresh token needed"}), 401
    
    @jwt.revoked_token_loader
    def revoked_token_callback(callback):
        return jsonify({"error": "Revoked token"}), 401
    
    @jwt.user_lookup_error_loader
    def user_lookup_error_callback(jwt_header, jwt_data):
        return jsonify({"error": "Cannot find user. Do you need to logout?"}), 401

def register_flask_errorhandler(app):
    @app.errorhandler(404)
    def not_found(error):
        return jsonify({"error": "Not found"}), 404
    
    @app.errorhandler(405)
    def method_not_allowed(error):
        return jsonify({"error": "Method not allowed"}), 405
    
    @app.errorhandler(500)
    def internal_server_error(error):
        return jsonify({"error": "Internal server error"}), 500