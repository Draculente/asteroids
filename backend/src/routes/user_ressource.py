from flask import Blueprint
from flask import request, jsonify
from werkzeug.security import generate_password_hash, check_password_hash
from flask_jwt_extended import create_access_token
from flask_jwt_extended import current_user
from flask_jwt_extended import jwt_required
from ..models import User
from .. import db

user = Blueprint("user", __name__, url_prefix="/user")

@user.route("/", methods=["GET"])
@jwt_required()
def get_user_info():
    return jsonify({"username": current_user.username}), 200


@user.route("/", methods=["POST"])
def create_user():
    json = request.json
    if not json:
        return jsonify({"error": "JSON body missing"}), 400
    if not "username" in json:
        return jsonify({"error": "username missing"}), 400
    if not "password" in json:
        return jsonify({"error": "password missing"}), 400
    
    # Validate username and password
    if len(json["username"]) < 3 or len(json["username"]) > 25:
        return jsonify({"error": "username must be between 3 and 25 characters"}), 400
    if len(json["password"]) < 4 or len(json["password"]) > 50:
        return jsonify({"error": "password must be between 8 and 50 characters"}), 400
    
    # Check if user already exists
    if User.query.filter_by(username=json["username"]).first():
        return jsonify({"error": "username already exists"}), 400
    user = User(username=json["username"], password=generate_password_hash(json["password"]))
    db.session.add(user)
    db.session.commit()
    return jsonify({"success": "user created"}), 201

@user.route("/", methods=["DELETE"])
@jwt_required()
def delete_user():
    db.session.delete(current_user)
    db.session.commit()
    return jsonify({"success": "user deleted"}), 200

@user.route("/login", methods=["POST"])
def login():
    json = request.json
    if not json:
        return jsonify({"error": "JSON body missing"}), 400
    if not "username" in json:
        return jsonify({"error": "username missing"}), 400
    if not "password" in json:
        return jsonify({"error": "password missing"}), 400
    user = User.query.filter_by(username=json["username"]).first()
    if not user or not check_password_hash(user.password, json["password"]):
        return jsonify({"error": "username or password wrong"}), 400
    access_token = create_access_token(identity=user)
    return jsonify(access_token=access_token), 200
