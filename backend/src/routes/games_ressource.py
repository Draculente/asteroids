from flask import Blueprint, jsonify, request
from flask_jwt_extended import current_user, jwt_required
from ..models import Game, Item, ItemLevel
from .. import db
from .items_ressource import create_item

games = Blueprint("games", __name__, url_prefix="/games")

@games.route("/", methods=["GET"])
@jwt_required()
def get_games():
    # If query param "latest" is set to true, return only the latest game
    if request.args.get("latest") == "true":
        game = Game.query.filter_by(user_id=current_user.id).order_by(Game.id.desc()).first()
        return jsonify([game]), 200
    else: 
        games = Game.query.filter_by(user_id=current_user.id).all()
        return jsonify(games), 200

@games.route("/<int:game_id>", methods=["GET"])
@jwt_required()
def get_game(game_id):
    game = Game.query.filter_by(id=game_id, user_id=current_user.id).first()
    if not game:
        return jsonify({"error": "game not found"}), 404
    return jsonify(game), 200

@games.route("/", methods=["POST"])
@jwt_required()
def create_game():
    score = request.json.get("score")
    if not score and score != 0:
        return jsonify({"error": "score missing"}), 400
    coins = request.json.get("coins")
    if not coins and coins != 0:
        return jsonify({"error": "coins missing"}), 400
    lives = request.json.get("lives")
    if not lives and lives != 0:
        return jsonify({"error": "lives missing"}), 400
    ended = request.json.get("ended")
    if type(ended) != bool:
        return jsonify({"error": "ended missing"}), 400
    enemy_spawn_timeout = request.json.get("enemy_spawn_timeout")
    if not enemy_spawn_timeout and enemy_spawn_timeout != 0:
        return jsonify({"error": "enemy_spawn_timeout missing"}), 400
    

    items = request.json.get("items")
    if not items:
        items = []

    game = Game(user_id=current_user.id, score=score, coins=coins, items=[], lives=lives, ended=ended, enemy_spawn_timeout=enemy_spawn_timeout)
    db.session.add(game)
    db.session.flush() # flush to get the id of the game (but don't commit the transaction yet)

    # Get or create items
    created_items, error = get_or_create_items(items, game.id)
    if error:
        return jsonify({"error": error}), 400
    
    game.items = created_items

    print(game.as_dict())

    db.session.commit()

    return jsonify(game), 201

@games.route("/<int:game_id>", methods=["DELETE"])
@jwt_required()
def delete_game(game_id):
    game = Game.query.filter_by(id=game_id).first()
    if not game:
        jsonify({"message": "game deleted"}), 200
    db.session.delete(game)
    db.session.commit()
    return jsonify({"message": "game deleted"}), 200

@games.route("/<int:game_id>", methods=["PUT"])
@jwt_required()
def update_game(game_id):
    game = Game.query.filter_by(id=game_id).first()
    if not game:
        return jsonify({"error": "game not found"}), 404
    
    score = request.json.get("score")
    if type(score) == float or type(score) == int: 
        game.score = score

    coins = request.json.get("coins")
    if type(coins) == int:
        game.coins = coins

    lives = request.json.get("lives")
    if type(lives) == int:
        game.lives = lives

    ended = request.json.get("ended")
    if type(ended) == bool:
        game.ended = ended

    enemy_spawn_timeout = request.json.get("enemy_spawn_timeout")
    if type(enemy_spawn_timeout) == float or type(enemy_spawn_timeout) == int:
        game.enemy_spawn_timeout = enemy_spawn_timeout
    
    items = request.json.get("items")
    if items:
        items, error = get_or_create_items(items, game.id)
        if error:
            return jsonify({"error": error}), 400
        game.items = items
    
    db.session.commit()
    return jsonify(game), 200

def get_or_create_items(item_levels, game_id=None):
    result = []
    for item_level in item_levels:
        item_level_obj, error = get_or_create_item(item_level, game_id)
        if error:
            return None, error
        result.append(item_level_obj)
    return result, None

def get_or_create_item(item_level, game_id=None):
        if not item_level.get("level") and item_level.get("level") != 0:
            return None, "Item level missing"
        
        if not item_level.get("item"):
            return None, "Item missing"
        
        print(item_level)

        item_obj = None

        if type(item_level["item"].get("id")) == int:
            item_obj = Item.query.filter_by(id=item_level["item"]["id"]).first()

        if not item_obj:
            item_obj, error = create_item(item_level["item"].get("id"), item_level["item"].get("name"), item_level["item"].get("description"), item_level["item"].get("price"))
            if error:
                return None, "Error creating item: " + error
        # else: 
        #     item_obj = Item.query.filter_by(id=item_level["item"]["id"]).first()
        
        item_level_obj = None

        if game_id: 
            item_level_obj = ItemLevel.query.filter_by(item_id=item_obj.id, game_id=game_id).first()
        
        if not item_level_obj:
            item_level_obj = ItemLevel(item_id=item_obj.id, level=item_level["level"], game_id=game_id)
            db.session.add(item_level_obj)
        else: 
            item_level_obj.level = item_level["level"]

        item_level_obj.item = item_obj

        print("type item_level_obj", type(item_level_obj))
        return item_level_obj, None