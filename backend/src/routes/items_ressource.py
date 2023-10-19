from flask import Blueprint, jsonify, request
from flask_jwt_extended import current_user, jwt_required
from ..models import Game, Item
from .. import db
from sqlalchemy.exc import IntegrityError

items = Blueprint("items", __name__, url_prefix="/items")

@items.route("/", methods=["GET"])
@jwt_required()
def get_items():
    items = Item.query.all()
    return jsonify(items), 200

@items.route("/<int:item_id>", methods=["GET"])
@jwt_required()
def get_item(item_id):
    item = Item.query.filter_by(id=item_id).first()
    if not item:
        return jsonify({"error": "item not found"}), 404
    return jsonify(item), 200

@items.route("/", methods=["POST"])
@jwt_required()
def create_item_request_handler():
    name = request.json.get("name")
    description = request.json.get("description")
    price = request.json.get("price")
    item, error = create_item(name, description, price)
    if error:
        return jsonify({"error": error}), 400
    
    return jsonify(item), 201

def create_item(id, name, description, price):
    if type(id) != int:
        return None, "id missing"
    if not name:
        return None, "name missing"
    if not description:
        return None, "description missing"
    if not price:
        return None, "price missing"
    item = Item(id=id, name=name, description=description, price=price)
    db.session.add(item)
    db.session.commit()
    return item, None

@items.route("/<int:item_id>", methods=["DELETE"])
@jwt_required()
def delete_item(item_id):
    item = Item.query.filter_by(id=item_id).first()
    if not item:
        return jsonify({"error": "item not found"}), 404
    try:
        db.session.delete(item)
        db.session.commit()
        return jsonify({"message": "item deleted"}), 200
    except IntegrityError:
        return jsonify({"error": "item is used in a game"}), 400

@items.route("/<int:item_id>", methods=["PUT"])
@jwt_required()
def update_item(item_id):
    item = Item.query.filter_by(id=item_id).first()
    if not item:
        return jsonify({"error": "item not found"}), 404
    name = request.json.get("name")
    description = request.json.get("description")
    price = request.json.get("price")
    if name:
        item.name = name
    if description:
        item.description = description
    if price:
        item.price = price
    db.session.commit()
    return jsonify(item), 200